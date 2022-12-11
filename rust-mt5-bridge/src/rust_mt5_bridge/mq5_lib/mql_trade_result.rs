//! Mapping of https://www.mql5.com/en/docs/constants/structures/mqltraderesult to Rust


use super::{
	types::*,
	mt5_codes::Mt5TradeServerReturnCodes,
};


/// Rust version of the Metatrader 5 `MqlTradeResult` structure. From the site:/
/// As result of a trade request, a trade server returns data about the trade request processing result as a special predefined structure of MqlTradeResult type.\
/// auto-generated from https://www.mql5.com/en/docs/constants/structures/mqltraderesult
#[repr(C, packed(4))]
#[derive(/*disable debug on this structure for production since it will cause a copy due to 'packed(4)' above*/Debug,Copy,Clone)]
pub struct Mq5MqlTradeResult {
	/// Operation return code
	pub retcode: u32,	// make an enum out of those values -- https://www.mql5.com/en/docs/constants/errorswarnings/enum_trade_return_codes
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
impl Mq5MqlTradeResult {

	pub fn from_ptr_to_internal(mq5_mql_trade_result: *const Mq5MqlTradeResult) -> MqlTradeResult {

		let mq5_mql_trade_result = unsafe { &*mq5_mql_trade_result };

		log::debug!("on_trade_transaction(xx): _____: {:#?}", mq5_mql_trade_result);

		MqlTradeResult {
			retcode: Mt5TradeServerReturnCodes::from(mq5_mql_trade_result.retcode),
			deal: mq5_mql_trade_result.deal,
			order: mq5_mql_trade_result.order,
			volume: mq5_mql_trade_result.volume,
			price: mq5_mql_trade_result.price,
			bid: mq5_mql_trade_result.bid,
			ask: mq5_mql_trade_result.ask,
			comment: string_from_mql_string(&mq5_mql_trade_result.comment),
			request_id: mq5_mql_trade_result.request_id,
			retcode_external: mq5_mql_trade_result.retcode_external,
		}
	}

}

/// Rust version of the Metatrader 5 [Mq5MqlTradeResult], with correct alignment, redundant fields removed, dates, strings & enums resolved and copied to Rust -- so the MQL reference may be freed as soon as possible in MT5
#[derive(Debug)]
pub struct MqlTradeResult {
	/// Operation return code
	pub retcode: Mt5TradeServerReturnCodes,
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
	pub comment: String,
	/// Request ID set by the terminal during the dispatch
	pub request_id: u32,
	/// Return code of an external trading system
	pub retcode_external: u32,

}