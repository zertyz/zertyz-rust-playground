//! Mapping of https://www.mql5.com/en/docs/constants/environment_state/accountinformation to Rust


use super::{
	types::*,
	super::mql_rust_enum::{MqlRustEnumDescriptor},
};
use std::str::FromStr;
use once_cell::sync::Lazy;
use strum::{EnumString,FromRepr};


/// Holds all symbol information -- struct crafted from MT5's `AccountInfoInteger()`, `AccountInfoDouble()` and `AccountInfoString()`
#[repr(C)]
#[derive(Debug)]
pub struct AccountInfoBridge {
	/// Account balance in the deposit currency
	pub account_balance: f64,
	/// Account credit in the deposit currency
	pub account_credit: f64,
	/// Current profit of an account in the deposit currency
	pub account_profit: f64,
	/// Account equity in the deposit currency
	pub account_equity: f64,
	/// Account margin used in the deposit currency
	pub account_margin: f64,
	/// Free margin of an account in the deposit currency
	pub account_margin_free: f64,
	/// Account margin level in percents
	pub account_margin_level: f64,
	/// Margin call level. Depending on the set ACCOUNT_MARGIN_SO_MODE is expressed in percents or in the deposit currency
	pub account_margin_so_call: f64,
	/// Margin stop out level. Depending on the set ACCOUNT_MARGIN_SO_MODE is expressed in percents or in the deposit currency
	pub account_margin_so_so: f64,
	/// Initial margin. The amount reserved on an account to cover the margin of all pending orders 
	pub account_margin_initial: f64,
	/// Maintenance margin. The minimum equity reserved on an account to cover the minimum amount of all open positions
	pub account_margin_maintenance: f64,
	/// The current assets of an account
	pub account_assets: f64,
	/// The current liabilities on an account
	pub account_liabilities: f64,
	/// The current blocked commission amount on an account
	pub account_commission_blocked: f64,
	/// Account number
	pub account_login: i64,
	/// Account leverage
	pub account_leverage: i64,
	/// Client name
	pub account_name: MQ5String,
	/// Trade server name
	pub account_server: MQ5String,
	/// Account currency
	pub account_currency: MQ5String,
	/// Name of a company that serves the account
	pub account_company: MQ5String,
	/// Account trade mode
	pub account_trade_mode: i32, //EnumAccountTradeMode,
	/// Maximum allowed number of active pending orders
	pub account_limit_orders: i32,
	/// Mode for setting the minimal allowed margin
	pub account_margin_so_mode: i32, //EnumAccountStopoutMode,
	/// Margin calculation mode
	pub account_margin_mode: i32, //EnumAccountMarginMode,
	/// The number of decimal places in the account currency, which are required for an accurate display of trading results
	pub account_currency_digits: i32,
	/// Allowed trade for the current account
	pub account_trade_allowed: bool,
	/// Allowed trade for an Expert Advisor
	pub account_trade_expert: bool,
	/// An indication showing that positions can only be closed by FIFO rule. If the property value is set to
	pub account_fifo_close: bool,
	/// Allowed opposite positions on a single symbol
	pub account_hedge_allowed: bool,
}
impl AccountInfoBridge {

	pub fn from_ptr_to_internal(account_info_bridge: *const AccountInfoBridge) -> AccountInfoRust {

		let account_info_bridge = unsafe { &*account_info_bridge };

		log::debug!("report_account_info(xx): _____: {:#?}", account_info_bridge);

		AccountInfoRust {
			account_balance: account_info_bridge.account_balance,
			account_credit: account_info_bridge.account_credit,
			account_profit: account_info_bridge.account_profit,
			account_equity: account_info_bridge.account_equity,
			account_margin: account_info_bridge.account_margin,
			account_margin_free: account_info_bridge.account_margin_free,
			account_margin_level: account_info_bridge.account_margin_level,
			account_margin_so_call: account_info_bridge.account_margin_so_call,
			account_margin_so_so: account_info_bridge.account_margin_so_so,
			account_margin_initial: account_info_bridge.account_margin_initial,
			account_margin_maintenance: account_info_bridge.account_margin_maintenance,
			account_assets: account_info_bridge.account_assets,
			account_liabilities: account_info_bridge.account_liabilities,
			account_commission_blocked: account_info_bridge.account_commission_blocked,
			account_login: account_info_bridge.account_login,
			account_leverage: account_info_bridge.account_leverage,
			account_name: string_from_mql_string(&account_info_bridge.account_name),
			account_server: string_from_mql_string(&account_info_bridge.account_server),
			account_currency: string_from_mql_string(&account_info_bridge.account_currency),
			account_company: string_from_mql_string(&account_info_bridge.account_company),
			account_trade_mode: ENUM_ACCOUNT_TRADE_MODE.resolve_rust_variant(account_info_bridge.account_trade_mode),
			account_limit_orders: account_info_bridge.account_limit_orders,
			account_margin_so_mode: ENUM_ACCOUNT_STOPOUT_MODE.resolve_rust_variant(account_info_bridge.account_margin_so_mode),
			account_margin_mode: ENUM_ACCOUNT_MARGIN_MODE.resolve_rust_variant(account_info_bridge.account_margin_mode),
			account_currency_digits: account_info_bridge.account_currency_digits,
			account_trade_allowed: account_info_bridge.account_trade_allowed,
			account_trade_expert: account_info_bridge.account_trade_expert,
			account_fifo_close: account_info_bridge.account_fifo_close,
			account_hedge_allowed: account_info_bridge.account_hedge_allowed,
		}
	}
}

/// Rust version of the Metatrader 5 [AccountInfoBridge], with correct alignment, redundant fields removed, strings and enums resolved and copied to Rust -- so the MQL reference may be freed as soon as possible in MT5
#[derive(Debug)]
pub struct AccountInfoRust {
	/// Account balance in the deposit currency
	pub account_balance: f64,
	/// Account credit in the deposit currency
	pub account_credit: f64,
	/// Current profit of an account in the deposit currency
	pub account_profit: f64,
	/// Account equity in the deposit currency
	pub account_equity: f64,
	/// Account margin used in the deposit currency
	pub account_margin: f64,
	/// Free margin of an account in the deposit currency
	pub account_margin_free: f64,
	/// Account margin level in percents
	pub account_margin_level: f64,
	/// Margin call level. Depending on the set ACCOUNT_MARGIN_SO_MODE is expressed in percents or in the deposit currency
	pub account_margin_so_call: f64,
	/// Margin stop out level. Depending on the set ACCOUNT_MARGIN_SO_MODE is expressed in percents or in the deposit currency
	pub account_margin_so_so: f64,
	/// Initial margin. The amount reserved on an account to cover the margin of all pending orders
	pub account_margin_initial: f64,
	/// Maintenance margin. The minimum equity reserved on an account to cover the minimum amount of all open positions
	pub account_margin_maintenance: f64,
	/// The current assets of an account
	pub account_assets: f64,
	/// The current liabilities on an account
	pub account_liabilities: f64,
	/// The current blocked commission amount on an account
	pub account_commission_blocked: f64,
	/// Account number
	pub account_login: i64,
	/// Account leverage
	pub account_leverage: i64,
	/// Client name
	pub account_name: String,
	/// Trade server name
	pub account_server: String,
	/// Account currency
	pub account_currency: String,
	/// Name of a company that serves the account
	pub account_company: String,
	/// Account trade mode
	pub account_trade_mode: EnumAccountTradeMode,
	/// Maximum allowed number of active pending orders
	pub account_limit_orders: i32,
	/// Mode for setting the minimal allowed margin
	pub account_margin_so_mode: EnumAccountStopoutMode,
	/// Margin calculation mode
	pub account_margin_mode: EnumAccountMarginMode,
	/// The number of decimal places in the account currency, which are required for an accurate display of trading results
	pub account_currency_digits: i32,
	/// Allowed trade for the current account
	pub account_trade_allowed: bool,
	/// Allowed trade for an Expert Advisor
	pub account_trade_expert: bool,
	/// An indication showing that positions can only be closed by FIFO rule. If the property value is set to
	pub account_fifo_close: bool,
	/// Allowed opposite positions on a single symbol
	pub account_hedge_allowed: bool,
}


/// Account stop out mode in money/
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/accountinformation
#[repr(i32)]
#[derive(Debug,PartialEq,EnumString,FromRepr,Clone,Copy)]
pub enum EnumAccountMarginMode {
	/// Used for the OTC markets to interpret positions in the &quot;netting&quot; mode (only one position can exist for one symbol). The margin is calculated based on the symbol type (SYMBOL_TRADE_CALC_MODE).
	AccountMarginModeRetailNetting,
	/// Used for the exchange markets. Margin is calculated based on the discounts specified in symbol settings. Discounts are set by the broker, but not less than the values set by the exchange.
	AccountMarginModeExchange,
	/// Used for the exchange markets where individual positions are possible (hedging, multiple positions can exist for one symbol). The margin is calculated based on the symbol type (SYMBOL_TRADE_CALC_MODE) taking into account the hedged margin (SYMBOL_MARGIN_HEDGED).
	AccountMarginModeRetailHedging,

	/// in case MQL Code is out of sync with the DLL version...
	UnknownMqlVariantValue = -1,
}
impl Into<i32> for EnumAccountMarginMode {
	fn into(self) -> i32 {
		self as i32
	}
}
impl From<i32> for EnumAccountMarginMode {
	fn from(variant_value: i32) -> Self {
		if let Some(variant) = Self::from_repr(variant_value) {
			return variant;
		}
		Self::UnknownMqlVariantValue
	}
}

/// In case equity is not enough for maintaining open positions, the Stop Out situation, i.e. forced closing occurs. The minimum margin level at which Stop Out occurs can be set in percentage or in monetary terms. To find out the mode set for the account use the ENUM_ACCOUNT_STOPOUT_MODE enumeration./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/accountinformation
#[repr(i32)]
#[derive(Debug,PartialEq,EnumString,FromRepr,Clone,Copy)]
pub enum EnumAccountStopoutMode {
	/// Account stop out mode in percents
	AccountStopoutModePercent,
	/// Account stop out mode in money
	AccountStopoutModeMoney,

	/// in case MQL Code is out of sync with the DLL version...
	UnknownMqlVariantValue = -1,
}
impl Into<i32> for EnumAccountStopoutMode {
	fn into(self) -> i32 {
		self as i32
	}
}
impl From<i32> for EnumAccountStopoutMode {
	fn from(variant_value: i32) -> Self {
		if let Some(variant) = Self::from_repr(variant_value) {
			return variant;
		}
		Self::UnknownMqlVariantValue
	}
}

/// There are several types of accounts that can be opened on a trade server. The type of account on which an MQL5 program is running can be found out using the ENUM_ACCOUNT_TRADE_MODE enumeration./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/accountinformation
#[repr(i32)]
#[derive(Debug,PartialEq,EnumString,FromRepr,Clone,Copy)]
pub enum EnumAccountTradeMode {
	/// Demo account
	AccountTradeModeDemo,
	/// Contest account
	AccountTradeModeContest,
	/// Real account
	AccountTradeModeReal,

	/// in case MQL Code is out of sync with the DLL version...
	UnknownMqlVariantValue = -1,
}
impl Into<i32> for EnumAccountTradeMode {
	fn into(self) -> i32 {
		self as i32
	}
}
impl From<i32> for EnumAccountTradeMode {
	fn from(variant_value: i32) -> Self {
		if let Some(variant) = Self::from_repr(variant_value) {
			return variant;
		}
		Self::UnknownMqlVariantValue
	}
}

static ENUM_ACCOUNT_MARGIN_MODE:  Lazy<&MqlRustEnumDescriptor> = Lazy::new(|| MqlRustEnumDescriptor::new("EnumAccountMarginMode", &EnumAccountMarginMode::from_str));
static ENUM_ACCOUNT_STOPOUT_MODE: Lazy<&MqlRustEnumDescriptor> = Lazy::new(|| MqlRustEnumDescriptor::new("EnumAccountStopoutMode", &EnumAccountStopoutMode::from_str));
static ENUM_ACCOUNT_TRADE_MODE:   Lazy<&MqlRustEnumDescriptor> = Lazy::new(|| MqlRustEnumDescriptor::new("EnumAccountTradeMode", &EnumAccountTradeMode::from_str));

/// called when the program starts -- to register the MQL<=>Rust Enums
pub fn init() {
	log::info!("Internally registering ENUM '{}'", ENUM_ACCOUNT_MARGIN_MODE.name());
	log::info!("Internally registering ENUM '{}'", ENUM_ACCOUNT_STOPOUT_MODE.name());
	log::info!("Internally registering ENUM '{}'", ENUM_ACCOUNT_TRADE_MODE.name());
}