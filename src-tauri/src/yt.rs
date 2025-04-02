use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tauri_plugin_shell::{process::CommandEvent, ShellExt};
use uuid::Uuid;

#[derive(Serialize, Clone)]
#[serde(tag = "type")] // This makes the variant name appear as "type"
enum DownloadStatus {
    Started,
    Progress { message: String },
    AlreadyDownloaded,
    Finished,
    Error { message: String },
}

// TODO: provide a abort button to frontned
// TODO: return data format

#[tauri::command(rename_all = "snake_case")]
pub async fn download_yt_sections(
    app: AppHandle,
    url: String,
    start: i32,
    end: i32,
) -> Result<String, String> {
    let already_download_error = format!("has already been downloaded");

    let uuid = Uuid::new_v4().to_string();
    app.emit("download_status", DownloadStatus::Started)
        .map_err(|e| e.to_string())?;

    let (mut rx, _child) = app
        .shell()
        .command("yt-dlp")
        .args([
            "--download-sections",
            &format!("*{}-{}", start, end),
            "-f",
            "mp4",
            "--extract-audio",
            "--audio-format",
            "mp3",
            "-k", // Keep video file after audio extraction
            "-o",
            &format!("~/.config/yt-clip/{}.%(ext)s", uuid),
            &url,
        ])
        .spawn()
        .map_err(|e| e.to_string())?;
    while let Some(event) = rx.recv().await {
        match event {
            CommandEvent::Stdout(bytes) => {
                if let Ok(text) = String::from_utf8(bytes) {
                    if text.contains(&already_download_error) {
                        app.emit("download_status", DownloadStatus::AlreadyDownloaded)
                            .map_err(|e| e.to_string())?;
                        return Err("Video has already been downloaded".to_string());
                    }
                    dbg!(&text);
                    app.emit(
                        "download_status",
                        DownloadStatus::Progress { message: text },
                    )
                    .map_err(|e| e.to_string())?;
                } else {
                    dbg!("Parsing failed");
                }
            }
            CommandEvent::Stderr(error_bytes) => {
                if let Ok(error_text) = String::from_utf8(error_bytes) {
                    if error_text.contains("ERROR:") {
                        app.emit(
                            "download_status",
                            DownloadStatus::Error {
                                message: error_text,
                            },
                        )
                    } else {
                        // Treat as progress information
                        app.emit(
                            "download_status",
                            DownloadStatus::Progress {
                                message: error_text,
                            },
                        )
                    }
                    .map_err(|e| e.to_string())?;
                }
            }
            CommandEvent::Error(error) => {
                // Handle process-level errors
                app.emit(
                    "download_status",
                    DownloadStatus::Error {
                        message: error.to_string(),
                    },
                )
                .map_err(|e| e.to_string())?;
            }
            CommandEvent::Terminated(payload) => {
                // Handle process termination
                if payload.code != Some(0) {
                    return Err(format!("Process terminated with code: {:?}", payload.code));
                }
            }
            _ => {}
        }
    }
    //
    app.emit("download_status", DownloadStatus::Finished)
        .map_err(|e| e.to_string())?;

    Ok("Download completed successfully".to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn _download_yt_sections_block(app: AppHandle, url: String) {
    let shell = app.shell();
    let output = tauri::async_runtime::block_on(async move {
        let url_arg = url.as_str();
        shell
            .command("yt-dlp")
            .args(["--download-sections", "*1:10-1:25", "-f", "mp4", url_arg])
            .output()
            .await
            .unwrap()
    });
    if output.status.success() {
        println!("Result: {:?}", String::from_utf8(output.stdout));
    } else {
        println!("Exit with code: {}", output.status.code().unwrap());
    }
}
