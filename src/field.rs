use super::*;
use anyhow::{anyhow, Result};
pub use serde_json::Value;
use std::str::FromStr;
use strum_macros::{AsRefStr, EnumString};

#[derive(Debug, Eq, PartialEq, Hash, Clone, EnumString, AsRefStr)]
pub enum Field {
    #[strum(serialize = "")]
    Undefined(String),
    #[strum(serialize = "")]
    UndefinedWithoutInterval(String),
    #[strum(serialize = "Recommend.Other")]
    RecommendOther,
    #[strum(serialize = "Recommend.All")]
    RecommendAll,
    #[strum(serialize = "Recommend.MA")]
    RecommendMA,
    #[strum(serialize = "name")]
    Name,
    #[strum(serialize = "exchange")]
    Exchange,
    #[strum(serialize = "description")]
    Description,
    #[strum(serialize = "type")]
    Type,
    #[strum(serialize = "subtype")]
    SubType,
    #[strum(serialize = "update_mode")]
    UpdateMode,
    #[strum(serialize = "pricescale")]
    PriceScale,
    #[strum(serialize = "minmov")]
    MinMov,
    #[strum(serialize = "minmove2")]
    MinMove2,
    #[strum(serialize = "fractional")]
    Fractional,
    #[strum(serialize = "open")]
    Open,
    #[strum(serialize = "close")]
    Close,
    #[strum(serialize = "volume")]
    Volume,
    #[strum(serialize = "change")]
    Change,
    #[strum(serialize = "change_abs")]
    ChangeAbs,
    #[strum(serialize = "gap")]
    Gap,
    #[strum(serialize = "High.W")]
    HighW,
    #[strum(serialize = "Low.W")]
    LowW,
    #[strum(serialize = "Perf.W")]
    PerfW,
    #[strum(serialize = "High.1M")]
    High1M,
    #[strum(serialize = "Low.1M")]
    Low1M,
    #[strum(serialize = "Perf.1M")]
    Perf1M,
    #[strum(serialize = "High.3M")]
    High3M,
    #[strum(serialize = "Low.3M")]
    Low3M,
    #[strum(serialize = "Perf.3M")]
    Perf3M,
    #[strum(serialize = "High.6M")]
    High6M,
    #[strum(serialize = "Low.6M")]
    Low6M,
    #[strum(serialize = "Perf.6M")]
    Perf6M,
    #[strum(serialize = "High.All")]
    HighAll,
    #[strum(serialize = "Low.All")]
    LowAll,
    #[strum(serialize = "RSI")]
    RSI,
    #[strum(serialize = "RSI[1]")]
    RSI1,
    #[strum(serialize = "RSI7")]
    RSI7,
    #[strum(serialize = "Stoch.K")]
    StochK,
    #[strum(serialize = "Stoch.D")]
    StochD,
    #[strum(serialize = "Stoch.K[1]")]
    StochK1,
    #[strum(serialize = "Stoch.D[1]")]
    StochD1,
    #[strum(serialize = "CCI20")]
    CCI20,
    #[strum(serialize = "CCI20[1]")]
    CCI201,
    #[strum(serialize = "ADX")]
    ADX,
    #[strum(serialize = "ADX+DI")]
    ADXplusDI,
    #[strum(serialize = "ADX-DI")]
    ADXminusDI,
    #[strum(serialize = "ADX+DI[1]")]
    ADXplusDI1,
    #[strum(serialize = "ADX-DI[1]")]
    ADXminusDI1,
    #[strum(serialize = "AO")]
    AO,
    #[strum(serialize = "AO[1]")]
    AO1,
    #[strum(serialize = "AO[2]")]
    AO2,
    #[strum(serialize = "Mom")]
    Mom,
    #[strum(serialize = "Mom[1]")]
    Mom1,
    #[strum(serialize = "MACD.macd")]
    MACDmacd,
    #[strum(serialize = "MACD.signal")]
    MACDsignal,
    #[strum(serialize = "Rec.Stoch.RSI")]
    RecStochRSI,
    #[strum(serialize = "Stoch.RSI.K")]
    StochRSIK,
    #[strum(serialize = "Stoch.RSI.D")]
    StochRSID,
    #[strum(serialize = "Rec.WR")]
    RecWR,
    #[strum(serialize = "W.R")]
    WR,
    #[strum(serialize = "Rec.BBPower")]
    RecBBPower,
    #[strum(serialize = "BBPower")]
    BBPower,
    #[strum(serialize = "Rec.UO")]
    RecUO,
    #[strum(serialize = "UO")]
    UO,
    #[strum(serialize = "EMA5")]
    EMA5,
    #[strum(serialize = "SMA5")]
    SMA5,
    #[strum(serialize = "EMA10")]
    EMA10,
    #[strum(serialize = "SMA10")]
    SMA10,
    #[strum(serialize = "EMA20")]
    EMA20,
    #[strum(serialize = "SMA20")]
    SMA20,
    #[strum(serialize = "EMA30")]
    EMA30,
    #[strum(serialize = "SMA30")]
    SMA30,
    #[strum(serialize = "EMA50")]
    EMA50,
    #[strum(serialize = "SMA50")]
    SMA50,
    #[strum(serialize = "EMA100")]
    EMA100,
    #[strum(serialize = "SMA100")]
    SMA100,
    #[strum(serialize = "EMA200")]
    EMA200,
    #[strum(serialize = "SMA200")]
    SMA200,
    #[strum(serialize = "Rec.Ichimoku")]
    RecIchimoku,
    #[strum(serialize = "Ichimoku.BLine")]
    IchimokuBLine,
    #[strum(serialize = "Rec.VWMA")]
    RecVWMA,
    #[strum(serialize = "VWMA")]
    VWMA,
    #[strum(serialize = "Rec.HullMA9")]
    RecHullMA9,
    #[strum(serialize = "HullMA9")]
    HullMA9,
    #[strum(serialize = "Pivot.M.Classic.S3")]
    PivotMClassicS3,
    #[strum(serialize = "Pivot.M.Classic.S2")]
    PivotMClassicS2,
    #[strum(serialize = "Pivot.M.Classic.S1")]
    PivotMClassicS1,
    #[strum(serialize = "Pivot.M.Classic.Middle")]
    PivotMClassicMiddle,
    #[strum(serialize = "Pivot.M.Classic.R1")]
    PivotMClassicR1,
    #[strum(serialize = "Pivot.M.Classic.R2")]
    PivotMClassicR2,
    #[strum(serialize = "Pivot.M.Classic.R3")]
    PivotMClassicR3,
    #[strum(serialize = "Pivot.M.Fibonacci.S3")]
    PivotMFibonacciS3,
    #[strum(serialize = "Pivot.M.Fibonacci.S2")]
    PivotMFibonacciS2,
    #[strum(serialize = "Pivot.M.Fibonacci.S1")]
    PivotMFibonacciS1,
    #[strum(serialize = "Pivot.M.Fibonacci.Middle")]
    PivotMFibonacciMiddle,
    #[strum(serialize = "Pivot.M.Fibonacci.R1")]
    PivotMFibonacciR1,
    #[strum(serialize = "Pivot.M.Fibonacci.R2")]
    PivotMFibonacciR2,
    #[strum(serialize = "Pivot.M.Fibonacci.R3")]
    PivotMFibonacciR3,
    #[strum(serialize = "Pivot.M.Camarilla.S3")]
    PivotMCamarillaS3,
    #[strum(serialize = "Pivot.M.Camarilla.S2")]
    PivotMCamarillaS2,
    #[strum(serialize = "Pivot.M.Camarilla.S1")]
    PivotMCamarillaS1,
    #[strum(serialize = "Pivot.M.Camarilla.Middle")]
    PivotMCamarillaMiddle,
    #[strum(serialize = "Pivot.M.Camarilla.R1")]
    PivotMCamarillaR1,
    #[strum(serialize = "Pivot.M.Camarilla.R2")]
    PivotMCamarillaR2,
    #[strum(serialize = "Pivot.M.Camarilla.R3")]
    PivotMCamarillaR3,
    #[strum(serialize = "Pivot.M.Woodie.S3")]
    PivotMWoodieS3,
    #[strum(serialize = "Pivot.M.Woodie.S2")]
    PivotMWoodieS2,
    #[strum(serialize = "Pivot.M.Woodie.S1")]
    PivotMWoodieS1,
    #[strum(serialize = "Pivot.M.Woodie.Middle")]
    PivotMWoodieMiddle,
    #[strum(serialize = "Pivot.M.Woodie.R1")]
    PivotMWoodieR1,
    #[strum(serialize = "Pivot.M.Woodie.R2")]
    PivotMWoodieR2,
    #[strum(serialize = "Pivot.M.Woodie.R3")]
    PivotMWoodieR3,
    #[strum(serialize = "Pivot.M.Demark.S1")]
    PivotMDemarkS1,
    #[strum(serialize = "Pivot.M.Demark.Middle")]
    PivotMDemarkMiddle,
    #[strum(serialize = "Pivot.M.Demark.R1")]
    PivotMDemarkR1,
    #[strum(serialize = "P.SAR")]
    PSAR,
    #[strum(serialize = "BB.lower")]
    BBlower,
    #[strum(serialize = "BB.upper")]
    BBupper,
    #[strum(serialize = "price_52_week_high")]
    Price52WeekHigh,
    #[strum(serialize = "price_52_week_low")]
    Price52WeekLow,
    #[strum(serialize = "Aroon.Down")]
    AroonDown,
    #[strum(serialize = "Aroon.Up")]
    AroonUp,
    #[strum(serialize = "ADR")]
    ADR,
    #[strum(serialize = "ATR")]
    ATR,
    #[strum(serialize = "average_volume_10d_calc")]
    AverageVolume10dCalc,
    #[strum(serialize = "Perf.Y")]
    PerfY,
    #[strum(serialize = "Perf.YTD")]
    PerfYTD,
    #[strum(serialize = "average_volume_30d_calc")]
    AverageVolume30dCalc,
    #[strum(serialize = "average_volume_60d_calc")]
    AverageVolume60dCalc,
    #[strum(serialize = "average_volume_90d_calc")]
    AverageVolume90dCalc,
    #[strum(serialize = "change_from_open_abs")]
    ChangeFromOpenAbs,
    #[strum(serialize = "change_from_open")]
    ChangeFromOpen,
    #[strum(serialize = "DonchCh20.Lower")]
    DonchCh20Lower,
    #[strum(serialize = "DonchCh20.Upper")]
    DonchCh20Upper,
    #[strum(serialize = "Ichimoku.CLine")]
    IchimokuCLine,
    #[strum(serialize = "Ichimoku.Lead1")]
    IchimokuLead1,
    #[strum(serialize = "Ichimoku.Lead2")]
    IchimokuLead2,
    #[strum(serialize = "KltChnl.lower")]
    KltChnllower,
    #[strum(serialize = "KltChnl.upper")]
    KltChnlupper,
    #[strum(serialize = "market_cap_calc")]
    MarketCapCalc,
    #[strum(serialize = "ROC")]
    ROC,
    #[strum(serialize = "relative_volume_10d_calc")]
    RelativeVolume10dCalc,
    #[strum(serialize = "Volatility.D")]
    VolatilityD,
    #[strum(serialize = "Volatility.M")]
    VolatilityM,
    #[strum(serialize = "Volatility.W")]
    VolatilityW,
    #[strum(serialize = "VWAP")]
    VWAP,
    #[strum(serialize = "Candle.AbandonedBaby.Bearish")]
    CandleAbandonedBabyBearish,
    #[strum(serialize = "Candle.AbandonedBaby.Bullish")]
    CandleAbandonedBabyBullish,
    #[strum(serialize = "Candle.Engulfing.Bearish")]
    CandleEngulfingBearish,
    #[strum(serialize = "Candle.Harami.Bearish")]
    CandleHaramiBearish,
    #[strum(serialize = "Candle.Engulfing.Bullish")]
    CandleEngulfingBullish,
    #[strum(serialize = "Candle.Harami.Bullish")]
    CandleHaramiBullish,
    #[strum(serialize = "Candle.Doji")]
    CandleDoji,
    #[strum(serialize = "Candle.Doji.Dragonfly")]
    CandleDojiDragonfly,
    #[strum(serialize = "Candle.EveningStar")]
    CandleEveningStar,
    #[strum(serialize = "Candle.Doji.Gravestone")]
    CandleDojiGravestone,
    #[strum(serialize = "Candle.Hammer")]
    CandleHammer,
    #[strum(serialize = "Candle.HangingMan")]
    CandleHangingMan,
    #[strum(serialize = "Candle.InvertedHammer")]
    CandleInvertedHammer,
    #[strum(serialize = "Candle.Kicking.Bearish")]
    CandleKickingBearish,
    #[strum(serialize = "Candle.Kicking.Bullish")]
    CandleKickingBullish,
    #[strum(serialize = "Candle.LongShadow.Lower")]
    CandleLongShadowLower,
    #[strum(serialize = "Candle.LongShadow.Upper")]
    CandleLongShadowUpper,
    #[strum(serialize = "Candle.Marubozu.Black")]
    CandleMarubozuBlack,
    #[strum(serialize = "Candle.Marubozu.White")]
    CandleMarubozuWhite,
    #[strum(serialize = "Candle.MorningStar")]
    CandleMorningStar,
    #[strum(serialize = "Candle.ShootingStar")]
    CandleShootingStar,
    #[strum(serialize = "Candle.SpinningTop.Black")]
    CandleSpinningTopBlack,
    #[strum(serialize = "Candle.SpinningTop.White")]
    CandleSpinningTopWhite,
    #[strum(serialize = "Candle.3BlackCrows")]
    Candle3BlackCrows,
    #[strum(serialize = "Candle.3WhiteSoldiers")]
    Candle3WhiteSoldiers,
    #[strum(serialize = "Candle.TriStar.Bearish")]
    CandleTriStarBearish,
    #[strum(serialize = "Candle.TriStar.Bullish")]
    CandleTriStarBullish,
}

impl Field {
    /// Parses a field from a string.
    pub fn parse(s: &str) -> Result<Field> {
        Field::from_str(s).map_err(|_| anyhow!("Invalid field: {}", s))
    }

    /// Parses a field, returning an undefined field if it fails
    pub fn parse_undefined(s: &str) -> Field {
        let parts: Vec<&str> = s.split("|").collect();
        let undefined_func = if parts.len() > 1 {
            Field::undefined_with_interval
        } else {
            Field::undefined
        };
        let field = parts.get(0).map_or(undefined_func(""), |x| {
            Field::from_str(x).unwrap_or(undefined_func(x))
        });
        field
    }

    /// Parses a field and an interval from a string.
    pub fn parse_with_interval(s: &str) -> Result<(Field, Option<Interval>)> {
        let parts: Vec<&str> = s.split("|").collect();
        let field = parts
            .get(0)
            .and_then(|x| Field::from_str(x).ok())
            .ok_or(anyhow!("parse field error"))?;
        let interval = parts.get(1).and_then(|x| Interval::parse(x).ok());
        Ok((field, interval))
    }

    /// Parses a field with interval, returning an undefined field if it fails
    pub fn parse_undefined_with_interval(s: &str) -> (Field, Option<Interval>) {
        let parts: Vec<&str> = s.split("|").collect();
        let field = parts
            .get(0)
            .map_or(Field::undefined_with_interval(""), |x| {
                Field::from_str(x).unwrap_or(Field::undefined_with_interval(x))
            });
        let interval = parts.get(1).and_then(|x| Interval::parse(x).ok());
        (field, interval)
    }

    /// Creates an undefined field.
    pub fn undefined(x: &str) -> Field {
        Field::Undefined(x.to_owned())
    }

    /// Creates an undefined field with interval.
    pub fn undefined_with_interval(x: &str) -> Field {
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

impl ToString for Field {
    fn to_string(&self) -> String {
        self.to_str_ref().to_owned()
    }
}

impl AsRef<Field> for Field {
    fn as_ref(&self) -> &Field {
        &self
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
