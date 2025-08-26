use anyhow::Result;
use futures::StreamExt;
use reqwest;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone, specta::Type)]
pub struct TranscriptionResponse {
    pub segments: Vec<Segment>,
    pub output_file: String,
    pub language: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, specta::Type)]
pub struct Segment {
    pub start: f64,
    pub end: f64,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, specta::Type)]
pub struct StatusUpdate {
    pub status: String,
    pub message: String,
    pub progress: Option<f64>,
    pub result: Option<TranscriptionResponse>,
}

// Event payload for Tauri frontend
#[derive(Debug, Serialize, Clone, specta::Type)]
pub struct TranscriptionProgress {
    pub audio_id: String,
    pub status: String,
    pub message: String,
    pub progress: Option<f64>,
}

#[derive(Debug, Serialize, Clone, specta::Type)]
pub struct TranscriptionComplete {
    pub audio_id: String,
    pub language: String,
    pub output_file: String,
    pub segments_count: usize,
}

pub struct WhisperXClient {
    base_url: String,
    client: reqwest::Client,
}

impl WhisperXClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Transcribe with streaming status updates
    pub async fn transcribe_streaming<P: AsRef<Path>>(
        &self,
        file_path: P,
        model: Option<&str>,
        lang: Option<&str>,
        model_path: Option<&str>,
        output_dir: Option<&str>,
        initial_prompt: Option<&str>,
        mut status_callback: impl FnMut(StatusUpdate),
    ) -> Result<TranscriptionResponse> {
        let file_path = file_path.as_ref();

        // Read the file
        let file_bytes = tokio::fs::read(file_path).await?;
        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("audio.wav");

        // Create multipart form
        let mut form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(file_bytes)
                .file_name(filename.to_string())
                .mime_str("audio/wav")?,
        );

        // Add optional parameters
        if let Some(model) = model {
            form = form.text("model", model.to_string());
        }
        if let Some(lang) = lang {
            form = form.text("lang", lang.to_string());
        }
        if let Some(model_path) = model_path {
            form = form.text("model_path", model_path.to_string());
        }
        if let Some(output_dir) = output_dir {
            form = form.text("output_dir", output_dir.to_string());
        }
        if let Some(initial_prompt) = initial_prompt {
            form = form.text("initial_prompt", initial_prompt.to_string());
        }

        // Send the request to streaming endpoint
        let response = self
            .client
            .post(&format!("{}/transcribe", self.base_url))
            .multipart(form)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Request failed: {}", error_text));
        }

        // Process the streaming response
        let mut stream = response.bytes_stream();
        let mut buffer = Vec::new();
        let mut final_result: Option<TranscriptionResponse> = None;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            buffer.extend_from_slice(&chunk);

            // Process complete lines
            let buffer_str = String::from_utf8_lossy(&buffer);
            let lines: Vec<&str> = buffer_str.lines().collect();

            for line in &lines[..lines.len().saturating_sub(1)] {
                if line.starts_with("data: ") {
                    let json_str = &line[6..]; // Remove "data: " prefix
                    if let Ok(status_update) = serde_json::from_str::<StatusUpdate>(json_str) {
                        // Check if this is the final result
                        if status_update.status == "complete" {
                            if let Some(result) = &status_update.result {
                                final_result = Some(TranscriptionResponse {
                                    segments: result.segments.clone(),
                                    output_file: result.output_file.clone(),
                                    language: result.language.clone(),
                                });
                            }
                        } else if status_update.status == "error" {
                            return Err(anyhow::anyhow!(
                                "Transcription error: {}",
                                status_update.message
                            ));
                        }

                        // Call the status callback
                        status_callback(status_update);
                    }
                }
            }

            // Keep the last incomplete line in buffer
            if let Some(last_line) = lines.last() {
                buffer = last_line.as_bytes().to_vec();
            } else {
                buffer.clear();
            }
        }

        final_result.ok_or_else(|| anyhow::anyhow!("No final result received"))
    }

    /// Original non-streaming transcribe method (unchanged)
    pub async fn transcribe<P: AsRef<Path>>(
        &self,
        file_path: P,
        model: Option<&str>,
        lang: Option<&str>,
        model_path: Option<&str>,
        output_dir: Option<&str>,
        initial_prompt: Option<&str>,
    ) -> Result<TranscriptionResponse> {
        let file_path = file_path.as_ref();

        // Read the file
        let file_bytes = tokio::fs::read(file_path).await?;
        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("audio.wav");

        // Create multipart form
        let mut form = reqwest::multipart::Form::new().part(
            "file",
            reqwest::multipart::Part::bytes(file_bytes)
                .file_name(filename.to_string())
                .mime_str("audio/wav")?,
        );

        // Add optional parameters
        if let Some(model) = model {
            form = form.text("model", model.to_string());
        }
        if let Some(lang) = lang {
            form = form.text("lang", lang.to_string());
        }
        if let Some(model_path) = model_path {
            form = form.text("model_path", model_path.to_string());
        }
        if let Some(output_dir) = output_dir {
            form = form.text("output_dir", output_dir.to_string());
        }
        if let Some(initial_prompt) = initial_prompt {
            form = form.text("initial_prompt", initial_prompt.to_string());
        }

        // Send the request to sync endpoint
        let response = self
            .client
            .post(&format!("{}/transcribe-sync", self.base_url))
            .multipart(form)
            .send()
            .await?;

        if response.status().is_success() {
            let transcription: TranscriptionResponse = response.json().await?;
            Ok(transcription)
        } else {
            let error_text = response.text().await?;
            Err(anyhow::anyhow!("Request failed: {}", error_text))
        }
    }

    /// Health check
    pub async fn health_check(&self) -> Result<bool> {
        let response = self
            .client
            .get(&format!("{}/health", self.base_url))
            .send()
            .await?;

        Ok(response.status().is_success())
    }
}
