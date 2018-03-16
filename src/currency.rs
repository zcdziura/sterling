use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, Serialize, Deserialize)]
pub struct Currency {
    pub name: String,
    pub rate: usize,
    pub value: Option<usize>,
    pub alias: String,
    pub plural: Option<String>,
}

impl Currency {
    pub fn new(name: &str, rate: usize, alias: &str, plural: Option<String>) -> Currency {
        Currency {
            name: name.to_owned(),
            rate,
            value: None,
            alias: alias.to_owned(),
            plural,
        }
    }

    pub fn with_value(&mut self, value: usize) -> Currency {
        Currency {
            name: self.name.clone(),
            rate: self.rate,
            value: Some(value),
            alias: self.alias.clone(),
            plural: self.plural.clone(),
        }
    }

    pub fn alias_display(&self) -> String {
        self.value.unwrap_or(0).to_string() + &self.alias
    }

    pub fn full_display(&self) -> String {
        let mut display = self.value.unwrap_or(0).to_string() + " ";

        if self.value.unwrap_or(0) > 1 {
            match &self.plural {
                &Some(ref plural) => display = display + &plural,
                &None => display = display + &self.name,
            }
        } else {
            display = display + &self.name;
        }

        display
    }
}

// impl Display for Currency {
//     fn
// }

impl Ord for Currency {
    fn cmp(&self, other: &Currency) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Currency {
    fn partial_cmp(&self, other: &Currency) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Currency {
    fn eq(&self, other: &Currency) -> bool {
        self.value == other.value
    }
}
