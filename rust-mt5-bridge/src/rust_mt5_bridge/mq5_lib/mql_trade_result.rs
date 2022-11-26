//! Mapping of https://www.mql5.com/en/docs/constants/structures/mqltraderesult to Rust


use super::types::*;


/// Rust version of the Metatrader 5 `MqlTradeResult` structure. From the site:/
/// As result of a <a href="/en/docs/constants/structures/mqltraderequest" class="topiclink">trade request</a>, a trade server returns data about the trade request processing result as a special predefined structure of MqlTradeResult type.\
/// auto-generated from https://www.mql5.com/en/docs/constants/structures/mqltraderesult
#[repr(C, packed(4))]
#[derive(/*disable debug on this structure for production since it will cause a copy due to 'packed(4)' above*/Debug,Copy,Clone)]
pub struct MqlTradeResult {
	/// Operation return code
	pub retcode: u32,
	/// Deal ticket, if it is performed
	pub deal: u64,
	/// Order ticket, if it is placed
	pub order: u64,
	/// Deal volume, confirmed by broker
	pub volume: f64,
	/// Deal price, confirmed by broker
	pub price: f64,
	/// Current Bid price
	pub bid: f64,
	/// Current Ask price
	pub ask: f64,
	/// Broker comment to operation (by default it is filled by description of trade server return code)
	pub comment: MQ5String,
	/// Request ID set by the terminal during the dispatch 
	pub request_id: u32,
	/// Return code of an external trading system
	pub retcode_external: u32,
}

