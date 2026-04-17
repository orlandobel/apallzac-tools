// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use simple_file_logger::{init_logger, LogLevel};

fn main() {
    if let Err(e) = init_logger("apallzac-tools", LogLevel::Info) {
        eprintln!("Failed to initialize file logger: {}", e);
    }
    apallzac_tools_lib::run()
}
