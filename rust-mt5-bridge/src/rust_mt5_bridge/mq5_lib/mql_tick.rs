//! Mapping of https://www.mql5.com/en/docs/constants/structures/mqltick to Rust


use std::fmt::{Debug, Formatter};
use chrono::NaiveDateTime;
use super::{
    super::types::{TickEvent, Trade, Spread, TradeParty},
    types::*,
};


/// Representation of the Metatrader 5 `MqlTick` structure. From the site:/
/// This is a structure for storing the latest prices of the symbol. It is designed for fast retrieval of the most requested information about current prices.\
/// auto-generated from https://www.mql5.com/en/docs/constants/structures/mqltick
#[repr(C)]	// note: by using `MQ5UnalignedF64` for the unaligned `f64` field, we avoid having to use `#[repr(C, packed(4))]`, which would require the struct to be Copy as well
pub struct Mq5MqlTick {
	/// Time of the last prices update
	pub time: MQ5DateTime,
	/// Current Bid price
	pub bid: f64,
	/// Current Ask price
	pub ask: f64,
	/// Price of the last deal (Last)
	pub last: f64,
	/// Volume for the current Last price
	pub volume: u64,
	/// Time of a price last update in milliseconds
	pub time_msc: i64,
	/// Tick flags
	pub flags: u32,
	/// Volume for the current Last price with greater accuracy
	/// -- just an `f64`, but outside Rust's 8-byte alignment requirements for the type
	pub volume_real: MQ5UnalignedF64,
}
impl Mq5MqlTick {
	/// Copies the reference to the original [Mq5MqlTick] struct to its Rust optimized version -- which may
	/// be worked on later, as the original reference must be returned (as fast as possible) to be reused
	pub fn to_internal<'a>(&self, symbol: &'a String) -> MqlTick<'a> {
		let volume_real = f64::from_ne_bytes(self.volume_real);
		MqlTick {
			symbol,
			time_msc: self.time_msc,
			bid:      self.bid,
			ask:      self.ask,
			last:     self.last,
			volume:   volume_real,
			flags:    self.flags,
		}
	}
}
impl Debug for Mq5MqlTick {
	// Debug is manually implemented here to allow parsing the unaligned `MQ5UnalignedF64` field
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let Self { time, bid, ask, last, volume, time_msc, flags, volume_real } = self;
		let volume_real = f64::from_ne_bytes(*volume_real);
		write!(f, "Mq5MqlTick {{ time: {time}, bid: {bid}, ask: {ask}, last: {last}, volume: {volume}, time_msc: {time_msc}, flags: {flags}, volume_real: {volume_real} }}")
	}
}


/// Rust version of the internal Metatrader 5 struct [Mq5MqlTick], with correct alignment and redundant fields removed
#[derive(Debug)]
pub struct MqlTick<'a> {
	/// The symbol for which this tick was generated
	pub symbol: &'a String,
	/// Time of the last trade or book top update (in ms)
	pub time_msc: i64,
	/// Current Bid price (from observation: the current book top)
	pub bid: f64,
	/// Current Ask price (from observation: the current book top)
	pub ask: f64,
	/// Price of the last deal (Last)
	pub last: f64,
	/// Volume for the current Last price
	pub volume: f64,
	/// Tick flags
	pub flags: u32,
}

impl MqlTick<'_> {

	/// Extracts events from the Metatrader 5 `OnTick()` event./
	/// The provided information is tricky and rather poorly documented -- it is most likely that it is dependent on the broker being used./
	/// For instance, on the "Clear broker (for B3)", flags are known to come zeroed out for legitimate trades --
	/// other discrepancies also happen, like a tick event having the information of both Buying and Selling flags as well as, some times, even bids > asks./
	/// For that particular broker, some trades misses the event completely, but, fortunately, only when there isn't a price change.
	pub fn to_event(&self) -> TickEvent {

		// `NaiveDateTime` with millisecond precision
		let datetime = NaiveDateTime::from_timestamp(self.time_msc as i64 / 1000, 1000_000 * (self.time_msc % 1000) as u32);

		// trade event?
		if self.flags == 0 || self.flags & (TICK_FLAG_BID|TICK_FLAG_ASK|TICK_FLAG_VOLUME) != self.flags {
			let trade = Trade {
				symbol:    self.symbol,
				time:      datetime,
				// NOTE: According to production data (see tests), the aggressor determination is not always accurate. Further analysis
				//       may improve the classification -- for instance, by keeping track of the last bid and ask price points
				aggressor: if (self.flags & TICK_FLAG_BUY > 0 && self.flags & TICK_FLAG_SELL > 0) ||
					(self.last == self.ask && self.last == self.bid) {
					TradeParty::Ambiguous {bid: self.bid, ask: self.ask}
				} else if self.flags & TICK_FLAG_BUY > 0 || self.last >= self.ask {
					TradeParty::Buyer {bid: self.bid, ask: self.ask}
				} else if self.flags & TICK_FLAG_SELL > 0 || self.last <= self.bid {
					TradeParty::Seller {bid: self.bid, ask: self.ask}
				} else {
					TradeParty::Unspecified {bid: self.bid, ask: self.ask}
				},
				quantity:  self.volume as u32,
				price:     self.last,
			};
			TickEvent::Trade(trade)
		} else {
			// spread event (book top)
			let spread = Spread {
				symbol:    self.symbol,
				time:      datetime,
				best_bid:  self.bid,
				best_ask:  self.ask,
			};
			TickEvent::Spread(spread)
		}

	}
}

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


/// Dumps the Rust internal values of the constants used in `MqlTick::flags`
pub fn serialize_mql_tick_flag_constants() -> String {
	format!("MqlTick::flags {{ TICK_FLAG_BID: {TICK_FLAG_BID}, TICK_FLAG_ASK: {TICK_FLAG_ASK}, TICK_FLAG_LAST: {TICK_FLAG_LAST}, TICK_FLAG_VOLUME: {TICK_FLAG_VOLUME}, TICK_FLAG_BUY: {TICK_FLAG_BUY}, TICK_FLAG_SELL: {TICK_FLAG_SELL} }}")
}