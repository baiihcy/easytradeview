use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Interval {
    Undefined(String),
    Min1,
    Min5,
    Min15,
    Min30,
    Hour1,
    Hour2,
    Hour4,
    Day1,
    Week1,
    Month1,
}

impl Interval {
    /// Create an undefined interval.
    pub fn undefined(s: &str) -> Interval {
        Interval::Undefined(s.to_owned())
    }

    /// Parses an interval from a string.
    pub fn parse(s: &str) -> Result<Interval> {
        match s {
            "1" | "1m" => Ok(Interval::Min1),
            "5" | "5m" => Ok(Interval::Min5),
            "15" | "15m" => Ok(Interval::Min15),
            "60" | "1h" => Ok(Interval::Hour1),
            "120" | "2h" => Ok(Interval::Hour2),
            "240" | "4h" => Ok(Interval::Hour4),
            "1d" => Ok(Interval::Day1),
            "1W" => Ok(Interval::Week1),
            "1M" => Ok(Interval::Month1),
            _ => Err(anyhow!("Invalid interval: {}", s)),
        }
    }

    /// Parses an interval from a string, or return the undefined interval if it fails.
    pub fn parse_undefined(s: &str) -> Interval {
        Self::parse(s).unwrap_or(Self::undefined(s))
    }

    /// Get the string representation of the interval as a suffix for a field.
    pub fn as_field_suffix(&self) -> String {
        match self {
            Interval::Day1 => "".to_owned(),
            _ => "|".to_owned() + self.as_ref(),
        }
    }

    pub fn all_intervals() -> &'static [Interval] {
        &[
            Interval::Min1,
            Interval::Min5,
            Interval::Min15,
            Interval::Min30,
            Interval::Hour1,
            Interval::Hour2,
            Interval::Hour4,
            Interval::Day1,
            Interval::Week1,
            Interval::Month1,
        ]
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::Day1
    }
}

impl AsRef<str> for Interval {
    fn as_ref(&self) -> &str {
        match self {
            Interval::Undefined(x) => x,
            Interval::Min1 => "1",
            Interval::Min5 => "5",
            Interval::Min15 => "15",
            Interval::Min30 => "30",
            Interval::Hour1 => "60",
            Interval::Hour2 => "120",
            Interval::Hour4 => "240",
            Interval::Day1 => "1d",
            Interval::Week1 => "1W",
            Interval::Month1 => "1M",
        }
    }
}

impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Interval::Undefined(x) => x,
            Interval::Min1 => "1m",
            Interval::Min5 => "5m",
            Interval::Min15 => "15m",
            Interval::Min30 => "30m",
            Interval::Hour1 => "1h",
            Interval::Hour2 => "2h",
            Interval::Hour4 => "4h",
            Interval::Day1 => "1d",
            Interval::Week1 => "1w",
            Interval::Month1 => "1M",
        };
        write!(f, "{}", s)
    }
}
