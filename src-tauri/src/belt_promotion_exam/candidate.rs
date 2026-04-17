use std::str::FromStr;

use super::belts::BELTS;
use crate::excel_reader::column_configurations::BeltPromotionConfiguration;
use excelstream::Row;
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Candidate {
    pub school: Option<String>,
    pub name: String,
    pub trainer: String,
    pub belt: BELTS,
    pub belt_size: String,
}

impl Candidate {
    // TODO :: Validate case of some data is missing on row
    pub fn new(
        config: &BeltPromotionConfiguration,
        data: &Row,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let school = if config.school != -1 {
            match data.get(config.school as usize) {
                Some(val) => Some(val.to_string()),
                None => {
                    error!("Candidate@new - Missing school data at column {}", config.school);
                    None
                }
            }
        } else {
            None
        };

        let belt_string = match data.get(config.belt as usize) {
            Some(val) => val.to_string(),
            None => {
                error!("Candidate@new - Missing belt data at column {}", config.belt);
                return Err("Missing belt data".into());
            }
        };
        let belt = BELTS::from_str(&belt_string)?;

        let name = match data.get(config.name as usize) {
            Some(val) => val.to_string(),
            None => {
                error!("Candidate@new - Missing name data at column {}", config.name);
                return Err("Missing name data".into());
            }
        };
        let trainer = match data.get(config.trainer as usize) {
            Some(val) => val.to_string(),
            None => {
                error!("Candidate@new - Missing trainer data at column {}", config.trainer);
                return Err("Missing trainer data".into());
            }
        };
        let belt_size = match data.get(config.belt_size as usize) {
            Some(val) => val.to_string(),
            None => {
                error!("Candidate@new - Missing belt_size data at column {}", config.belt_size);
                return Err("Missing belt_size data".into());
            }
        };

        Ok(Candidate {
            school,
            name,
            trainer,
            belt,
            belt_size,
        })
    }
}
