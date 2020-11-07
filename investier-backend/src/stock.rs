use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StockType {
    ETF,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StockSymbol(pub String);

impl fmt::Display for StockSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Stock {
    stock_type: StockType,
    symbol: StockSymbol,
    amount: i32,
}

impl Stock {
    pub fn new(symbol: StockSymbol, amount: i32) -> Self {
        Stock {
            stock_type: StockType::ETF,
            amount: amount,
            symbol: symbol,
        }
    }

    pub fn add_amount(&mut self, amount: i32) {
        self.amount += amount;
    }

    pub fn get_amount(&self) -> i32 {
        self.amount
    }

    pub fn get_symbol(&self) -> &StockSymbol {
        return &self.symbol
    }
}
