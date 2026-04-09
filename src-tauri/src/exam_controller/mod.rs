use std::collections::HashMap;
use acroform::{AcroFormDocument, FieldValue};
use lopdf::{Dictionary, Document, Object, content::{Content, Operation}};
use field_render::FieldRender;

use crate::belt_promotion_exam::{
    candidate::Candidate,
    belts::BELTS
};

mod field_render;

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
        values.insert("date".to_string(), FieldValue::Text(self.date.clone()));
        values.insert("name".to_string(), FieldValue::Text(candidate.name.clone()));
        values.insert("belt_size".to_string(), FieldValue::Text(candidate.belt_size.clone()));
        values.insert("trainer".to_string(), FieldValue::Text(candidate.trainer.clone()));

        template.fill_and_save(values, &output_path)
        .map_err(|e| {
            println!("Error saving document: {:?}", e.to_string());
            Box::new(e) as Box<dyn std::error::Error + 'static>
        })?;

        self.flatten_document(&output_path, &candidate.belt)
        .map_err(|e| {
            println!("Error flattening document: {:?}", e.to_string());
            Box::new(e)
        }).unwrap();

        Ok(())
    }

    fn flatten_document(&self, path: &str, belt: &BELTS) -> Result<(), Box<dyn std::error::Error>> {
        let mut document = Document::load(path)?;

        // --- Fase 1: recolectar datos (préstamos inmutables) ---

        let catalog_id = document.trailer.get(b"Root")?.as_reference()?;
        let form_id = document.get_object(catalog_id)?.as_dict()?
            .get(b"AcroForm")?.as_reference()?;

        // Página única del PDF
        let page_id = *document
            .get_pages()
            .values()
            .next()
            .ok_or("El documento no tiene páginas")?;

        // IDs de las anotaciones en la página.
        // Se itera desde /Annots en lugar de /Fields para capturar todos los widgets,
        // incluyendo los que tienen /Rect solo en la anotación (padre en /Fields sin /Rect).
        let annot_ids: Vec<lopdf::ObjectId> = {
            let page = document.get_dictionary(page_id)?;
            match page.get(b"Annots") {
                Ok(Object::Array(arr)) => {
                    arr.iter().filter_map(|o| o.as_reference().ok()).collect()
                }
                Ok(Object::Reference(id)) => {
                    let id = *id;
                    document
                        .get_object(id)?
                        .as_array()?
                        .iter()
                        .filter_map(|o| o.as_reference().ok())
                        .collect()
                }
                _ => vec![],
            }
        };

        // Datos de renderizado por widget
       let render_data: Vec<FieldRender> = FieldRender::from_annot_ids(&annot_ids, &document, belt)?;

        // --- Fase 2: construir stream de contenido con el texto plano ---
        let mut operations: Vec<Operation> = Vec::new();
        for rd in &render_data {
            operations.push(Operation::new("BT", vec![]));

            // Color negro
            operations.push(Operation::new("g", vec![Object::Real(0.0)]));

            // Fuente y tamaño
            operations.push(Operation::new("Tf", vec![
                Object::Name(rd.font_name().clone()),
                Object::Real(rd.font_size()),
            ]));

            // Mover al origen del campo
            operations.push(Operation::new("Td", vec![
                Object::Real(rd.x()),
                Object::Real(rd.y()),
            ]));

            // Imprimir texto
            operations.push(Operation::new("Tj", vec![
                Object::String(rd.text_bytes().clone(), lopdf::StringFormat::Literal),
            ]));
            
            operations.push(Operation::new("ET", vec![]));
        }

        let content_bytes = Content { operations }.encode()?;
        document.add_page_contents(page_id, content_bytes)?;

        // --- Fase 4: eliminar anotaciones de widget y el AcroForm ---

        document.get_object_mut(page_id).and_then(Object::as_dict_mut)?.remove(b"Annots");
        document.catalog_mut()?.remove(b"AcroForm");

        document.save(path)?;
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
            belt: BELTS::CAFE1,
            belt_size: "CH".to_string()
        };

        let result = controller.create_sheet(&candidate, "brown3.pdf");
        assert!(result.is_ok());

        result.unwrap();
    }
}