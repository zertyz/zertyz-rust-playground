//! Mapping of https://www.mql5.com/en/docs/constants/structures/mqltraderequest to Rust


use super::types::*;


/// Rust version of the Metatrader 5 `MqlTradeRequest` structure. From the site:/
/// Interaction between the client terminal and a trade server for executing the order placing operation is performed by using trade requests. The trade request is represented by the special predefined <a href="/en/docs/basis/types/classes" class="topiclink">structure</a> of MqlTradeRequest type, which contain all the fields necessary to perform trade deals. The request processing result is represented by the structure of <a href="/en/docs/constants/structures/mqltraderesult" class="topiclink">MqlTradeResult</a> type.\
/// auto-generated from https://www.mql5.com/en/docs/constants/structures/mqltraderequest
#[repr(C, packed(4))]
#[derive(/*disable debug on this structure for production since it will cause a copy due to 'packed(4)' above*/Debug,Copy,Clone)]
pub struct MqlTradeRequest {
	/// Trade operation type
	pub action: EnumTradeRequestActions,
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
	pub type_: EnumOrderType,
	/// Order execution type
	pub type_filling: EnumOrderTypeFilling,
	/// Order expiration type
	pub type_time: EnumOrderTypeTime,
	/// Order expiration time (for the orders of ORDER_TIME_SPECIFIED type)
	pub expiration: MQ5DateTime,
	/// Order comment
	pub comment: MQ5String,
	/// Position ticket
	pub position: u64,
	/// The ticket of an opposite position
	pub position_by: u64,
}

/// Trading is done by sending orders to open positions using the OrderSend() function, as well as to place, modify or delete pending orders. Each trade order refers to the type of the requested operation. Trading operations are described in the ENUM_TRADE_REQUEST_ACTIONS enumeration./
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/enum_trade_request_actions
#[repr(i32)]
#[derive(Debug,Copy,Clone)]
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

	// this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
	Unmapped6,  Unmapped7,  Unmapped8,  Unmapped9,  Unmapped10, Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
	Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
	Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
	Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
	Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
	Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,
}

/// When sending a trade request using the OrderSend() function, some operations require the indication of the order type. The order type is specified in the type field of the special structure MqlTradeRequest, and can accept values of the ENUM_ORDER_TYPE enumeration./
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/orderproperties
#[repr(i32)]
#[derive(Debug,Copy,Clone)]
pub enum EnumOrderType {
	/// Market Buy order
	OrderTypeBuy,
	/// Market Sell order
	OrderTypeSell,
	/// Buy Limit pending order
	OrderTypeBuyLimit,
	/// Sell Limit pending order
	OrderTypeSellLimit,
	/// Buy Stop pending order
	OrderTypeBuyStop,
	/// Sell Stop pending order
	OrderTypeSellStop,
	/// Upon reaching the order price, a pending Buy Limit order is placed at the StopLimit price
	OrderTypeBuyStopLimit,
	/// Upon reaching the order price, a pending Sell Limit order is placed at the StopLimit price
	OrderTypeSellStopLimit,
	/// Order to close a position by an opposite one
	OrderTypeCloseBy,

	// this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
	Unmapped9,  Unmapped10, Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
	Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
	Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
	Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
	Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
	Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,
}

/// SYMBOL_TRADE_EXECUTION_EXCHANGE/
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/orderproperties
#[repr(i32)]
#[derive(Debug,Copy,Clone)]
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

	// this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
	Unmapped3,  Unmapped4,  Unmapped5,  Unmapped6,  Unmapped7,
	Unmapped8,  Unmapped9,  Unmapped10, Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
	Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
	Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
	Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
	Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
	Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,

}

/// The order validity period can be set in the type_time field of the special structure MqlTradeRequest when sending a trade request using the OrderSend() function. Values of the ENUM_ORDER_TYPE_TIME enumeration are allowed. To obtain the value of this property use the function OrderGetInteger() or HistoryOrderGetInteger() with the ORDER_TYPE_TIME modifier./
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/orderproperties
#[repr(i32)]
#[derive(Debug,Copy,Clone)]
pub enum EnumOrderTypeTime {
	/// Good till cancel order
	OrderTimeGtc,
	/// Good till current trade day order
	OrderTimeDay,
	/// Good till expired order
	OrderTimeSpecified,
	/// The order will be effective till 23:59:59 of the specified day. If this time is outside a trading session, the order expires in the nearest trading time.
	OrderTimeSpecifiedDay,

	// this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
	Unmapped4,  Unmapped5,  Unmapped6,  Unmapped7,
	Unmapped8,  Unmapped9,  Unmapped10, Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
	Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
	Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
	Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
	Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
	Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,

}