use super::*;
use anyhow::{anyhow, Result};
use field_attr::FieldAttr;
pub use serde_json::Value;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Field {
    Undefined(String),
    UndefinedWithoutInterval(String),
    RecommendOther,
    RecommendAll,
    RecommendMA,
    Name,
    Exchange,
    Description,
    Type,
    SubType,
    UpdateMode,
    PriceScale,
    MinMov,
    MinMove2,
    Fractional,
    Open,
    Close,
    Volume,
    Change,
    ChangeAbs,
    Gap,
    HighW,
    LowW,
    PerfW,
    High1M,
    Low1M,
    Perf1M,
    High3M,
    Low3M,
    Perf3M,
    High6M,
    Low6M,
    Perf6M,
    HighAll,
    LowAll,
    RSI,
    RSI1,
    RSI7,
    StochK,
    StochD,
    StochK1,
    StochD1,
    CCI20,
    CCI201,
    ADX,
    ADXplusDI,
    ADXminusDI,
    ADXplusDI1,
    ADXminusDI1,
    AO,
    AO1,
    AO2,
    Mom,
    Mom1,
    MACDmacd,
    MACDsignal,
    RecStochRSI,
    StochRSIK,
    StochRSID,
    RecWR,
    WR,
    RecBBPower,
    BBPower,
    RecUO,
    UO,
    EMA5,
    SMA5,
    EMA10,
    SMA10,
    EMA20,
    SMA20,
    EMA30,
    SMA30,
    EMA50,
    SMA50,
    EMA100,
    SMA100,
    EMA200,
    SMA200,
    RecIchimoku,
    IchimokuBLine,
    RecVWMA,
    VWMA,
    RecHullMA9,
    HullMA9,
    PivotMClassicS3,
    PivotMClassicS2,
    PivotMClassicS1,
    PivotMClassicMiddle,
    PivotMClassicR1,
    PivotMClassicR2,
    PivotMClassicR3,
    PivotMFibonacciS3,
    PivotMFibonacciS2,
    PivotMFibonacciS1,
    PivotMFibonacciMiddle,
    PivotMFibonacciR1,
    PivotMFibonacciR2,
    PivotMFibonacciR3,
    PivotMCamarillaS3,
    PivotMCamarillaS2,
    PivotMCamarillaS1,
    PivotMCamarillaMiddle,
    PivotMCamarillaR1,
    PivotMCamarillaR2,
    PivotMCamarillaR3,
    PivotMWoodieS3,
    PivotMWoodieS2,
    PivotMWoodieS1,
    PivotMWoodieMiddle,
    PivotMWoodieR1,
    PivotMWoodieR2,
    PivotMWoodieR3,
    PivotMDemarkS1,
    PivotMDemarkMiddle,
    PivotMDemarkR1,
    PSAR,
    BBlower,
    BBupper,
    Price52WeekHigh,
    Price52WeekLow,
    AroonDown,
    AroonUp,
    ADR,
    ATR,
    AverageVolume10dCalc,
    PerfY,
    PerfYTD,
    AverageVolume30dCalc,
    AverageVolume60dCalc,
    AverageVolume90dCalc,
    ChangeFromOpenAbs,
    ChangeFromOpen,
    DonchCh20Lower,
    DonchCh20Upper,
    IchimokuCLine,
    IchimokuLead1,
    IchimokuLead2,
    KltChnllower,
    KltChnlupper,
    MarketCapCalc,
    ROC,
    RelativeVolume10dCalc,
    VolatilityD,
    VolatilityM,
    VolatilityW,
    VWAP,
    CandleAbandonedBabyBearish,
    CandleAbandonedBabyBullish,
    CandleEngulfingBearish,
    CandleHaramiBearish,
    CandleEngulfingBullish,
    CandleHaramiBullish,
    CandleDoji,
    CandleDojiDragonfly,
    CandleEveningStar,
    CandleDojiGravestone,
    CandleHammer,
    CandleHangingMan,
    CandleInvertedHammer,
    CandleKickingBearish,
    CandleKickingBullish,
    CandleLongShadowLower,
    CandleLongShadowUpper,
    CandleMarubozuBlack,
    CandleMarubozuWhite,
    CandleMorningStar,
    CandleShootingStar,
    CandleSpinningTopBlack,
    CandleSpinningTopWhite,
    Candle3BlackCrows,
    Candle3WhiteSoldiers,
    CandleTriStarBearish,
    CandleTriStarBullish,
}

impl Field {
    /// Parses a field from a string.
    pub fn parse(s: &str) -> Result<Field> {
        FieldAttr::find_by_name(s)
            .and_then(|x| Some(x.field.clone()))
            .ok_or(anyhow!("Invalid field: {}", s))
    }

    /// Parses a field, returning an undefined field if it fails
    pub fn parse_undefined(s: &str) -> Field {
        let parts: Vec<&str> = s.split("|").collect();
        let undefined_func = if parts.len() > 1 {
            Field::undefined_without_interval
        } else {
            Field::undefined
        };
        let field = parts.get(0).map_or(undefined_func(""), |x| {
            Field::parse(x).unwrap_or(undefined_func(x))
        });
        field
    }

    /// Parses a field and an interval from a string.
    pub fn parse_with_interval(s: &str) -> Result<(Field, Option<Interval>)> {
        let parts: Vec<&str> = s.split("|").collect();
        let field = parts
            .get(0)
            .and_then(|x| Field::parse(x).ok())
            .ok_or(anyhow!("parse field error"))?;
        let interval = parts.get(1).map(|x| Interval::parse_undefined(x));
        Ok((field, interval))
    }

    /// Parses a field with interval, returning an undefined field if it fails
    pub fn parse_undefined_with_interval(s: &str) -> (Field, Option<Interval>) {
        let parts: Vec<&str> = s.split("|").collect();
        let field = parts.get(0).map_or(Field::undefined(""), |x| {
            Field::parse(x).unwrap_or(Field::undefined(x))
        });
        let interval = parts.get(1).map(|x| Interval::parse_undefined(x));
        (field, interval)
    }

    /// Creates an undefined field.
    pub fn undefined(x: &str) -> Field {
        Field::Undefined(x.to_owned())
    }

    /// Creates an undefined field without interval.
    pub fn undefined_without_interval(x: &str) -> Field {
        Field::UndefinedWithoutInterval(x.to_owned())
    }

    /// Checkes if the field is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            Field::Undefined(x) => x.is_empty(),
            Field::UndefinedWithoutInterval(x) => x.is_empty(),
            _ => false,
        }
    }

    /// Get the string representation of the field.
    pub fn to_str_ref(&self) -> &str {
        match self {
            Field::Undefined(x) => x,
            Field::UndefinedWithoutInterval(x) => x,
            _ => self.as_ref(),
        }
    }

    /// Get the string representation of the field with interval.
    pub fn to_string_with_interval(&self, interval: &Interval) -> String {
        match self {
            Field::Undefined(x) => x.to_string() + &interval.as_field_suffix(),
            Field::UndefinedWithoutInterval(x) => x.to_string(),
            Field::Name => self.to_string(),
            Field::Exchange => self.to_string(),
            Field::Description => self.to_string(),
            Field::SubType => self.to_string(),
            Field::UpdateMode => self.to_string(),
            _ => self.to_string() + &interval.as_field_suffix(),
        }
    }

    /// Converts the field to a `FieldWithInterval` instance.
    pub fn with_interval(self, interval: &Interval) -> FieldWithInterval {
        FieldWithInterval::new(self, interval.clone())
    }

    /// Get the recommended fields.
    pub fn recommends() -> &'static [Field] {
        static VALUES: [Field; 3] = [
            Field::RecommendAll,
            Field::RecommendMA,
            Field::RecommendOther,
        ];
        &VALUES
    }

    /// Get the oscillator indicators fields.
    pub fn oscillator_indicators() -> &'static [Field] {
        static VALUES: [Field; 30] = [
            Field::RSI,
            Field::RSI1,
            Field::RSI7,
            Field::StochK,
            Field::StochD,
            Field::StochK1,
            Field::StochD1,
            Field::CCI20,
            Field::CCI201,
            Field::ADX,
            Field::ADXplusDI,
            Field::ADXminusDI,
            Field::ADXplusDI1,
            Field::ADXminusDI1,
            Field::AO,
            Field::AO1,
            Field::AO2,
            Field::Mom,
            Field::Mom1,
            Field::MACDmacd,
            Field::MACDsignal,
            Field::StochRSIK,
            Field::StochRSID,
            Field::RecStochRSI,
            Field::WR,
            Field::RecWR,
            Field::BBPower,
            Field::RecBBPower,
            Field::UO,
            Field::RecUO,
        ];
        &VALUES
    }

    /// Get the moving average indicators fields.
    pub fn move_average_indicators() -> &'static [Field] {
        static VALUES: [Field; 20] = [
            Field::EMA5,
            Field::SMA5,
            Field::EMA10,
            Field::SMA10,
            Field::EMA20,
            Field::SMA20,
            Field::EMA30,
            Field::SMA30,
            Field::EMA50,
            Field::SMA50,
            Field::EMA100,
            Field::SMA100,
            Field::EMA200,
            Field::SMA200,
            Field::VWMA,
            Field::RecVWMA,
            Field::HullMA9,
            Field::RecHullMA9,
            Field::IchimokuBLine,
            Field::RecIchimoku,
        ];
        &VALUES
    }

    /// Get the pivot indicators fields.
    pub fn pivot_indicators() -> &'static [Field] {
        static VALUES: [Field; 31] = [
            Field::PivotMClassicS3,
            Field::PivotMClassicS2,
            Field::PivotMClassicS1,
            Field::PivotMClassicMiddle,
            Field::PivotMClassicR1,
            Field::PivotMClassicR2,
            Field::PivotMClassicR3,
            Field::PivotMFibonacciS3,
            Field::PivotMFibonacciS2,
            Field::PivotMFibonacciS1,
            Field::PivotMFibonacciMiddle,
            Field::PivotMFibonacciR1,
            Field::PivotMFibonacciR2,
            Field::PivotMFibonacciR3,
            Field::PivotMCamarillaS3,
            Field::PivotMCamarillaS2,
            Field::PivotMCamarillaS1,
            Field::PivotMCamarillaMiddle,
            Field::PivotMCamarillaR1,
            Field::PivotMCamarillaR2,
            Field::PivotMCamarillaR3,
            Field::PivotMWoodieS3,
            Field::PivotMWoodieS2,
            Field::PivotMWoodieS1,
            Field::PivotMWoodieMiddle,
            Field::PivotMWoodieR1,
            Field::PivotMWoodieR2,
            Field::PivotMWoodieR3,
            Field::PivotMDemarkS1,
            Field::PivotMDemarkMiddle,
            Field::PivotMDemarkR1,
        ];
        &VALUES
    }
}

impl AsRef<Field> for Field {
    fn as_ref(&self) -> &Field {
        &self
    }
}

impl AsRef<str> for Field {
    fn as_ref(&self) -> &str {
        match self {
            Field::Undefined(x) => x,
            Field::UndefinedWithoutInterval(x) => x,
            _ => FieldAttr::find(self)
                .and_then(|x| Some(x.name))
                .unwrap_or(""),
        }
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        let s: &str = self.as_ref();
        s.to_owned()
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct FieldWithInterval {
    pub field: Field,
    pub interval: Interval,
}

impl FieldWithInterval {
    /// Create a new `FieldWithInterval` instance.
    pub fn new(field: Field, interval: Interval) -> Self {
        Self { field, interval }
    }

    /// Parses a string to create a `FieldWithInterval` instance without requiring the interval to be defined.
    /// This function assumes a default interval if none is provided in the input string.
    pub fn parse(s: &str) -> Result<FieldWithInterval> {
        let (field, interval) = Field::parse_with_interval(s)?;
        let interval = interval.unwrap_or_default();
        Ok(Self { field, interval })
    }

    /// Parses a string to create a `FieldWithInterval` instance, using a default interval if none is specified in the input.
    pub fn parse_with_default_interval(
        s: &str,
        default_interval: &Interval,
    ) -> Result<FieldWithInterval> {
        let (field, interval) = Field::parse_with_interval(s)?;
        let interval = interval.unwrap_or(default_interval.clone());
        Ok(Self { field, interval })
    }

    /// Parses a string to create a `FieldWithInterval` instance without requiring the interval to be defined.
    /// If the field is not found, it will be set to an undefined field.
    /// If the interval is not specified in the input string, it will be set to `Interval::default()`.
    pub fn parse_undefined(s: &str) -> FieldWithInterval {
        let (field, interval) = Field::parse_undefined_with_interval(s);
        let interval = interval.unwrap_or_default();
        Self { field, interval }
    }

    /// Parses a string to create a `FieldWithInterval` instance, using a default interval if none is specified in the input.
    /// If the field is not found, it will be set to an undefined field.
    pub fn parse_undefined_with_default_interval(
        s: &str,
        default_interval: &Interval,
    ) -> FieldWithInterval {
        let (field, interval) = Field::parse_undefined_with_interval(s);
        let interval = interval.unwrap_or(default_interval.clone());
        Self { field, interval }
    }

    /// Get the reference to the field.
    pub fn field(&self) -> &Field {
        &self.field
    }

    /// Get the reference to the interval.
    pub fn interval(&self) -> &Interval {
        &self.interval
    }
}

impl AsRef<Field> for FieldWithInterval {
    fn as_ref(&self) -> &Field {
        &self.field
    }
}

impl From<&str> for FieldWithInterval {
    fn from(s: &str) -> Self {
        FieldWithInterval::parse_undefined(s)
    }
}

impl ToString for FieldWithInterval {
    fn to_string(&self) -> String {
        self.field.to_string_with_interval(&self.interval)
    }
}

impl std::fmt::Debug for FieldWithInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.field.to_string_with_interval(&self.interval))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_field() {
        assert!(Field::parse("undefined").is_err());
        assert!(matches!(Field::parse("open"), Ok(Field::Open)));
        assert_eq!(
            Field::parse_undefined("undefined"),
            Field::undefined("undefined")
        );
    }

    #[test]
    fn test_field_to_string() {
        let open_str: &str = Field::Open.as_ref();
        assert_eq!(open_str, "open");
        assert_eq!(Field::Open.to_string(), "open");

        let undefined_field = Field::undefined("undefined");
        let undefined_str: &str = undefined_field.as_ref();
        assert_eq!(undefined_str, "undefined");
        assert_eq!(undefined_str.to_string(), "undefined");
    }

    #[test]
    fn test_parse_field_with_interval() {
        assert!(FieldWithInterval::parse("undefined").is_err());

        let open_with_default_interval = FieldWithInterval::new(Field::Open, Interval::default());
        assert!(
            matches!(FieldWithInterval::parse("open"), Ok(x) if x == open_with_default_interval)
        );

        let open_with_1hour = FieldWithInterval::new(Field::Open, Interval::Hour1);
        assert!(matches!(FieldWithInterval::parse("open|1h"), Ok(x) if x == open_with_1hour));
        assert!(
            matches!(FieldWithInterval::parse_with_default_interval("open", &Interval::Hour1), Ok(x) if x == open_with_1hour)
        );

        let open_with_undefined = FieldWithInterval::new(Field::Open, Interval::undefined("5h"));
        assert!(matches!(FieldWithInterval::parse("open|5h"), Ok(x) if x == open_with_undefined));

        let undefined_field =
            FieldWithInterval::new(Field::undefined("undefined"), Interval::undefined("5h"));
        assert_eq!(
            FieldWithInterval::parse_undefined("undefined|5h"),
            undefined_field
        );
        assert_eq!(
            FieldWithInterval::parse_undefined_with_default_interval(
                "undefined",
                &Interval::undefined("5h")
            ),
            undefined_field
        );
    }

    #[test]
    fn test_field_with_interval_to_string() {
        let open_with_1hour = FieldWithInterval::new(Field::Open, Interval::Hour1);
        assert_eq!(open_with_1hour.to_string(), "open|60");

        let open_with_undefined = FieldWithInterval::new(Field::Open, Interval::undefined("5h"));
        assert_eq!(open_with_undefined.to_string(), "open|5h");

        let undefined_field =
            FieldWithInterval::new(Field::undefined("undefined"), Interval::undefined("5h"));
        assert_eq!(undefined_field.to_string(), "undefined|5h");
    }
}
