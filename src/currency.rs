use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Currency {
    pub name: String,
    pub rate: usize,
    pub alias: String,
    pub plural: Option<String>,
    pub optional: Option<bool>,
}

impl Currency {
    pub fn new(
        name: &str,
        rate: usize,
        alias: &str,
        plural: Option<String>,
        optional: Option<bool>,
    ) -> Currency {
        Currency {
            name: name.to_owned(),
            rate,
            alias: alias.to_owned(),
            plural,
            optional,
        }
    }

    pub fn is_optional(&self) -> bool {
        match self.optional {
            Some(optional) => optional,
            None => false,
        }
    }
}

impl Ord for Currency {
    fn cmp(&self, other: &Currency) -> Ordering {
        other.rate.cmp(&self.rate)
    }
}

impl PartialOrd for Currency {
    fn partial_cmp(&self, other: &Currency) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Currency {
    fn eq(&self, other: &Currency) -> bool {
        self.rate == other.rate
    }
}
