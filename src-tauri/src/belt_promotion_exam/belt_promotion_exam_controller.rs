use super::candidate::Candidate;
use crate::excel_reader::{column_configurations::BeltPromotionConfiguration, workbook::Workbook};

pub struct BeltPromotionExamController {
    workbook: Workbook,
    col_config: BeltPromotionConfiguration,
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
