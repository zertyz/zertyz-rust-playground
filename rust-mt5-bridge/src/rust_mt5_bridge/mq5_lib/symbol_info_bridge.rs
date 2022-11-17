//! Mapping of https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants to Rust


use chrono::NaiveDateTime;
use widestring::U16CString;
use super::types::*;


/// Holds all symbol information -- struct crafted from MT5's `SymbolInfoInteger()`, `SymbolInfoDouble()` and `SymbolInfoString()`
#[repr(C)]
#[derive(Debug)]
pub struct SymbolInfoBridge {
    /// The sector of the economy to which the asset belongs
    pub symbol_sector: EnumSymbolSector,
    /// The industry or the economy branch to which the symbol belongs
    pub symbol_industry: EnumSymbolIndustry,
    /// The color of the background used for the symbol in Market Watch
    pub symbol_background_color: MQ5Color,
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
    pub symbol_time: MQ5DateTime,
    /// Time of the last quote in milliseconds since 1970.01.01
    pub symbol_time_msc: i64,
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
    pub symbol_start_time: MQ5DateTime,
    /// Date of the symbol trade end (usually used for futures)
    pub symbol_expiration_time: MQ5DateTime,
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
    pub symbol_basis: MQ5String,
    /// The name of the sector or category to which the financial symbol belongs
    pub symbol_category: MQ5String,
    /// The country to which the financial symbol belongs
    pub symbol_country: MQ5String,
    /// The sector of the economy to which the financial symbol belongs
    pub symbol_sector_name: MQ5String,
    /// The industry branch or the industry to which the financial symbol belongs
    pub symbol_industry_name: MQ5String,
    /// Basic currency of a symbol
    pub symbol_currency_base: MQ5String,
    /// Profit currency
    pub symbol_currency_profit: MQ5String,
    /// Margin currency
    pub symbol_currency_margin: MQ5String,
    /// Feeder of the current quote
    pub symbol_bank: MQ5String,
    /// Symbol description
    pub symbol_description: MQ5String,
    /// The name of the exchange in which the financial symbol is traded
    pub symbol_exchange: MQ5String,
    /// The formula used for the custom symbol pricing. If the name of a financial symbol used in the formula starts with a digit or contains a special character (&quot;&gt;&quot; &quot;, &quot;.&quot;, &quot;-&quot;, &quot;&amp;&quot;, &quot;#&quot; and so on) quotation marks should be used around this symbol name.
    pub symbol_formula: MQ5String,
    /// The name of a symbol in the ISIN system (International Securities Identification Number). The International Securities Identification Number is a 12-digit alphanumeric code that uniquely identifies a security. The presence of this symbol property is determined on the side of a trade server.
    pub symbol_isin: MQ5String,
    /// The address of the web page containing symbol information. This address will be displayed as a link when viewing symbol properties in the terminal
    pub symbol_page: MQ5String,
    /// Path in the symbol tree
    pub symbol_path: MQ5String,
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
impl SymbolInfoBridge {

    pub fn from_ptr_to_internal(symbol_info_bridge: *const SymbolInfoBridge) -> SymbolInfoRust {

        let symbol_info_bridge = unsafe { &*symbol_info_bridge };

        SymbolInfoRust {
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
            symbol_basis: string_from_mql_string(&symbol_info_bridge.symbol_basis),
            symbol_category: string_from_mql_string(&symbol_info_bridge.symbol_category),
            symbol_country: string_from_mql_string(&symbol_info_bridge.symbol_country),
            symbol_sector_name: string_from_mql_string(&symbol_info_bridge.symbol_sector_name),
            symbol_industry_name: string_from_mql_string(&symbol_info_bridge.symbol_industry_name),
            symbol_currency_base: string_from_mql_string(&symbol_info_bridge.symbol_currency_base),
            symbol_currency_profit: string_from_mql_string(&symbol_info_bridge.symbol_currency_profit),
            symbol_currency_margin: string_from_mql_string(&symbol_info_bridge.symbol_currency_margin),
            symbol_bank: string_from_mql_string(&symbol_info_bridge.symbol_bank),
            symbol_description: string_from_mql_string(&symbol_info_bridge.symbol_description),
            symbol_exchange: string_from_mql_string(&symbol_info_bridge.symbol_exchange),
            symbol_formula: string_from_mql_string(&symbol_info_bridge.symbol_formula),
            symbol_isin: string_from_mql_string(&symbol_info_bridge.symbol_isin),
            symbol_page: string_from_mql_string(&symbol_info_bridge.symbol_page),
            symbol_path: string_from_mql_string(&symbol_info_bridge.symbol_path),
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

/// Values of the ENUM_DAY_OF_WEEK enumeration are used for specifying days of week./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumDayOfWeek {
    /// Sunday
    Sunday,
    /// Monday
    Monday,
    /// Tuesday
    Tuesday,
    /// Wednesday
    Wednesday,
    /// Thursday
    Thursday,
    /// Friday
    Friday,
    /// Saturday
    Saturday,

    // this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
    Unmapped7,  Unmapped8,  Unmapped9,  Unmapped10, Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
    Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
    Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
    Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
    Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
    Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,
}

/// The ENUM_SYMBOL_CALC_MODE enumeration is used for obtaining information about how the margin requirements for a symbol are calculated./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumSymbolCalcMode {
    /// Forex mode - calculation of profit and margin for Forex
    SymbolCalcModeForex,
    /// Forex No Leverage mode – calculation of profit and margin for Forex symbols without taking into account the leverage
    SymbolCalcModeForexNoLeverage,
    /// Futures mode - calculation of margin and profit for futures
    SymbolCalcModeFutures,
    /// CFD mode - calculation of margin and profit for CFD
    SymbolCalcModeCfd,
    /// CFD index mode - calculation of margin and profit for CFD by indexes
    SymbolCalcModeCfdindex,
    /// CFD Leverage mode - calculation of margin and profit for CFD at leverage trading
    SymbolCalcModeCfdleverage,
    /// Exchange mode – calculation of margin and profit for trading securities on a stock exchange
    SymbolCalcModeExchStocks,
    /// Futures mode –  calculation of margin and profit for trading futures contracts on a stock exchange
    SymbolCalcModeExchFutures,
    /// FORTS Futures mode –  calculation of margin and profit for trading futures contracts on FORTS. The margin may be reduced by the amount of MarginDiscount deviation according to the following rules:/
    /// /
    /// 1. If the price of a long position (buy order) is less than the estimated price, MarginDiscount = Lots*((PriceSettle-PriceOrder)*TickPrice/TickSize)/
    /// /
    /// 2. If the price of a short position (sell order) exceeds the estimated price, MarginDiscount = Lots*((PriceOrder-PriceSettle)*TickPrice/TickSize)/
    /// /
    /// where:/
    /// /
    /// /
    /// PriceSettle – estimated (clearing) price of the previous session;/
    /// PriceOrder – average weighted position price or open price set in the order (request);/
    /// TickPrice – tick price (cost of the price change by one point)/
    /// TickSize – tick size (minimum price change step)/
    /// /
    /// /
    /// /
    /// /
    /// Margin: Lots * InitialMargin * Margin_Rate or Lots * MaintenanceMargin * Margin_Rate * Margin_Rate/
    /// /
    ///  /
    /// /
    /// Profit:  (close_price - open_price) * Lots * TickPrice / TickSize
    SymbolCalcModeExchFuturesForts,
    /// Exchange Bonds mode – calculation of margin and profit for trading bonds on a stock exchange
    SymbolCalcModeExchBonds,
    /// Exchange MOEX Stocks mode – calculation of margin and profit for trading securities on MOEX
    SymbolCalcModeExchStocksMoex,
    /// Exchange MOEX Bonds mode – calculation of margin and profit for trading bonds on MOEX
    SymbolCalcModeExchBondsMoex,
    /// Collateral mode - a symbol is used as a non-tradable asset on a trading account. The market value of an open position is calculated based on the volume, current market price, contract size and liquidity ratio. The value is included into Assets, which are added to Equity. Open positions of such symbols increase the Free Margin amount and are used as additional margin (collateral) for open positions of tradable instruments.
    SymbolCalcModeServCollateral,

    // this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
    Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
    Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
    Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
    Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
    Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
    Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,
}

/// A symbol price chart can be based on Bid or Last prices. The price selected for symbol charts also affects the generation and display of bars in the terminal. Possible values of the SYMBOL_CHART_MODE property are described in ENUM_SYMBOL_CHART_MODE/
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumSymbolChartMode {
    /// Bars are based on Bid prices
    SymbolChartModeBid,
    /// Bars are based on Last prices
    SymbolChartModeLast,

    // this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
    Unmapped2,  Unmapped3,   Unmapped4,  Unmapped5,  Unmapped6, Unmapped7,
    Unmapped8,  Unmapped9,  Unmapped10, Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
    Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
    Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
    Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
    Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
    Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,
}

/// Each financial instrument can be assigned to a specific type of industry or economy branch. An industry is a branch of an economy that produces a closely related set of raw materials, goods, or services. ENUM_SYMBOL_INDUSTRY lists industries which a trading instrument can belong to./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumSymbolIndustry {
    /// Undefined
    IndustryUndefined,
    /// Agricultural inputs
    IndustryAgriculturalInputs,
    /// Aluminium
    IndustryAluminium,
    /// Building materials
    IndustryBuildingMaterials,
    /// Chemicals
    IndustryChemicals,
    /// Coking coal
    IndustryCokingCoal,
    /// Copper
    IndustryCopper,
    /// Gold
    IndustryGold,
    /// Lumber and wood production
    IndustryLumberWood,
    /// Other industrial metals and mining
    IndustryIndustrialMetals,
    /// Other precious metals and mining
    IndustryPreciousMetals,
    /// Paper and paper products
    IndustryPaper,
    /// Silver
    IndustrySilver,
    /// Specialty chemicals
    IndustrySpecialtyChemicals,
    /// Steel
    IndustrySteel,
    /// Advertising agencies
    IndustryAdvertising,
    /// Broadcasting
    IndustryBroadcasting,
    /// Electronic gaming and multimedia
    IndustryGamingMultimedia,
    /// Entertainment
    IndustryEntertainment,
    /// Internet content and information
    IndustryInternetContent,
    /// Publishing
    IndustryPublishing,
    /// Telecom services
    IndustryTelecom,
    /// Apparel manufacturing
    IndustryApparelManufacturing,
    /// Apparel retail
    IndustryApparelRetail,
    /// Auto manufacturers
    IndustryAutoManufacturers,
    /// Auto parts
    IndustryAutoParts,
    /// Auto and truck dealerships
    IndustryAutoDealership,
    /// Department stores
    IndustryDepartmentStores,
    /// Footwear and accessories
    IndustryFootwearAccessories,
    /// Furnishing, fixtures and appliances
    IndustryFurnishings,
    /// Gambling
    IndustryGambling,
    /// Home improvement retail
    IndustryHomeImprovRetail,
    /// Internet retail
    IndustryInternetRetail,
    /// Leisure
    IndustryLeisure,
    /// Lodging
    IndustryLodging,
    /// Luxury goods
    IndustryLuxuryGoods,
    /// Packaging and containers
    IndustryPackagingContainers,
    /// Personal services
    IndustryPersonalServices,
    /// Recreational vehicles
    IndustryRecreationalVehicles,
    /// Residential construction
    IndustryResidentConstruction,
    /// Resorts and casinos
    IndustryResortsCasinos,
    /// Restaurants
    IndustryRestaurants,
    /// Specialty retail
    IndustrySpecialtyRetail,
    /// Textile manufacturing
    IndustryTextileManufacturing,
    /// Travel services
    IndustryTravelServices,
    /// Beverages - Brewers
    IndustryBeveragesBrewers,
    /// Beverages - Non-alcoholic
    IndustryBeveragesNonAlco,
    /// Beverages - Wineries and distilleries
    IndustryBeveragesWineries,
    /// Confectioners
    IndustryConfectioners,
    /// Discount stores
    IndustryDiscountStores,
    /// Education and training services
    IndustryEducationTrainig,
    /// Farm products
    IndustryFarmProducts,
    /// Food distribution
    IndustryFoodDistribution,
    /// Grocery stores
    IndustryGroceryStores,
    /// Household and personal products
    IndustryHouseholdProducts,
    /// Packaged foods
    IndustryPackagedFoods,
    /// Tobacco
    IndustryTobacco,
    /// Oil and gas drilling
    IndustryOilGasDrilling,
    /// Oil and gas extraction and processing
    IndustryOilGasEp,
    /// Oil and gas equipment and services
    IndustryOilGasEquipment,
    /// Oil and gas integrated
    IndustryOilGasIntegrated,
    /// Oil and gas midstream
    IndustryOilGasMidstream,
    /// Oil and gas refining and marketing
    IndustryOilGasRefining,
    /// Thermal coal
    IndustryThermalCoal,
    /// Uranium
    IndustryUranium,
    /// Exchange traded fund
    IndustryExchangeTradedFund,
    /// Assets management
    IndustryAssetsManagement,
    /// Banks - Diversified
    IndustryBanksDiversified,
    /// Banks - Regional
    IndustryBanksRegional,
    /// Capital markets
    IndustryCapitalMarkets,
    /// Closed-End fund - Debt
    IndustryCloseEndFundDebt,
    /// Closed-end fund - Equity
    IndustryCloseEndFundEquity,
    /// Closed-end fund - Foreign
    IndustryCloseEndFundForeign,
    /// Credit services
    IndustryCreditServices,
    /// Financial conglomerates
    IndustryFinancialConglomerate,
    /// Financial data and stock exchange
    IndustryFinancialDataExchange,
    /// Insurance brokers
    IndustryInsuranceBrokers,
    /// Insurance - Diversified
    IndustryInsuranceDiversified,
    /// Insurance - Life
    IndustryInsuranceLife,
    /// Insurance - Property and casualty
    IndustryInsuranceProperty,
    /// Insurance - Reinsurance
    IndustryInsuranceReinsurance,
    /// Insurance - Specialty
    IndustryInsuranceSpecialty,
    /// Mortgage finance
    IndustryMortgageFinance,
    /// Shell companies
    IndustryShellCompanies,
    /// Biotechnology
    IndustryBiotechnology,
    /// Diagnostics and research
    IndustryDiagnosticsResearch,
    /// Drugs manufacturers - general
    IndustryDrugsManufacturers,
    /// Drugs manufacturers - Specialty and generic
    IndustryDrugsManufacturersSpec,
    /// Healthcare plans
    IndustryHealthcarePlans,
    /// Health information services
    IndustryHealthInformation,
    /// Medical care facilities
    IndustryMedicalFacilities,
    /// Medical devices
    IndustryMedicalDevices,
    /// Medical distribution
    IndustryMedicalDistribution,
    /// Medical instruments and supplies
    IndustryMedicalInstruments,
    /// Pharmaceutical retailers
    IndustryPharmRetailers,
    /// Aerospace and defense
    IndustryAerospaceDefense,
    /// Airlines
    IndustryAirlines,
    /// Airports and air services
    IndustryAirportsServices,
    /// Building products and equipment
    IndustryBuildingProducts,
    /// Business equipment and supplies
    IndustryBusinessEquipment,
    /// Conglomerates
    IndustryConglomerates,
    /// Consulting services
    IndustryConsultingServices,
    /// Electrical equipment and parts
    IndustryElectricalEquipment,
    /// Engineering and construction
    IndustryEngineeringConstruction,
    /// Farm and heavy construction machinery
    IndustryFarmHeavyMachinery,
    /// Industrial distribution
    IndustryIndustrialDistribution,
    /// Infrastructure operations
    IndustryInfrastructureOperations,
    /// Integrated freight and logistics
    IndustryFreightLogistics,
    /// Marine shipping
    IndustryMarineShipping,
    /// Metal fabrication
    IndustryMetalFabrication,
    /// Pollution and treatment controls
    IndustryPollutionControl,
    /// Railroads
    IndustryRailroads,
    /// Rental and leasing services
    IndustryRentalLeasing,
    /// Security and protection services
    IndustrySecurityProtection,
    /// Specialty business services
    IndustrySpealityBusinessServices,
    /// Specialty industrial machinery
    IndustrySpealityMachinery,
    /// Stuffing and employment services
    IndustryStuffingEmployment,
    /// Tools and accessories
    IndustryToolsAccessories,
    /// Trucking
    IndustryTrucking,
    /// Waste management
    IndustryWasteManagement,
    /// Real estate - Development
    IndustryRealEstateDevelopment,
    /// Real estate - Diversified
    IndustryRealEstateDiversified,
    /// Real estate services
    IndustryRealEstateServices,
    /// REIT - Diversified
    IndustryReitDiversified,
    /// REIT - Healthcase facilities
    IndustryReitHealtcare,
    /// REIT - Hotel and motel
    IndustryReitHotelMotel,
    /// REIT - Industrial
    IndustryReitIndustrial,
    /// REIT - Mortgage
    IndustryReitMortage,
    /// REIT - Office
    IndustryReitOffice,
    /// REIT - Residential
    IndustryReitResidental,
    /// REIT - Retail
    IndustryReitRetail,
    /// REIT - Specialty
    IndustryReitSpeciality,
    /// Communication equipment
    IndustryCommunicationEquipment,
    /// Computer hardware
    IndustryComputerHardware,
    /// Consumer electronics
    IndustryConsumerElectronics,
    /// Electronic components
    IndustryElectronicComponents,
    /// Electronics and computer distribution
    IndustryElectronicDistribution,
    /// Information technology services
    IndustryItServices,
    /// Scientific and technical instruments
    IndustryScientificInstruments,
    /// Semiconductor equipment and materials
    IndustrySemiconductorEquipment,
    /// Semiconductors
    IndustrySemiconductors,
    /// Software - Application
    IndustrySoftwareApplication,
    /// Software - Infrastructure
    IndustrySoftwareInfrastructure,
    /// Solar
    IndustrySolar,
    /// Utilities - Diversified
    IndustryUtilitiesDiversified,
    /// Utilities - Independent power producers
    IndustryUtilitiesPowerproducers,
    /// Utilities - Renewable
    IndustryUtilitiesRenewable,
    /// Utilities - Regulated electric
    IndustryUtilitiesRegulatedElectric,
    /// Utilities - Regulated gas
    IndustryUtilitiesRegulatedGas,
    /// Utilities - Regulated water
    IndustryUtilitiesRegulatedWater,
    /// Start of the utilities services types enumeration. Corresponds to INDUSTRY_UTILITIES_DIVERSIFIED.
    IndustryUtilitiesFirst,
    /// End of the utilities services types enumeration. Corresponds to INDUSTRY_UTILITIES_REGULATED_WATER.
    IndustryUtilitiesLast,

    // this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
    Unmapped151, Unmapped152, Unmapped153, Unmapped154, Unmapped155, Unmapped156, Unmapped157, Unmapped158, Unmapped159,
    Unmapped160, Unmapped161, Unmapped162, Unmapped163, Unmapped164, Unmapped165, Unmapped166, Unmapped167, Unmapped168, Unmapped169, Unmapped170, Unmapped171, Unmapped172,
    Unmapped173, Unmapped174, Unmapped175, Unmapped176, Unmapped177, Unmapped178, Unmapped179, Unmapped180, Unmapped181, Unmapped182, Unmapped183, Unmapped184, Unmapped185,
    Unmapped186, Unmapped187, Unmapped188, Unmapped189, Unmapped190, Unmapped191, Unmapped192, Unmapped193, Unmapped194, Unmapped195, Unmapped196, Unmapped197, Unmapped198,

}

/// /
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumSymbolOptionMode {
    /// European option may only be exercised on a specified date (expiration, execution date, delivery date)
    SymbolOptionModeEuropean,
    /// American option may be exercised on any trading day or before expiry. The period within which a buyer can exercise the option is specified for it
    SymbolOptionModeAmerican ,

    // this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
    Unmapped2,  Unmapped3,   Unmapped4,  Unmapped5,  Unmapped6, Unmapped7,
    Unmapped8,  Unmapped9,  Unmapped10, Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
    Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
    Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
    Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
    Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
    Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,
}

/// An option is a contract, which gives the right, but not the obligation, to buy or sell an underlying asset (goods, stocks, futures, etc.) at a specified price on or before a specific date. The following enumerations describe option properties, including the option type and the right arising from it. /
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumSymbolOptionRight {
    /// A call option gives you the right to buy an asset at a specified price
    SymbolOptionRightCall,
    /// A put option gives you the right to sell an asset at a specified price
    SymbolOptionRightPut,

    // this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
    Unmapped2,  Unmapped3,   Unmapped4,  Unmapped5,  Unmapped6, Unmapped7,
    Unmapped8,  Unmapped9,  Unmapped10, Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
    Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
    Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
    Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
    Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
    Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,
}

/// If the SYMBOL_EXPIRATION_MODE property is set to SYMBOL_EXPIRATION_GTC (good till canceled), the expiration of pending orders, as well as of Stop Loss/Take Profit orders should be additionally set using the ENUM_SYMBOL_ORDER_GTC_MODE enumeration./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumSymbolOrderGtcMode {
    /// Pending orders and Stop Loss/Take Profit levels are valid for an unlimited period until their explicit cancellation
    SymbolOrdersGtc,
    /// Orders are valid during one trading day. At the end of the day, all Stop Loss and Take Profit levels, as well as pending orders are deleted.
    SymbolOrdersDaily,
    /// When a trade day changes, only pending orders are deleted, while Stop Loss and Take Profit levels are preserved.
    SymbolOrdersDailyExcludingStops,

    // this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
    Unmapped3,   Unmapped4,  Unmapped5,  Unmapped6, Unmapped7,
    Unmapped8,  Unmapped9,  Unmapped10, Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
    Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
    Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
    Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
    Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
    Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,
}

/// Financial instruments are categorized by sectors of the economy. An economic sector is a part of economic activity which has specific characteristics, economic goals, functions and behavior, which allow separating this sector from other parts of the economy. ENUM_SYMBOL_SECTOR lists the economic sectors which a trading instruments can belong to./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumSymbolSector {
    /// Undefined
    SectorUndefined,
    /// Basic materials
    SectorBasicMaterials,
    /// Communication services
    SectorCommunicationServices,
    /// Consumer cyclical
    SectorConsumerCyclical,
    /// Consumer defensive
    SectorConsumerDefensive,
    /// Currencies
    SectorCurrency,
    /// Cryptocurrencies
    SectorCurrencyCrypto,
    /// Energy
    SectorEnergy,
    /// Finance
    SectorFinancial,
    /// Healthcare
    SectorHealthcare,
    /// Industrials
    SectorIndustrials,
    /// Real estate
    SectorRealEstate,
    /// Technology
    SectorTechnology,
    /// Utilities
    SectorUtilities,

    // this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
    Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
    Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
    Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
    Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
    Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
    Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,

}

/// Methods of swap calculation at position transfer are specified in enumeration ENUM_SYMBOL_SWAP_MODE. The method of swap calculation determines the units of measure of the SYMBOL_SWAP_LONG and SYMBOL_SWAP_SHORT parameters. For example, if swaps are charged in the client deposit currency, then the values of those parameters are specified as an amount of money in the client deposit currency./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumSymbolSwapMode {
    /// Swaps disabled (no swaps)
    SymbolSwapModeDisabled,
    /// Swaps are charged in points
    SymbolSwapModePoints,
    /// Swaps are charged in money in base currency of the symbol
    SymbolSwapModeCurrencySymbol,
    /// Swaps are charged in money in margin currency of the symbol
    SymbolSwapModeCurrencyMargin,
    /// Swaps are charged in money, in client deposit currency
    SymbolSwapModeCurrencyDeposit,
    /// Swaps are charged as the specified annual interest from the instrument price at calculation of swap (standard bank year is 360 days)
    SymbolSwapModeInterestCurrent,
    /// Swaps are charged as the specified annual interest from the open price of position (standard bank year is 360 days)
    SymbolSwapModeInterestOpen,
    /// Swaps are charged by reopening positions. At the end of a trading day the position is closed. Next day it is reopened by the close price +/- specified number of points (parameters SYMBOL_SWAP_LONG and SYMBOL_SWAP_SHORT)
    SymbolSwapModeReopenCurrent,
    /// Swaps are charged by reopening positions. At the end of a trading day the position is closed. Next day it is reopened by the current Bid price +/- specified number of points (parameters SYMBOL_SWAP_LONG and SYMBOL_SWAP_SHORT)
    SymbolSwapModeReopenBid,

    // this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
    Unmapped9,  Unmapped10, Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
    Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
    Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
    Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
    Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
    Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,

}

/// Possible deal execution modes for a certain symbol are defined in enumeration ENUM_SYMBOL_TRADE_EXECUTION./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumSymbolTradeExecution {
    ///  Execution by request
    SymbolTradeExecutionRequest,
    /// Instant execution
    SymbolTradeExecutionInstant,
    /// Market execution
    SymbolTradeExecutionMarket,
    /// Exchange execution
    SymbolTradeExecutionExchange,

    // this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
    Unmapped4,  Unmapped5,  Unmapped6,  Unmapped7,
    Unmapped8,  Unmapped9,  Unmapped10, Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
    Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
    Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
    Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
    Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
    Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,

}

/// There are several symbol trading modes. Information about trading modes of a certain symbol is reflected in the values of enumeration ENUM_SYMBOL_TRADE_MODE./
/// auto-generated from https://www.mql5.com/en/docs/constants/environment_state/marketinfoconstants
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumSymbolTradeMode {
    /// Trade is disabled for the symbol
    SymbolTradeModeDisabled,
    /// Allowed only long positions
    SymbolTradeModeLongonly,
    /// Allowed only short positions
    SymbolTradeModeShortonly,
    /// Allowed only position close operations
    SymbolTradeModeCloseonly,
    /// No trade restrictions
    SymbolTradeModeFull,

    // this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
    Unmapped5,  Unmapped6,  Unmapped7,
    Unmapped8,  Unmapped9,  Unmapped10, Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
    Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
    Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
    Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
    Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
    Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,

}
