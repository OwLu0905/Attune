use tauri::Manager;

pub fn get_model_path(app: &tauri::AppHandle) -> Result<String, String> {
    let model_path = app.path().app_cache_dir().map_err(|e| e.to_string())?;
    Ok(model_path.to_string_lossy().into_owned())
}

pub fn get_data_path(app: &tauri::AppHandle) -> Result<String, String> {
    let data_path = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
    Ok(data_path.to_string_lossy().into_owned())
}
