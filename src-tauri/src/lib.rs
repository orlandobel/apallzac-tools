use belt_promotion_exam::commands as bpe_commands;

mod excel_reader;
mod belt_promotion_exam;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            bpe_commands::load_data_of_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
