use std::fmt::{Debug, Formatter};
use std::num::NonZeroU64;
use chrono::NaiveDateTime;


#[derive(Debug)]
pub struct Handle {
    pub client_type:   ClientType,
    pub account_token: String,
    pub algorithm:     String,  // this should be parsed
    pub symbol:        String,
    // what else should I keep here or just on the server? open positions, symbol information, book, trades, etc...
}

#[derive(Debug)]
pub enum ClientType {
    ProductionExpertAdvisor,
    TestingExpertAdvisor,
}

#[derive(Debug)]
pub enum Party {
    Ambiguous {bid: f64, ask: f64},
    Buyer,
    Seller,
    Unspecified {bid: f64, ask: f64},
}

#[derive(Debug)]
pub struct TradeEvent {
    pub time:      NaiveDateTime,
    pub aggressor: Party,
    pub quantity:  u32,
    pub price:     f64
}

#[derive(Debug)]
pub struct BookEvent {
    pub time:             NaiveDateTime,
    pub ask_price_update: OptionNonZeroF64,
    pub bid_price_update: OptionNonZeroF64,
    pub volume_update:    Option<NonZeroU64>,
}


///////////////////////////
// Metatrader 5 mappings //
///////////////////////////


/// Number of seconds since January 01, 1970
type MT5DateTime = u64;


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

// flags for `MqlTick::flags`
/////////////////////////////

/// tick has changed a Bid price
pub const TICK_FLAG_BID: u32 = 2;
/// a tick has changed an Ask price
pub const TICK_FLAG_ASK: u32 = 4;
/// a tick has changed the last deal price
pub const TICK_FLAG_LAST: u32 = 8;
/// a tick has changed a volume
pub const TICK_FLAG_VOLUME: u32 = 16;
/// a tick is a result of a buy deal
pub const TICK_FLAG_BUY: u32 = 32;
/// a tick is a result of a sell deal
pub const TICK_FLAG_SELL: u32 = 64;


/// Rust version of the Metatrader 5 `MqlTick` structure -- https://www.mql5.com/en/docs/constants/structures/mqltick
/// (for type mapping, refer to https://www.mql5.com/en/docs/basis/types)
#[repr(C)]
#[derive(Debug)]
pub struct MqlTick {
    /// Time of the last prices update
    time: MT5DateTime,
    /// Current Bid price
    bid: f64,
    /// Current Ask price
    ask: f64,
    /// Price of the last deal (Last)
    last: f64,
    /// Volume for the current Last price
    volume: u64,
    /// Time of a price last update in milliseconds
    time_msc: i64,
    /// Tick flags
    flags: u32,
    /// Volume for the current Last price with greater accuracy
    volume_real: f64,
}
impl MqlTick {

    pub fn to_trade_event(&self) -> Option<TradeEvent> {
        if self.flags > 0 && self.flags & (TICK_FLAG_BID|TICK_FLAG_ASK|TICK_FLAG_VOLUME) == self.flags {
            // book event only
            //     Some(BookEvent {
            //         time:             datetime,
            //         ask_price_update: if self.flags & TICK_FLAG_ASK > 0    { OptionNonZeroF64::Some(self.ask) } else { OptionNonZeroF64::None() },
            //         bid_price_update: if self.flags & TICK_FLAG_BID > 0    { OptionNonZeroF64::Some(self.bid) } else { OptionNonZeroF64::None() },
            //         volume_update:    NonZeroU64::new(self.volume),     // Volume exists for BID & ASK even if MT5 don't sent the VOLUME flag
            //     })
            None
        } else {
            // trade event
            // `NaiveDateTime` with millisecond precision from the fields `time` and `time_msc`
            let datetime = NaiveDateTime::from_timestamp(self.time as i64, 1000_000 * (self.time_msc % 1000) as u32);
            Some(TradeEvent {
                time:      datetime,
                aggressor: if (self.flags & TICK_FLAG_BUY > 0 && self.flags & TICK_FLAG_SELL > 0) ||
                    (self.last == self.ask && self.last == self.bid) {
                    Party::Ambiguous {bid: self.bid, ask: self.ask}
                } else if self.flags & TICK_FLAG_BUY > 0 || self.last == self.ask {
                    Party::Buyer
                } else if self.flags & TICK_FLAG_SELL > 0 || self.last == self.bid {
                    Party::Seller
                } else {
                    Party::Unspecified {bid: self.bid, ask: self.ask}
                },
                quantity: self.volume as u32,
                price:    self.last,
            })
        }
    }
}
impl Default for MqlTick {
    fn default() -> Self {
        Self {
            time: 2,
            bid: 3.0,
            ask: 4.0,
            last: 5.0,
            volume: 6,
            time_msc: 7,
            flags: 255,
            volume_real: 8.0
        }
    }
}

pub struct OptionNonZeroF64 {
    internal: f64,
}
impl OptionNonZeroF64 {
    pub fn Some(value: f64) -> Self {
        Self { internal: value }
    }
    pub fn None() -> Self {
        Self { internal: 0.0 }
    }
    pub fn is_some_and(&self, f: impl FnOnce(f64) -> bool) -> bool {
        self.internal > 0.0 && f(self.internal)
    }
}
impl Debug for OptionNonZeroF64 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.internal == 0.0 {
            write!(f, "None")
        } else {
            write!(f, "Some({})", self.internal)
        }
    }
}
