use std::collections::VecDeque;
pub use super::mq5_lib::*;

use std::fmt::{Debug, Formatter};
use chrono::NaiveDateTime;


#[derive(Debug)]
pub struct Handle {
    pub client_type:   ClientType,
    pub account_token: String,
    pub algorithm:     String,  // this should be parsed
    pub symbol:        String,
    pub books:         OrderBooks,
    // what else should I keep here or just on the server? open positions, symbol information, book, trades, etc...
}


#[derive(Debug)]
pub struct OrderBooks {
    /// keeps the selling intentions in ascending order (by price), with one entry for each price level
    pub sell_orders: VecDeque<MqlBookInfo>,
    /// keeps the buying intentions in descending order (by price), with one entry for each price level
    pub buy_orders:  VecDeque<MqlBookInfo>,
}
impl OrderBooks {
    /// Iterates over the book entries in the same order as MetaTrader presents theirs: Sell orders (descending by price), spread, Buy orders (ascending by price)
    pub fn iter(&self) -> impl Iterator<Item=&MqlBookInfo> {
        self.sell_orders.iter().chain(self.buy_orders.iter())
    }
}

#[derive(Debug,PartialEq)]
pub enum BookEvents {
    Add    { book: BookParties, price: f64, quantity: f64 },
    Del    { book: BookParties, price: f64, quantity: f64 },
    /// Contains the delta `quantity` -- how much to add to the old book to turn that into the new one
    Update { book: BookParties, price: f64, quantity: f64 },
}

#[derive(Debug,PartialEq)]
pub enum BookParties {
    Sellers,
    Buyers,
}
impl BookParties {
    pub fn from_mt5_enum_book(book_type: EnumBookType) -> Self {
        match book_type {
            EnumBookType::BookTypeSell | EnumBookType::BookTypeSellMarket=> Self::Sellers,
            EnumBookType::BookTypeBuy  | EnumBookType::BookTypeBuyMarket => Self::Buyers,
            _ => panic!("Unknown `EnumBookType`: {:?}", book_type)
        }
    }
    pub fn to_mt5_enum_book(&self) -> EnumBookType {
        match self {
            BookParties::Sellers => EnumBookType::BookTypeSell,
            BookParties::Buyers => EnumBookType::BookTypeBuy,
        }
    }
}

#[derive(Debug)]
pub enum ClientType {
    ProductionExpertAdvisor,
    TestingExpertAdvisor,
}

#[derive(Debug)]
pub enum TradeParty {
    Ambiguous   {bid: f64, ask: f64},
    Buyer       {bid: f64, ask: f64},
    Seller      {bid: f64, ask: f64},
    Unspecified {bid: f64, ask: f64},
}

#[derive(Debug)]
pub struct Trade<'a> {
    pub symbol:    &'a String,
    pub time:      NaiveDateTime,
    pub aggressor: TradeParty,
    pub quantity:  u32,
    pub price:     f64
}

#[derive(Debug)]
pub struct Spread<'a> {
    pub symbol:   &'a String,
    pub time:     NaiveDateTime,
    pub best_bid: f64,
    pub best_ask: f64,
}

pub enum TickEvent<'a> {
    Trade(Trade<'a>),
    Spread(Spread<'a>),
}

///////////////////////////
// Metatrader 5 mappings //
///////////////////////////


// Reasons for the `reason` param from the `OnDeinit(reason)` event
///////////////////////////////////////////////////////////////////
// https://www.mql5.com/en/docs/constants/namedconstants/uninit

/// Expert Advisor terminated its operation by calling the ExpertRemove() function
const REASON_PROGRAM: u32 = 0;
/// Program has been deleted from the chart
const REASON_REMOVE: u32 = 1;
/// Program has been recompiled
const REASON_RECOMPILE: u32 = 2;
/// Symbol or chart period has been changed
const REASON_CHARTCHANGE: u32 = 3;
/// Chart has been closed
const REASON_CHARTCLOSE: u32 = 4;
/// Input parameters have been changed by a user
const REASON_PARAMETERS: u32 = 5;
/// Another account has been activated or reconnection to the trade server has occurred due to changes in the account settings
const REASON_ACCOUNT: u32 = 6;
/// A new template has been applied
const REASON_TEMPLATE: u32 = 7;
/// This value means that OnInit() handler has returned a nonzero value
const REASON_INITFAILED: u32 = 8;
/// Terminal has been closed
const REASON_CLOSE: u32 = 9;

/// Dumps the Rust internal values of the constants used in `OnDeinit(reason)` event calls
pub fn serialize_on_deinit_reasons() -> String {
    format!("OnDeinit::reasons {{ REASON_PROGRAM: {REASON_PROGRAM}, REASON_REMOVE: {REASON_REMOVE}, REASON_RECOMPILE: {REASON_RECOMPILE}, REASON_CHARTCHANGE: {REASON_CHARTCHANGE}, REASON_CHARTCLOSE: {REASON_CHARTCLOSE}, REASON_PARAMETERS: {REASON_PARAMETERS}, REASON_ACCOUNT: {REASON_ACCOUNT}, REASON_TEMPLATE: {REASON_TEMPLATE}, REASON_INITFAILED: {REASON_INITFAILED}, REASON_CLOSE: {REASON_CLOSE} }}")
}


pub struct _OptionNonZeroF64 {
    internal: f64,
}
impl _OptionNonZeroF64 {
    pub fn some(value: f64) -> Self {
        Self { internal: value }
    }
    pub fn none() -> Self {
        Self { internal: 0.0 }
    }
    pub fn is_some_and(&self, f: impl FnOnce(f64) -> bool) -> bool {
        self.internal > 0.0 && f(self.internal)
    }
}
impl Debug for _OptionNonZeroF64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.internal == 0.0 {
            write!(f, "None")
        } else {
            write!(f, "Some({})", self.internal)
        }
    }
}
