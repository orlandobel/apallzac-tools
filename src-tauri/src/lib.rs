use std::sync::Mutex;
use belt_promotion_exam::commands as bpe_commands;
use app_state::AppState;

mod belt_promotion_exam;
mod excel_reader;
mod app_state;
mod exam_controller;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState (app_state::controllers::Controllers::None);
    
    tauri::Builder::default()
        .manage(Mutex::new(state))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            bpe_commands::load_data_of_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
