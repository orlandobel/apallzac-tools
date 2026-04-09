use std::collections::HashMap;
use acroform::{AcroFormDocument, FieldValue};
use lopdf::{Document, Object, content::{Content, Operation}};
use field_render::FieldRender;

use crate::belt_promotion_exam::{
    candidate::Candidate,
    belts::BELTS
};

mod field_render;

pub struct ExamController {
    date: String,
    base_path: String,
    exams_pdf: Document
}

impl ExamController {
    pub fn new() -> Self{
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let base_path = format!("{}/templates", manifest_dir);

        // Create a new empty PDF document for combining exams
        let combined_pdf = Document::with_version("1.4");

        ExamController { 
            date: "19/04/2026".to_string(),
            base_path,
            exams_pdf: combined_pdf
        }
    }

    pub fn create_exam_page(&mut self, candidate: &Candidate, file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let template_path = format!("{}/{}", self.base_path, file);
        let output_path = std::env::temp_dir()
            .join(format!(
            "exam_output_{}_{}.pdf",
            candidate.name.replace(' ', "_"),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or(0)
            ))
            .to_string_lossy()
            .into_owned();

        let mut template = AcroFormDocument::from_pdf(template_path).expect("error opening file");

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

        // Merge with combined exams file
        self.merge_documents(&output_path)?;

        // Delete temporary file
        if std::path::Path::new(&output_path).exists() {
            std::fs::remove_file(&output_path)?;
        }
        Ok(())
    }

    pub fn get_exams_as_base64(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        use base64::engine::{general_purpose, Engine};
        
        //let temp_path = std::env::temp_dir().join("exams.pdf").to_string_lossy().into_owned();
        //self.exams_pdf.save(&temp_path)?;

        //let buffer = std::fs::read(&temp_path)?;
        let mut buffer: Vec<u8> = Vec::new();
        self.exams_pdf.save_to(&mut buffer)?;
        let base64 = general_purpose::STANDARD.encode(&buffer);
        
        Ok(base64)
    }
    
    fn flatten_document(&self, path: &str, belt: &BELTS) -> Result<(), Box<dyn std::error::Error>> {
        let mut document = Document::load(path)?;

        // --- Fase 1: recolectar datos (préstamos inmutables) ---

        let catalog_id = document.trailer.get(b"Root")?.as_reference()?;
        let _form_id = document.get_object(catalog_id)?.as_dict()?
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

    fn merge_documents(&mut self, temp_pdf_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let temp_doc = Document::load(temp_pdf_path)?;

        if self.exams_pdf.get_pages().is_empty() {
            self.exams_pdf = temp_doc;
        } else {
            let current_temp_path = std::env::temp_dir().join("current_exam.pdf").to_string_lossy().into_owned();
            self.exams_pdf.save(&current_temp_path)?;

            let doc1 = Document::load(&current_temp_path)?;
            let doc2 = temp_doc;

            let mut merged_doc = Document::with_version("1.4");

            // Copy all objects and record old->new ID mappings for each source document.
            // The objects still contain references to the old IDs; we fix that below.
            let mut doc1_id_map: HashMap<lopdf::ObjectId, lopdf::ObjectId> = HashMap::new();
            for (id, obj) in doc1.objects.iter() {
                let new_id = merged_doc.add_object(obj.clone());
                doc1_id_map.insert(*id, new_id);
            }

            let mut doc2_id_map: HashMap<lopdf::ObjectId, lopdf::ObjectId> = HashMap::new();
            for (id, obj) in doc2.objects.iter() {
                let new_id = merged_doc.add_object(obj.clone());
                doc2_id_map.insert(*id, new_id);
            }

            // Remap all internal references so they point to the new IDs in merged_doc.
            // Without this step, /Contents, /Resources, /Font, etc. references inside
            // each page still point to the original document's object IDs, which don't
            // exist in merged_doc, causing pages to appear blank.
            for new_id in doc1_id_map.values().cloned().collect::<Vec<_>>() {
                if let Some(obj) = merged_doc.objects.remove(&new_id) {
                    merged_doc.objects.insert(new_id, remap_references(obj, &doc1_id_map));
                }
            }

            for new_id in doc2_id_map.values().cloned().collect::<Vec<_>>() {
                if let Some(obj) = merged_doc.objects.remove(&new_id) {
                    merged_doc.objects.insert(new_id, remap_references(obj, &doc2_id_map));
                }
            }

            // Build an ordered list of all pages using the remapped IDs.
            let mut all_pages: Vec<lopdf::ObjectId> = Vec::new();

            for (_page_num, page_id) in doc1.get_pages() {
                if let Some(&new_page_id) = doc1_id_map.get(&page_id) {
                    all_pages.push(new_page_id);
                }
            }

            for (_page_num, page_id) in doc2.get_pages() {
                if let Some(&new_page_id) = doc2_id_map.get(&page_id) {
                    all_pages.push(new_page_id);
                }
            }

            // Build a fresh pages tree that owns all pages from both documents.
            let pages_root_id = merged_doc.new_object_id();
            let mut pages_dict = lopdf::Dictionary::new();
            pages_dict.set(b"Type", Object::Name(b"Pages".to_vec()));
            pages_dict.set(b"Count", Object::Integer(all_pages.len() as i64));
            pages_dict.set(b"Kids", Object::Array(all_pages.iter().map(|&id| Object::Reference(id)).collect()));
            merged_doc.objects.insert(pages_root_id, Object::Dictionary(pages_dict));

            for page_ref in &all_pages {
                if let Some(page_obj) = merged_doc.objects.get_mut(page_ref) {
                    if let Ok(page_dict) = page_obj.as_dict_mut() {
                        page_dict.set(b"Parent", Object::Reference(pages_root_id));
                    }
                }
            }

            let catalog_id = merged_doc.new_object_id();
            let mut catalog_dict = lopdf::Dictionary::new();
            catalog_dict.set(b"Type", Object::Name(b"Catalog".to_vec()));
            catalog_dict.set(b"Pages", Object::Reference(pages_root_id));
            merged_doc.objects.insert(catalog_id, Object::Dictionary(catalog_dict));

            merged_doc.trailer.set(b"Root", Object::Reference(catalog_id));
            merged_doc.trailer.set(b"Size", Object::Integer(merged_doc.objects.len() as i64));

            self.exams_pdf = merged_doc;

            let _ = std::fs::remove_file(&current_temp_path);
        }

        Ok(())
    }
  
}

/// Recursively walks an [`Object`] and replaces every [`Object::Reference`] whose
/// ID appears in `id_map` with the corresponding new ID.  This is required when
/// copying objects from one [`Document`] into another: the copied objects still
/// contain references to the original document's object IDs, which need to be
/// translated to the new ones assigned by the destination document.
fn remap_references(obj: Object, id_map: &HashMap<lopdf::ObjectId, lopdf::ObjectId>) -> Object {
    match obj {
        Object::Reference(id) => Object::Reference(*id_map.get(&id).unwrap_or(&id)),
        Object::Array(arr) => Object::Array(
            arr.into_iter().map(|o| remap_references(o, id_map)).collect(),
        ),
        Object::Dictionary(dict) => {
            let mut new_dict = lopdf::Dictionary::new();
            for (k, v) in dict.into_iter() {
                new_dict.set(k, remap_references(v, id_map));
            }
            Object::Dictionary(new_dict)
        }
        Object::Stream(mut stream) => {
            let mut new_dict = lopdf::Dictionary::new();
            for (k, v) in stream.dict.into_iter() {
                new_dict.set(k, remap_references(v, id_map));
            }
            stream.dict = new_dict;
            Object::Stream(stream)
        }
        other => other,
    }
}

#[cfg(test)]
mod exam_controller_test {
    use super::*;

    #[test]
    fn test_create_exam_page() {
        let mut controller = ExamController::new();
        let candidate = Candidate {
            school: Some("Some School".to_string()),
            name: "John Doe".to_string(),
            trainer: "Jane Smith".to_string(),
            belt: BELTS::AMARILLO,
            belt_size: "CH".to_string()
        };

        let result = controller.create_exam_page(&candidate, "yellow.pdf");
        if let Err(ref e) = result {
            println!("Error creating single exam page: {:?}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_exam_two_pages() {
        let mut controller = ExamController::new();
        let candidates = vec![
            Candidate {
                school: Some("Some School".to_string()),
                name: "John Doe".to_string(),
                trainer: "Jane Smith".to_string(),
                belt: BELTS::AMARILLO,
                belt_size: "CH".to_string()
            },
            Candidate {
                school: Some("Some School".to_string()),
                name: "Jane Doe".to_string(),
                trainer: "John Smith".to_string(),
                belt: BELTS::CAFE1,
                belt_size: "CH".to_string()
            }
        ];

        controller.create_exam_page(&candidates[0], "yellow.pdf").unwrap();
        controller.create_exam_page(&candidates[1], "brown1.pdf").unwrap();
        
        assert!(true);
    }

    #[test]
    fn test_create_exam_three_pages() {
        let mut controller = ExamController::new();
        let candidates = vec![
            Candidate {
                school: Some("Some School".to_string()),
                name: "John Doe".to_string(),
                trainer: "Jane Smith".to_string(),
                belt: BELTS::AMARILLO,
                belt_size: "CH".to_string()
            },
            Candidate {
                school: Some("Some School".to_string()),
                name: "Jane Doe".to_string(),
                trainer: "John Smith".to_string(),
                belt: BELTS::CAFE1,
                belt_size: "CH".to_string()
            },
            Candidate {
                school: Some("Some School".to_string()),
                name: "John Doe".to_string(),
                trainer: "Jane Smith".to_string(),
                belt: BELTS::VERDE,
                belt_size: "CH".to_string()
            }
        ];

        controller.create_exam_page(&candidates[0], "yellow.pdf").unwrap();
        controller.create_exam_page(&candidates[1], "brown1.pdf").unwrap();
        controller.create_exam_page(&candidates[2], "green.pdf").unwrap();
        
        assert!(true);
    }

    #[test]
    fn test_get_exams_as_base64_wiht_hundred_candidates() {
        let mut controller = ExamController::new();
        let candidates = generate_candidates(100);

        for candidate in candidates {
            controller.create_exam_page(&candidate, "yellow.pdf").unwrap();
        }

        let base64 = controller.get_exams_as_base64().unwrap();
        assert!(!base64.is_empty());
    }

    fn generate_candidates(count: usize) -> Vec<Candidate> {
        let mut candidates: Vec<Candidate> = Vec::new();
        
        for i in 0..count {
            candidates.push(Candidate {
                school: Some("Some School".to_string()),
                name: format!("candidate_{}", i).to_string(),
                trainer: "Some trainer".to_string(),
                belt: BELTS::AMARILLO,
                belt_size: "CH".to_string()
            });
        }

        candidates
    }
}