use currency::Currency;
use regex::Regex;

pub fn convert_to_copper(amount: usize, coin_denomination: &str) -> usize {
    match coin_denomination {
        "p" => amount * 1000,
        "g" => amount * 100,
        "e" => amount * 50,
        "s" => amount * 10,
        "c" => amount,
        _ => unreachable!("Invalid coin type; must be a valid coin found in the PHB."),
    }
}

pub fn calculate_total_copper_value(coins: Vec<&str>) -> Result<usize, &'static str> {
    let regex: Regex = Regex::new(r"(\d+)([cegps])").unwrap();
    for coin in coins.iter() {
        if let None = regex.captures(coin) {
            return Err(
                "Sterling Error: Invalid coin value. Make sure all coins are denoted properly.",
            );
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

pub fn convert_currencies(copper_value: usize, currencies: Vec<Currency>) -> Vec<Currency> {
    exchange(copper_value, currencies)
        .iter()
        .filter(|c| (*c).value.unwrap_or(0) > 0)
        .cloned()
        .collect()
}

pub fn exchange(copper: usize, mut currencies: Vec<Currency>) -> Vec<Currency> {
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

#[cfg(test)]
mod tests {
    use convert::*;
    use currency::Currency;

    lazy_static! {
        static ref STANDARD_CURRENCIES: [Currency; 4] = [
            Currency::new("platinum", 1000000, "p", None, None),
            Currency::new("gold", 10000, "g", None, None),
            Currency::new("silver", 100, "s", None, None),
            Currency::new("copper", 1, "c", None, None),
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
            Currency::new("platinum", 1000000, "p", None, None).with_value(0),
            Currency::new("gold", 10000, "g", None, None).with_value(0),
            Currency::new("silver", 100, "s", None, None).with_value(0),
            Currency::new("copper", 1, "c", None, None).with_value(1),
        ];

        assert_eq!(currencies, exchange(1, STANDARD_CURRENCIES.to_vec()));
    }

    #[test]
    fn test_exchange_to_silver() {
        let currencies = vec![
            Currency::new("platinum", 1000000, "p", None, None).with_value(0),
            Currency::new("gold", 10000, "g", None, None).with_value(0),
            Currency::new("silver", 100, "s", None, None).with_value(1),
            Currency::new("copper", 1, "c", None, None).with_value(0),
        ];

        assert_eq!(currencies, exchange(100, STANDARD_CURRENCIES.to_vec()));
    }

    #[test]
    fn test_exchange_to_gold() {
        let currencies = vec![
            Currency::new("platinum", 1000000, "p", None, None).with_value(0),
            Currency::new("gold", 10000, "g", None, None).with_value(1),
            Currency::new("silver", 100, "s", None, None).with_value(0),
            Currency::new("copper", 1, "c", None, None).with_value(0),
        ];

        assert_eq!(currencies, exchange(10000, STANDARD_CURRENCIES.to_vec()));
    }

    #[test]
    fn test_exchange_to_platinum() {
        let currencies = vec![
            Currency::new("platinum", 1000000, "p", None, None).with_value(1),
            Currency::new("gold", 10000, "g", None, None).with_value(0),
            Currency::new("silver", 100, "s", None, None).with_value(0),
            Currency::new("copper", 1, "c", None, None).with_value(0),
        ];

        assert_eq!(currencies, exchange(1000000, STANDARD_CURRENCIES.to_vec()));
    }
}
