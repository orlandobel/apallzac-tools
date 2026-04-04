// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod excel_reader;

fn main() {
    apallzac_tools_lib::run()
}
