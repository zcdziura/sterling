use std::convert::From;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{self, BufReader, ErrorKind};

use serde_yaml;

use currency::Currency;

pub fn load_config(filename: &str) -> Result<Vec<Currency>, ConfigError> {
    let config_file = File::open(filename)?;
    let config: Vec<Currency> = serde_yaml::from_reader(BufReader::new(config_file))?;

    Ok(config)
}

pub fn parse_currency_config(
    config_result: Result<Vec<Currency>, ConfigError>,
    config_file_path: Option<&str>,
) -> Result<Vec<Currency>, String> {
    match config_result {
        Ok(values) => Ok(values),
        Err(error) => match error.kind {
            ErrorKind::NotFound => {
                if let Some(file_path) = config_file_path {
                    Err(format!(
                        "Sterling Error: Can't find configuration file: \"{}\"",
                        &file_path
                    ))
                } else {
                    Ok(silver_standard_config())
                }
            }
            _ => Err(format!("Sterling Error: {}", error)),
        },
    }
}

pub fn phb_config() -> Vec<Currency> {
    vec![
        Currency::new("platinum", 1000, "p", None, None),
        Currency::new("gold", 100, "g", None, None),
        Currency::new("electrum", 50, "e", None, Some(true)),
        Currency::new("silver", 10, "s", None, None),
        Currency::new("copper", 1, "c", None, None),
    ]
}

fn silver_standard_config() -> Vec<Currency> {
    vec![
        Currency::new("platinum", 1_000_000, "p", None, None),
        Currency::new("gold", 10_000, "g", None, None),
        Currency::new("silver", 100, "s", None, None),
        Currency::new("copper", 1, "c", None, None),
    ]
}

#[derive(Debug)]
pub struct ConfigError {
    desc: String,
    pub kind: ErrorKind,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Sterling Error: {}", self.desc)
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        &self.desc
    }
}

impl From<io::Error> for ConfigError {
    fn from(error: io::Error) -> Self {
        ConfigError {
            desc: error.description().to_owned(),
            kind: error.kind(),
        }
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(error: serde_yaml::Error) -> Self {
        ConfigError {
            desc: error.description().to_owned(),
            kind: ErrorKind::Other,
        }
    }
}
