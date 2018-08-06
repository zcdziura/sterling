#[macro_use]
extern crate clap;
extern crate regex;
extern crate sterling_ops;

use std::process;

use regex::Regex;
use sterling_ops::config;
use sterling_ops::currency::Currency;
use sterling_ops::*;

fn main() {
    let app = clap_app!(sterling =>
        (version: env!("CARGO_PKG_VERSION"))
        (about: "Converts a given D&D 5e currency value to the Silver Standard.")
        (@arg CONFIG: -c --config +takes_value "Specify location of config file; defaults to './sterling-conf.yml'.")
        (@arg PRINT_FULL: -f --full "Print currencies with their full name, rather than with their alias")
        (@arg OPTIONAL: -o --optional "Include currencies marked as optional when converting")
        (@arg VALUE: ... "The value to be converted; should be suffixed with the coin's short-hand abbreviation, i.e. p, g, e, s, or c.")
        (@subcommand add =>
            (about: "Add two currency amounts together; uses the currencies defined in your config file")
            (@arg AUGEND: +required "The augend of the addition function")
            (@arg ADDEND: +required "The addend of the addition function")
        )
        (@subcommand sub =>
            (about: "Subtract two currency amounts from one another; uses the currencies defined in your config file")
            (@arg MINUEND: +required "The minuend of the subtraction function")
            (@arg SUBTRAHEND: +required "The subtrahend of the subtraction function")
        )
        (@subcommand mul =>
            (about: "Multiply a scalar multiplicand by a currency amount; uses the currencies defined in your config file")
            (@arg MULTIPLIER: +required "The scalar multiplier of the multiplication function")
            (@arg MULTIPLICAND: +required ... "The currency values to be multiplied")
        )
        (@subcommand div =>
            (about: "Divide a currency amount by some scalar divisor; uses the currencies defined in your config file")
            (@arg DIVISOR: +required "The scalar divisor of the division function")
            (@arg DIVIDEND: +required ... "The currency values to be divided")
        )
        (@subcommand copper =>
            (about: "Calculate the copper value of a custom currency")
            (@arg VALUE: +required ... "The custom currency value")
        )
    );

    let matches = app.get_matches();
    let config_result = config::load_config(match matches.value_of("CONFIG") {
        Some(file) => file,
        None => "./sterling-conf.yml",
    });

    let currencies: Vec<Currency> =
        match config::parse_currency_config(config_result, matches.value_of("CONFIG")) {
            Ok(currencies) => currencies
                .into_iter()
                .filter(|c| {
                    let is_sub_command = match matches.subcommand_name() {
                        Some(_) => true,
                        None => false,
                    };

                    if is_sub_command {
                        true
                    } else {
                        (!matches.is_present("OPTIONAL") && !c.is_optional())
                            || matches.is_present("OPTIONAL")
                    }
                })
                .collect(),
            Err(error) => {
                eprintln!("{}", error);
                process::exit(1);
            }
        };

    let custom_currency_regex: Regex = Regex::new(&format!(
        "(\\d+)([{}])",
        currencies
            .iter()
            .map(|c| c.alias.clone())
            .fold(String::new(), |group, a| group + &a)
    )).unwrap();

    let operation_result = match matches.subcommand() {
        ("add", Some(command)) => add_operation(
            command.value_of("AUGEND").unwrap(),
            command.value_of("ADDEND").unwrap(),
            &custom_currency_regex,
            &currencies,
            matches.is_present("PRINT_FULL"),
        ),
        ("sub", Some(command)) => sub_operation(
            command.value_of("MINUEND").unwrap(),
            command.value_of("SUBTRAHEND").unwrap(),
            &custom_currency_regex,
            &currencies,
            matches.is_present("PRINT_FULL"),
        ),
        ("mul", Some(command)) => mul_operation(
            &command
                .values_of("MULTIPLICAND")
                .unwrap()
                .collect::<Vec<&str>>()
                .join(" "),
            command
                .value_of("MULTIPLIER")
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            &custom_currency_regex,
            &currencies,
            matches.is_present("PRINT_FULL"),
        ),
        ("div", Some(command)) => div_operation(
            &command
                .values_of("DIVIDEND")
                .unwrap()
                .collect::<Vec<&str>>()
                .join(" "),
            command
                .value_of("DIVISOR")
                .unwrap()
                .parse::<usize>()
                .unwrap(),
            &custom_currency_regex,
            &currencies,
            matches.is_present("PRINT_FULL"),
        ),
        ("copper", Some(command)) => copper_operation(
            &command
                .values_of("VALUE")
                .unwrap()
                .collect::<Vec<&str>>()
                .join(" "),
            &custom_currency_regex,
            &currencies,
        ),
        _ => {
            if let Some(values) = matches.values_of("VALUE") {
                default_operation(
                    &values.collect::<Vec<&str>>().join(" "),
                    &currencies,
                    matches.is_present("PRINT_FULL"),
                )
            } else {
                eprintln!("Sterling Error: please enter at least one value; should be suffixed with the coin's short-hand abbreviation, i.e. p, g, e, s, or c.");
                process::exit(1);
            }
        }
    };

    println!("{}", operation_result);
}
