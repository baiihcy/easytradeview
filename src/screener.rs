use anyhow::{anyhow, Result};
use std::str::FromStr;
use strum_macros::{AsRefStr, EnumString, VariantNames};

#[derive(Debug, Clone, PartialEq, Eq, EnumString, AsRefStr, VariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum Screener {
    #[strum(serialize = "")]
    Undefined(String),
    All,
    Australia,
    Brazil,
    Cfd,
    Canada,
    Crypto,
    Euronext,
    Forex,
    France,
    Germany,
    Hongkong,
    India,
    Indonesia,
    Italy,
    Malaysia,
    Philippines,
    Russia,
    Ksa,
    Rsa,
    Korea,
    Spain,
    Sweden,
    Taiwan,
    Thailand,
    Turkey,
    Uk,
    America,
    Vietnam,
}

impl Screener {
    /// Creates a new undefined screener
    pub fn undefined(s: &str) -> Screener {
        Screener::Undefined(s.to_owned())
    }

    /// Parses a screener
    pub fn parse(s: &str) -> Result<Screener> {
        Screener::from_str(s).map_err(|_| anyhow!("Invalid screener: {}", s))
    }

    /// Parses a screener, returning an undefined screener if it fails
    pub fn parse_undefined(s: &str) -> Screener {
        Screener::from_str(s).unwrap_or(Screener::undefined(s))
    }

    /// Get the string representation of the screener.
    pub fn as_str_ref(&self) -> &str {
        if let Screener::Undefined(x) = self {
            x
        } else {
            self.as_ref()
        }
    }

    /// Get all screener
    pub fn all() -> &'static [Screener] {
        const ALL: [Screener; 28] = [
            Screener::All,
            Screener::Australia,
            Screener::Brazil,
            Screener::Cfd,
            Screener::Canada,
            Screener::Crypto,
            Screener::Euronext,
            Screener::Forex,
            Screener::France,
            Screener::Germany,
            Screener::Hongkong,
            Screener::India,
            Screener::Indonesia,
            Screener::Italy,
            Screener::Malaysia,
            Screener::Philippines,
            Screener::Russia,
            Screener::Ksa,
            Screener::Rsa,
            Screener::Korea,
            Screener::Spain,
            Screener::Sweden,
            Screener::Taiwan,
            Screener::Thailand,
            Screener::Turkey,
            Screener::Uk,
            Screener::America,
            Screener::Vietnam,
        ];
        &ALL
    }
}

impl ToString for Screener {
    fn to_string(&self) -> String {
        self.as_str_ref().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screener_from_str() {
        assert!(Screener::from_str("undefined").is_err());
        assert!(Screener::from_str("undefined_scnner").is_err());
        assert_eq!(Screener::from_str(""), Ok(Screener::undefined("")));
        assert_eq!(Screener::from_str("all"), Ok(Screener::All));
    }

    #[test]
    fn test_screener_as_str_ref() {
        assert_eq!(Screener::undefined("undefined_screener").as_ref(), "");
        assert_eq!(
            Screener::undefined("undefined_screener").as_str_ref(),
            "undefined_screener"
        );
        assert_eq!(Screener::All.as_ref(), "all");
        assert_eq!(Screener::All.as_str_ref(), "all");
    }
}
