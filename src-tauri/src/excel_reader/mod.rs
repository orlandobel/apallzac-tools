use excelstream::{ ExcelReader, streaming_reader::{RowStructIterator, StreamingReader} };
use column_configurations::Configurations;

mod column_configurations;

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

    pub fn get_sheet(&mut self) -> Result<RowStructIterator<'_>, Box<dyn std::error::Error>> {
        let sheet = self.reader.rows_by_index(0)?;
        Ok(sheet)
    }

    fn get_column_config(&mut self, first_row: &excelstream::Row) -> Result<Configurations, Box<dyn std::error::Error>> {
        let config = column_configurations::Configurations::BeltPromotionConfiguration(
            column_configurations::BeltPromotionConfiguration::new(first_row)?
        );

        Ok(config)
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

    #[test]
    fn test_get_column_config() {
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
        let mut workbook = Workbook::new(path_str.to_string()).expect("No se pudo crear Workbook");

        let mut sheet = workbook.get_sheet().expect("No se pudo obtener la hoja");
        let row = sheet.next().unwrap().expect("No se pudo obtener la fila");
        drop(sheet); // Liberar la referencia mutable a workbook

        let column_config = workbook.get_column_config(&row);
        assert!(column_config.is_ok(), "No se pudo obtener la configuración de columnas");

        println!("✓ Test exitoso: Configuración de columnas obtenida correctamente");
    }

}

