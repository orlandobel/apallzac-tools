use base64::{engine::general_purpose, Engine};
use std::{fs, path::Path};
use tauri::Manager;

/// Obtiene la ruta de la carpeta de Documentos del usuario de forma multiplataforma.
/// - Windows: C:\Users\{username}\Documents
/// - macOS: /Users/{username}/Documents
/// - Linux: /home/{username}/Documents
#[tauri::command]
pub fn get_documents_dir(app_handle: tauri::AppHandle) -> Result<String, String> {
    app_handle
        .path()
        .document_dir()
        .map_err(|e| e.to_string())
        .and_then(|path| {
            path.to_str()
                .map(|s| s.to_string())
                .ok_or_else(|| "Could not convert path to string".to_string())
        })
}

#[tauri::command]
pub fn save_file(path: String, file: String) -> Result<(), String> {
    let bytes = general_purpose::STANDARD
        .decode(&file)
        .map_err(|e| format!("Failed to decode base64: {e}"))?;

    let dest = Path::new(&path);

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create directories: {e}"))?;
    }

    fs::write(dest, &bytes).map_err(|e| format!("Failed to write file: {e}"))?;

    Ok(())
}