use crate::WsState;

#[tauri::command]
pub async fn ws_send(state: tauri::State<'_, WsState>, message: String) -> Result<(), String> {
    state.ws_client.write(message).await;

    Ok(())
}

