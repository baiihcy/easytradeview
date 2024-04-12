use std::collections::HashMap;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Screener {
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

lazy_static! {
    static ref SCREENER_STRINGS: Vec<(Screener, &'static str)> = vec![
        (Screener::All, "all"),
        (Screener::Australia, "australia"),
        (Screener::Brazil, "brazil"),
        (Screener::Cfd, "cfd"),
        (Screener::Canada, "canada"),
        (Screener::Crypto, "crypto"),
        (Screener::Euronext, "euronext"),
        (Screener::Forex, "forex"),
        (Screener::France, "france"),
        (Screener::Germany, "germany"),
        (Screener::Hongkong, "hongkong"),
        (Screener::India, "india"),
        (Screener::Indonesia, "indonesia"),
        (Screener::Italy, "italy"),
        (Screener::Malaysia, "malaysia"),
        (Screener::Philippines, "philippines"),
        (Screener::Russia, "russia"),
        (Screener::Ksa, "ksa"),
        (Screener::Rsa, "rsa"),
        (Screener::Korea, "korea"),
        (Screener::Spain, "spain"),
        (Screener::Sweden, "sweden"),
        (Screener::Taiwan, "taiwan"),
        (Screener::Thailand, "thailand"),
        (Screener::Turkey, "turkey"),
        (Screener::Uk, "uk"),
        (Screener::America, "america"),
        (Screener::Vietnam, "vietnam"),
    ];
}

impl Screener {
    /// Parses a screener
    pub fn parse(s: &str) -> Result<Screener> {
        lazy_static! {
            static ref MAP: HashMap<&'static str, Screener> =
                SCREENER_STRINGS.iter().map(|x| (x.1, x.0)).collect();
        }
        MAP.get(s)
            .cloned()
            .ok_or(anyhow!("Invalid screener: {}", s))
    }

    /// Get all screener
    pub fn all_screeners() -> &'static [Screener] {
        lazy_static! {
            static ref ALL_SCREENERS: Vec<Screener> =
                SCREENER_STRINGS.iter().map(|x| x.0).collect();
        }
        &ALL_SCREENERS
    }
}

impl AsRef<str> for Screener {
    fn as_ref(&self) -> &str {
        lazy_static! {
            static ref MAP: HashMap<Screener, &'static str> =
                SCREENER_STRINGS.iter().map(|x| (x.0, x.1)).collect();
        }
        MAP.get(self).map(|x| *x).unwrap_or("")
    }
}

impl ToString for Screener {
    fn to_string(&self) -> String {
        self.as_ref().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_screener() {
        assert!(Screener::parse("").is_err());
        assert!(Screener::parse("undefined").is_err());
        assert!(matches!(Screener::parse("all"), Ok(Screener::All)));
    }
}
