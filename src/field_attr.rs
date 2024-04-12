use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::field::*;

pub struct FieldAttr {
    pub field: Field,
    pub name: &'static str,
    pub has_interval: bool,
}

impl FieldAttr {
    pub fn new(field: Field, name: &'static str, has_interval: bool) -> FieldAttr {
        FieldAttr {
            field,
            name,
            has_interval,
        }
    }

    pub fn all_field_attrs() -> &'static [FieldAttr] {
        ALL_FIELD_ATTRS.as_slice()
    }

    pub fn find(field: &Field) -> Option<&'static FieldAttr> {
        lazy_static! {
            static ref MAP: HashMap<Field, &'static FieldAttr> = ALL_FIELD_ATTRS
                .iter()
                .map(|x| (x.field.clone(), x))
                .collect();
        };
        MAP.get(field).map(|x| *x)
    }

    pub fn find_by_name(name: &str) -> Option<&'static FieldAttr> {
        lazy_static! {
            static ref MAP: HashMap<&'static str, &'static FieldAttr> =
                ALL_FIELD_ATTRS.iter().map(|x| (x.name, x)).collect();
        };
        MAP.get(name).map(|x| *x)
    }
}

lazy_static! {
    static ref ALL_FIELD_ATTRS: Vec<FieldAttr> = vec![
        FieldAttr::new(Field::RecommendOther, "Recommend.Other", true),
        FieldAttr::new(Field::RecommendAll, "Recommend.All", true),
        FieldAttr::new(Field::RecommendMA, "Recommend.MA", true),
        FieldAttr::new(Field::Name, "name", false),
        FieldAttr::new(Field::Exchange, "exchange", false),
        FieldAttr::new(Field::Description, "description", false),
        FieldAttr::new(Field::Type, "type", false),
        FieldAttr::new(Field::SubType, "subtype", false),
        FieldAttr::new(Field::UpdateMode, "update_mode", false),
        FieldAttr::new(Field::PriceScale, "pricescale", false),
        FieldAttr::new(Field::MinMov, "minmov", false),
        FieldAttr::new(Field::MinMove2, "minmove2", false),
        FieldAttr::new(Field::Fractional, "fractional", false),
        FieldAttr::new(Field::Open, "open", true),
        FieldAttr::new(Field::Close, "close", true),
        FieldAttr::new(Field::Volume, "volume", true),
        FieldAttr::new(Field::Change, "change", true),
        FieldAttr::new(Field::ChangeAbs, "change_abs", true),
        FieldAttr::new(Field::Gap, "gap", true),
        FieldAttr::new(Field::HighW, "High.W", true),
        FieldAttr::new(Field::LowW, "Low.W", true),
        FieldAttr::new(Field::PerfW, "Perf.W", true),
        FieldAttr::new(Field::High1M, "High.1M", true),
        FieldAttr::new(Field::Low1M, "Low.1M", true),
        FieldAttr::new(Field::Perf1M, "Perf.1M", true),
        FieldAttr::new(Field::High3M, "High.3M", true),
        FieldAttr::new(Field::Low3M, "Low.3M", true),
        FieldAttr::new(Field::Perf3M, "Perf.3M", true),
        FieldAttr::new(Field::High6M, "High.6M", true),
        FieldAttr::new(Field::Low6M, "Low.6M", true),
        FieldAttr::new(Field::Perf6M, "Perf.6M", true),
        FieldAttr::new(Field::HighAll, "High.All", true),
        FieldAttr::new(Field::LowAll, "Low.All", true),
        FieldAttr::new(Field::RSI, "RSI", true),
        FieldAttr::new(Field::RSI1, "RSI[1]", true),
        FieldAttr::new(Field::RSI7, "RSI7", true),
        FieldAttr::new(Field::StochK, "Stoch.K", true),
        FieldAttr::new(Field::StochD, "Stoch.D", true),
        FieldAttr::new(Field::StochK1, "Stoch.K[1]", true),
        FieldAttr::new(Field::StochD1, "Stoch.D[1]", true),
        FieldAttr::new(Field::CCI20, "CCI20", true),
        FieldAttr::new(Field::CCI201, "CCI20[1]", true),
        FieldAttr::new(Field::ADX, "ADX", true),
        FieldAttr::new(Field::ADXplusDI, "ADX+DI", true),
        FieldAttr::new(Field::ADXminusDI, "ADX-DI", true),
        FieldAttr::new(Field::ADXplusDI1, "ADX+DI[1]", true),
        FieldAttr::new(Field::ADXminusDI1, "ADX-DI[1]", true),
        FieldAttr::new(Field::AO, "AO", true),
        FieldAttr::new(Field::AO1, "AO[1]", true),
        FieldAttr::new(Field::AO2, "AO[2]", true),
        FieldAttr::new(Field::Mom, "Mom", true),
        FieldAttr::new(Field::Mom1, "Mom[1]", true),
        FieldAttr::new(Field::MACDmacd, "MACD.macd", true),
        FieldAttr::new(Field::MACDsignal, "MACD.signal", true),
        FieldAttr::new(Field::RecStochRSI, "Rec.Stoch.RSI", true),
        FieldAttr::new(Field::StochRSIK, "Stoch.RSI.K", true),
        FieldAttr::new(Field::StochRSID, "Stoch.RSI.D", true),
        FieldAttr::new(Field::RecWR, "Rec.WR", true),
        FieldAttr::new(Field::WR, "W.R", true),
        FieldAttr::new(Field::RecBBPower, "Rec.BBPower", true),
        FieldAttr::new(Field::BBPower, "BBPower", true),
        FieldAttr::new(Field::RecUO, "Rec.UO", true),
        FieldAttr::new(Field::UO, "UO", true),
        FieldAttr::new(Field::EMA5, "EMA5", true),
        FieldAttr::new(Field::SMA5, "SMA5", true),
        FieldAttr::new(Field::EMA10, "EMA10", true),
        FieldAttr::new(Field::SMA10, "SMA10", true),
        FieldAttr::new(Field::EMA20, "EMA20", true),
        FieldAttr::new(Field::SMA20, "SMA20", true),
        FieldAttr::new(Field::EMA30, "EMA30", true),
        FieldAttr::new(Field::SMA30, "SMA30", true),
        FieldAttr::new(Field::EMA50, "EMA50", true),
        FieldAttr::new(Field::SMA50, "SMA50", true),
        FieldAttr::new(Field::EMA100, "EMA100", true),
        FieldAttr::new(Field::SMA100, "SMA100", true),
        FieldAttr::new(Field::EMA200, "EMA200", true),
        FieldAttr::new(Field::SMA200, "SMA200", true),
        FieldAttr::new(Field::RecIchimoku, "Rec.Ichimoku", true),
        FieldAttr::new(Field::IchimokuBLine, "Ichimoku.BLine", true),
        FieldAttr::new(Field::RecVWMA, "Rec.VWMA", true),
        FieldAttr::new(Field::VWMA, "VWMA", true),
        FieldAttr::new(Field::RecHullMA9, "Rec.HullMA9", true),
        FieldAttr::new(Field::HullMA9, "HullMA9", true),
        FieldAttr::new(Field::PivotMClassicS3, "Pivot.M.Classic.S3", true),
        FieldAttr::new(Field::PivotMClassicS2, "Pivot.M.Classic.S2", true),
        FieldAttr::new(Field::PivotMClassicS1, "Pivot.M.Classic.S1", true),
        FieldAttr::new(Field::PivotMClassicMiddle, "Pivot.M.Classic.Middle", true),
        FieldAttr::new(Field::PivotMClassicR1, "Pivot.M.Classic.R1", true),
        FieldAttr::new(Field::PivotMClassicR2, "Pivot.M.Classic.R2", true),
        FieldAttr::new(Field::PivotMClassicR3, "Pivot.M.Classic.R3", true),
        FieldAttr::new(Field::PivotMFibonacciS3, "Pivot.M.Fibonacci.S3", true),
        FieldAttr::new(Field::PivotMFibonacciS2, "Pivot.M.Fibonacci.S2", true),
        FieldAttr::new(Field::PivotMFibonacciS1, "Pivot.M.Fibonacci.S1", true),
        FieldAttr::new(
            Field::PivotMFibonacciMiddle,
            "Pivot.M.Fibonacci.Middle",
            true
        ),
        FieldAttr::new(Field::PivotMFibonacciR1, "Pivot.M.Fibonacci.R1", true),
        FieldAttr::new(Field::PivotMFibonacciR2, "Pivot.M.Fibonacci.R2", true),
        FieldAttr::new(Field::PivotMFibonacciR3, "Pivot.M.Fibonacci.R3", true),
        FieldAttr::new(Field::PivotMCamarillaS3, "Pivot.M.Camarilla.S3", true),
        FieldAttr::new(Field::PivotMCamarillaS2, "Pivot.M.Camarilla.S2", true),
        FieldAttr::new(Field::PivotMCamarillaS1, "Pivot.M.Camarilla.S1", true),
        FieldAttr::new(
            Field::PivotMCamarillaMiddle,
            "Pivot.M.Camarilla.Middle",
            true
        ),
        FieldAttr::new(Field::PivotMCamarillaR1, "Pivot.M.Camarilla.R1", true),
        FieldAttr::new(Field::PivotMCamarillaR2, "Pivot.M.Camarilla.R2", true),
        FieldAttr::new(Field::PivotMCamarillaR3, "Pivot.M.Camarilla.R3", true),
        FieldAttr::new(Field::PivotMWoodieS3, "Pivot.M.Woodie.S3", true),
        FieldAttr::new(Field::PivotMWoodieS2, "Pivot.M.Woodie.S2", true),
        FieldAttr::new(Field::PivotMWoodieS1, "Pivot.M.Woodie.S1", true),
        FieldAttr::new(Field::PivotMWoodieMiddle, "Pivot.M.Woodie.Middle", true),
        FieldAttr::new(Field::PivotMWoodieR1, "Pivot.M.Woodie.R1", true),
        FieldAttr::new(Field::PivotMWoodieR2, "Pivot.M.Woodie.R2", true),
        FieldAttr::new(Field::PivotMWoodieR3, "Pivot.M.Woodie.R3", true),
        FieldAttr::new(Field::PivotMDemarkS1, "Pivot.M.Demark.S1", true),
        FieldAttr::new(Field::PivotMDemarkMiddle, "Pivot.M.Demark.Middle", true),
        FieldAttr::new(Field::PivotMDemarkR1, "Pivot.M.Demark.R1", true),
        FieldAttr::new(Field::PSAR, "P.SAR", true),
        FieldAttr::new(Field::BBlower, "BB.lower", true),
        FieldAttr::new(Field::BBupper, "BB.upper", true),
        FieldAttr::new(Field::Price52WeekHigh, "price_52_week_high", true),
        FieldAttr::new(Field::Price52WeekLow, "price_52_week_low", true),
        FieldAttr::new(Field::AroonDown, "Aroon.Down", true),
        FieldAttr::new(Field::AroonUp, "Aroon.Up", true),
        FieldAttr::new(Field::ADR, "ADR", true),
        FieldAttr::new(Field::ATR, "ATR", true),
        FieldAttr::new(Field::AverageVolume10dCalc, "average_volume_10d_calc", true),
        FieldAttr::new(Field::PerfY, "Perf.Y", true),
        FieldAttr::new(Field::PerfYTD, "Perf.YTD", true),
        FieldAttr::new(Field::AverageVolume30dCalc, "average_volume_30d_calc", true),
        FieldAttr::new(Field::AverageVolume60dCalc, "average_volume_60d_calc", true),
        FieldAttr::new(Field::AverageVolume90dCalc, "average_volume_90d_calc", true),
        FieldAttr::new(Field::ChangeFromOpenAbs, "change_from_open_abs", true),
        FieldAttr::new(Field::ChangeFromOpen, "change_from_open", true),
        FieldAttr::new(Field::DonchCh20Lower, "DonchCh20.Lower", true),
        FieldAttr::new(Field::DonchCh20Upper, "DonchCh20.Upper", true),
        FieldAttr::new(Field::IchimokuCLine, "Ichimoku.CLine", true),
        FieldAttr::new(Field::IchimokuLead1, "Ichimoku.Lead1", true),
        FieldAttr::new(Field::IchimokuLead2, "Ichimoku.Lead2", true),
        FieldAttr::new(Field::KltChnllower, "KltChnl.lower", true),
        FieldAttr::new(Field::KltChnlupper, "KltChnl.upper", true),
        FieldAttr::new(Field::MarketCapCalc, "market_cap_calc", true),
        FieldAttr::new(Field::ROC, "ROC", true),
        FieldAttr::new(
            Field::RelativeVolume10dCalc,
            "relative_volume_10d_calc",
            true
        ),
        FieldAttr::new(Field::VolatilityD, "Volatility.D", true),
        FieldAttr::new(Field::VolatilityM, "Volatility.M", true),
        FieldAttr::new(Field::VolatilityW, "Volatility.W", true),
        FieldAttr::new(Field::VWAP, "VWAP", true),
        FieldAttr::new(
            Field::CandleAbandonedBabyBearish,
            "Candle.AbandonedBaby.Bearish",
            true
        ),
        FieldAttr::new(
            Field::CandleAbandonedBabyBullish,
            "Candle.AbandonedBaby.Bullish",
            true
        ),
        FieldAttr::new(
            Field::CandleEngulfingBearish,
            "Candle.Engulfing.Bearish",
            true
        ),
        FieldAttr::new(Field::CandleHaramiBearish, "Candle.Harami.Bearish", true),
        FieldAttr::new(
            Field::CandleEngulfingBullish,
            "Candle.Engulfing.Bullish",
            true
        ),
        FieldAttr::new(Field::CandleHaramiBullish, "Candle.Harami.Bullish", true),
        FieldAttr::new(Field::CandleDoji, "Candle.Doji", true),
        FieldAttr::new(Field::CandleDojiDragonfly, "Candle.Doji.Dragonfly", true),
        FieldAttr::new(Field::CandleEveningStar, "Candle.EveningStar", true),
        FieldAttr::new(Field::CandleDojiGravestone, "Candle.Doji.Gravestone", true),
        FieldAttr::new(Field::CandleHammer, "Candle.Hammer", true),
        FieldAttr::new(Field::CandleHangingMan, "Candle.HangingMan", true),
        FieldAttr::new(Field::CandleInvertedHammer, "Candle.InvertedHammer", true),
        FieldAttr::new(Field::CandleKickingBearish, "Candle.Kicking.Bearish", true),
        FieldAttr::new(Field::CandleKickingBullish, "Candle.Kicking.Bullish", true),
        FieldAttr::new(
            Field::CandleLongShadowLower,
            "Candle.LongShadow.Lower",
            true
        ),
        FieldAttr::new(
            Field::CandleLongShadowUpper,
            "Candle.LongShadow.Upper",
            true
        ),
        FieldAttr::new(Field::CandleMarubozuBlack, "Candle.Marubozu.Black", true),
        FieldAttr::new(Field::CandleMarubozuWhite, "Candle.Marubozu.White", true),
        FieldAttr::new(Field::CandleMorningStar, "Candle.MorningStar", true),
        FieldAttr::new(Field::CandleShootingStar, "Candle.ShootingStar", true),
        FieldAttr::new(
            Field::CandleSpinningTopBlack,
            "Candle.SpinningTop.Black",
            true
        ),
        FieldAttr::new(
            Field::CandleSpinningTopWhite,
            "Candle.SpinningTop.White",
            true
        ),
        FieldAttr::new(Field::Candle3BlackCrows, "Candle.3BlackCrows", true),
        FieldAttr::new(Field::Candle3WhiteSoldiers, "Candle.3WhiteSoldiers", true),
        FieldAttr::new(Field::CandleTriStarBearish, "Candle.TriStar.Bearish", true),
        FieldAttr::new(Field::CandleTriStarBullish, "Candle.TriStar.Bullish", true),
    ];
}
