use serde::Serialize;

use crate::WsState;

#[derive(Serialize)]
struct MessagePayload {
    message_type: String,
    file_name: String,
    message: String,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn ws_send(
    state: tauri::State<'_, WsState>,
    message_type: String,
    file_name: String,
    message: String,
) -> Result<String, String> {
    let data = MessagePayload {
        message_type,
        file_name,
        message,
    };
    let _ = state.ws_client.send_json::<MessagePayload>(&data).await;

    Ok(format!("OK!!!"))
}
