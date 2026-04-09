use excelstream::{
    streaming_reader::{RowStructIterator, StreamingReader},
    ExcelReader,
};

// TODO :: function to reload file
#[allow(dead_code)]
pub struct Workbook {
    path: String,
    reader: StreamingReader,
}

impl Workbook {
    pub fn new(path: String) -> Result<Self, Box<dyn std::error::Error>> {
        let reader = ExcelReader::open(&path)?;

        Ok(Workbook { path, reader })
    }

    pub fn get_sheet(&mut self) -> Result<RowStructIterator<'_>, Box<dyn std::error::Error>> {
        let sheet = self.reader.rows_by_index(0)?;
        Ok(sheet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_create_and_read_workbook_from_public() {
        let path_str = get_file_path("examen ejemplo 1.xlsx");

        // Crear el workbook y leer el Excel
        let workbook = Workbook::new(path_str.clone()).expect("No se pudo crear Workbook");

        // Verificar que el workbook se creó correctamente
        assert_eq!(workbook.path, path_str);

        println!("✓ Test exitoso: Workbook creado y Excel leído correctamente");
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
