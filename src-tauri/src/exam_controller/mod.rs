use std::collections::HashMap;
use acroform::{AcroFormDocument, FieldValue};

use crate::belt_promotion_exam::{
    candidate::Candidate,
    belts::BELTS
};

pub struct ExamController {
    date: String,
    base_path: String

}

impl ExamController {
    pub fn new() -> Self{
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let base_path = format!("{}/templates", manifest_dir);

        ExamController { 
            date: "19/04/2026".to_string(),
            base_path
        }
    }

    pub fn create_sheet(&self, candidate: &Candidate, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let template_path = format!("{}/{}", self.base_path, file);
        let output_path = format!("{}/output.pdf", self.base_path);
        println!("Creating sheet from candidate: {:?}", candidate);

        let mut template = AcroFormDocument::from_pdf(template_path).expect("error opening file");

        let fields = template.fields()
            .map_err(|e| { 
                println!("Error getting fields: {:?}", e.to_string());
                Box::new(e) as Box<dyn std::error::Error> 
            })?;

        let mut values: HashMap<String, FieldValue> = HashMap::new();
        values.insert("date".to_string(), FieldValue::Text("2024-06-01".to_string()));
        values.insert("name".to_string(), FieldValue::Text(candidate.name.clone()));
        values.insert("belt_size".to_string(), FieldValue::Text(candidate.belt_size.clone()));
        values.insert("trainer".to_string(), FieldValue::Text(candidate.trainer.clone()));

        template.fill_and_save(values, &output_path)?;

        Ok(())
    }
}


#[cfg(test)]
mod exam_controller_test {
    use super::*;

    #[test]
    fn test_create_exam_sheet() {
        let controller = ExamController::new();
        let candidate = Candidate {
            school: Some("Some School".to_string()),
            name: "John Doe".to_string(),
            trainer: "Jane Smith".to_string(),
            belt: BELTS::AMARILLO,
            belt_size: "CH".to_string()
        };

        let result = controller.create_sheet(&candidate, "yellow.pdf");
        assert!(result.is_ok());

        result.unwrap();
    }
}