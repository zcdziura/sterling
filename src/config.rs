use std::convert::From;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::{self, BufReader, ErrorKind};

use serde_yaml;

use currency::Currency;

pub fn load_config(filename: &str) -> Result<Vec<Currency>, ConfigError> {
    let config_file = File::open(filename)?;
    let mut configs: Vec<Currency> = serde_yaml::from_reader(BufReader::new(config_file))?;
    configs.sort_by(|a, b| b.cmp(a));

    Ok(configs)
}

pub fn default_config() -> Vec<Currency> {
    vec![
        Currency::new("platinum", 1000000, "p", None, None),
        Currency::new("gold", 10000, "g", None, None),
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
