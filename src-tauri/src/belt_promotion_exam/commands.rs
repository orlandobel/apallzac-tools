use std::sync::Mutex;
use crate::app_state::AppState;
use crate::app_state::controllers::Controllers;
use crate::belt_promotion_exam::{
    belt_promotion_exam_controller::BeltPromotionExamController, candidate::Candidate,
};

#[tauri::command]
pub fn load_data_of_file(state: tauri::State<'_, Mutex<AppState>>, path: &str) -> Result<Vec<Candidate>, String> {
    let mut app = state
        .lock()
        .map_err(|e| format!("Failed to acquire app state lock: {}", e))?;

    match &mut app.0 {
        Controllers::BPEController(controller) => {
            controller.load_data().map_err(|e| e.to_string())
        }
        Controllers::None => {
            let mut controller = BeltPromotionExamController::new(path)
                .map_err(|e| e.to_string())?;
            let data = controller.load_data().map_err(|e| e.to_string())?;
            app.0 = Controllers::BPEController(controller);
            Ok(data)
        }
    }
}
