#[macro_use]
extern crate clap;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

mod config;
mod currency;

use std::io::ErrorKind;
use std::process;

use config::ConfigError;
use currency::Currency;
use regex::Regex;

fn main() {
    let app = clap_app!(sterling =>
        (version: "0.2.0")
        (about: "Converts a given D&D 5e currency value to the Silver Standard.")
        (@arg CONFIG: -c --config +takes_value "Specify location of config file; defaults to './sterling-conf.yml'.")
        (@arg PRINT_FULL: -f --full "Print currencies with full name, rather than with alias.")
        (@arg VALUE: ... "The value to be converted; should be suffixed with the coin's short-hand abbreviation, i.e. p, g, e, s, or c.")
    );

    let matches = app.get_matches();
    let config_result = config::load_config(match matches.value_of("CONFIG") {
        Some(file) => file,
        None => "./sterling-conf.yml",
    });

    let currencies = match parse_currency_config(config_result, matches.value_of("CONFIG")) {
        Ok(currencies) => currencies,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    if let Some(values) = matches.values_of("VALUE") {
        let coins: Vec<&str> = values.collect();
        let total_copper_value = match calculate_total_copper_value(coins) {
            Ok(total_copper_value) => total_copper_value,
            Err(err) => {
                eprintln!("{}", err);
                process::exit(1);
            }
        };

        let converted_currencies = convert_currencies(total_copper_value, currencies);
        let display_strings: Vec<String> =
            create_display_strings(converted_currencies, matches.is_present("PRINT_FULL"));

        println!("{}", (&display_strings).join(", "));
    } else {
        eprintln!("Please enter at least one value; should be suffixed with the coin's short-hand abbreviation, i.e. p, g, e, s, or c.");
        process::exit(1);
    }
}

fn parse_currency_config(
    config_result: Result<Vec<Currency>, ConfigError>,
    config_file_path: Option<&str>,
) -> Result<Vec<Currency>, String> {
    match config_result {
        Ok(values) => Ok(values),
        Err(error) => match error.kind {
            ErrorKind::NotFound => {
                if let Some(file_path) = config_file_path {
                    Err(format!("Sterling Error: Can't find configuration file: \"{}\"", &file_path))
                } else {
                    Ok(config::default_config())
                }
            },
            _ => Err(format!("Sterling Error: {}", error)),
        },
    }
}

fn convert_to_copper(amount: usize, coin_denomination: &str) -> usize {
    match coin_denomination {
        "p" => amount * 1000,
        "g" => amount * 100,
        "e" => amount * 50,
        "s" => amount * 10,
        "c" => amount,
        _ => unreachable!("Invalid coin type; must be a valid coin found in the PHB."),
    }
}

fn calculate_total_copper_value(coins: Vec<&str>) -> Result<usize, &'static str> {
    let regex: Regex = Regex::new(r"(\d+)([cegps])").unwrap();
    for coin in coins.iter() {
        if let None = regex.captures(coin) {
            return Err(
                "Sterling Error: Invalid coin value. Make sure all coins are denoted properly."
            )
        }
    }

    let converted_values = coins.iter().map(|coin| {
        let captures = regex.captures(coin).unwrap();
        let amount: usize = captures[1].parse().unwrap();
        let denomination = captures[2].to_owned();
        convert_to_copper(amount, &denomination)
    });
    
    Ok(converted_values.fold(0 as usize, |total, value| total + value))
}

fn exchange(copper: usize, mut currencies: Vec<Currency>) -> Vec<Currency> {
    let mut val = copper;
    currencies
        .iter_mut()
        .map(|currency| {
            let value = val / currency.rate;
            val = val % currency.rate;

            currency.with_value(value)
        })
        .collect()
}

fn convert_currencies(copper_value: usize, currencies: Vec<Currency>) -> Vec<Currency> {
    exchange(copper_value, currencies)
        .iter()
        .filter(|c| (*c).value.unwrap_or(0) > 0)
        .cloned()
        .collect()
}

fn create_display_strings(converted_currencies: Vec<Currency>, is_print_full: bool) -> Vec<String> {
    converted_currencies
        .iter()
        .map(|c| {
            if is_print_full {
                c.full_display()
            } else {
                c.alias_display()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use currency::Currency;

    lazy_static! {
        static ref STANDARD_CURRENCIES: [Currency; 4] = [
            Currency::new("platinum", 1000000, "p", None),
            Currency::new("gold", 10000, "g", None),
            Currency::new("silver", 100, "s", None),
            Currency::new("copper", 1, "c", None),
        ];
    }

    #[test]
    fn test_convert_copper_to_copper() {
        assert_eq!(1, convert_to_copper(1, "c"));
    }

    #[test]
    fn test_convert_silver_to_copper() {
        assert_eq!(10, convert_to_copper(1, "s"));
    }

    #[test]
    fn test_convert_electrum_to_copper() {
        assert_eq!(50, convert_to_copper(1, "e"));
    }

    #[test]
    fn test_convert_gold_to_copper() {
        assert_eq!(100, convert_to_copper(1, "g"));
    }

    #[test]
    fn test_convert_platinum_to_copper() {
        assert_eq!(1000, convert_to_copper(1, "p"));
    }

    #[test]
    fn test_calculate_total_copper_value() {
        let values = vec!["1p", "1g", "1e", "1s", "1c"];
        assert_eq!(1161, calculate_total_copper_value(values).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_calculate_total_copper_value_bad_inputs() {
        let values = vec!["1p", "1g", "1f", "1s", "1c"];
        assert_eq!(1161, calculate_total_copper_value(values).unwrap());
    }

    #[test]
    fn test_exchange_to_copper() {
        let currencies = vec![
            Currency::new("platinum", 1000000, "p", None).with_value(0),
            Currency::new("gold", 10000, "g", None).with_value(0),
            Currency::new("silver", 100, "s", None).with_value(0),
            Currency::new("copper", 1, "c", None).with_value(1),
        ];

        assert_eq!(currencies, exchange(1, STANDARD_CURRENCIES.to_vec()));
    }

    #[test]
    fn test_exchange_to_silver() {
        let currencies = vec![
            Currency::new("platinum", 1000000, "p", None).with_value(0),
            Currency::new("gold", 10000, "g", None).with_value(0),
            Currency::new("silver", 100, "s", None).with_value(1),
            Currency::new("copper", 1, "c", None).with_value(0),
        ];

        assert_eq!(currencies, exchange(100, STANDARD_CURRENCIES.to_vec()));
    }

    #[test]
    fn test_exchange_to_gold() {
        let currencies = vec![
            Currency::new("platinum", 1000000, "p", None).with_value(0),
            Currency::new("gold", 10000, "g", None).with_value(1),
            Currency::new("silver", 100, "s", None).with_value(0),
            Currency::new("copper", 1, "c", None).with_value(0),
        ];

        assert_eq!(currencies, exchange(10000, STANDARD_CURRENCIES.to_vec()));
    }

    #[test]
    fn test_exchange_to_platinum() {
        let currencies = vec![
            Currency::new("platinum", 1000000, "p", None).with_value(1),
            Currency::new("gold", 10000, "g", None).with_value(0),
            Currency::new("silver", 100, "s", None).with_value(0),
            Currency::new("copper", 1, "c", None).with_value(0),
        ];

        assert_eq!(currencies, exchange(1000000, STANDARD_CURRENCIES.to_vec()));
    }
}
