use super::*;
use anyhow::{Context, Result};
use reqwest::Url;
use serde_json::Value;

const SCANNER_URL: &str = "https://scanner.tradingview.com/";

#[derive(Debug)]
pub struct TradingView {
    screener: String,
    exchange: String,
    client: reqwest::Client,
}

impl TradingView {
    /// Creates a new TradingView instance.
    pub fn new<S1, S2>(screener: S1, exchange: S2) -> TradingView
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        Self {
            screener: screener.as_ref().to_lowercase(),
            exchange: exchange.as_ref().to_owned(),
            client: reqwest::Client::new(),
        }
    }

    /// Creates a new TradingView instance with a custom reqwest::Client.
    pub fn new_with_client<S1, S2>(
        screener: S1,
        exchange: S2,
        client: reqwest::Client,
    ) -> TradingView
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        Self {
            screener: screener.as_ref().to_lowercase(),
            exchange: exchange.as_ref().to_owned(),
            client,
        }
    }

    /// Retrieves the specified fields for a given symbol, with all fields sharing the same interval.
    pub async fn get_symbol_fields<S1, S2>(
        &self,
        symbol: S1,
        interval: S2,
        fields: &[Field],
    ) -> Result<SimpleSymbolValues>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        let mut url = Url::parse(SCANNER_URL)?.join("symbol")?;

        let symbol = self.exchange.clone() + ":" + symbol.as_ref();
        let interval = Interval::parse_undefined(interval.as_ref());
        let fields: Vec<String> = fields
            .iter()
            .map(|x| x.to_string_with_interval(&interval))
            .collect();

        {
            let mut query = url.query_pairs_mut();
            query.append_pair("symbol", &symbol);
            query.append_pair("fields", &fields.join(","));
            query.append_pair("no_404", "true");
        }

        let response = self.client.get(url).send().await?;
        if let Err(err) = response.error_for_status_ref() {
            return Err(err).context(response.text().await?);
        }

        let mut symbol_values: SimpleSymbolValues = SimpleSymbolValues::new(symbol.as_str());
        let json: Value = response.json().await.context(format!("Parse json error"))?;
        if let Some(obj) = json.as_object() {
            for (k, v) in obj {
                let (field, _) = Field::parse_undefined_with_interval(k);
                symbol_values.values_mut().insert(field, v.clone());
            }
        }
        Ok(symbol_values)
    }

    /// Retrieves the specified fields for a given symbol, each field with its own interval.
    pub async fn get_symbol_fields_with_interval<S>(
        &self,
        symbol: S,
        fields: &[FieldWithInterval],
    ) -> Result<TimedSymbolValues>
    where
        S: AsRef<str>,
    {
        let mut url = Url::parse(SCANNER_URL)?.join("symbol")?;

        let symbol = self.exchange.clone() + ":" + symbol.as_ref();
        let fields: Vec<String> = fields.iter().map(|x| x.to_string()).collect();

        {
            let mut query = url.query_pairs_mut();
            query.append_pair("symbol", &symbol);
            query.append_pair("fields", &fields.join(","));
            query.append_pair("no_404", "true");
        }

        let response = self.client.get(url).send().await?;
        if let Err(err) = response.error_for_status_ref() {
            return Err(err).context(response.text().await?);
        }

        let mut symbol_values = TimedSymbolValues::new(symbol.as_str());
        let json: Value = response.json().await.context(format!("Parse json error"))?;
        if let Some(obj) = json.as_object() {
            for (k, v) in obj {
                let field = FieldWithInterval::parse_undefined(k);
                symbol_values.values_mut().insert(field, v.clone());
            }
        }
        Ok(symbol_values)
    }

    /// Searches for symbols by given exchanges and types. Retrieves basic fields along with specified extra fields.
    pub async fn search_symbols<S1, S2, S3>(
        &self,
        exchanges: &[S1],
        types: &[S2],
        name_filter: S3,
        extra_fields: &[FieldWithInterval],
    ) -> Result<Vec<TimedSymbolValues>>
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
        S3: AsRef<str>,
    {
        let url = Url::parse(SCANNER_URL)?
            .join(&format!("{}/", self.screener))?
            .join("scan")?;
        let fields: Vec<FieldWithInterval> = [
            Field::Exchange,
            Field::Name,
            Field::Type,
            Field::SubType,
            Field::Description,
        ]
        .into_iter()
        .map(|x| x.with_interval(&Interval::default()))
        .chain(extra_fields.iter().cloned())
        .collect();

        let types: Vec<&str> = types.iter().map(|x| x.as_ref()).collect();
        let exchanges: Vec<&str> = exchanges.iter().map(|x| x.as_ref()).collect();
        let columns: Vec<String> = fields.iter().map(|x| x.to_string()).collect();
        let name_filter: &str = name_filter.as_ref();

        let data = serde_json::json!({
            "symbols": {
                "tickers": [],
                "query": {
                    "types": types,
                    "exchanges": exchanges,
                }
            },
            "filter": [
                {
                    "left": "name",
                    "operation": "match",
                    "right": name_filter,
                }
            ],
            "columns": columns
        });

        let response = self.client.post(url).json(&data).send().await?;
        if let Err(err) = response.error_for_status_ref() {
            return Err(err).context(response.text().await?);
        }

        let mut symbol_values_vec: Vec<TimedSymbolValues> = Vec::new();
        let json_body: serde_json::Value = response.json().await?;
        let data_array: Vec<Value> = vec![];
        let data_array = json_body["data"].as_array().unwrap_or(&data_array);
        for data in data_array.iter() {
            if let Some(symbol) = data["s"].as_str() {
                let mut symbol_values = TimedSymbolValues::new(symbol);
                if let Some(arr) = data["d"].as_array() {
                    for (i, v) in arr.iter().enumerate() {
                        symbol_values
                            .values_mut()
                            .insert(fields[i].clone(), v.to_owned());
                    }
                }
                symbol_values_vec.push(symbol_values);
            }
        }
        Ok(symbol_values_vec)
    }

    /// Retrieves the specified fields for the given symbols, each field with its own interval.
    pub async fn scan_symbols<S>(
        &self,
        symbols: &[S],
        fields: &[FieldWithInterval],
    ) -> Result<Vec<TimedSymbolValues>>
    where
        S: AsRef<str>,
    {
        let url = Url::parse(SCANNER_URL)?
            .join(&format!("{}/", self.screener))?
            .join("scan")?;

        let tickers: Vec<String> = symbols
            .iter()
            .map(|x| self.exchange.clone() + ":" + x.as_ref())
            .collect();
        let columns: Vec<String> = fields.iter().map(|x| x.to_string()).collect();

        let data = serde_json::json!({
            "symbols": {
                "tickers": tickers,
                "query": {
                }
            },
            "columns": columns
        });

        let response = self.client.post(url).json(&data).send().await?;
        if let Err(err) = response.error_for_status_ref() {
            return Err(err).context(response.text().await?);
        }

        let mut symbol_values_vec: Vec<TimedSymbolValues> = Vec::new();
        let json_body: serde_json::Value = response.json().await?;
        let data_array: Vec<Value> = vec![];
        let data_array = json_body["data"].as_array().unwrap_or(&data_array);
        for data in data_array.iter() {
            if let Some(symbol) = data["s"].as_str() {
                let mut symbol_values = TimedSymbolValues::new(symbol);
                if let Some(arr) = data["d"].as_array() {
                    for (i, v) in arr.iter().enumerate() {
                        symbol_values
                            .values_mut()
                            .insert(fields[i].clone(), v.to_owned());
                    }
                }
                symbol_values_vec.push(symbol_values);
            }
        }
        Ok(symbol_values_vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_symbol_fields() -> Result<()> {
        let tradingview = TradingView::new(Screener::Crypto, "OKX");
        let interval = Interval::Hour1;
        let fields: Vec<Field> = vec![]
            .iter()
            .chain(Field::recommends().iter())
            .chain(Field::oscillator_indicators().iter())
            .chain(Field::move_average_indicators().iter())
            .cloned()
            .collect();
        let data = tradingview
            .get_symbol_fields("BTCUSDT.P", &interval, fields.as_slice())
            .await
            .context("get symbol fields error")?;
        assert_eq!(data.values().len(), fields.len());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_symbol_fields_with_interval() -> Result<()> {
        let tradingview = TradingView::new(Screener::Crypto, "OKX");
        let interval = Interval::Hour1;
        let fields: Vec<FieldWithInterval> = vec![]
            .into_iter()
            .chain(Field::recommends().iter())
            .chain(Field::oscillator_indicators().iter())
            .chain(Field::move_average_indicators().iter())
            .cloned()
            .map(|x| x.with_interval(&interval))
            .collect();
        let data = tradingview
            .get_symbol_fields_with_interval("BTCUSDT.P", fields.as_slice())
            .await
            .context("get symbol fields with interval error")?;
        assert_eq!(data.values().len(), fields.len());
        Ok(())
    }

    #[tokio::test]
    async fn test_search_symbols() -> Result<()> {
        let tradingview = TradingView::new(&Screener::Crypto, "");
        let interval = Interval::Hour1;
        let exchanges: Vec<&str> = vec![];
        let types: Vec<&str> = vec!["spot"];
        let name_filter = "USDT";
        let extra_fields: Vec<FieldWithInterval> =
            vec![Field::Open, Field::Close, Field::ChangeFromOpen]
                .into_iter()
                .map(|x| x.with_interval(&interval))
                .collect();
        let data = tradingview
            .search_symbols(&exchanges, &types, name_filter, extra_fields.as_slice())
            .await
            .context("search symbols error")?;

        assert!(data.len() > 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_scan_symbols() -> Result<()> {
        let tradingview = TradingView::new(Screener::Crypto, "OKX");
        let interval = Interval::Hour1;
        let symbols = ["BTCUSDT.P"];
        let fields: Vec<FieldWithInterval> = vec![Field::Name, Field::Exchange, Field::SubType]
            .iter()
            .chain(Field::oscillator_indicators().iter())
            .chain(Field::move_average_indicators().iter())
            .cloned()
            .map(|x| x.with_interval(&interval))
            .collect();
        let data = tradingview
            .scan_symbols(&symbols, fields.as_slice())
            .await
            .context("scan symbols error")?;
        assert_eq!(data.len(), symbols.len());
        assert_eq!(data[0].values().len(), fields.len());
        Ok(())
    }
}
