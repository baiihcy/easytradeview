# EasyTradeView
EasyTradeView is a wrapper around the TradingView website's API. It allows users to fetch real-time trading data and technological analysis without the need for logging in or authentication.

## Features
- **Real-time Trading Data**: Retrieve real-time trading data across multiple exchanges, query multiple stocks at once, and customize data columns.
- **Technological Analysis**: Calculate technical indicators and trading suggestions identical to those on the TradingView website.
- **No Authentication Required**: Use the API functionalities without the need for any login credentials.

## Installation
Add `EasyTradeView` to your Rust project's `Cargo.toml`:
```toml
[dependencies]
easytradeview = "0.2.0"
```
## Usage
For usage examples, please see the examples directory within this repository. 

### Examples Overview
* **tradingview_get.rs**: Demonstrates how to retrieve real-time trading data.
* **tradingview_ta.rs**: Shows how to calculate technical analysis for a given stock or asset.