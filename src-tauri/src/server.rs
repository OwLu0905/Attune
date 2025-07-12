use serde::Serialize;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::Url;
use tauri_plugin_oauth::cancel;

use tauri_plugin_oauth::start_with_config;
use tauri_plugin_oauth::OauthConfig;

#[derive(Serialize, Clone, specta::Type)]
#[serde(tag = "type")] // This makes the variant name appear as "type"
enum OAuthState {
    Verify { code: String },
    Error { message: String },
}

#[tauri::command]
#[specta::specta]
pub async fn start_oauth_server(app_handle: AppHandle, state: String) -> Result<u16, String> {
    let close_res = r#"<html>
<head>
    <title>Authentication Complete</title>
</head>
<body>
    <p>Authentication complete...</p>

    <script>
        // Log message and close the window immediately
        console.log("Authentication complete. This window will close.");
        window.close();
        // Fallback if window.close() doesn't work (depends on browser security policies)
        setTimeout(function() {
            document.body.innerHTML = "<p>Authentication complete. You may close this window.</p>";
        }, 500);
    </script>
</body>
</html>"#;
    let config = OauthConfig {
        ports: Some(vec![8001]),          // Try port 8000 first, then 8001
        response: Some(close_res.into()), // Use default response"
    };

    let app_handle = app_handle.clone();
    start_with_config(config, move |url| {
        let parsed_url = Url::parse(&url).unwrap();
        let query_params: std::collections::HashMap<_, _> =
            parsed_url.query_pairs().into_owned().collect();

        if let Some(state_query) = query_params.get("state") {
            if *state_query != state {
                let _ = app_handle.emit(
                    "oauth_code_state",
                    OAuthState::Error {
                        message: "Invalid Request".to_string(),
                    },
                );
            }
        };

        if let Some(code_query) = query_params.get("code") {
            let _ = app_handle.emit(
                "oauth_code_state",
                OAuthState::Verify {
                    code: code_query.clone(),
                },
            );
        }
    })
    .map_err(|err| err.to_string())
}

#[tauri::command]
#[specta::specta]
pub fn stop_oauth_server(port: u16) -> Result<String, String> {
    match cancel(port) {
        Ok(()) => Ok(format!("Closed server on (port: {}) successfully", port)),
        Err(_) => Err(format!("Can't closed server on (port: {})", port)),
    }
}
