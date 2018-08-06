extern crate lazysort;
#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate separator;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use std::collections::HashMap;

pub mod config;
mod convert;
pub mod currency;

use currency::Currency;
use regex::Regex;
use separator::Separatable;

pub fn add_operation(
    augend: &str,
    addend: &str,
    custom_currency_regex: &Regex,
    currencies: &[Currency],
    print_full: bool,
) -> String {
    let lhs =
        convert::calculate_total_copper_value(augend, custom_currency_regex, get_rates(currencies));

    let rhs = convert::calculate_total_copper_value(
        addend,
        &custom_currency_regex,
        get_rates(currencies.as_ref()),
    );

    convert::exchange_currencies(lhs + rhs, currencies, print_full).join(", ")
}

pub fn sub_operation(
    minuend: &str,
    subtrahend: &str,
    custom_currency_regex: &Regex,
    currencies: &[Currency],
    print_full: bool,
) -> String {
    let lhs = convert::calculate_total_copper_value(
        minuend,
        custom_currency_regex,
        get_rates(currencies),
    );

    let rhs = convert::calculate_total_copper_value(
        subtrahend,
        custom_currency_regex,
        get_rates(currencies),
    );

    let difference = if lhs > rhs { lhs - rhs } else { rhs - lhs };
    convert::exchange_currencies(difference, currencies, print_full).join(", ")
}

pub fn mul_operation(
    multiplicand: &str,
    multiplier: usize,
    custom_currency_regex: &Regex,
    currencies: &[Currency],
    print_full: bool,
) -> String {
    let lhs = convert::calculate_total_copper_value(
        multiplicand,
        custom_currency_regex,
        get_rates(currencies),
    );

    convert::exchange_currencies(lhs * multiplier, currencies, print_full).join(", ")
}

pub fn div_operation(
    dividend: &str,
    divisor: usize,
    custom_currency_regex: &Regex,
    currencies: &[Currency],
    print_full: bool,
) -> String {
    let lhs = convert::calculate_total_copper_value(
        dividend,
        custom_currency_regex,
        get_rates(currencies),
    );

    convert::exchange_currencies(lhs / divisor, currencies, print_full).join(", ")
}

pub fn copper_operation(
    values: &str,
    custom_currency_regex: &Regex,
    currencies: &[Currency],
) -> String {
    let copper_value =
        convert::calculate_total_copper_value(values, custom_currency_regex, get_rates(currencies));

    format!("{}c", copper_value.separated_string())
}

pub fn default_operation(values: &str, currencies: &[Currency], print_full: bool) -> String {
    let copper_value = convert::calculate_total_copper_value(
        values,
        &Regex::new(r"(0|(?:[1-9](?:\d+|\d{0,2}(?:,\d{3})*)))+([cegps])").unwrap(),
        get_rates(config::phb_config().as_ref()),
    );

    let exchanged_currencies = convert::exchange_currencies(copper_value, currencies, print_full);

    exchanged_currencies.join(", ")
}

fn get_rates(currencies: &[Currency]) -> HashMap<String, usize> {
    currencies
        .iter()
        .map(|c| c.alias.clone())
        .zip(currencies.iter().map(|c| c.rate))
        .collect()
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use currency::Currency;
    use regex::Regex;

    use super::{
        add_operation, copper_operation, default_operation, div_operation, get_rates,
        mul_operation, sub_operation,
    };

    lazy_static! {
        static ref CURRENCIES: Vec<Currency> = vec![
            Currency::new("penny", 1, "p", Some("pence".to_owned()), None),
            Currency::new("shilling", 100, "s", Some("sterling".to_owned()), None),
        ];
        static ref CUSTOM_CURRENCY_REGEX: Regex = Regex::new(&format!(
            "(\\d+)([{}])",
            CURRENCIES
                .iter()
                .map(|c| c.alias.clone())
                .fold(String::new(), |group, a| group + &a)
        )).unwrap();
    }

    #[test]
    fn test_get_rates() {
        let rates: HashMap<_, _> = vec![("p".to_owned(), 1usize), ("s".to_owned(), 100usize)]
            .into_iter()
            .collect();
        assert_eq!(rates, get_rates(&CURRENCIES));
    }

    #[test]
    fn test_add_operation_same_currencies() {
        let result = "3p".to_owned();
        assert_eq!(
            result,
            add_operation("1p", "2p", &CUSTOM_CURRENCY_REGEX, &CURRENCIES, false)
        );
    }

    #[test]
    fn test_add_operation_diff_currencies() {
        let result = "1s, 1p".to_owned();
        assert_eq!(
            result,
            add_operation("1s", "1p", &CUSTOM_CURRENCY_REGEX, &CURRENCIES, false)
        );
    }

    #[test]
    fn test_sub_operation_smaller_subtrahend() {
        let result = "1p".to_owned();
        assert_eq!(
            result,
            sub_operation("2p", "1p", &CUSTOM_CURRENCY_REGEX, &CURRENCIES, false)
        );
    }

    #[test]
    fn test_sub_operation_larger_subtrahend() {
        let result = "1p".to_owned();
        assert_eq!(
            result,
            sub_operation("1p", "2p", &CUSTOM_CURRENCY_REGEX, &CURRENCIES, false)
        );
    }

    #[test]
    fn test_mul_operation() {
        let result = "6p".to_owned();
        assert_eq!(
            result,
            mul_operation("3p", 2, &CUSTOM_CURRENCY_REGEX, &CURRENCIES, false)
        );
    }

    #[test]
    fn test_div_operation_even_dividend() {
        let result = "2p".to_owned();
        assert_eq!(
            result,
            div_operation("4p", 2, &CUSTOM_CURRENCY_REGEX, &CURRENCIES, false)
        );
    }

    #[test]
    fn test_div_operation_odd_dividend() {
        let result = "1p".to_owned();
        assert_eq!(
            result,
            div_operation("3p", 2, &CUSTOM_CURRENCY_REGEX, &CURRENCIES, false)
        );
    }

    #[test]
    fn test_copper_operation() {
        let result = "103c".to_owned();
        assert_eq!(
            result,
            copper_operation("1s 3p", &CUSTOM_CURRENCY_REGEX, &CURRENCIES)
        );
    }

    #[test]
    fn test_default_operation() {
        let result = "1 shilling";
        assert_eq!(result, default_operation("1g", &CURRENCIES, true));
    }

    #[test]
    fn test_default_operation_plural_output() {
        let result = "2 sterling";
        assert_eq!(result, default_operation("2g", &CURRENCIES, true));
    }
}
