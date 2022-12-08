//! Mapping of https://www.mql5.com/en/docs/constants/structures/mqltraderequest to Rust


use super::{
	types::*,
	super::{
		types::{ENUM_ORDER_TYPE, ENUM_ORDER_TYPE_TIME, EnumOrderType, EnumOrderTypeTime},
		mql_rust_enum::{MqlRustEnumDescriptor}
	},
};
use std::str::FromStr;
use chrono::NaiveDateTime;
use once_cell::sync::Lazy;
use strum::{EnumString,FromRepr};


/// Rust version of the Metatrader 5 `MqlTradeRequest` structure. From the site:/
/// Interaction between the client terminal and a trade server for executing the order placing operation is performed by using trade requests. The trade request is represented by the special predefined <a href="/en/docs/basis/types/classes" class="topiclink">structure</a> of MqlTradeRequest type, which contain all the fields necessary to perform trade deals. The request processing result is represented by the structure of <a href="/en/docs/constants/structures/mqltraderesult" class="topiclink">MqlTradeResult</a> type.\
/// auto-generated from https://www.mql5.com/en/docs/constants/structures/mqltraderequest
#[repr(C, packed(4))]
#[derive(/*disable debug on this structure for production since it will cause a copy due to 'packed(4)' above*/Debug,Copy,Clone)]
pub struct Mq5MqlTradeRequest {
	/// Trade operation type
	pub action: i32, // EnumTradeRequestActions,
	/// Expert Advisor ID (magic number)
	pub magic: u64,
	/// Order ticket
	pub order: u64,
	/// Trade symbol
	pub symbol: MQ5String,
	/// Requested volume for a deal in lots
	pub volume: f64,
	/// Price
	pub price: f64,
	/// StopLimit level of the order
	pub stoplimit: f64,
	/// Stop Loss level of the order
	pub sl: f64,
	/// Take Profit level of the order
	pub tp: f64,
	/// Maximal possible deviation from the requested price
	pub deviation: u64,
	/// Order type
	pub order_type: i32, // EnumOrderType,
	/// Order execution type
	pub order_type_filling: i32, // EnumOrderTypeFilling,
	/// Order expiration type
	pub order_type_time: i32, // EnumOrderTypeTime,
	/// Order expiration time (for the orders of ORDER_TIME_SPECIFIED type)
	pub expiration: MQ5DateTime,
	/// Order comment
	pub comment: MQ5String,
	/// Position ticket
	pub position: u64,
	/// The ticket of an opposite position
	pub position_by: u64,
}
impl Mq5MqlTradeRequest {

	pub fn from_ptr_to_internal(mq5_mql_trade_transaction: *const Mq5MqlTradeRequest) -> MqlTradeRequest {

		let mq5_mql_trade_transaction = unsafe { &*mq5_mql_trade_transaction };

		log::debug!("on_trade_transaction(xx): _____: {:#?}", mq5_mql_trade_transaction);

		MqlTradeRequest {
			action: ENUM_TRADE_REQUEST_ACTIONS.resolve_rust_variant(mq5_mql_trade_transaction.action),
			magic: mq5_mql_trade_transaction.magic,
			order: mq5_mql_trade_transaction.order,
			symbol: string_from_mql_string(&mq5_mql_trade_transaction.symbol),
			volume: mq5_mql_trade_transaction.volume,
			price: mq5_mql_trade_transaction.price,
			stoplimit: mq5_mql_trade_transaction.stoplimit,
			sl: mq5_mql_trade_transaction.sl,
			tp: mq5_mql_trade_transaction.tp,
			deviation: mq5_mql_trade_transaction.deviation,
			order_type: ENUM_ORDER_TYPE.resolve_rust_variant(mq5_mql_trade_transaction.order_type),
			order_type_filling: ENUM_ORDER_TYPE_FILLING.resolve_rust_variant(mq5_mql_trade_transaction.order_type_filling),
			order_type_time: ENUM_ORDER_TYPE_TIME.resolve_rust_variant(mq5_mql_trade_transaction.order_type_time),
			expiration: NaiveDateTime::from_timestamp(mq5_mql_trade_transaction.expiration as i64, 0),
			comment: string_from_mql_string(&mq5_mql_trade_transaction.comment),
			position: mq5_mql_trade_transaction.position,
			position_by: mq5_mql_trade_transaction.position_by,
		}
	}
}

/// Rust version of the Metatrader 5 [Mq5MqlTradeRequest] structure, with correct alignment, redundant fields removed, dates, strings and enums resolved and copied to Rust -- so the MQL reference may be freed as soon as possible in MT5
#[derive(Debug)]
pub struct MqlTradeRequest {
	/// Trade operation type
	pub action: EnumTradeRequestActions,
	/// Expert Advisor ID (magic number)
	pub magic: u64,
	/// Order ticket
	pub order: u64,
	/// Trade symbol
	pub symbol: String,
	/// Requested volume for a deal in lots
	pub volume: f64,
	/// Price
	pub price: f64,
	/// StopLimit level of the order
	pub stoplimit: f64,
	/// Stop Loss level of the order
	pub sl: f64,
	/// Take Profit level of the order
	pub tp: f64,
	/// Maximal possible deviation from the requested price
	pub deviation: u64,
	/// Order type
	pub order_type: EnumOrderType,
	/// Order execution type
	pub order_type_filling: EnumOrderTypeFilling,
	/// Order expiration type
	pub order_type_time: EnumOrderTypeTime,
	/// Order expiration time (for the orders of ORDER_TIME_SPECIFIED type)
	pub expiration: NaiveDateTime,
	/// Order comment
	pub comment: String,
	/// Position ticket
	pub position: u64,
	/// The ticket of an opposite position
	pub position_by: u64,
}

/// Trading is done by sending orders to open positions using the OrderSend() function, as well as to place, modify or delete pending orders. Each trade order refers to the type of the requested operation. Trading operations are described in the ENUM_TRADE_REQUEST_ACTIONS enumeration./
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/enum_trade_request_actions
#[repr(i32)]
#[derive(Debug,PartialEq,EnumString,FromRepr,Clone,Copy)]
pub enum EnumTradeRequestActions {
	/// Place a trade order for an immediate execution with the specified parameters (market order)
	TradeActionDeal,
	/// Place a trade order for the execution under specified conditions (pending order)
	TradeActionPending,
	/// Modify Stop Loss and Take Profit values of an opened position
	TradeActionSltp,
	/// Modify the parameters of the order placed previously
	TradeActionModify,
	/// Delete the pending order placed previously
	TradeActionRemove,
	/// Close a position by an opposite one
	TradeActionCloseBy,

	/// in case MQL Code is out of sync with the DLL version...
	UnknownMqlVariantValue = -1,
}
impl Into<i32> for EnumTradeRequestActions {
	fn into(self) -> i32 {
		self as i32
	}
}
impl From<i32> for EnumTradeRequestActions {
	fn from(variant_value: i32) -> Self {
		if let Some(variant) = Self::from_repr(variant_value) {
			return variant;
		}
		Self::UnknownMqlVariantValue
	}
}

/// SYMBOL_TRADE_EXECUTION_EXCHANGE/
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/orderproperties
#[repr(i32)]
#[derive(Debug,PartialEq,EnumString,FromRepr,Clone,Copy)]
pub enum EnumOrderTypeFilling {
	/// Fill or Kill/
	/// An order can be executed in the specified volume only./
	/// If the necessary amount of a financial instrument is currently unavailable in the market, the order will not be executed./
	/// The desired volume can be made up of several available offers./
	/// The possibility of using FOK orders is determined at the trade server.
	OrderFillingFok,
	/// Immediate Or Cancel./
	/// A trader agrees to execute a deal with the volume maximally available in the market within that indicated in the order./
	/// If the request cannot be filled completely, an order with the available volume will be executed, and the remaining volume will be canceled./
	/// The possibility of using IOC orders is determined at the trade server./
	OrderFillingIoc,
	/// In case of partial filling, an order with remaining volume is not canceled but processed further./
	/// Return orders are not allowed in the Market Execution mode (market execution — SYMBOL_TRADE_EXECUTION_MARKET).
	OrderFillingReturn,

	/// in case MQL Code is out of sync with the DLL version...
	UnknownMqlVariantValue = -1,
}
impl Into<i32> for EnumOrderTypeFilling {
	fn into(self) -> i32 {
		self as i32
	}
}
impl From<i32> for EnumOrderTypeFilling {
	fn from(variant_value: i32) -> Self {
		if let Some(variant) = Self::from_repr(variant_value) {
			return variant;
		}
		Self::UnknownMqlVariantValue
	}
}

pub static ENUM_TRADE_REQUEST_ACTIONS: Lazy<&MqlRustEnumDescriptor> = Lazy::new(|| MqlRustEnumDescriptor::new("EnumTradeRequestActions", &EnumTradeRequestActions::from_str));
pub static ENUM_ORDER_TYPE_FILLING:    Lazy<&MqlRustEnumDescriptor> = Lazy::new(|| MqlRustEnumDescriptor::new("EnumOrderTypeFilling", &EnumOrderTypeFilling::from_str));

/// called when the program starts -- to register the MQL<=>Rust Enums
pub fn init() {
	log::info!("Internally registering ENUM '{}'", ENUM_TRADE_REQUEST_ACTIONS.name());
	log::info!("Internally registering ENUM '{}'", ENUM_ORDER_TYPE_FILLING.name());
}