use super::*;
use anyhow::{Context, Result};
use serde::Serialize;
use std::{collections::HashMap, ops::Add};
use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Debug, PartialEq, Clone, Copy, EnumString, Display, AsRefStr, Serialize)]
pub enum Recommendation {
    #[strum(serialize = "STRONG_SELL")]
    StrongSell,
    #[strum(serialize = "SELL")]
    Sell,
    #[strum(serialize = "NEUTRAL")]
    Neutral,
    #[strum(serialize = "BUY")]
    Buy,
    #[strum(serialize = "STRONG_BUY")]
    StrongBuy,
}
impl Default for Recommendation {
    fn default() -> Self {
        Recommendation::Neutral
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize)]
pub enum Signal {
    RecommendOther,
    RecommendAll,
    RecommendMA,
    RSI,
    StochK,
    CCI20,
    ADX,
    AO,
    Mon,
    MACD,
    StochRsiK,
    WR,
    BBPower,
    UO,
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
    IchimokuBLine,
    VWMA,
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
}

#[derive(Debug, Default, Clone, Copy, Serialize)]
pub struct RecommendCounter {
    pub strong_sell: u32,
    pub sell: u32,
    pub neutral: u32,
    pub buy: u32,
    pub strong_buy: u32,
}

impl RecommendCounter {
    pub fn increase1(&mut self, recommend: Recommendation) {
        match recommend {
            Recommendation::StrongSell => self.strong_sell += 1,
            Recommendation::Sell => self.sell += 1,
            Recommendation::Neutral => self.neutral += 1,
            Recommendation::Buy => self.buy += 1,
            Recommendation::StrongBuy => self.strong_buy += 1,
        }
    }

    pub fn get(&mut self, recommend: Recommendation) -> u32 {
        match recommend {
            Recommendation::StrongSell => self.strong_sell,
            Recommendation::Sell => self.sell,
            Recommendation::Neutral => self.neutral,
            Recommendation::Buy => self.buy,
            Recommendation::StrongBuy => self.strong_buy,
        }
    }

    pub fn count(&self) -> u32 {
        self.strong_sell + self.sell + self.neutral + self.buy + self.strong_buy
    }
}

impl std::fmt::Display for RecommendCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ STRONG_SELL:{:>2} SELL:{:>2} NETURAL:{:>2} BUY:{:>2} STRONG_BUY:{:>2} }}",
            self.strong_sell, self.sell, self.neutral, self.buy, self.strong_buy
        )
    }
}

impl Add for RecommendCounter {
    type Output = RecommendCounter;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            strong_sell: self.strong_sell + rhs.strong_sell,
            sell: self.sell + rhs.sell,
            neutral: self.neutral + rhs.neutral,
            buy: self.buy + rhs.buy,
            strong_buy: self.strong_buy + rhs.strong_buy,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Analysis {
    pub recommend_summary: Recommendation,
    pub recommend_oscillators: Recommendation,
    pub recommend_move_averages: Recommendation,

    pub counter_summary: RecommendCounter,
    pub counter_oscillators: RecommendCounter,
    pub counter_move_averages: RecommendCounter,

    pub signal_summary: f64,
    pub signal_oscillators: f64,
    pub signal_move_averages: f64,

    pub signals: HashMap<Signal, Recommendation>,
}

impl Analysis {
    /// Returns a static reference to an array of `Field` instances required for technical analysis.
    pub fn ta_fields() -> &'static [Field] {
        static VALUES: [Field; 43] = [
            Field::RecommendAll,
            Field::RecommendMA,
            Field::RecommendOther,
            Field::Close,
            Field::RSI,
            Field::RSI1,
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
            Field::RecStochRSI,
            Field::RecWR,
            Field::RecBBPower,
            Field::RecUO,
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
            Field::RecIchimoku,
            Field::RecVWMA,
            Field::RecHullMA9,
        ];
        &VALUES
    }

    /// Retrieves symbol values for the given symbol from `tradingview` and computes technical analysis.
    pub async fn get_technical_analysis<S: AsRef<str>>(
        tradingview: &TradingView,
        symbol: S,
        interval: &Interval,
    ) -> Result<Analysis> {
        let values = tradingview
            .get_symbol_fields(&symbol, &interval, Analysis::ta_fields())
            .await
            .context("get symbol fields error")?;
        Ok(Analysis::compute(&values.get_f64_values()))
    }

    /// Compute technical analysis from symbol values.
    pub fn compute(values: &HashMap<Field, f64>) -> Analysis {
        let mut analysis = Analysis::default();

        /* recommend signals */
        {
            if let Some(signal) = values.get(&Field::RecommendAll) {
                analysis.recommend_summary = Analysis::compute_recommend_signal(*signal);
                analysis.signal_summary = *signal;
            }
            if let Some(signal) = values.get(&Field::RecommendOther) {
                analysis.recommend_oscillators = Analysis::compute_recommend_signal(*signal);
                analysis.signal_oscillators = *signal;
            }
            if let Some(signal) = values.get(&Field::RecommendMA) {
                analysis.recommend_move_averages = Analysis::compute_recommend_signal(*signal);
                analysis.signal_move_averages = *signal;
            }
        }

        /* oscillators */
        {
            // RSI
            if let Some(vals) = get_all_values(values, &[Field::RSI, Field::RSI1]) {
                analysis.add_oscillator_signal(
                    Signal::RSI,
                    Analysis::compute_rsi_signal(vals[0], vals[1]),
                );
            }
            // Stoch.K
            if let Some(vals) = get_all_values(
                values,
                &[Field::StochK, Field::StochD, Field::StochK1, Field::StochD1],
            ) {
                analysis.add_oscillator_signal(
                    Signal::StochK,
                    Analysis::compute_stoch_signal(vals[0], vals[1], vals[2], vals[3]),
                );
            }
            // CCI20
            if let Some(vals) = get_all_values(values, &[Field::CCI20, Field::CCI201]) {
                analysis.add_oscillator_signal(
                    Signal::CCI20,
                    Analysis::compute_cci20_signal(vals[0], vals[1]),
                );
            }
            // ADX
            if let Some(vals) = get_all_values(
                values,
                &[
                    Field::ADX,
                    Field::ADXplusDI,
                    Field::ADXminusDI,
                    Field::ADXplusDI1,
                    Field::ADXminusDI1,
                ],
            ) {
                analysis.add_oscillator_signal(
                    Signal::ADX,
                    Analysis::compute_adx_signal(vals[0], vals[1], vals[2], vals[3], vals[4]),
                );
            }
            // AO
            if let Some(vals) = get_all_values(values, &[Field::AO, Field::AO1, Field::AO2]) {
                analysis.add_oscillator_signal(
                    Signal::AO,
                    Analysis::compute_ao_signal(vals[0], vals[1], vals[2]),
                );
            }
            // Mom
            if let Some(vals) = get_all_values(values, &[Field::Mom, Field::Mom1]) {
                analysis.add_oscillator_signal(
                    Signal::Mon,
                    Analysis::compute_mom_signal(vals[0], vals[1]),
                );
            }
            // MACD
            if let Some(vals) = get_all_values(values, &[Field::MACDmacd, Field::MACDsignal]) {
                analysis.add_oscillator_signal(
                    Signal::MACD,
                    Analysis::compute_macd_signal(vals[0], vals[1]),
                );
            }
            // Stoch.RSI.K
            if let Some(vals) = get_all_values(values, &[Field::RecStochRSI]) {
                analysis.add_oscillator_signal(
                    Signal::StochRsiK,
                    Analysis::compute_simple_signal(vals[0]),
                );
            }
            // WR
            if let Some(vals) = get_all_values(values, &[Field::RecWR]) {
                analysis
                    .add_oscillator_signal(Signal::WR, Analysis::compute_simple_signal(vals[0]));
            }
            // BBPower
            if let Some(vals) = get_all_values(values, &[Field::RecBBPower]) {
                analysis.add_oscillator_signal(
                    Signal::BBPower,
                    Analysis::compute_simple_signal(vals[0]),
                );
            }
            // UO
            if let Some(vals) = get_all_values(values, &[Field::RecUO]) {
                analysis
                    .add_oscillator_signal(Signal::UO, Analysis::compute_simple_signal(vals[0]));
            }
        }

        /* move averages */
        {
            if let Some(close) = values.get(&Field::Close) {
                if let Some(val) = values.get(&Field::SMA10) {
                    analysis.add_move_average_signal(
                        Signal::SMA10,
                        Analysis::compute_ma_signal(*val, *close),
                    );
                }
                if let Some(val) = values.get(&Field::EMA10) {
                    analysis.add_move_average_signal(
                        Signal::EMA10,
                        Analysis::compute_ma_signal(*val, *close),
                    );
                }
                if let Some(val) = values.get(&Field::SMA20) {
                    analysis.add_move_average_signal(
                        Signal::SMA20,
                        Analysis::compute_ma_signal(*val, *close),
                    );
                }
                if let Some(val) = values.get(&Field::EMA20) {
                    analysis.add_move_average_signal(
                        Signal::EMA20,
                        Analysis::compute_ma_signal(*val, *close),
                    );
                }
                if let Some(val) = values.get(&Field::SMA30) {
                    analysis.add_move_average_signal(
                        Signal::SMA30,
                        Analysis::compute_ma_signal(*val, *close),
                    );
                }
                if let Some(val) = values.get(&Field::EMA30) {
                    analysis.add_move_average_signal(
                        Signal::EMA30,
                        Analysis::compute_ma_signal(*val, *close),
                    );
                }
                if let Some(val) = values.get(&Field::SMA50) {
                    analysis.add_move_average_signal(
                        Signal::SMA50,
                        Analysis::compute_ma_signal(*val, *close),
                    );
                }
                if let Some(val) = values.get(&Field::EMA50) {
                    analysis.add_move_average_signal(
                        Signal::EMA50,
                        Analysis::compute_ma_signal(*val, *close),
                    );
                }
                if let Some(val) = values.get(&Field::SMA100) {
                    analysis.add_move_average_signal(
                        Signal::SMA100,
                        Analysis::compute_ma_signal(*val, *close),
                    );
                }
                if let Some(val) = values.get(&Field::EMA100) {
                    analysis.add_move_average_signal(
                        Signal::EMA100,
                        Analysis::compute_ma_signal(*val, *close),
                    );
                }
                if let Some(val) = values.get(&Field::SMA200) {
                    analysis.add_move_average_signal(
                        Signal::SMA200,
                        Analysis::compute_ma_signal(*val, *close),
                    );
                }
                if let Some(val) = values.get(&Field::EMA200) {
                    analysis.add_move_average_signal(
                        Signal::EMA200,
                        Analysis::compute_ma_signal(*val, *close),
                    );
                }
            }
            if let Some(val) = values.get(&Field::RecIchimoku) {
                analysis.add_move_average_signal(
                    Signal::IchimokuBLine,
                    Analysis::compute_simple_signal(*val),
                );
            }
            if let Some(val) = values.get(&Field::RecVWMA) {
                analysis
                    .add_move_average_signal(Signal::VWMA, Analysis::compute_simple_signal(*val));
            }
            if let Some(val) = values.get(&Field::RecVWMA) {
                analysis.add_move_average_signal(
                    Signal::HullMA9,
                    Analysis::compute_simple_signal(*val),
                );
            }
        }

        analysis
    }

    /// Adds a signal to the analysis.
    fn add_signal(&mut self, signal: Signal, recommend: Recommendation) {
        self.counter_summary.increase1(recommend);
        self.signals.insert(signal, recommend);
    }

    /// Adds an oscillator signal to the analysis.
    fn add_oscillator_signal(&mut self, signal: Signal, recommend: Recommendation) {
        self.counter_oscillators.increase1(recommend);
        self.counter_summary.increase1(recommend);
        self.signals.insert(signal, recommend);
    }

    /// Adds a moving average signal to the analysis.
    fn add_move_average_signal(&mut self, signal: Signal, recommend: Recommendation) {
        self.counter_move_averages.increase1(recommend);
        self.counter_summary.increase1(recommend);
        self.signals.insert(signal, recommend);
    }

    fn compute_ma_signal(ma: f64, close: f64) -> Recommendation {
        if ma < close {
            Recommendation::Buy
        } else if ma > close {
            Recommendation::Sell
        } else {
            Recommendation::Neutral
        }
    }

    fn compute_rsi_signal(rsi: f64, rsi1: f64) -> Recommendation {
        if rsi < 30. && rsi1 < rsi {
            Recommendation::Buy
        } else if rsi > 70. && rsi1 > rsi {
            Recommendation::Sell
        } else {
            Recommendation::Neutral
        }
    }

    fn compute_stoch_signal(k: f64, d: f64, k1: f64, d1: f64) -> Recommendation {
        if k < 20. && d < 20. && k > d && k1 < d1 {
            Recommendation::Buy
        } else if k > 80. && d > 80. && k < d && k1 > d1 {
            Recommendation::Sell
        } else {
            Recommendation::Neutral
        }
    }

    fn compute_cci20_signal(cci20: f64, cci201: f64) -> Recommendation {
        if cci20 < -100. && cci20 > cci201 {
            Recommendation::Buy
        } else if cci20 > 100. && cci20 < cci201 {
            Recommendation::Sell
        } else {
            Recommendation::Neutral
        }
    }

    fn compute_adx_signal(
        adx: f64,
        adxpdi: f64,
        adxndi: f64,
        adxpdi1: f64,
        adxndi1: f64,
    ) -> Recommendation {
        if adx > 20. && adxpdi1 < adxndi1 && adxpdi > adxndi {
            Recommendation::Buy
        } else if adx > 20. && adxpdi1 > adxndi1 && adxpdi < adxndi {
            Recommendation::Sell
        } else {
            Recommendation::Neutral
        }
    }

    fn compute_ao_signal(ao: f64, ao1: f64, ao2: f64) -> Recommendation {
        if ao > 0. && ao1 < 0. || ao > 0. && ao1 > 0. && ao > ao1 && ao2 > ao1 {
            Recommendation::Buy
        } else if ao < 0. && ao1 > 0. || ao < 0. && ao1 < 0. && ao < ao1 && ao2 < ao1 {
            Recommendation::Sell
        } else {
            Recommendation::Neutral
        }
    }

    fn compute_mom_signal(mon: f64, mon1: f64) -> Recommendation {
        if mon > mon1 {
            Recommendation::Buy
        } else if mon < mon1 {
            Recommendation::Sell
        } else {
            Recommendation::Neutral
        }
    }

    fn compute_macd_signal(macd: f64, signal: f64) -> Recommendation {
        if macd > signal {
            Recommendation::Buy
        } else if macd < signal {
            Recommendation::Sell
        } else {
            Recommendation::Neutral
        }
    }

    fn compute_bbbuy_signal(close: f64, bblower: f64) -> Recommendation {
        if close < bblower {
            Recommendation::Buy
        } else {
            Recommendation::Neutral
        }
    }

    fn compute_bbsell_signal(close: f64, bbupper: f64) -> Recommendation {
        if close > bbupper {
            Recommendation::Sell
        } else {
            Recommendation::Neutral
        }
    }

    fn compute_psar_signal(psar: f64, open: f64) -> Recommendation {
        if psar < open {
            Recommendation::Buy
        } else if psar > open {
            Recommendation::Sell
        } else {
            Recommendation::Neutral
        }
    }

    fn compute_recommend_signal(signal: f64) -> Recommendation {
        if signal >= -1. && signal < -0.5 {
            Recommendation::StrongSell
        } else if signal >= -0.5 && signal < -0.1 {
            Recommendation::Sell
        } else if signal >= -0.1 && signal <= 0.1 {
            Recommendation::Neutral
        } else if signal > 0.1 && signal <= 0.5 {
            Recommendation::Buy
        } else if signal > 0.5 && signal <= 1. {
            Recommendation::StrongBuy
        } else {
            Recommendation::default()
        }
    }

    fn compute_simple_signal(signal: f64) -> Recommendation {
        if signal == -1. {
            Recommendation::Sell
        } else if signal == 1. {
            Recommendation::Buy
        } else {
            Recommendation::Neutral
        }
    }
}

impl std::fmt::Display for Analysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{:>13} :  {:>11}({:>5.2})  {}", "SUMMARY", self.recommend_summary, self.signal_summary, self.counter_summary)
            .and_then(|_| writeln!(f, "{:>13} :  {:>11}({:>5.2})  {}", "OSCILLATORS", self.recommend_oscillators, self.signal_oscillators, self.counter_oscillators))
            .and_then(|_| writeln!(f, "{:>13} :  {:>11}({:>5.2})  {}", "MOVE_AVERAGES", self.recommend_move_averages, self.signal_move_averages, self.counter_move_averages))
    }
}

fn get_all_values<K, V>(map: &HashMap<K, V>, keys: &[K]) -> Option<Vec<V>>
where
    K: std::cmp::Eq + std::cmp::PartialEq + std::hash::Hash,
    V: Clone,
{
    let vals: Vec<_> = keys.iter().filter_map(|x| map.get(x)).cloned().collect();
    if keys.len() == vals.len() {
        Some(vals)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_analysis_compute() -> Result<()> {
        let tradingview = TradingView::new(&Screener::Crypto, "OKX");
        let symbol = "BTCUSDT.P";
        let interval = Interval::Hour1;
        let analysis = Analysis::get_technical_analysis(&tradingview, &symbol, &interval)
            .await
            .context("get technical analysis error")?;

        assert!(analysis.counter_summary.count() > 0);
        Ok(())
    }
}
