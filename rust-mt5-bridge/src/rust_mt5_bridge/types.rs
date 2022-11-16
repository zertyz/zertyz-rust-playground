pub use super::mq5_std_lib::*;

use std::fmt::{Debug, Formatter};
use std::num::NonZeroU64;
use chrono::NaiveDateTime;
use widestring::U16CString;


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
    Ambiguous   {bid: f64, ask: f64},
    Buyer       {bid: f64, ask: f64},
    Seller      {bid: f64, ask: f64},
    Unspecified {bid: f64, ask: f64},
}

#[derive(Debug)]
pub struct Trade {
    pub time:      NaiveDateTime,
    pub aggressor: Party,
    pub quantity:  u32,
    pub price:     f64
}

#[derive(Debug)]
pub struct Spread {
    pub time:             NaiveDateTime,
    pub best_bid: f64,
    pub best_ask: f64,
}

pub enum TickEvent {
    Trade(Trade),
    Spread(Spread),
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


impl MqlTick {

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



/// Rust version of the Metatrader 5 [SymbolInfoBridge], with dates, colors and strings resolved and copied to Rust, so we may set them free for being freed in MT5
#[derive(Debug)]
pub struct SymbolInfoRust {
    /// The sector of the economy to which the asset belongs
    pub symbol_sector: EnumSymbolSector,
    /// The industry or the economy branch to which the symbol belongs
    pub symbol_industry: EnumSymbolIndustry,
    /// The color of the background used for the symbol in Market Watch (R, G, B)
    pub symbol_background_color: (u8, u8, u8),
    /// The price type used for generating symbols bars, i.e. Bid or Last
    pub symbol_chart_mode: EnumSymbolChartMode,
    /// Number of deals in the current session
    pub symbol_session_deals: i64,
    /// Number of Buy orders at the moment
    pub symbol_session_buy_orders: i64,
    /// Number of Sell orders at the moment
    pub symbol_session_sell_orders: i64,
    /// Volume of the last deal
    pub symbol_volume: i64,
    /// Maximal day volume
    pub symbol_volumehigh: i64,
    /// Minimal day volume
    pub symbol_volumelow: i64,
    /// Time of the last quote
    pub symbol_time: NaiveDateTime,
    /// Digits after a decimal point
    pub symbol_digits: i32,
    /// Spread value in points
    pub symbol_spread: i32,
    /// Maximal number of requests shown in Depth of Market. For symbols that have no queue of requests, the value is equal to zero.
    pub symbol_ticks_bookdepth: i32,
    /// Contract price calculation mode
    pub symbol_trade_calc_mode: EnumSymbolCalcMode,
    /// Order execution type
    pub symbol_trade_mode: EnumSymbolTradeMode,
    /// Date of the symbol trade beginning (usually used for futures)
    pub symbol_start_time: NaiveDateTime,
    /// Date of the symbol trade end (usually used for futures)
    pub symbol_expiration_time: NaiveDateTime,
    /// Minimal indention in points from the current close price to place Stop orders
    pub symbol_trade_stops_level: i32,
    /// Distance to freeze trade operations in points
    pub symbol_trade_freeze_level: i32,
    /// Deal execution mode
    pub symbol_trade_exemode: EnumSymbolTradeExecution,
    /// Swap calculation model
    pub symbol_swap_mode: EnumSymbolSwapMode,
    /// The day of week to charge 3-day swap rollover
    pub symbol_swap_rollover3days: EnumDayOfWeek,
    /// Flags of allowed order expiration modes
    pub symbol_expiration_mode : i32,
    /// Flags of allowed order filling modes
    pub symbol_filling_mode: i32,
    /// Flags of allowed order types
    pub symbol_order_mode: i32,
    /// Expiration of Stop Loss and Take Profit orders, if SYMBOL_EXPIRATION_MODE=SYMBOL_EXPIRATION_GTC (Good till canceled)
    pub symbol_order_gtc_mode: EnumSymbolOrderGtcMode,
    /// Option type
    pub symbol_option_mode: EnumSymbolOptionMode,
    /// Option right (Call/Put)
    pub symbol_option_right: EnumSymbolOptionRight,
    /// Bid - best sell offer
    pub symbol_bid: f64,
    /// Maximal Bid of the day
    pub symbol_bidhigh: f64,
    /// Minimal Bid of the day
    pub symbol_bidlow: f64,
    /// Ask - best buy offer
    pub symbol_ask: f64,
    /// Maximal Ask of the day
    pub symbol_askhigh: f64,
    /// Minimal Ask of the day
    pub symbol_asklow: f64,
    /// Price of the last deal
    pub symbol_last: f64,
    /// Maximal Last of the day
    pub symbol_lasthigh: f64,
    /// Minimal Last of the day
    pub symbol_lastlow: f64,
    /// Volume of the last deal
    pub symbol_volume_real: f64,
    /// Maximum Volume of the day
    pub symbol_volumehigh_real: f64,
    /// Minimum Volume of the day
    pub symbol_volumelow_real: f64,
    /// The strike price of an option. The price at which an option buyer can buy (in a Call option) or sell (in a Put option) the underlying asset, and the option seller is obliged to sell or buy the appropriate amount of the underlying asset.
    pub symbol_option_strike: f64,
    /// Symbol point value
    pub symbol_point: f64,
    /// Value of SYMBOL_TRADE_TICK_VALUE_PROFIT
    pub symbol_trade_tick_value: f64,
    /// Calculated tick price for a profitable position
    pub symbol_trade_tick_value_profit: f64,
    /// Calculated tick price for a losing position
    pub symbol_trade_tick_value_loss: f64,
    /// Minimal price change
    pub symbol_trade_tick_size: f64,
    /// Trade contract size
    pub symbol_trade_contract_size: f64,
    /// Accrued interest – accumulated coupon interest, i.e. part of the coupon interest calculated in proportion to the number of days since the coupon bond issuance or the last coupon interest payment
    pub symbol_trade_accrued_interest: f64,
    /// Face value – initial bond value set by the issuer
    pub symbol_trade_face_value: f64,
    /// Liquidity Rate is the share of the asset that can be used for the margin.
    pub symbol_trade_liquidity_rate: f64,
    /// Minimal volume for a deal
    pub symbol_volume_min: f64,
    /// Maximal volume for a deal
    pub symbol_volume_max: f64,
    /// Minimal volume change step for deal execution
    pub symbol_volume_step: f64,
    /// Maximum allowed aggregate volume of an open position and pending orders in one direction (buy or sell) for the symbol. For example, with the limitation of 5 lots, you can have an open buy position with the volume of 5 lots and place a pending order Sell Limit with the volume of 5 lots. But in this case you cannot place a Buy Limit pending order (since the total volume in one direction will exceed the limitation) or place Sell Limit with the volume more than 5 lots.
    pub symbol_volume_limit: f64,
    /// Long swap value
    pub symbol_swap_long: f64,
    /// Short swap value
    pub symbol_swap_short: f64,
    /// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from SUNDAY to the next day. There following values are supported:
    pub symbol_swap_sunday: f64,
    /// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Monday to Tuesday
    pub symbol_swap_monday: f64,
    /// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Tuesday to Wednesday
    pub symbol_swap_tuesday: f64,
    /// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Wednesday to Thursday
    pub symbol_swap_wednesday: f64,
    /// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Thursday to Friday
    pub symbol_swap_thursday: f64,
    /// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Friday to Saturday
    pub symbol_swap_friday: f64,
    /// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Saturday to Sunday
    pub symbol_swap_saturday: f64,
    /// Initial margin means the amount in the margin currency required for opening a position with the volume of one lot. It is used for checking a client's assets when he or she enters the market.
    pub symbol_margin_initial: f64,
    /// The maintenance margin. If it is set, it sets the margin amount in the margin currency of the symbol, charged from one lot. It is used for checking a client's assets when his/her account state changes. If the maintenance margin is equal to 0, the initial margin is used.
    pub symbol_margin_maintenance: f64,
    /// Summary volume of current session deals
    pub symbol_session_volume: f64,
    /// Summary turnover of the current session
    pub symbol_session_turnover: f64,
    /// Summary open interest
    pub symbol_session_interest: f64,
    /// Current volume of Buy orders
    pub symbol_session_buy_orders_volume: f64,
    /// Current volume of Sell orders
    pub symbol_session_sell_orders_volume: f64,
    /// Open price of the current session
    pub symbol_session_open: f64,
    /// Close price of the current session
    pub symbol_session_close: f64,
    /// Average weighted price of the current session
    pub symbol_session_aw: f64,
    /// Settlement price of the current session
    pub symbol_session_price_settlement: f64,
    /// Minimal price of the current session
    pub symbol_session_price_limit_min: f64,
    /// Maximal price of the current session
    pub symbol_session_price_limit_max: f64,
    /// Contract size or margin value per one lot of hedged positions (oppositely directed positions of one symbol). Two margin calculation methods are possible for hedged positions. The calculation method is defined by the broker.
    pub symbol_margin_hedged: f64,
    /// Change of the current price relative to the end of the previous trading day in %
    pub symbol_price_change: f64,
    /// Price volatility in %
    pub symbol_price_volatility: f64,
    /// Theoretical option price
    pub symbol_price_theoretical: f64,
    /// Option/warrant delta shows the value the option price changes by, when the underlying asset price changes by 1
    pub symbol_price_delta: f64,
    /// Option/warrant theta shows the number of points the option price is to lose every day due to a temporary breakup, i.e. when the expiration date approaches
    pub symbol_price_theta: f64,
    /// Option/warrant gamma shows the change rate of delta – how quickly or slowly the option premium changes
    pub symbol_price_gamma: f64,
    /// Option/warrant vega shows the number of points the option price changes by when the volatility changes by 1%
    pub symbol_price_vega: f64,
    /// Option/warrant rho reflects the sensitivity of the theoretical option price to the interest rate changing by 1%
    pub symbol_price_rho: f64,
    /// Option/warrant omega. Option elasticity shows a relative percentage change of the option price by the percentage change of the underlying asset price
    pub symbol_price_omega: f64,
    /// Option/warrant sensitivity shows by how many points the price of the option's underlying asset should change so that the price of the option changes by one point
    pub symbol_price_sensitivity: f64,
    /// The underlying asset of a derivative
    pub symbol_basis: String,
    /// The name of the sector or category to which the financial symbol belongs
    pub symbol_category: String,
    /// The country to which the financial symbol belongs
    pub symbol_country: String,
    /// The sector of the economy to which the financial symbol belongs
    pub symbol_sector_name: String,
    /// The industry branch or the industry to which the financial symbol belongs
    pub symbol_industry_name: String,
    /// Basic currency of a symbol
    pub symbol_currency_base: String,
    /// Profit currency
    pub symbol_currency_profit: String,
    /// Margin currency
    pub symbol_currency_margin: String,
    /// Feeder of the current quote
    pub symbol_bank: String,
    /// Symbol description
    pub symbol_description: String,
    /// The name of the exchange in which the financial symbol is traded
    pub symbol_exchange: String,
    /// The formula used for the custom symbol pricing. If the name of a financial symbol used in the formula starts with a digit or contains a special character (&quot;&gt;&quot; &quot;, &quot;.&quot;, &quot;-&quot;, &quot;&amp;&quot;, &quot;#&quot; and so on) quotation marks should be used around this symbol name.
    pub symbol_formula: String,
    /// The name of a symbol in the ISIN system (International Securities Identification Number). The International Securities Identification Number is a 12-digit alphanumeric code that uniquely identifies a security. The presence of this symbol property is determined on the side of a trade server.
    pub symbol_isin: String,
    /// The address of the web page containing symbol information. This address will be displayed as a link when viewing symbol properties in the terminal
    pub symbol_page: String,
    /// Path in the symbol tree
    pub symbol_path: String,
    /// Symbol data arrives with a delay. The property can be requested only for symbols selected in MarketWatch (SYMBOL_SELECT = true). The ERR_MARKET_NOT_SELECTED (4302) error is generated for other symbols
    pub symbol_subscription_delay: bool,
    /// It is a custom symbol – the symbol has been created synthetically based on other symbols from the Market Watch and/or external data sources
    pub symbol_custom: bool,
    /// Symbol with this name exists
    pub symbol_exist: bool,
    /// Symbol is selected in Market Watch
    pub symbol_select: bool,
    /// Symbol is visible in Market Watch.
    pub symbol_visible: bool,
    /// Indication of a floating spread
    pub symbol_spread_float: bool,
    /// Calculating hedging margin using the larger leg (Buy or Sell)
    pub symbol_margin_hedged_use_leg: bool,
}
impl SymbolInfoRust {
    pub fn from(symbol_info_bridge: &SymbolInfoBridge) -> Self {
        Self {
            symbol_sector: symbol_info_bridge.symbol_sector,
            symbol_industry: symbol_info_bridge.symbol_industry,
            symbol_background_color: ((symbol_info_bridge.symbol_background_color & 0x00FF0000) as u8, (symbol_info_bridge.symbol_background_color & 0x0000FF00) as u8, (symbol_info_bridge.symbol_background_color & 0x000000FF) as u8),
            symbol_chart_mode: symbol_info_bridge.symbol_chart_mode,
            symbol_session_deals: symbol_info_bridge.symbol_session_deals,
            symbol_session_buy_orders: symbol_info_bridge.symbol_session_buy_orders,
            symbol_session_sell_orders: symbol_info_bridge.symbol_session_sell_orders,
            symbol_volume: symbol_info_bridge.symbol_volume,
            symbol_volumehigh: symbol_info_bridge.symbol_volumehigh,
            symbol_volumelow: symbol_info_bridge.symbol_volumelow,
            symbol_time: NaiveDateTime::from_timestamp(symbol_info_bridge.symbol_time as i64, 1000_000 * (symbol_info_bridge.symbol_time_msc % 1000) as u32),
            symbol_digits: symbol_info_bridge.symbol_digits,
            symbol_spread: symbol_info_bridge.symbol_spread,
            symbol_ticks_bookdepth: symbol_info_bridge.symbol_ticks_bookdepth,
            symbol_trade_calc_mode: symbol_info_bridge.symbol_trade_calc_mode,
            symbol_trade_mode: symbol_info_bridge.symbol_trade_mode,
            symbol_start_time: NaiveDateTime::from_timestamp(symbol_info_bridge.symbol_start_time as i64, 0),
            symbol_expiration_time: NaiveDateTime::from_timestamp(symbol_info_bridge.symbol_expiration_time as i64, 0),
            symbol_trade_stops_level: symbol_info_bridge.symbol_trade_stops_level,
            symbol_trade_freeze_level: symbol_info_bridge.symbol_trade_freeze_level,
            symbol_trade_exemode: symbol_info_bridge.symbol_trade_exemode,
            symbol_swap_mode: symbol_info_bridge.symbol_swap_mode,
            symbol_swap_rollover3days: symbol_info_bridge.symbol_swap_rollover3days,
            symbol_expiration_mode: symbol_info_bridge.symbol_expiration_mode,
            symbol_filling_mode: symbol_info_bridge.symbol_filling_mode,
            symbol_order_mode: symbol_info_bridge.symbol_order_mode,
            symbol_order_gtc_mode: symbol_info_bridge.symbol_order_gtc_mode,
            symbol_option_mode: symbol_info_bridge.symbol_option_mode,
            symbol_option_right: symbol_info_bridge.symbol_option_right,
            symbol_bid: symbol_info_bridge.symbol_bid,
            symbol_bidhigh: symbol_info_bridge.symbol_bidhigh,
            symbol_bidlow: symbol_info_bridge.symbol_bidlow,
            symbol_ask: symbol_info_bridge.symbol_ask,
            symbol_askhigh: symbol_info_bridge.symbol_askhigh,
            symbol_asklow: symbol_info_bridge.symbol_asklow,
            symbol_last: symbol_info_bridge.symbol_last,
            symbol_lasthigh: symbol_info_bridge.symbol_lasthigh,
            symbol_lastlow: symbol_info_bridge.symbol_lastlow,
            symbol_volume_real: symbol_info_bridge.symbol_volume_real,
            symbol_volumehigh_real: symbol_info_bridge.symbol_volumehigh_real,
            symbol_volumelow_real: symbol_info_bridge.symbol_volumelow_real,
            symbol_option_strike: symbol_info_bridge.symbol_option_strike,
            symbol_point: symbol_info_bridge.symbol_point,
            symbol_trade_tick_value: symbol_info_bridge.symbol_trade_tick_value,
            symbol_trade_tick_value_profit: symbol_info_bridge.symbol_trade_tick_value_profit,
            symbol_trade_tick_value_loss: symbol_info_bridge.symbol_trade_tick_value_loss,
            symbol_trade_tick_size: symbol_info_bridge.symbol_trade_tick_size,
            symbol_trade_contract_size: symbol_info_bridge.symbol_trade_contract_size,
            symbol_trade_accrued_interest: symbol_info_bridge.symbol_trade_accrued_interest,
            symbol_trade_face_value: symbol_info_bridge.symbol_trade_face_value,
            symbol_trade_liquidity_rate: symbol_info_bridge.symbol_trade_liquidity_rate,
            symbol_volume_min: symbol_info_bridge.symbol_volume_min,
            symbol_volume_max: symbol_info_bridge.symbol_volume_max,
            symbol_volume_step: symbol_info_bridge.symbol_volume_step,
            symbol_volume_limit: symbol_info_bridge.symbol_volume_limit,
            symbol_swap_long: symbol_info_bridge.symbol_swap_long,
            symbol_swap_short: symbol_info_bridge.symbol_swap_short,
            symbol_swap_sunday: symbol_info_bridge.symbol_swap_sunday,
            symbol_swap_monday: symbol_info_bridge.symbol_swap_monday,
            symbol_swap_tuesday: symbol_info_bridge.symbol_swap_tuesday,
            symbol_swap_wednesday: symbol_info_bridge.symbol_swap_wednesday,
            symbol_swap_thursday: symbol_info_bridge.symbol_swap_thursday,
            symbol_swap_friday: symbol_info_bridge.symbol_swap_friday,
            symbol_swap_saturday: symbol_info_bridge.symbol_swap_saturday,
            symbol_margin_initial: symbol_info_bridge.symbol_margin_initial,
            symbol_margin_maintenance: symbol_info_bridge.symbol_margin_maintenance,
            symbol_session_volume: symbol_info_bridge.symbol_session_volume,
            symbol_session_turnover: symbol_info_bridge.symbol_session_turnover,
            symbol_session_interest: symbol_info_bridge.symbol_session_interest,
            symbol_session_buy_orders_volume: symbol_info_bridge.symbol_session_buy_orders_volume,
            symbol_session_sell_orders_volume: symbol_info_bridge.symbol_session_sell_orders_volume,
            symbol_session_open: symbol_info_bridge.symbol_session_open,
            symbol_session_close: symbol_info_bridge.symbol_session_close,
            symbol_session_aw: symbol_info_bridge.symbol_session_aw,
            symbol_session_price_settlement: symbol_info_bridge.symbol_session_price_settlement,
            symbol_session_price_limit_min: symbol_info_bridge.symbol_session_price_limit_min,
            symbol_session_price_limit_max: symbol_info_bridge.symbol_session_price_limit_max,
            symbol_margin_hedged: symbol_info_bridge.symbol_margin_hedged,
            symbol_price_change: symbol_info_bridge.symbol_price_change,
            symbol_price_volatility: symbol_info_bridge.symbol_price_volatility,
            symbol_price_theoretical: symbol_info_bridge.symbol_price_theoretical,
            symbol_price_delta: symbol_info_bridge.symbol_price_delta,
            symbol_price_theta: symbol_info_bridge.symbol_price_theta,
            symbol_price_gamma: symbol_info_bridge.symbol_price_gamma,
            symbol_price_vega: symbol_info_bridge.symbol_price_vega,
            symbol_price_rho: symbol_info_bridge.symbol_price_rho,
            symbol_price_omega: symbol_info_bridge.symbol_price_omega,
            symbol_price_sensitivity: symbol_info_bridge.symbol_price_sensitivity,
            // symbol_basis: unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(symbol_info_bridge.symbol_basis) as *const u16) }.to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for 'symbol_basis'»»")),
            // symbol_category: unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(symbol_info_bridge.symbol_category) as *const u16) }.to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for '????'»»")),
            // symbol_country: unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(symbol_info_bridge.symbol_country) as *const u16) }.to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for '????'»»")),
            // symbol_sector_name: unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(symbol_info_bridge.symbol_sector_name) as *const u16) }.to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for '????'»»")),
            // symbol_industry_name: unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(symbol_info_bridge.symbol_industry_name) as *const u16) }.to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for '????'»»")),
            // symbol_currency_base: unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(symbol_info_bridge.symbol_currency_base) as *const u16) }.to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for '????'»»")),
            // symbol_currency_profit: unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(symbol_info_bridge.symbol_currency_profit) as *const u16) }.to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for '????'»»")),
            // symbol_currency_margin: unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(symbol_info_bridge.symbol_currency_margin) as *const u16) }.to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for '????'»»")),
            // symbol_bank: unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(symbol_info_bridge.symbol_bank) as *const u16) }.to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for '????'»»")),
            // symbol_description: unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(symbol_info_bridge.symbol_description) as *const u16) }.to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for '????'»»")),
            // symbol_exchange: unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(symbol_info_bridge.symbol_exchange) as *const u16) }.to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for '????'»»")),
            // symbol_formula: unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(symbol_info_bridge.symbol_formula) as *const u16) }.to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for '????'»»")),
            // symbol_isin: unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(symbol_info_bridge.symbol_isin) as *const u16) }.to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for '????'»»")),
            // symbol_page: unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(symbol_info_bridge.symbol_page) as *const u16) }.to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for '????'»»")),
            // symbol_path: unsafe { U16CString::from_ptr_str(std::ptr::addr_of!(symbol_info_bridge.symbol_path) as *const u16) }.to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for '????'»»")),
            symbol_basis: String::from("fix MT5 strings...."),
            symbol_category: String::from("fix MT5 strings...."),
            symbol_country: String::from("fix MT5 strings...."),
            symbol_sector_name: String::from("fix MT5 strings...."),
            symbol_industry_name: String::from("fix MT5 strings...."),
            symbol_currency_base: String::from("fix MT5 strings...."),
            symbol_currency_profit: String::from("fix MT5 strings...."),
            symbol_currency_margin: String::from("fix MT5 strings...."),
            symbol_bank: String::from("fix MT5 strings...."),
            symbol_description: String::from("fix MT5 strings...."),
            symbol_exchange: String::from("fix MT5 strings...."),
            symbol_formula: String::from("fix MT5 strings...."),
            symbol_isin: String::from("fix MT5 strings...."),
            symbol_page: String::from("fix MT5 strings...."),
            symbol_path: String::from("fix MT5 strings...."),
            symbol_subscription_delay: symbol_info_bridge.symbol_subscription_delay,
            symbol_custom: symbol_info_bridge.symbol_custom,
            symbol_exist: symbol_info_bridge.symbol_exist,
            symbol_select: symbol_info_bridge.symbol_select,
            symbol_visible: symbol_info_bridge.symbol_visible,
            symbol_spread_float: symbol_info_bridge.symbol_spread_float,
            symbol_margin_hedged_use_leg: symbol_info_bridge.symbol_margin_hedged_use_leg,
        }
    }
}
