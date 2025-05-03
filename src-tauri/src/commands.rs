use crate::AppState;

// remember to call `.manage(MyState::default())`
#[tauri::command]
pub async fn ws_send(state: tauri::State<'_, AppState>, message:String) -> Result<(),String>{
    state.ws_client.write(message).await;
    
    Ok(())
}