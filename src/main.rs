#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fmt;
use std::ops::Add;
use std::process;

use regex::Regex;

fn main() {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)([cegps])?").unwrap();
    }

    let app = clap_app!(sterling =>
        (version: "0.1.0")
        (about: "Converts a given D&D 5e currency value to the Silver Standard.")
        (@arg VALUE: ... "The value to be converted; should be suffixed with the coin's short-hand abbreviation, i.e. p, g, e, s, or c. Defaults coin type to 'g'.")
    );

    if let Some(values) = app.get_matches().values_of("VALUE") {
        let coins: Vec<&str> = values.collect();

        let total_copper_value: usize = coins
            .iter()
            .map(|coin| {
                if let Some(captures) = RE.captures(coin) {
                    let amount: usize = captures.get(1).unwrap().as_str().parse().unwrap();
                    let denomination = captures.get(2).map_or("g", |d| d.as_str());
                    
                    convert_to_copper(amount, denomination)
                } else {
                    panic!("Error: Invalid coin value \"{}\"", coin);
                }
            })
            .fold(0 as usize, |total, value| total + value);

        println!("{}", exchange_copper(total_copper_value));
    } else {
        println!("Please enter at least one value; should be suffixed with the coin's short-hand abbreviation, i.e. p, g, e, s, or c. Defaults coin type to 'g'.");
        process::exit(1);
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

fn exchange_copper(copper: usize) -> CurrencyValue {
    CurrencyValue {
        platinum: copper / 1000000,
        gold: (copper % 1000000) / 10000,
        silver: ((copper % 1000000) % 10000) / 100,
        copper: ((copper % 1000000) % 10000) % 100,
    }
}

#[derive(Debug)]
struct CurrencyValue {
    platinum: usize,
    gold: usize,
    silver: usize,
    copper: usize,
}

impl fmt::Display for CurrencyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &CurrencyValue {
            platinum,
            gold,
            silver,
            copper,
        } = self;

        let mut output = String::new();

        if platinum > 0 {
            output = output + &format!("{}p ", platinum);
        }

        if gold > 0 {
            output = output + &format!("{}g ", gold);
        }

        if silver > 0 {
            output = output + &format!("{}s ", silver);
        }

        if copper > 0 {
            output = output + &format!("{}c", copper);
        } else if output.is_empty() {
            output.push_str("0cp");
        }

        write!(f, "{}", output)
    }
}

impl Add for CurrencyValue {
    type Output = CurrencyValue;

    fn add(self, other: CurrencyValue) -> CurrencyValue {
        CurrencyValue {
            platinum: self.platinum + other.platinum,
            gold: self.gold + other.gold,
            silver: self.silver + other.silver,
            copper: self.copper + other.copper,
        }
    }
}
