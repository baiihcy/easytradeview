use std::str::FromStr;

use anyhow::{Context, Result};
use clap::Parser;
use easytradeview::{Analysis, Interval, Screener, TradingView};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "crypto")]
    screener: String,

    #[arg(long, default_value = "OKX")]
    exchange: String,

    #[arg(long, default_value = "BTCUSDT")]
    symbol: String,

    #[arg(short, long, default_value = "1d")]
    interval: String,

    #[arg(long, default_value_t = false)]
    json: bool,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    // Parse arguments
    let args = Args::parse();
    let screener =
        Screener::from_str(&args.screener).unwrap_or(Screener::undefined(&args.screener));
    let exchange = args.exchange;
    let symbol = args.symbol;
    let interval = Interval::parse(&args.interval).unwrap_or_default();

    // Initialize TradingView client and get technical analysis with the given parameters
    let tradingview = TradingView::new(&screener, &exchange);
    let analysis = Analysis::get_technical_analysis(&tradingview, &symbol, &interval)
        .await
        .context("get technical analysis error")?;

    // Output data, formatted as JSON if specified
    if args.json {
        let value = serde_json::json!({
            "screener": screener.as_str_ref(),
            "exchange": &exchange,
            "symbol": &symbol,
            "interval": interval.as_str_ref(),
            "analysis": analysis,
        });
        println!("{}", serde_json::to_string_pretty(&value).unwrap());
    } else {
        println!("{:>13} : {}", "screener", screener.as_str_ref());
        println!("{:>13} : {}", "exchange", &exchange);
        println!("{:>13} : {}", "symbol", &symbol);
        println!("{:>13} : {}", "interval", interval.as_str_ref());
        println!("{}", analysis);
    }
    Ok(())
}
