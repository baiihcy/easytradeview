use std::collections::HashMap;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use easytradeview::{Field, FieldWithInterval, Interval, Screener, SymbolValues, TradingView};
use serde_json::{json, Value};

#[derive(Parser)]
struct Opts {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    Search(SerachArgs),
    Scan(ScanArgs),
    Get(GetArgs),
    ListScreeners,
}

#[derive(Debug, Args)]
struct SerachArgs {
    #[arg(long, default_value = "crypto")]
    screener: String,

    #[arg(long)]
    exchanges: Vec<String>,

    #[arg(long)]
    types: Vec<String>,

    #[arg(long, value_parser, num_args = 1.., value_delimiter = ',')]
    fields: Vec<String>,
}

#[derive(Debug, Args)]
struct ScanArgs {
    #[arg(long, default_value = "crypto")]
    screener: String,

    #[arg(long, default_value = "OKX")]
    exchange: String,

    #[arg(short, long, default_value = "1d")]
    default_interval: String,

    #[arg(long, default_value = "BTCUSDT", value_parser, num_args = 1.., value_delimiter = ',')]
    symbols: Vec<String>,

    #[arg(long, default_value = "1d", value_parser, num_args = 1.., value_delimiter = ',')]
    fields: Vec<String>,
}

#[derive(Debug, Args)]
struct GetArgs {
    #[arg(long, default_value = "crypto")]
    screener: String,

    #[arg(long, default_value = "OKX")]
    exchange: String,

    #[arg(short, long, default_value = "1d")]
    interval: String,

    #[arg(long, default_value = "BTCUSDT")]
    symbol: String,

    #[arg(long, default_value = "1d", value_parser, num_args = 1.., value_delimiter = ',')]
    fields: Vec<String>,

    #[arg(long, default_value_t = false)]
    json: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Search(args) => {
            // Parse arguments
            let screener = Screener::parse_undefined(&args.screener);
            let exchanges = args.exchanges;
            let types = args.types;
            // Prepare extra fields if specified
            let extra_fields: Vec<FieldWithInterval> = args
                .fields
                .into_iter()
                .map(|x| FieldWithInterval::parse_undefined(&x))
                .collect();

            // Initialize TradingView client and search symbols with given parameters
            let tv = TradingView::new(&screener, "");
            let values = tv
                .search_symbols(&exchanges, &types, &extra_fields)
                .await
                .context("search symbols error")?;

            // Prepare fields to be printed
            let fields = [
                Field::Exchange,
                Field::Name,
                Field::Type,
                Field::Description,
            ]
            .into_iter()
            .map(|x| x.with_interval(&Interval::default()))
            .chain(extra_fields.into_iter())
            .collect::<Vec<_>>();

            // Assemble data rows for output
            let mut table: Vec<Vec<String>> = vec![];
            for vals in values {
                let mut row = vec![];
                for field in fields.iter() {
                    let val = vals
                        .values()
                        .get(&field)
                        .map_or("".into(), |x| x.to_string());
                    row.push(val);
                }
                table.push(row);
            }

            // Print csv header
            println!(
                "{}",
                fields
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
            // Print csv data rows
            for vals in table {
                println!("{}", vals.join(","));
            }
        }
        SubCommand::Scan(args) => {
            // Parse arguments
            let screener = Screener::parse_undefined(&args.screener);
            let exchange = args.exchange;
            let interval =
                Interval::parse(&args.default_interval).context("parse interval error")?;
            let symbols = args.symbols;
            // Prepare fields with default intervals if not specified
            let fields = args
                .fields
                .iter()
                .map(|x| FieldWithInterval::parse_undefined_with_default_interval(x, &interval))
                .collect::<Vec<_>>();

            // Initialize TradingView client and retrieve data for specified symbols and fields
            let tv = TradingView::new(&screener, exchange);
            let symbols = tv
                .scan_symbols(&symbols, &fields)
                .await
                .context("scan symbols error")?;

            // Assemble data rows for output
            let mut table: Vec<Vec<String>> = vec![];
            for vals in symbols {
                let mut row = vec![format!("\"{}\"", vals.symbol())];
                for field in fields.iter() {
                    let val = vals
                        .values()
                        .get(&field)
                        .map_or("".into(), |x| x.to_string());
                    row.push(val);
                }
                table.push(row);
            }

            // Print csv header
            println!(
                "symbol,{}",
                fields
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            );
            // Print csv data rows
            for vals in table {
                println!("{}", vals.join(","));
            }
        }
        SubCommand::Get(args) => {
            // Parse arguments
            let screener = Screener::parse_undefined(&args.screener);
            let exchange = args.exchange;
            let interval = Interval::parse(&args.interval).context("Failed to parse interval")?;
            let symbol = args.symbol;
            // Prepare fields with default intervals if not specified
            let fields = args
                .fields
                .iter()
                .map(|x| FieldWithInterval::parse_undefined_with_default_interval(x, &interval))
                .collect::<Vec<_>>();

            // Initialize TradingView client and retrieve data for specified symbol and fields
            let tv = TradingView::new(&screener, exchange);
            let symbol = tv
                .get_symbol_fields_with_interval(&symbol, &fields)
                .await
                .context("Failed to retrieve symbol fields")?;

            // Collect key-value pairs for output, including screener, symbol, interval, and field data
            let mut values: Vec<(String, Value)> = Vec::new();
            values.extend(vec![
                ("screener".to_string(), json!(screener.to_string())),
                ("symbol".to_string(), json!(symbol.symbol().to_string())),
                ("interval".to_string(), json!(interval.to_string())),
            ]);
            for field in &fields {
                if let Some(val) = symbol.values().get(field) {
                    values.push((field.to_string(), val.clone()));
                }
            }

            // Output data, formatted as JSON if specified, else as plain key-value pairs
            if args.json {
                println!("{}", serde_json::to_string_pretty(&values.into_iter().collect::<HashMap<_, _>>())?);
            } else {
                for (k, v) in values {
                    println!("{:>13} : {}", k, v.to_string());
                }
            }
        }
        SubCommand::ListScreeners => {
            for screener in Screener::all() {
                println!("{:?}", screener);
            }
        }
    };
    Ok(())
}