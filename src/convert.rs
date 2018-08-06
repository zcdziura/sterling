use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;
use separator::Separatable;

use currency::Currency;
use lazysort::Sorted;

pub fn calculate_total_copper_value(
    values: &str,
    currency_regex: &Regex,
    rates: HashMap<String, usize>,
) -> usize {
    currency_regex
        .captures_iter(&values)
        .fold(0, |sum, capture| {
            let value: usize = str::replace(&capture[1], ",", "").parse().unwrap();
            let rate: &usize = rates.get(&capture[2]).unwrap();
            let product = value * rate;

            sum + product
        })
}

pub fn exchange_currencies(
    copper_value: usize,
    currencies: &[Currency],
    print_full_name: bool,
) -> Vec<String> {
    let mut val = copper_value;
    currencies
        .iter()
        .sorted()
        .filter(|currency| currency.optional.unwrap_or(true))
        .map(|currency| {
            let value = val / currency.rate;
            val = val % currency.rate;

            (
                value,
                if print_full_name {
                    if value > 1 {
                        match (&currency).plural {
                            Some(ref plural) => String::from_str(plural).unwrap(),
                            None => format!("{}s", &currency.name),
                        }
                    } else {
                        String::from_str(&currency.name).unwrap()
                    }
                } else {
                    String::from_str(&currency.alias).unwrap()
                },
            )
        })
        .filter(|tuple| tuple.0 > 0)
        .map(|tuple| {
            format!(
                "{}{}{}",
                tuple.0.separated_string(),
                if tuple.1.len() > 1 { " " } else { "" },
                tuple.1
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{calculate_total_copper_value, exchange_currencies};
    use currency::Currency;
    use regex::Regex;

    lazy_static! {
        static ref STANDARD_CURRENCIES: Vec<Currency> = vec![
            Currency::new("guilder", 10_000, "g", None, None),
            Currency::new("shilling", 100, "s", Some("sterling".to_owned()), None),
            Currency::new("penny", 1, "p", Some("pence".to_owned()), None),
        ];
        static ref CURRENCY_REGEX: Regex = Regex::new(r"(\d+)([Ngsp])").unwrap();
    }

    #[test]
    fn test_calculate_total_copper_value() {
        let rates: HashMap<String, usize> = vec![
            ("g".to_owned(), 10_000usize),
            ("s".to_owned(), 100usize),
            ("p".to_owned(), 1usize),
        ].into_iter()
            .collect();

        let result = 10101usize;
        assert_eq!(
            result,
            calculate_total_copper_value("1g 1s 1p", &CURRENCY_REGEX, rates)
        );
    }

    #[test]
    fn test_exchange_currencies() {
        let result = vec!["1g".to_owned(), "1s".to_owned(), "1p".to_owned()];
        assert_eq!(
            result,
            exchange_currencies(10101, &STANDARD_CURRENCIES, false)
        );
    }
}
