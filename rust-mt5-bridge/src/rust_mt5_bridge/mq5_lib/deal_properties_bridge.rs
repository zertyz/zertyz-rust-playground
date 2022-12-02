//! Mapping of https://www.mql5.com/en/docs/constants/tradingconstants/dealproperties to Rust


use chrono::NaiveDateTime;
use super::types::*;


/// Holds all deals information -- struct crafted from MT5's `HistoryDealGetDouble()`, `HistoryDealGetString()` and `HistoryDealGetInteger()`
#[repr(C)]
#[derive(Debug)]
pub struct DealPropertiesBridge {
	/// Deal volume
	pub deal_volume: f64,
	/// Deal price
	pub deal_price: f64,
	/// Deal commission
	pub deal_commission: f64,
	/// Cumulative swap on close
	pub deal_swap: f64,
	/// Deal profit
	pub deal_profit: f64,
	/// Fee for making a deal charged immediately after performing a deal
	pub deal_fee: f64,
	/// Stop Loss level
	pub deal_sl: f64,
	/// Take Profit level
	pub deal_tp: f64,
	/// Deal symbol
	pub deal_symbol: MQ5String,
	/// Deal comment
	pub deal_comment: MQ5String,
	/// Deal identifier in an external trading system (on the Exchange) 
	pub deal_external_id: MQ5String,
	/// Deal ticket. Unique number assigned to each deal
	pub deal_ticket: i64,
	/// Deal order number
	pub deal_order: i64,
	/// The time of a deal execution in milliseconds since 01.01.1970
	pub deal_time_msc: i64,
	/// Deal magic number (see ORDER_MAGIC)
	pub deal_magic: i64,
	/// Identifier of a position, in the opening, modification or closing of which this deal took part. Each position has a unique identifier that is assigned to all deals executed for the symbol during the entire lifetime of the position.
	pub deal_position_id: i64,
	/// Deal time
	pub deal_time: MQ5DateTime,
	/// Deal type
	pub deal_type: EnumDealType,
	/// Deal entry - entry in, entry out, reverse
	pub deal_entry: EnumDealEntry,
	/// The reason or source for deal execution
	pub deal_reason: EnumDealReason,
}
impl DealPropertiesBridge {

	pub fn from_ptr_to_internal(deal_properties_bridge: *const DealPropertiesBridge) -> DealPropertiesRust {
		let deal_properties_bridge = unsafe { &*deal_properties_bridge };

		log::debug!("report_deal_properties(xx): _____: {:#?}", deal_properties_bridge);

		DealPropertiesRust {
			deal_volume: deal_properties_bridge.deal_volume,
			deal_price: deal_properties_bridge.deal_price,
			deal_commission: deal_properties_bridge.deal_commission,
			deal_swap: deal_properties_bridge.deal_swap,
			deal_profit: deal_properties_bridge.deal_profit,
			deal_fee: deal_properties_bridge.deal_fee,
			deal_sl: deal_properties_bridge.deal_sl,
			deal_tp: deal_properties_bridge.deal_tp,
			deal_symbol: string_from_mql_string(&deal_properties_bridge.deal_symbol),
			deal_comment: string_from_mql_string(&deal_properties_bridge.deal_comment),
			deal_external_id: string_from_mql_string(&deal_properties_bridge.deal_external_id),
			deal_ticket: deal_properties_bridge.deal_ticket,
			deal_order: deal_properties_bridge.deal_order,
			deal_magic: deal_properties_bridge.deal_magic,
			deal_position_id: deal_properties_bridge.deal_position_id,
			deal_time: NaiveDateTime::from_timestamp(deal_properties_bridge.deal_time as i64, 1000_000 * (deal_properties_bridge.deal_time_msc % 1000) as u32),
			deal_type: deal_properties_bridge.deal_type,
			deal_entry: deal_properties_bridge.deal_entry,
			deal_reason: deal_properties_bridge.deal_reason,
		}
	}
}

/// Rust version of the Metatrader 5 [DealPropertiesBridge], with dates & strings resolved and copied to Rust, so we may set them free for being freed in MT5
#[derive(Debug)]
pub struct DealPropertiesRust {
	/// Deal volume
	pub deal_volume: f64,
	/// Deal price
	pub deal_price: f64,
	/// Deal commission
	pub deal_commission: f64,
	/// Cumulative swap on close
	pub deal_swap: f64,
	/// Deal profit
	pub deal_profit: f64,
	/// Fee for making a deal charged immediately after performing a deal
	pub deal_fee: f64,
	/// Stop Loss level
	pub deal_sl: f64,
	/// Take Profit level
	pub deal_tp: f64,
	/// Deal symbol
	pub deal_symbol: String,
	/// Deal comment
	pub deal_comment: String,
	/// Deal identifier in an external trading system (on the Exchange)
	pub deal_external_id: String,
	/// Deal ticket. Unique number assigned to each deal
	pub deal_ticket: i64,
	/// Deal order number
	pub deal_order: i64,
	/// Deal magic number (see ORDER_MAGIC)
	pub deal_magic: i64,
	/// Identifier of a position, in the opening, modification or closing of which this deal took part. Each position has a unique identifier that is assigned to all deals executed for the symbol during the entire lifetime of the position.
	pub deal_position_id: i64,
	/// Deal time
	pub deal_time: NaiveDateTime,
	/// Deal type
	pub deal_type: EnumDealType,
	/// Deal entry - entry in, entry out, reverse
	pub deal_entry: EnumDealEntry,
	/// The reason or source for deal execution
	pub deal_reason: EnumDealReason,
}

/// All these situations are described by values from the ENUM_DEAL_ENTRY enumeration. In order to receive this information about a deal, use the HistoryDealGetInteger() function with the DEAL_ENTRY modifier./
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/dealproperties
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumDealEntry {
	/// Entry in
	DealEntryIn,
	/// Entry out
	DealEntryOut,
	///  Reverse
	DealEntryInout,
	/// Close a position by an opposite one
	DealEntryOutBy,

	// this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
	Unmapped4,  Unmapped5,  Unmapped6,  Unmapped7,
	Unmapped8,  Unmapped9,  Unmapped10, Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
	Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
	Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
	Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
	Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
	Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,
}

/// The reason for deal execution is contained in the DEAL_REASON property. A deal can be executed as a result of triggering of an order placed from a mobile application or an MQL5 program, as well as as a result of the StopOut event, variation margin calculation, etc. Possible values of DEAL_REASON are described in the ENUM_DEAL_REASON enumeration. For non-trading deals resulting from balance, credit, commission and other operations, DEAL_REASON_CLIENT is indicated as the reason./
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/dealproperties
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumDealReason {
	/// The deal was executed as a result of activation of an order placed from a desktop terminal
	DealReasonClient,
	/// The deal was executed as a result of activation of an order placed from a mobile application
	DealReasonMobile,
	/// The deal was executed as a result of activation of an order placed from the web platform
	DealReasonWeb,
	/// The deal was executed as a result of activation of an order placed from an MQL5 program, i.e. an Expert Advisor or a script
	DealReasonExpert,
	/// The deal was executed as a result of Stop Loss activation
	DealReasonSl,
	/// The deal was executed as a result of Take Profit activation
	DealReasonTp,
	/// The deal was executed as a result of the Stop Out event
	DealReasonSo,
	/// The deal was executed due to a rollover
	DealReasonRollover,
	/// The deal was executed after charging the variation margin
	DealReasonVmargin,
	/// The deal was executed after the split (price reduction) of an instrument, which had an open position during split announcement
	DealReasonSplit,

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

