use crate::belt_promotion_exam::{belt_promotion_exam_controller::BeltPromotionExamController, candidate::Candidate};


// TODO :: create a state an use in command
#[tauri::command]
pub fn load_data_of_file(path: &str) -> Result<Vec<Candidate>, String> {
    let mut controller = BeltPromotionExamController::new(path).map_err(|e| e.to_string())?;
    let data: Vec<Candidate> = controller.load_data().map_err(|e| e.to_string())?;
    
    Ok(data)
}