use lopdf::Object;

use crate::belt_promotion_exam::belts::BELTS;

pub struct FieldRender {
    x: f32,
    y: f32,
    font_name: Vec<u8>,
    font_size: f32,
    text_bytes: Vec<u8>,
}

impl FieldRender {
    fn new(x: f32, y: f32, font_name: Vec<u8>, font_size: f32, text_bytes: Vec<u8>) -> Self {
        FieldRender { x, y, font_name, font_size, text_bytes }
    }

    pub fn from_annot_ids(annot_ids: &Vec<lopdf::ObjectId>, document: &lopdf::Document, belt: &BELTS) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let mut field_renders: Vec<Self> = Vec::new();

        for &annot_id in annot_ids {
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

            let (font_name, mut font_size) = Self::set_font_conf(belt);
            if font_size == 0.0 {
                font_size = 10.0;
            }

            let x = rect[0] + 2.0;
            let field_height = rect[3] - rect[1];
            let y = rect[1] + ((field_height - font_size) / 2.0).max(0.0);

            field_renders.push(FieldRender::new(x, y, font_name, font_size, text_bytes));
        }

        Ok(field_renders)
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn font_name(&self) -> Vec<u8> {
        self.font_name.clone()
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn text_bytes(&self) -> Vec<u8> {
        self.text_bytes.clone()
    }


    fn set_font_conf(belt: &BELTS) -> (Vec<u8>, f32) {        
        let font_name = b"Calibri".to_vec();
        
        let font_size = match belt {
            BELTS::AMARILLO => { 11.0f32 },
            BELTS::NARANJA => { 11.0f32 },
            BELTS::MORADO => { 11.0f32 },
            BELTS::AZUL => { 11.0f32 },
            BELTS::VERDE => { 9.0f32 },
            BELTS::CAFE => { 8.5f32 },
            BELTS::CAFE1 => { 8.5f32 },
            BELTS::CAFE2 => { 8.5f32 },
            BELTS::CAFE3 => { 8.5f32 }
        };
        
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