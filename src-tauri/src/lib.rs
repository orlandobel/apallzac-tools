use std::sync::Mutex;
use tauri_plugin_printer_wkhtml_bin;

use app_state::AppState;
use belt_promotion_exam::commands as bpe_commands;

pub mod app_state;
pub mod belt_promotion_exam;
pub mod exam_controller;
pub mod excel_reader;
pub mod system_utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState(app_state::controllers::Controllers::None);

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .manage(Mutex::new(state))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_printer_wkhtml_bin::init())
        .invoke_handler(tauri::generate_handler![
            bpe_commands::load_data_of_file,
            bpe_commands::generate_exams,
            bpe_commands::get_loaded_candidates,
            bpe_commands::get_existing_document,
            system_utils::get_documents_dir,
            system_utils::save_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
