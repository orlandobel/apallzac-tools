use crate::app_state::controllers::Controllers;
use crate::app_state::AppState;
use crate::belt_promotion_exam::{
    belt_promotion_exam_controller::BeltPromotionExamController, candidate::Candidate,
};
use std::sync::Mutex;

#[tauri::command]
pub fn load_data_of_file(
    state: tauri::State<'_, Mutex<AppState>>,
    path: &str,
) -> Result<Vec<Candidate>, String> {
    let mut app = state
        .lock()
        .map_err(|e| format!("Failed to acquire app state lock: {}", e))?;

    if path.is_empty() {
        return Ok(Vec::new());
    }

    match &mut app.0 {
        Controllers::BPEController(controller) => {
            controller.replace_workbook(path).map_err(|e| e.to_string())?;
            controller.load_data().map_err(|e| e.to_string())
        },
        Controllers::None => {
            let mut controller =
                BeltPromotionExamController::new(path).map_err(|e| e.to_string())?;
            let data = controller.load_data().map_err(|e| e.to_string())?;
            app.0 = Controllers::BPEController(controller);
            Ok(data)
        }
    }
}

#[tauri::command]
pub async fn generate_exams(
    handler: tauri::AppHandle,
    state: tauri::State<'_, Mutex<AppState>>,
    date: &str,
) -> Result<(), String> {
    let mut app_state = state
        .lock()
        .map_err(|e| format!("Failed to acquire app state lock: {}", e))?;

    match &mut app_state.0 {
        Controllers::BPEController(controller) => controller
            .generate_exams(date, handler)
            .map_err(|e| e.to_string()),
        Controllers::None => Err("No controller found".to_string()),
    }
}

#[tauri::command]
pub async fn get_loaded_candidates(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<Candidate>, String> {
    let mut app_state = state
        .lock()
        .map_err(|e| format!("Failed to acquire app state lock: {}", e))?;

    match &mut app_state.0 {
        Controllers::BPEController(controller) => Ok(controller.get_loaded_candidates()),
        Controllers::None => Err("No controller found".to_string()),
    }
}

#[tauri::command]
pub async fn get_existing_document(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Option<String>, String> {
    let app = state
        .lock()
        .map_err(|e| format!("Failed to acquire app state lock: {}", e))
        .unwrap();

    match &app.0 {
        Controllers::BPEController(controller) => Ok(controller.get_existing_document()),
        Controllers::None => Ok(None),
    }
}
