use tauri::Emitter;

use super::candidate::Candidate;
use super::belts::BELTS;
use crate::exam_controller::ExamController;
use crate::excel_reader::{column_configurations::BeltPromotionConfiguration, workbook::Workbook};

pub struct BeltPromotionExamController {
    workbook: Workbook,
    col_config: BeltPromotionConfiguration,
    exam: Option<String>,
}

impl BeltPromotionExamController {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut workbook = Workbook::new(path.to_string())?;
        let mut sheet = workbook.get_sheet()?;

        let first_row = sheet.next().unwrap()?;
        let col_config = BeltPromotionConfiguration::new(&first_row)?;
        drop(sheet);

        Ok(BeltPromotionExamController {
            workbook,
            col_config,
            exam: None,
        })
    }

    pub fn load_data(&mut self) -> Result<Vec<Candidate>, Box<dyn std::error::Error>> {
        let mut candidates: Vec<Candidate> = Vec::new();
        let sheet = self.workbook.get_sheet()?;

        for row in sheet.skip(1) {
            let data = row?;
            let candidate = Candidate::new(&self.col_config, &data)?;
            candidates.push(candidate);
        }

        Ok(candidates)
    }

    pub fn generate_exams(&mut self, date: &str, handler: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        let candidates = self.load_data()?;
        let mut exam_controller = ExamController::new(date);
        
        let mut sorted_candidates = candidates.into_iter().collect::<Vec<Candidate>>();
        sorted_candidates.sort_by(|a, b| a.belt.cmp(&b.belt));

        for candidate in sorted_candidates {
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
        }
        
        let exams = exam_controller.get_exams_as_base64()?;

        handler.emit("document-created", &exams)?;
        
        self.exam = Some(exams);
        Ok(())
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
        println!("CARGO_MANIFEST_DIR: {}", manifest_dir);

        let mut full_path = PathBuf::from(manifest_dir);
        full_path.push("..");
        full_path.push("public");
        full_path.push("tests");
        full_path.push(file_name);

        let path_str = full_path
            .to_str()
            .expect("No se pudo convertir la ruta a string");

        println!("Intentando abrir archivo en: {}", path_str);

        // Verificar que el archivo existe
        assert!(
            std::path::Path::new(path_str).exists(),
            "El archivo no existe en la ruta: {}",
            path_str
        );

        path_str.to_string()
    }
}
