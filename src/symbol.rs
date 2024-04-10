use super::*;
use std::collections::HashMap;

pub trait SymbolValues {
    type Field: AsRef<Field> + Eq + PartialEq + std::hash::Hash + Clone;

    /// Returns the symbol name.
    fn symbol(&self) -> &str;

    /// Returns the symbol values.
    fn values(&self) -> &HashMap<Self::Field, Value>;

    /// Returns the symbol values.
    fn values_mut(&mut self) -> &mut HashMap<Self::Field, Value>;

    /// Returns the symbol values as f64.
    fn get_f64_values(&self) -> HashMap<Self::Field, f64> {
        let mut values: HashMap<Self::Field, f64> = HashMap::new();
        for (f, v) in self.values() {
            if let Some(x) = v.as_f64() {
                values.insert(f.clone(), x);
            }
        }
        values
    }
}

#[derive(Debug, Default)]
pub struct SimpleSymbolValues {
    symbol: String,
    values: HashMap<Field, Value>,
}

impl SimpleSymbolValues {
    /// Creates a new `SimpleSymbolValues` instance.
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            values: HashMap::new(),
        }
    }
}

impl SymbolValues for SimpleSymbolValues {
    type Field = Field;

    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn values(&self) -> &HashMap<Self::Field, Value> {
        &self.values
    }

    fn values_mut(&mut self) -> &mut HashMap<Self::Field, Value> {
        &mut self.values
    }
}

#[derive(Debug, Default)]
pub struct TimedSymbolValues {
    symbol: String,
    values: HashMap<FieldWithInterval, Value>,
}

impl TimedSymbolValues {
    /// Creates a new `TimedSymbolValues` instance.
    pub fn new(symbol: &str) -> Self {
        Self {
            symbol: symbol.to_owned(),
            values: HashMap::new(),
        }
    }

    /// Filters the values by interval, returning a `SimpleSymbolValues` instance.
    pub fn filter_interval(&self, interval: Interval) -> SimpleSymbolValues {
        let mut simple_values = SimpleSymbolValues::new(&self.symbol);
        simple_values.values = self
            .values
            .iter()
            .filter(|(k, _)| k.interval == interval)
            .map(|(k, v)| (k.field.clone(), v.clone()))
            .collect();
        simple_values
    }
}

impl SymbolValues for TimedSymbolValues {
    type Field = FieldWithInterval;

    fn symbol(&self) -> &str {
        &self.symbol
    }

    fn values(&self) -> &HashMap<Self::Field, Value> {
        &self.values
    }

    fn values_mut(&mut self) -> &mut HashMap<Self::Field, Value> {
        &mut self.values
    }
}
