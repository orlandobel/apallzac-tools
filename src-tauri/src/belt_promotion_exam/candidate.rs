use std::str::FromStr;

use excelstream::Row;
use serde::{Deserialize, Serialize};
use super::belts::BELTS;
use crate::excel_reader::column_configurations::BeltPromotionConfiguration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    school: Option<String>,
    name: String,
    trainer: String,
    belt: BELTS,
    belt_size: String, 
}

impl Candidate {
    // TODO :: Validate case of some data is missing on row
    pub fn new(config: &BeltPromotionConfiguration, data: &Row) -> Result<Self, Box<dyn std::error::Error>> {
        let school = if config.school != -1 { 
            Some(data.get(config.school as usize).unwrap().to_string()) 
        } else {
            None
        };

        let belt_string = data.get(config.belt as usize).unwrap().to_string();
        let belt = BELTS::from_str(&belt_string)?;

        let name = data.get(config.name as usize).unwrap().to_string();
        let trainer = data.get(config.trainer as usize).unwrap().to_string();
        let belt_size = data.get(config.belt_size as usize).unwrap().to_string();
        
        Ok(Candidate {
            school,
            name,
            trainer,
            belt,
            belt_size,
        })
    }
}