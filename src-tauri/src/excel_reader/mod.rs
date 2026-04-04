use excelstream::{ ExcelReader, streaming_reader::StreamingReader };

pub struct Workbook {
    path: String,
    reader: StreamingReader,
}

impl Workbook {
    pub fn new(path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let reader = ExcelReader::open(&path)?;

        Ok(Workbook { 
            path, 
            reader, 
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_create_and_read_workbook_from_public() {
        // Construir la ruta al archivo Excel en public/tests
        // CARGO_MANIFEST_DIR es: C:\Users\orlan\Desktop\apallzac-tools\src-tauri
        // Necesitamos ir a: C:\Users\orlan\Desktop\apallzac-tools\public\tests\examen ejemplo 1.xlsx
        
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        println!("CARGO_MANIFEST_DIR: {}", manifest_dir);
        
        // Construir ruta base usando parent()
        let mut base_path = PathBuf::from(manifest_dir);
        base_path.pop(); // Salir de src-tauri
        base_path.pop(); // Salir del nombre del proyecto
        
        // En realidad, solo necesitamos salir un nivel, ya que estamos en src-tauri
        let mut full_path = PathBuf::from(manifest_dir);
        full_path.push("..");
        full_path.push("public");
        full_path.push("tests");
        full_path.push("examen ejemplo 1.xlsx");

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

        // Crear el workbook y leer el Excel
        let workbook = Workbook::new(path_str.to_string()).expect("No se pudo crear Workbook");

        // Verificar que el workbook se creó correctamente
        assert_eq!(workbook.path, path_str);

        println!("✓ Test exitoso: Workbook creado y Excel leído correctamente");
    }
}

