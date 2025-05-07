use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_shell::{process::CommandEvent, ShellExt};

use crate::config::{get_data_path, get_model_path};

#[tauri::command(rename_all = "snake_case")]
pub async fn start_transcribe(
    app_handle: AppHandle,
    file_name: String,
    model: String,
) -> Result<(), String> {
    let command = app_handle
        .shell()
        .sidecar("whip_v2")
        .expect("whip_v2 sidecar does not exist");

    let model_path = get_model_path(&app_handle).unwrap_or(format!("/"));

    let data_path = get_data_path(&app_handle).unwrap_or(format!("/data/"));

    let (mut rx, _child) = command
        .args([
            "--file",
            &format!("{}/{}/audio.mp3", data_path, file_name),
            "--model",
            &model,
            "--model_path",
            &format!("{}/models", model_path),
            "--lang",
            "en",
            "--output",
            &format!("{}/{}/subtitle", data_path, file_name),
        ])
        .spawn()
        .expect("spawn failed");

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
