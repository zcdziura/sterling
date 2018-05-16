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
mod convert;
mod currency;

use std::collections::HashMap;
use std::io::ErrorKind;
use std::process;

use config::ConfigError;
use currency::Currency;
use regex::Regex;

fn main() {
    let app = clap_app!(sterling =>
        (version: env!("CARGO_PKG_VERSION"))
        (about: "Converts a given D&D 5e currency value to the Silver Standard.")
        (@arg CONFIG: -c --config +takes_value "Specify location of config file; defaults to './sterling-conf.yml'.")
        (@arg PRINT_FULL: -f --full "Print currencies with full name, rather than with alias.")
        (@arg OPTIONAL: -o --optional "Include currencies marked as optional when converting.")
        (@arg VALUE: ... "The value to be converted; should be suffixed with the coin's short-hand abbreviation, i.e. p, g, e, s, or c.")
        (@subcommand add =>
            (about: "Add two currency amounts together; uses the currencies defined in your config file")
            (@arg AUGEND: +required "The augend of the addition function; i.e. the left side")
            (@arg ADDEND: +required "The addend of the addition function; i.e. the right side")
            (@arg PRINT_FULL: -f --full "Print currencies with full name, rather than with alias.")
        )
        (@subcommand sub =>
            (about: "Subtract two currency amounts from one another; uses the currencies defined in your config file")
            (@arg MINUEND: +required "The minuend of the subtraction function; i.e. the left side")
            (@arg SUBTRAHEND: +required "The subtrahend of the subtraction function; i.e. the right side")
            (@arg PRINT_FULL: -f --full "Print currencies with full name, rather than with alias.")
        )
    );

    let matches = app.get_matches();
    let config_result = config::load_config(match matches.value_of("CONFIG") {
        Some(file) => file,
        None => "./sterling-conf.yml",
    });

    let currencies: Vec<Currency> =
        match parse_currency_config(config_result, matches.value_of("CONFIG")) {
            Ok(currencies) => currencies
                .iter()
                .filter(|c| {
                    let has_add_subcommand = match matches.subcommand_matches("add") {
                        Some(_) => true,
                        None => false,
                    };

                    if has_add_subcommand {
                        true
                    } else if !matches.is_present("OPTIONAL") {
                        !c.is_optional()
                    } else {
                        true
                    }
                })
                .cloned()
                .collect(),
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1);
            }
        };

    if let Some(matches) = matches.subcommand_matches("add") {
        let (lhs, rhs) = get_copper_value(
            &currencies,
            matches.value_of("AUGEND").unwrap(),
            matches.value_of("ADDEND").unwrap(),
        );

        let converted_currencies = convert::convert_currencies(lhs + rhs, currencies);
        let display_strings: Vec<String> =
            create_display_strings(converted_currencies, matches.is_present("PRINT_FULL"));

        println!("{}", (&display_strings).join(", "));
    } else if let Some(matches) = matches.subcommand_matches("sub") {
        let (lhs, rhs) = get_copper_value(
            &currencies,
            matches.value_of("MINUEND").unwrap(),
            matches.value_of("SUBTRAHEND").unwrap(),
        );

        let difference = if lhs > rhs { lhs - rhs } else { rhs - lhs };

        let converted_currencies = convert::convert_currencies(difference, currencies);
        let display_strings: Vec<String> =
            create_display_strings(converted_currencies, matches.is_present("PRINT_FULL"));

        println!("{}", (&display_strings).join(", "));
    } else if let Some(values) = matches.values_of("VALUE") {
        let coins: Vec<&str> = values.collect();
        let total_copper_value = match convert::calculate_total_copper_value(coins) {
            Ok(total_copper_value) => total_copper_value,
            Err(err) => {
                eprintln!("{}", err);
                process::exit(1);
            }
        };

        let converted_currencies = convert::convert_currencies(total_copper_value, currencies);
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
                    Err(format!(
                        "Sterling Error: Can't find configuration file: \"{}\"",
                        &file_path
                    ))
                } else {
                    Ok(config::default_config())
                }
            }
            _ => Err(format!("Sterling Error: {}", error)),
        },
    }
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

fn get_copper_value(currencies: &[Currency], lhs: &str, rhs: &str) -> (usize, usize) {
    let mut rates: HashMap<String, usize> = HashMap::with_capacity(currencies.len());
    for currency in currencies {
        rates.insert(currency.alias.clone(), currency.rate);
    }

    let aliases = currencies
        .iter()
        .cloned()
        .map(|c| c.alias)
        .fold(String::new(), |group, a| group + &a);

    let regex: Regex = Regex::new(&format!("(\\d+)([{}])", aliases)).unwrap();

    let left_hand_side: usize = regex.captures_iter(lhs).fold(0, |sum, cap| {
        let value: usize = cap[1].parse().unwrap();
        let rate: usize = *rates.get(&cap[2]).unwrap();
        let product = value * rate;

        sum + product
    });

    let right_hand_side: usize = regex.captures_iter(rhs).fold(0, |sum, cap| {
        let value: usize = cap[1].parse().unwrap();
        let rate: usize = *rates.get(&cap[2]).unwrap();
        let product = value * rate;

        sum + product
    });

    (left_hand_side, right_hand_side)
}
