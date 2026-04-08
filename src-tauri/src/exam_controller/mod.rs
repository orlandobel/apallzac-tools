use std::collections::HashMap;
use acroform::{AcroFormDocument, FieldValue};
use lopdf::{Dictionary, Document, Object, content::{Content, Operation}};

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
        values.insert("date".to_string(), FieldValue::Text(self.date.clone()));
        values.insert("name".to_string(), FieldValue::Text(candidate.name.clone()));
        values.insert("belt_size".to_string(), FieldValue::Text(candidate.belt_size.clone()));
        values.insert("trainer".to_string(), FieldValue::Text(candidate.trainer.clone()));

        template.fill_and_save(values, &output_path)
        .map_err(|e| {
            println!("Error saving document: {:?}", e.to_string());
            Box::new(e) as Box<dyn std::error::Error + 'static>
        })?;

        self.flatten_document(&output_path)
        .map_err(|e| {
            println!("Error flattening document: {:?}", e.to_string());
            Box::new(e)
        }).unwrap();

        Ok(())
    }

    fn flatten_document(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut document = Document::load(path)?;

        // --- Fase 1: recolectar datos (préstamos inmutables) ---

        let catalog_id = document.trailer.get(b"Root")?.as_reference()?;
        let form_id = document.get_object(catalog_id)?.as_dict()?
            .get(b"AcroForm")?.as_reference()?;

        // /DA global del AcroForm (fuente por defecto)
        let form_da: Option<Vec<u8>> = {
            let form = document.get_object(form_id)?.as_dict()?;
            match form.get(b"DA") {
                Ok(Object::String(da, _)) => Some(da.clone()),
                _ => None,
            }
        };

        // Recursos de fuente del AcroForm (/DR /Font)
        let font_resources: HashMap<Vec<u8>, Object> = {
            let form = document.get_object(form_id)?.as_dict()?;
            match form.get(b"DR") {
                Ok(Object::Dictionary(dr)) => match dr.get(b"Font") {
                    Ok(Object::Dictionary(fonts)) => {
                        fonts.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
                    }
                    _ => HashMap::new(),
                },
                _ => HashMap::new(),
            }
        };

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
        struct FieldRender {
            x: f32,
            y: f32,
            font_name: Vec<u8>,
            font_size: f32,
            text_bytes: Vec<u8>,
        }

        let mut render_data: Vec<FieldRender> = Vec::new();

        for &annot_id in &annot_ids {
            let dict = document.get_object(annot_id)?.as_dict()?.clone();

            // Solo anotaciones de tipo Widget
            let is_widget = matches!(
                dict.get(b"Subtype"),
                Ok(Object::Name(n)) if n == b"Widget"
            );
            if !is_widget {
                continue;
            }

            // Rectángulo [x1, y1, x2, y2] — debe existir en la anotación
            let rect: Vec<f32> = match dict.get(b"Rect") {
                Ok(Object::Array(arr)) if arr.len() >= 4 => arr
                    .iter()
                    .take(4)
                    .map(|o| o.as_float().unwrap_or(0.0))
                    .collect(),
                _ => continue,
            };

            // /V: se busca primero en el widget; si no, en el campo padre (/Parent)
            let value_bytes: Vec<u8> = {
                let direct = match dict.get(b"V") {
                    Ok(Object::String(bytes, _)) if !bytes.is_empty() => {
                        Some(bytes.clone())
                    }
                    _ => None,
                };
                if let Some(v) = direct {
                    v
                } else {
                    let parent_id = match dict.get(b"Parent") {
                        Ok(Object::Reference(id)) => *id,
                        _ => continue,
                    };
                    let parent = document.get_object(parent_id)?.as_dict()?.clone();
                    match parent.get(b"V") {
                        Ok(Object::String(bytes, _)) if !bytes.is_empty() => bytes.clone(),
                        _ => continue,
                    }
                }
            };

            let text_bytes = Self::pdf_string_to_latin1(&value_bytes);
            if text_bytes.is_empty() {
                continue;
            }

            // /DA: widget → padre → /AcroForm → por defecto
            let da_bytes: Option<Vec<u8>> = match dict.get(b"DA") {
                Ok(Object::String(da, _)) => Some(da.clone()),
                _ => {
                    if let Ok(Object::Reference(parent_id)) = dict.get(b"Parent") {
                        let parent_id = *parent_id;
                        document
                            .get_object(parent_id)
                            .ok()
                            .and_then(|o| o.as_dict().ok())
                            .and_then(|d| d.get(b"DA").ok())
                            .and_then(|o| {
                                if let Object::String(da, _) = o {
                                    Some(da.clone())
                                } else {
                                    None
                                }
                            })
                    } else {
                        None
                    }
                }
            };

            let (font_name, mut font_size) = da_bytes
                .as_deref()
                .map(Self::parse_da)
                .or_else(|| form_da.as_deref().map(Self::parse_da))
                .unwrap_or_else(|| (b"Helv".to_vec(), 10.0f32));
            if font_size == 0.0 {
                font_size = 10.0;
            }

            let x = rect[0] + 2.0;
            let field_height = rect[3] - rect[1];
            let y = rect[1] + ((field_height - font_size) / 2.0).max(0.0);

            render_data.push(FieldRender { x, y, font_name, font_size, text_bytes });
        }

        // --- Fase 2: construir stream de contenido con el texto plano ---

        let mut operations: Vec<Operation> = Vec::new();
        for rd in &render_data {
            operations.push(Operation::new("BT", vec![]));
            // Color negro
            operations.push(Operation::new("g", vec![Object::Real(0.0)]));
            // Fuente y tamaño
            operations.push(Operation::new("Tf", vec![
                Object::Name(rd.font_name.clone()),
                Object::Real(rd.font_size),
            ]));
            // Mover al origen del campo
            operations.push(Operation::new("Td", vec![
                Object::Real(rd.x),
                Object::Real(rd.y),
            ]));
            // Imprimir texto
            operations.push(Operation::new("Tj", vec![
                Object::String(rd.text_bytes.clone(), lopdf::StringFormat::Literal),
            ]));
            operations.push(Operation::new("ET", vec![]));
        }

        let content_bytes = Content { operations }.encode()?;
        document.add_page_contents(page_id, content_bytes)?;

        // --- Fase 3: copiar fuentes del AcroForm a los recursos de la página ---

        if !font_resources.is_empty() {
            // Detectar si /Resources es un objeto indirecto
            let res_id = {
                let page = document.get_dictionary(page_id)?;
                page.get(b"Resources").and_then(|o| o.as_reference()).ok()
            };

            if let Some(res_id) = res_id {
                let res_dict = document.get_dictionary_mut(res_id)?;
                Self::merge_fonts(res_dict, &font_resources);
            } else {
                let page = document.get_object_mut(page_id).and_then(Object::as_dict_mut)?;
                let mut res_dict = match page.get(b"Resources").ok().cloned() {
                    Some(Object::Dictionary(d)) => d,
                    _ => Dictionary::new(),
                };
                Self::merge_fonts(&mut res_dict, &font_resources);
                page.set(b"Resources", Object::Dictionary(res_dict));
            }
        }

        // --- Fase 4: eliminar anotaciones de widget y el AcroForm ---

        document.get_object_mut(page_id).and_then(Object::as_dict_mut)?.remove(b"Annots");
        document.catalog_mut()?.remove(b"AcroForm");

        document.save(path)?;
        Ok(())
    }

    /// Inserta las entradas de `fonts` en el subdirectorio /Font del diccionario de recursos.
    fn merge_fonts(res_dict: &mut Dictionary, fonts: &HashMap<Vec<u8>, Object>) {
        let existing = res_dict.get(b"Font").ok().cloned();
        let mut font_dict = match existing {
            Some(Object::Dictionary(d)) => d,
            _ => Dictionary::new(),
        };
        for (k, v) in fonts {
            font_dict.set(k.clone(), v.clone());
        }
        res_dict.set(b"Font", Object::Dictionary(font_dict));
    }

    /// Parsea la cadena /DA del campo (ej. "/Helv 10 Tf 0 g") para extraer
    /// el nombre de la fuente y el tamaño.
    fn parse_da(da: &[u8]) -> (Vec<u8>, f32) {        let s = String::from_utf8_lossy(da);
        let tokens: Vec<&str> = s.split_whitespace().collect();
        let mut font_name = b"Helv".to_vec();
        let mut font_size = 10.0f32;
        for i in 2..tokens.len() {
            if tokens[i] == "Tf" {
                font_name = tokens[i - 2].trim_start_matches('/').as_bytes().to_vec();
                font_size = tokens[i - 1].parse().unwrap_or(10.0);
                break;
            }
        }
        (font_name, font_size)
    }

    /// Convierte una cadena PDF a bytes Latin-1 (WinAnsiEncoding).
    /// Maneja tanto cadenas en PDFDocEncoding como UTF-16BE (con BOM 0xFE 0xFF).
    fn pdf_string_to_latin1(bytes: &[u8]) -> Vec<u8> {
        if bytes.starts_with(&[0xFE, 0xFF]) {
            let utf16: Vec<u16> = bytes[2..]
                .chunks_exact(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect();
            String::from_utf16_lossy(&utf16)
                .chars()
                .map(|c| if (c as u32) <= 0xFF { c as u8 } else { b'?' })
                .collect()
        } else {
            bytes.to_vec()
        }
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