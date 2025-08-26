use tauri::{AppHandle, Emitter};
use tauri_plugin_shell::{process::CommandEvent, ShellExt};
use tokio::fs::remove_file;

use std::io::ErrorKind;

use crate::{
    config::{get_data_path, get_model_path},
    query::{
        audio::update_audio_initial_prompt, setting::get_app_settings,
        user::get_user_by_session_token,
    },
    service::wx::{TranscriptionComplete, TranscriptionProgress, WhisperXClient},
    DbState,
};

async fn remove_file_safe(path: &str) -> tokio::io::Result<()> {
    match remove_file(path).await {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(()), // File doesn't exist, that's fine
        Err(e) => Err(e),                                    // Some other error occurred
    }
}

#[tauri::command]
#[specta::specta]
pub async fn start_transcribe(
    app_handle: AppHandle,
    audio_id: String,
    model: String,
) -> Result<(), String> {
    let command = app_handle
        .shell()
        .sidecar("whip_v2")
        .expect("whip_v2 sidecar does not exist");

    // .sidecar("wpx")
    // .expect("wpx sidecar does not exist");

    let model_path = get_model_path(&app_handle).unwrap_or(format!("/"));

    let data_path = get_data_path(&app_handle).unwrap_or(format!("/data/"));

    let check_data = &format!("{}/{}/subtitle.json", data_path, audio_id);

    remove_file_safe(check_data).await.unwrap();

    let (mut rx, _child) = command
        .args([
            "--file",
            &format!("{}/{}/audio.m4a", data_path, audio_id),
            "--model",
            &model,
            "--model_path",
            &format!("{}/models", model_path),
            "--lang",
            "en",
            "--output",
            &format!("{}/{}/subtitle", data_path, audio_id),
        ])
        .spawn()
        .expect("spawn failed");

    // TODO: emit message

    let handle = tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(bytes) => {
                    if let Ok(text) = String::from_utf8(bytes) {
                        dbg!("Received from Stdout ", text);
                    }
                }
                CommandEvent::Stderr(bytes) => {
                    if let Ok(text) = String::from_utf8(bytes) {
                        dbg!("Received from Stderr ", text);
                    }
                }
                CommandEvent::Error(error) => {
                    dbg!("Error:", error);
                }
                CommandEvent::Terminated(payload) => {
                    if payload.code != Some(0) {
                        dbg!(format!("Process terminated with code: {:?}", payload.code));
                    }
                }
                _ => {
                    dbg!("Unkoown Error");
                }
            }
        }
    });

    let _ = handle.await.unwrap();
    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn start_transcribe_service(
    app_handle: AppHandle,
    audio_id: String,
    model: String,
) -> Result<(), String> {
    let model_path = get_model_path(&app_handle).unwrap_or(format!("/"));

    let data_path = get_data_path(&app_handle).unwrap_or(format!("/data/"));

    let check_data = &format!("{}/{}/subtitle.json", data_path, audio_id);

    remove_file_safe(check_data).await.unwrap();

    let client = WhisperXClient::new("http://localhost:8081");

    match client.health_check().await {
        Ok(true) => println!("✅ Service is healthy"),
        Ok(false) => println!("❌ Service is not healthy"),
        Err(e) => println!("❌ Failed to check health: {}", e),
    }

    match client
        .transcribe(
            &format!("{}/{}/audio.m4a", data_path, audio_id),
            Some(&model), // model
            Some("en"),   // language
            Some(&format!("{}/models", model_path)),
            Some(&format!("{}/{}/subtitle", data_path, audio_id)),
            None, // initial_prompt not used in this function
        )
        .await
    {
        Ok(response) => {
            println!("✅ Transcription completed!");
            println!("🗣️  Language detected: {}", response.language);
            println!("📄 Output file: {}", response.output_file);
            println!("📝 Segments ({} total):", response.segments.len());

            // Print first few segments
            for (i, segment) in response.segments.iter().take(3).enumerate() {
                println!(
                    "  {}. [{:.2}s - {:.2}s]: {}",
                    i + 1,
                    segment.start,
                    segment.end,
                    segment.text.trim()
                );
            }

            if response.segments.len() > 3 {
                println!("  ... and {} more segments", response.segments.len() - 3);
            }
        }
        Err(e) => {
            println!("❌ Transcription failed: {}", e);
        }
    }

    Ok(())
}

#[tauri::command]
#[specta::specta]
pub async fn start_transcribe_service_streaming(
    app_handle: AppHandle,
    state: tauri::State<'_, DbState>,
    token: String,
    audio_id: String,
    model: String,
    initial_prompt: String,
) -> Result<(), String> {
    let model_path = get_model_path(&app_handle).unwrap_or(format!("/"));
    let data_path = get_data_path(&app_handle).unwrap_or(format!("/data/"));
    let check_data = &format!("{}/{}/subtitle.json", data_path, audio_id);
    remove_file_safe(check_data).await.unwrap();

    let db = &state.db;

    // Get the proxy URL from app_settings table
    let proxy_url = match get_app_settings(db).await {
        Ok(settings) => settings
            .model_proxy
            .unwrap_or_else(|| "http://localhost:8081".to_string()),
        Err(_) => "http://localhost:8081".to_string(), // Fallback to default
    };

    let client = WhisperXClient::new(&proxy_url);

    // Health check
    match client.health_check().await {
        Ok(true) => println!("✅ Service is healthy"),
        Ok(false) => {
            println!("❌ Service is not healthy");
            return Err("Service is not healthy".to_string());
        }
        Err(e) => {
            println!("❌ Failed to check health: {}", e);
            return Err(format!("Failed to check health: {}", e));
        }
    }

    let user_info = get_user_by_session_token(db, &app_handle, token)
        .await
        .map_err(|e| format!("Failed to get user by session token: {}", e))?;

    if let Err(e) = update_audio_initial_prompt(
        db,
        user_info.unwrap().user_id,
        audio_id.clone(),
        Some(initial_prompt.clone()),
    )
    .await
    {
        println!("Failed to update audio initial prompt: {}", e);
    }

    let audio_id_clone = audio_id.clone();
    let app_handle_clone = app_handle.clone();

    match client
        .transcribe_streaming(
            &format!("{}/{}/audio.m4a", data_path, audio_id),
            Some(&model),
            Some("en"),
            Some(&format!("{}/models", model_path)),
            Some(&format!("{}/{}/subtitle", data_path, audio_id)),
            if initial_prompt.is_empty() {
                None
            } else {
                Some(&initial_prompt)
            },
            |status_update| {
                // Emit progress events to the frontend
                let progress_event = TranscriptionProgress {
                    audio_id: audio_id_clone.clone(),
                    status: status_update.status.clone(),
                    message: status_update.message.clone(),
                    progress: status_update.progress,
                };

                // Emit to frontend
                app_handle_clone
                    .emit("transcription-progress", &progress_event)
                    .unwrap();

                // Also log to console
                println!("📊 [{}] {}", status_update.status, status_update.message);
                if let Some(progress) = status_update.progress {
                    println!("    Progress: {:.1}%", progress * 100.0);
                }
            },
        )
        .await
    {
        Ok(response) => {
            println!("✅ Transcription completed!");
            println!("🗣️  Language detected: {}", response.language);
            println!("📄 Output file: {}", response.output_file);
            println!("📝 Segments ({} total):", response.segments.len());

            // Print first few segments
            for (i, segment) in response.segments.iter().take(3).enumerate() {
                println!(
                    "  {}. [{:.2}s - {:.2}s]: {}",
                    i + 1,
                    segment.start,
                    segment.end,
                    segment.text.trim()
                );
            }

            if response.segments.len() > 3 {
                println!("  ... and {} more segments", response.segments.len() - 3);
            }

            // Emit completion event to frontend
            let completion_event = TranscriptionComplete {
                audio_id: audio_id.clone(),
                language: response.language,
                output_file: response.output_file,
                segments_count: response.segments.len(),
            };
            app_handle
                .emit("transcription-complete", &completion_event)
                .unwrap();

            Ok(())
        }
        Err(e) => {
            println!("❌ Transcription failed: {}", e);

            // Emit error event to frontend
            let error_event = TranscriptionProgress {
                audio_id: audio_id.clone(),
                status: "error".to_string(),
                message: e.to_string(),
                progress: None,
            };
            app_handle
                .emit("transcription-error", &error_event)
                .unwrap();

            Err(format!("Transcription failed: {}", e))
        }
    }
}

#[tauri::command]
#[specta::specta]
pub async fn check_model_health(state: tauri::State<'_, DbState>) -> Result<bool, String> {
    let db = &state.db;

    // Get the proxy URL from app_settings table
    let proxy_url = match get_app_settings(db).await {
        Ok(settings) => settings
            .model_proxy
            .unwrap_or_else(|| "http://localhost:8081".to_string()),
        Err(_) => "http://localhost:8081".to_string(), // Fallback to default
    };

    let client = WhisperXClient::new(&proxy_url);
    // Health check
    match client.health_check().await {
        Ok(true) => {
            println!("✅ AI Model service is healthy");
            Ok(true)
        }
        Ok(false) => {
            println!("❌ AI Model service is not healthy");
            Ok(false)
        }
        Err(e) => {
            println!("❌ Failed to check AI Model health: {}", e);
            Err(format!("Failed to check AI Model health: {}", e))
        }
    }
}
