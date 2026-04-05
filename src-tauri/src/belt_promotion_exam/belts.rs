
use std::str::FromStr;

use unicode_normalization::UnicodeNormalization;

#[derive(Debug)]
pub enum BELTS {
    AMARILLO,
    NARANJA,
    MORADO,
    AZUL,
    VERDE,
    CAFE,
    CAFE1,
    CAFE2,
    CAFE3
}

impl FromStr for BELTS {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let normalized = input.nfd().filter(char::is_ascii).collect::<String>();
        let upper = normalized.to_uppercase();

        println!("Normalized and uppercased input: {}", upper);
        match upper.as_str() {
            "AMARILLO"  => Ok(BELTS::AMARILLO),
            "NARANJA" => Ok(BELTS::NARANJA),
            "MORADO" => Ok(BELTS::MORADO),
            "AZUL" => Ok(BELTS::AZUL),
            "VERDE" => Ok(BELTS::VERDE),
            "CAFE" => Ok(BELTS::CAFE),
            // TODO :: this three cases are temporal, change when get definitives
            "CAFE1" => Ok(BELTS::CAFE1),
            "CAFE2" => Ok(BELTS::CAFE2),
            "CAFE3" => Ok(BELTS::CAFE3),
            _      => {
                let msg = format!("No se encontro cinturon: {}", input);
                Err(msg)
            },
        }
    }
}