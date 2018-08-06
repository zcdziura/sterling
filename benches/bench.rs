#[macro_use]
extern crate criterion;
#[macro_use]
extern crate lazy_static;
extern crate sterling_ops;

use criterion::Criterion;
use sterling_ops::currency::Currency;
use sterling_ops::*;

lazy_static! {
    static ref CURRENCIES: Vec<Currency> = vec![
        Currency::new("penny", 1, "p", Some("pence".to_owned()), None),
        Currency::new("shilling", 100, "s", Some("sterling".to_owned()), None),
        Currency::new("guilder", 10_000, "g", None, None),
        Currency::new("note", 1_000_000, "N", None, Some(true)),
    ];
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("default operation", |b| {
        b.iter(|| default_operation("3p 5s 7s 132c", &CURRENCIES, true))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
