use crate::app_state::controllers::Controllers;
use crate::app_state::AppState;
use crate::belt_promotion_exam::{
    belt_promotion_exam_controller::BeltPromotionExamController, candidate::Candidate,
};
use log::error;
use std::sync::Mutex;

#[tauri::command]
pub fn load_data_of_file(
    state: tauri::State<'_, Mutex<AppState>>,
    path: &str,
) -> Result<Vec<Candidate>, String> {
    let mut app = state
        .lock()
        .map_err(|e| {
            let err = format!("Failed to acquire app state lock: {}", e);
            println!("AppState@load_data_of_file - Failed to acquire app state lock :: {}", e);
            error!("AppState@load_data_of_file - Failed to acquire app state lock :: {}", e);
            err
        })?;

    if path.is_empty() {
        return Ok(Vec::new());
    }

    match &mut app.0 {
        Controllers::BPEController(controller) => {
            controller.replace_workbook(path).map_err(|e| {
                let err = e.to_string();
                println!("BeltPromotionExamController@load_data_of_file - Failed to replace workbook :: {}", e);
                error!("BeltPromotionExamController@load_data_of_file - Failed to replace workbook :: {}", e);
                err
            })?;
            controller.load_data().map_err(|e| {
                let err = e.to_string();
                println!("BeltPromotionExamController@load_data_of_file - Failed to load data :: {}", e);
                error!("BeltPromotionExamController@load_data_of_file - Failed to load data :: {}", e);
                err
            })
        },
        Controllers::None => {
            let mut controller =
                BeltPromotionExamController::new(path).map_err(|e| {
                    let err = e.to_string();
                    println!("BeltPromotionExamController@load_data_of_file - Failed to create controller :: {}", e);
                    error!("BeltPromotionExamController@load_data_of_file - Failed to create controller :: {}", e);
                    err
                })?;
            let data = controller.load_data().map_err(|e| {
                let err = e.to_string();
                println!("BeltPromotionExamController@load_data_of_file - Failed to load data :: {}", e);
                error!("BeltPromotionExamController@load_data_of_file - Failed to load data :: {}", e);
                err
            })?;
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
        .map_err(|e| {
            let err = format!("Failed to acquire app state lock: {}", e);
            println!("AppState@generate_exams - Failed to acquire app state lock :: {}", e);
            error!("AppState@generate_exams - Failed to acquire app state lock :: {}", e);
            err
        })?;

    match &mut app_state.0 {
        Controllers::BPEController(controller) => controller
            .generate_exams(date, handler)
            .map_err(|e| {
                let err = e.to_string();
                println!("BeltPromotionExamController@generate_exams - Failed to generate exams :: {}", e);
                error!("BeltPromotionExamController@generate_exams - Failed to generate exams :: {}", e);
                err
            }),
        Controllers::None => Err("No controller found".to_string()),
    }
}

#[tauri::command]
pub async fn get_loaded_candidates(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Vec<Candidate>, String> {
    let mut app_state = state
        .lock()
        .map_err(|e| {
            let err = format!("Failed to acquire app state lock: {}", e);
            println!("AppState@get_loaded_candidates - Failed to acquire app state lock :: {}", e);
            error!("AppState@get_loaded_candidates - Failed to acquire app state lock :: {}", e);
            err
        })?;

    match &mut app_state.0 {
        Controllers::BPEController(controller) => Ok(controller.get_loaded_candidates()),
        Controllers::None => Err("No controller found".to_string()),
    }
}

#[tauri::command]
pub async fn get_existing_document(
    state: tauri::State<'_, Mutex<AppState>>,
) -> Result<Option<String>, String> {
    let app = match state.lock() {
        Ok(app) => app,
        Err(e) => {
            let err = format!("Failed to acquire app state lock: {}", e);
            println!("AppState@get_existing_document - Failed to acquire app state lock :: {}", e);
            error!("AppState@get_existing_document - Failed to acquire app state lock :: {}", e);
            return Err(err);
        }
    };

    match &app.0 {
        Controllers::BPEController(controller) => Ok(controller.get_existing_document()),
        Controllers::None => Ok(None),
    }
}
