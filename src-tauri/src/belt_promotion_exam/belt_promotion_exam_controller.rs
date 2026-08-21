use tauri::{Emitter, Manager};

use super::belts::BELTS;
use super::candidate::Candidate;
use crate::exam_controller::ExamController;
use crate::excel_reader::{column_configurations::BeltPromotionConfiguration, workbook::Workbook};
use log::{error, info};

pub struct BeltPromotionExamController {
    workbook: Workbook,
    col_config: BeltPromotionConfiguration,
    candidates: Vec<Candidate>,
    exam: Option<String>,
}

impl BeltPromotionExamController {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut workbook = Workbook::new(path.to_string())?;
        let mut sheet = workbook.get_sheet()?;

        let first_row = match sheet.next() {
            Some(row) => row?,
            None => {
                let msg = format!("BeltPromotionExamController@new - No rows found in Excel sheet: {}", path);
                println!("{}", msg);
                error!("{}", msg);
                return Err(msg.into());
            }
        };
        let col_config = BeltPromotionConfiguration::new(&first_row)?;
        drop(sheet);

        Ok(BeltPromotionExamController {
            workbook,
            col_config,
            candidates: Vec::new(),
            exam: None,
        })
    }

    pub fn replace_workbook(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.workbook = Workbook::new(path.to_string())?;
        let first_row = match self.workbook.get_sheet()?.next() {
            Some(row) => row?,
            None => {
                let msg = format!("BeltPromotionExamController@replace_workbook - No rows found in Excel sheet during workbook replacement: {}", path);
                println!("{}", msg);
                error!("{}", msg);
                return Err(msg.into());
            }
        };
        self.col_config = BeltPromotionConfiguration::new(&first_row)?;
        self.candidates.clear();
        Ok(())
    }

    pub fn load_data(&mut self) -> Result<Vec<Candidate>, Box<dyn std::error::Error>> {
        let sheet = self.workbook.get_sheet()?;

        for row in sheet.skip(1) {
            let data = row?;
            let candidate = Candidate::new(&self.col_config, &data)?;
            self.candidates.push(candidate);
        }

        Ok(self.candidates.clone())
    }

    pub fn generate_exams(
        &mut self,
        date: &str,
        handler: tauri::AppHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let candidates = self.candidates.clone();

        // Resolver ruta de templates: usa resource_dir() en producción o CARGO_MANIFEST_DIR/templates en desarrollo
        let templates_path = handler
            .path()
            .resource_dir()?
            .join("templates")
            .join("exams")
            .to_string_lossy()
            .into_owned();
        info!("BeltPromotionExamController@generate_exams - Templates path: {}", templates_path);

        let mut exam_controller = ExamController::new(date, &templates_path);

        let mut sorted_candidates = candidates.into_iter().collect::<Vec<Candidate>>();
        sorted_candidates.sort_by(|a, b| a.belt.cmp(&b.belt));

        info!("BeltPromotionExamController@generate_exams - Total candidates: {}", sorted_candidates.len());
        handler.emit("total-candidates", sorted_candidates.len())?;
        for (index, candidate) in sorted_candidates.iter().enumerate() {
            let exam = match candidate.belt {
                BELTS::AMARILLO => "yellow.pdf",
                BELTS::NARANJA => "orange.pdf",
                BELTS::MORADO => "purple.pdf",
                BELTS::AZUL => "blue.pdf",
                BELTS::VERDE => "green.pdf",
                BELTS::CAFE => "brown.pdf",
                BELTS::CAFE1 => "brown1.pdf",
                BELTS::CAFE2 => "brown2.pdf",
                BELTS::CAFE3 => "brown3.pdf",
            };

            exam_controller.create_exam_page(&candidate, exam)?;
            info!("BeltPromotionExamController@generate_exams - Created {} exams", index);
            handler.emit("current-progress", ())?;
        }

        let exams = exam_controller.get_exams_as_base64()?;

        handler.emit("document-created", &exams)?;

        self.exam = Some(exams);
        Ok(())
    }

    pub fn get_loaded_candidates(&self) -> Vec<Candidate> {
        self.candidates.clone()
    }

    pub fn get_existing_document(&self) -> Option<String> {
        self.exam.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_create_belt_promotion_exam_controller() {
        let file_path = get_file_path("examen ejemplo 1.xlsx");
        let controller = BeltPromotionExamController::new(&file_path);
        assert!(controller.is_ok());
    }

    #[test]
    fn test_load_data() {
        let file_path = get_file_path("examen ejemplo 1.xlsx");
        let mut controller = BeltPromotionExamController::new(&file_path).unwrap();

        let result = controller.load_data();
        assert!(result.is_ok());

        let candidates = result.unwrap();
        assert!(!candidates.is_empty());
    }

    /* This function is intended to get the file path for the test Excel file, is not a test it self */
    fn get_file_path(file_name: &str) -> String {
        // Build path to Excel located on public/tests
        // CARGO_MANIFEST_DIR es: $HOME\apallzac-tools\src-tauri

        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        println!("BeltPromotionExamController@get_exam_as_base64 - CARGO_MANIFEST_DIR: {}", manifest_dir);

        let mut full_path = PathBuf::from(manifest_dir);
        full_path.push("..");
        full_path.push("public");
        full_path.push("tests");
        full_path.push(file_name);

        let path_str = match full_path.to_str() {
            Some(s) => s,
            None => {
                error!("BeltPromotionExamController@get_exam_as_base64 - No se pudo convertir la ruta a string :: {:?}", full_path);
                panic!("No se pudo convertir la ruta a string");
            }
        };

        println!("BeltPromotionExamController@get_exam_as_base64 - Intentando abrir archivo en: {}", path_str);

        // Verificar que el archivo existe
        assert!(
            std::path::Path::new(path_str).exists(),
            "El archivo no existe en la ruta: {}",
            path_str
        );

        path_str.to_string()
    }
}
