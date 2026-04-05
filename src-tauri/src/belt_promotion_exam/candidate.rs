use excelstream::Row;
use crate::excel_reader::column_configurations::BeltPromotionConfiguration;

#[derive(Debug)]
pub struct Candidate {
    school: Option<String>,
    name: String,
    trainer: String,
    belt: String,
    belt_size: String, 
}

impl Candidate {
    // TODO :: Validate case of some data is missing on row
    pub fn new(config: &BeltPromotionConfiguration, data: &Row) -> Self {
        let school = if config.school != -1 { 
            Some(data.get(config.school as usize).unwrap().to_string()) 
        } else {
            None
        };

        let name = data.get(config.name as usize).unwrap().to_string();
        let trainer = data.get(config.trainer as usize).unwrap().to_string();
        let belt = data.get(config.belt as usize).unwrap().to_string();
        let belt_size = data.get(config.belt_size as usize).unwrap().to_string();
        
        Candidate {
            school,
            name,
            trainer,
            belt,
            belt_size,
        }
    }
}