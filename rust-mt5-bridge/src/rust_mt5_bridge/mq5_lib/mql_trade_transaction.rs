//! Mapping of https://www.mql5.com/en/docs/constants/structures/mqltradetransaction to Rust


use super::types::*;


/// Rust version of the Metatrader 5 `MqlTradeTransaction` structure. From the site:/
/// When performing some definite actions on a trade account, its state changes. Such actions include:\
/// auto-generated from https://www.mql5.com/en/docs/constants/structures/mqltradetransaction
#[repr(C, packed(4))]
#[derive(/*disable debug on this structure for production since it will cause a copy due to 'packed(4)' above*/Debug,Copy,Clone)]
pub struct MqlTradeTransaction {
	/// Deal ticket
	pub deal: u64,
	/// Order ticket
	pub order: u64,
	/// Trade symbol name
	pub symbol: MQ5String,
	/// Trade transaction type
	pub transaction_type: EnumTradeTransactionType,
	/// Order type
	pub order_type: EnumOrderType,
	/// Order state
	pub order_state: EnumOrderState,
	/// Deal type
	pub deal_type: EnumDealType,
	/// Order type by action period
	pub time_type: EnumOrderTypeTime,
	/// Order expiration time
	pub time_expiration: MQ5DateTime,
	/// Price 
	pub price: f64,
	/// Stop limit order activation price
	pub price_trigger: f64,
	/// Stop Loss level
	pub price_sl: f64,
	/// Take Profit level
	pub price_tp: f64,
	/// Volume in lots
	pub volume: f64,
	/// Position ticket
	pub position: u64,
	/// Ticket of an opposite position
	pub position_by: u64,
}

/// Trade transaction type is submitted in the type parameter of MqlTradeTransaction structure. Possible types of trade transactions are described by the following enumeration:/
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/enum_trade_transaction_type
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumTradeTransactionType {
	/// Adding a new open order.
	TradeTransactionOrderAdd,
	/// Updating an open order. The updates include not only evident changes from the client terminal or a trade server sides but also changes of an order state when setting it (for example, transition from ORDER_STATE_STARTED to ORDER_STATE_PLACED or from ORDER_STATE_PLACED to ORDER_STATE_PARTIAL, etc.).
	TradeTransactionOrderUpdate,
	/// Removing an order from the list of the open ones. An order can be deleted from the open ones as a result of setting an appropriate request or execution (filling) and moving to the history.
	TradeTransactionOrderDelete,
	/// Adding a deal to the history. The action is performed as a result of an order execution or performing operations with an account balance.
	TradeTransactionDealAdd,
	/// Updating a deal in the history. There may be cases when a previously executed deal is changed on a server. For example, a deal has been changed in an external trading system (exchange) where it was previously transferred by a broker.
	TradeTransactionDealUpdate,
	/// Deleting a deal from the history. There may be cases when a previously executed deal is deleted from a server. For example, a deal has been deleted in an external trading system (exchange) where it was previously transferred by a broker.
	TradeTransactionDealDelete,
	/// Adding an order to the history as a result of execution or cancellation.
	TradeTransactionHistoryAdd,
	/// Changing an order located in the orders history. This type is provided for enhancing functionality on a trade server side.
	TradeTransactionHistoryUpdate,
	/// Deleting an order from the orders history. This type is provided for enhancing functionality on a trade server side.
	TradeTransactionHistoryDelete,
	/// Changing a position not related to a deal execution. This type of transaction shows that a position has been changed on a trade server side. Position volume, open price, Stop Loss and Take Profit levels can be changed. Data on changes are submitted in MqlTradeTransaction structure via OnTradeTransaction handler. Position change (adding, changing or closing), as a result of a deal execution, does not lead to the occurrence of TRADE_TRANSACTION_POSITION transaction.
	TradeTransactionPosition,
	/// Notification of the fact that a trade request has been processed by a server and processing result has been received. Only type field (trade transaction type) must be analyzed for such transactions in MqlTradeTransaction structure. The second and third parameters of OnTradeTransaction (request and result) must be analyzed for additional data.
	TradeTransactionRequest,

	// this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
	Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
	Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
	Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
	Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
	Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
	Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,

}

/// When sending a trade request using the OrderSend() function, some operations require the indication of the order type. The order type is specified in the type field of the special structure MqlTradeRequest, and can accept values of the ENUM_ORDER_TYPE enumeration./
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/orderproperties
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
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

/// Each order has a status that describes its state. To obtain information, use OrderGetInteger() or HistoryOrderGetInteger() with the ORDER_STATE modifier. Allowed values are stored in the ENUM_ORDER_STATE enumeration./
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/orderproperties
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumOrderState {
	/// Order checked, but not yet accepted by broker
	OrderStateStarted,
	/// Order accepted
	OrderStatePlaced,
	/// Order canceled by client
	OrderStateCanceled,
	/// Order partially executed
	OrderStatePartial,
	/// Order fully executed
	OrderStateFilled,
	/// Order rejected
	OrderStateRejected,
	/// Order expired
	OrderStateExpired,
	/// Order is being registered (placing to the trading system)
	OrderStateRequestAdd,
	/// Order is being modified (changing its parameters)
	OrderStateRequestModify,
	/// Order is being deleted (deleting from the trading system)
	OrderStateRequestCancel,

	// this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
	Unmapped10, Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
	Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
	Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
	Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
	Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
	Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,

}

/// Each deal is characterized by a type, allowed values are enumerated in ENUM_DEAL_TYPE. In order to obtain information about the deal type, use the HistoryDealGetInteger() function with the DEAL_TYPE modifier./
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/dealproperties
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumDealType {
	/// Buy
	DealTypeBuy,
	/// Sell
	DealTypeSell,
	/// Balance
	DealTypeBalance,
	/// Credit
	DealTypeCredit,
	/// Additional charge
	DealTypeCharge,
	/// Correction
	DealTypeCorrection,
	/// Bonus
	DealTypeBonus,
	/// Additional commission
	DealTypeCommission,
	/// Daily commission
	DealTypeCommissionDaily,
	/// Monthly commission
	DealTypeCommissionMonthly,
	/// Daily agent commission
	DealTypeCommissionAgentDaily,
	/// Monthly agent commission
	DealTypeCommissionAgentMonthly,
	/// Interest rate
	DealTypeInterest,
	/// Canceled buy deal. There can be a situation when a previously executed buy deal is canceled. In this case, the type of the previously executed deal (DEAL_TYPE_BUY) is changed to DEAL_TYPE_BUY_CANCELED, and its profit/loss is zeroized. Previously obtained profit/loss is charged/withdrawn using a separated balance operation
	DealTypeBuyCanceled,
	/// Canceled sell deal. There can be a situation when a previously executed sell deal is canceled. In this case, the type of the previously executed deal (DEAL_TYPE_SELL) is changed to DEAL_TYPE_SELL_CANCELED, and its profit/loss is zeroized. Previously obtained profit/loss is charged/withdrawn using a separated balance operation
	DealTypeSellCanceled,
	/// Dividend operations
	DealDividend,
	/// Franked (non-taxable) dividend operations
	DealDividendFranked,
	/// Tax charges
	DealTax,

	// this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
	Unmapped18, Unmapped19, Unmapped20,
	Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
	Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
	Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
	Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
	Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,

}

/// The order validity period can be set in the type_time field of the special structure MqlTradeRequest when sending a trade request using the OrderSend() function. Values of the ENUM_ORDER_TYPE_TIME enumeration are allowed. To obtain the value of this property use the function OrderGetInteger() or HistoryOrderGetInteger() with the ORDER_TYPE_TIME modifier./
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/orderproperties
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
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

