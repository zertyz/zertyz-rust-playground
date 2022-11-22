//! Mapping of https://www.mql5.com/en/docs/constants/structures/mqltick to Rust


use chrono::NaiveDateTime;
use super::{
	super::types::{TickEvent, Trade, Spread, Party},
	types::*,
};


/// Rust version of the Metatrader 5 `MqlTick` structure. From the site:/
/// This is a structure for storing the latest prices of the symbol. It is designed for fast retrieval of the most requested information about current prices.\
/// auto-generated from https://www.mql5.com/en/docs/constants/structures/mqltick
#[repr(C, packed(4))]
/*Once `from_ptr_to_internal() is all good, remove this and use that instead`*/#[derive(Copy,Clone,Debug)]
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
	/// (this is an `f64` that falls out of 8-bytes alignment)
	pub volume_real: f64//MQ5UnalignedF64,
}
impl Mq5MqlTick {
// 	pub fn from_ptr_to_internal(tick: *const Mq5MqlTick) -> MqlTick {
// 		let tick = unsafe { &*tick };
// 		let volume_real = f64::from_ne_bytes(tick.volume_real);
// 		MqlTick {
// 			time_msc: tick.time_msc,
// 			bid:      tick.bid,
// 			ask:      tick.ask,
// 			last:     tick.last,
// 			volume:   volume_real,
// 			flags:    tick.flags,
// 		}
// 	}

	/// Extracts events from the Metatrader 5 `OnTick()` event./
	/// The provided information is tricky and rather poorly documented -- it is most likely that it is dependent on the broker being used./
	/// For instance, on the "Clear broker (for B3)", flags are known to come zeroed out for legitimate trades --
	/// other discrepancies also happen, like a tick event having the information of both Buying and Selling flags as well as, some times, even bids > asks./
	/// For that particular broker, some trades misses the event completely, but, fortunately, only when there isn't a price change.
	pub fn to_event(&self) -> TickEvent {

		// `NaiveDateTime` with millisecond precision from the fields `time` and `time_msc`
		let datetime = NaiveDateTime::from_timestamp(self.time as i64, 1000_000 * (self.time_msc % 1000) as u32);

		// trade event?
		if self.flags == 0 || self.flags & (TICK_FLAG_BID|TICK_FLAG_ASK|TICK_FLAG_VOLUME) != self.flags {
			let trade = Trade {
				time:      datetime,
				// NOTE: According to production data (see tests), the aggressor determination is not always accurate. Further analysis
				//       may improve the classification -- for instance, by keeping track of the last bid and ask price points
				aggressor: if (self.flags & TICK_FLAG_BUY > 0 && self.flags & TICK_FLAG_SELL > 0) ||
					(self.last == self.ask && self.last == self.bid) {
					Party::Ambiguous {bid: self.bid, ask: self.ask}
				} else if self.flags & TICK_FLAG_BUY > 0 || self.last >= self.ask {
					Party::Buyer {bid: self.bid, ask: self.ask}
				} else if self.flags & TICK_FLAG_SELL > 0 || self.last <= self.bid {
					Party::Seller {bid: self.bid, ask: self.ask}
				} else {
					Party::Unspecified {bid: self.bid, ask: self.ask}
				},
				quantity: self.volume as u32,
				price:    self.last,
			};
			TickEvent::Trade(trade)
		} else {
			// spread event (book top)
			let spread = Spread {
				time:     datetime,
				best_bid: self.bid,
				best_ask: self.ask,
			};
			TickEvent::Spread(spread)
		}

	}
}


/// Rust version of the internal Metatrader 5 struct [Mq5MqlTick], with redundant fields removed and
/// reordered by type to solve alignment issues
#[derive(Debug)]
pub struct MqlTick {
	/// Time of the last trade or book top update (in ms)
	pub time_msc: i64,
	/// Current Bid price (book top)
	pub bid: f64,
	/// Current Ask price (book top)
	pub ask: f64,
	/// Price of the last deal (Last)
	pub last: f64,
	/// Volume for the current Last price
	pub volume: f64,
	/// Tick flags
	pub flags: u32,
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
