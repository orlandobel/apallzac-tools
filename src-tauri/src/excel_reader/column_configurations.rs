use unicode_normalization::UnicodeNormalization;

#[allow(dead_code)]
pub enum Configurations {
    BeltPromotionConfiguration(BeltPromotionConfiguration),
}

trait SheetConfig {
    fn check_for_errors(&self) -> Vec<String>;
}

pub struct BeltPromotionConfiguration {
    pub school: isize,
    pub trainer: isize,
    pub name: isize,
    pub belt: isize,
    pub belt_size: isize,
}

impl BeltPromotionConfiguration {
    pub fn new(row: &excelstream::Row) -> Result<Self, String> {
        let mut belt_promotion_configuration = BeltPromotionConfiguration {
            school: -1,
            trainer: -1,
            name: -1,
            belt: -1,
            belt_size: -1,
        };

        let cells = row.cells.iter();
        for (index, cell) in cells.enumerate() {
            if cell.is_empty() {
                continue;
            }

            let cell_name = cell.to_string().nfc().collect::<String>();
            let cell_upper = cell_name.to_uppercase();

            match cell_upper.as_str() {
                "ESCUELA" => belt_promotion_configuration.school = index as isize,
                "PROFESOR" => belt_promotion_configuration.trainer = index as isize,
                "NOMBRE" => belt_promotion_configuration.name = index as isize,
                "CINTURON" => belt_promotion_configuration.belt = index as isize,
                "TALLA" => belt_promotion_configuration.belt_size = index as isize,
                _ => {}
            }
        }

        let missing = belt_promotion_configuration.check_for_errors();
        if !missing.is_empty() {
            let msg = format!(
                "No se encontraron las siguientes columnas: {:?}",
                missing.join(", ")
            );
            return Err(msg);
        }

        Ok(belt_promotion_configuration)
    }
}

impl SheetConfig for BeltPromotionConfiguration {
    fn check_for_errors(&self) -> Vec<String> {
        let mut missing_columns: Vec<String> = Vec::new();

        if self.trainer == -1 {
            missing_columns.push("PROFESOR".to_string());
        }
        if self.name == -1 {
            missing_columns.push("NOMBRE".to_string());
        }
        if self.belt == -1 {
            missing_columns.push("CINTURON".to_string());
        }
        if self.belt_size == -1 {
            missing_columns.push("TALLA".to_string());
        }

        missing_columns
    }
}
