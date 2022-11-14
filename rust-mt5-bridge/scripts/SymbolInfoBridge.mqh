struct SymbolInfoBridge {
	/// Symbol data arrives with a delay. The property can be requested only for symbols selected in MarketWatch (SYMBOL_SELECT = true). The ERR_MARKET_NOT_SELECTED (4302) error is generated for other symbols
	symbol_subscription_delay: bool,
	/// The sector of the economy to which the asset belongs
	symbol_sector: ENUM_SYMBOL_SECTOR,
	/// The industry or the economy branch to which the symbol belongs
	symbol_industry: ENUM_SYMBOL_INDUSTRY,
	/// It is a custom symbol – the symbol has been created synthetically based on other symbols from the Market Watch and/or external data sources
	symbol_custom: bool,
	/// The color of the background used for the symbol in Market Watch
	symbol_background_color: color,
	/// The price type used for generating symbols bars, i.e. Bid or Last
	symbol_chart_mode: ENUM_SYMBOL_CHART_MODE,
	/// Symbol with this name exists
	symbol_exist: bool,
	/// Symbol is selected in Market Watch
	symbol_select: bool,
	/// Symbol is visible in Market Watch.
	symbol_visible: bool,
	/// Number of deals in the current session
	symbol_session_deals: long,
	/// Number of Buy orders at the moment
	symbol_session_buy_orders: long,
	/// Number of Sell orders at the moment
	symbol_session_sell_orders: long,
	/// Volume of the last deal
	symbol_volume: long,
	/// Maximal day volume
	symbol_volumehigh: long,
	/// Minimal day volume
	symbol_volumelow: long,
	/// Time of the last quote
	symbol_time: datetime,
	/// Time of the last quote in milliseconds since 1970.01.01
	symbol_time_msc: long,
	/// Digits after a decimal point
	symbol_digits: int,
	/// Indication of a floating spread
	symbol_spread_float: bool,
	/// Spread value in points
	symbol_spread: int,
	/// Maximal number of requests shown in Depth of Market. For symbols that have no queue of requests, the value is equal to zero.
	symbol_ticks_bookdepth: int,
	/// Contract price calculation mode
	symbol_trade_calc_mode: ENUM_SYMBOL_CALC_MODE,
	/// Order execution type
	symbol_trade_mode: ENUM_SYMBOL_TRADE_MODE,
	/// Date of the symbol trade beginning (usually used for futures)
	symbol_start_time: datetime,
	/// Date of the symbol trade end (usually used for futures)
	symbol_expiration_time: datetime,
	/// Minimal indention in points from the current close price to place Stop orders
	symbol_trade_stops_level: int,
	/// Distance to freeze trade operations in points
	symbol_trade_freeze_level: int,
	/// Deal execution mode
	symbol_trade_exemode: ENUM_SYMBOL_TRADE_EXECUTION,
	/// Swap calculation model
	symbol_swap_mode: ENUM_SYMBOL_SWAP_MODE,
	/// The day of week to charge 3-day swap rollover
	symbol_swap_rollover3days: ENUM_DAY_OF_WEEK,
	/// Calculating hedging margin using the larger leg (Buy or Sell)
	symbol_margin_hedged_use_leg: bool,
	/// Flags of allowed order expiration modes
	symbol_expiration_mode : int,
	/// Flags of allowed order filling modes
	symbol_filling_mode: int,
	/// Flags of allowed order types
	symbol_order_mode: int,
	/// Expiration of Stop Loss and Take Profit orders, if SYMBOL_EXPIRATION_MODE=SYMBOL_EXPIRATION_GTC (Good till canceled)
	symbol_order_gtc_mode: ENUM_SYMBOL_ORDER_GTC_MODE,
	/// Option type
	symbol_option_mode: ENUM_SYMBOL_OPTION_MODE,
	/// Option right (Call/Put)
	symbol_option_right: ENUM_SYMBOL_OPTION_RIGHT,
	/// Bid - best sell offer
	symbol_bid: double,
	/// Maximal Bid of the day
	symbol_bidhigh: double,
	/// Minimal Bid of the day
	symbol_bidlow: double,
	/// Ask - best buy offer
	symbol_ask: double,
	/// Maximal Ask of the day
	symbol_askhigh: double,
	/// Minimal Ask of the day
	symbol_asklow: double,
	/// Price of the last deal
	symbol_last: double,
	/// Maximal Last of the day
	symbol_lasthigh: double,
	/// Minimal Last of the day
	symbol_lastlow: double,
	/// Volume of the last deal
	symbol_volume_real: double,
	/// Maximum Volume of the day
	symbol_volumehigh_real: double,
	/// Minimum Volume of the day
	symbol_volumelow_real: double,
	/// The strike price of an option. The price at which an option buyer can buy (in a Call option) or sell (in a Put option) the underlying asset, and the option seller is obliged to sell or buy the appropriate amount of the underlying asset.
	symbol_option_strike: double,
	/// Symbol point value
	symbol_point: double,
	/// Value of SYMBOL_TRADE_TICK_VALUE_PROFIT
	symbol_trade_tick_value: double,
	/// Calculated tick price for a profitable position
	symbol_trade_tick_value_profit: double,
	/// Calculated tick price for a losing position
	symbol_trade_tick_value_loss: double,
	/// Minimal price change
	symbol_trade_tick_size: double,
	/// Trade contract size
	symbol_trade_contract_size: double,
	/// Accrued interest – accumulated coupon interest, i.e. part of the coupon interest calculated in proportion to the number of days since the coupon bond issuance or the last coupon interest payment
	symbol_trade_accrued_interest: double,
	/// Face value – initial bond value set by the issuer
	symbol_trade_face_value: double,
	/// Liquidity Rate is the share of the asset that can be used for the margin.
	symbol_trade_liquidity_rate: double,
	/// Minimal volume for a deal
	symbol_volume_min: double,
	/// Maximal volume for a deal
	symbol_volume_max: double,
	/// Minimal volume change step for deal execution
	symbol_volume_step: double,
	/// Maximum allowed aggregate volume of an open position and pending orders in one direction (buy or sell) for the symbol. For example, with the limitation of 5 lots, you can have an open buy position with the volume of 5 lots and place a pending order Sell Limit with the volume of 5 lots. But in this case you cannot place a Buy Limit pending order (since the total volume in one direction will exceed the limitation) or place Sell Limit with the volume more than 5 lots.
	symbol_volume_limit: double,
	/// Long swap value
	symbol_swap_long: double,
	/// Short swap value
	symbol_swap_short: double,
	/// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from SUNDAY to the next day. There following values are supported:
	symbol_swap_sunday: double,
	/// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Monday to Tuesday
	symbol_swap_monday: double,
	/// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Tuesday to Wednesday
	symbol_swap_tuesday: double,
	/// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Wednesday to Thursday
	symbol_swap_wednesday: double,
	/// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Thursday to Friday
	symbol_swap_thursday: double,
	/// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Friday to Saturday
	symbol_swap_friday: double,
	/// Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Saturday to Sunday
	symbol_swap_saturday: double,
	/// Initial margin means the amount in the margin currency required for opening a position with the volume of one lot. It is used for checking a client's assets when he or she enters the market.
	symbol_margin_initial: double,
	/// The maintenance margin. If it is set, it sets the margin amount in the margin currency of the symbol, charged from one lot. It is used for checking a client's assets when his/her account state changes. If the maintenance margin is equal to 0, the initial margin is used.
	symbol_margin_maintenance: double,
	/// Summary volume of current session deals
	symbol_session_volume: double,
	/// Summary turnover of the current session
	symbol_session_turnover: double,
	/// Summary open interest
	symbol_session_interest: double,
	/// Current volume of Buy orders
	symbol_session_buy_orders_volume: double,
	/// Current volume of Sell orders
	symbol_session_sell_orders_volume: double,
	/// Open price of the current session
	symbol_session_open: double,
	/// Close price of the current session
	symbol_session_close: double,
	/// Average weighted price of the current session
	symbol_session_aw: double,
	/// Settlement price of the current session
	symbol_session_price_settlement: double,
	/// Minimal price of the current session
	symbol_session_price_limit_min: double,
	/// Maximal price of the current session
	symbol_session_price_limit_max: double,
	/// Contract size or margin value per one lot of hedged positions (oppositely directed positions of one symbol). Two margin calculation methods are possible for hedged positions. The calculation method is defined by the broker. 
	symbol_margin_hedged: double,
	/// Change of the current price relative to the end of the previous trading day in %
	symbol_price_change: double,
	/// Price volatility in %
	symbol_price_volatility: double,
	/// Theoretical option price
	symbol_price_theoretical: double,
	/// Option/warrant delta shows the value the option price changes by, when the underlying asset price changes by 1
	symbol_price_delta: double,
	/// Option/warrant theta shows the number of points the option price is to lose every day due to a temporary breakup, i.e. when the expiration date approaches
	symbol_price_theta: double,
	/// Option/warrant gamma shows the change rate of delta – how quickly or slowly the option premium changes
	symbol_price_gamma: double,
	/// Option/warrant vega shows the number of points the option price changes by when the volatility changes by 1%
	symbol_price_vega: double,
	/// Option/warrant rho reflects the sensitivity of the theoretical option price to the interest rate changing by 1%
	symbol_price_rho: double,
	/// Option/warrant omega. Option elasticity shows a relative percentage change of the option price by the percentage change of the underlying asset price
	symbol_price_omega: double,
	/// Option/warrant sensitivity shows by how many points the price of the option's underlying asset should change so that the price of the option changes by one point
	symbol_price_sensitivity: double,
	/// The underlying asset of a derivative
	symbol_basis: string,
	/// The name of the sector or category to which the financial symbol belongs
	symbol_category: string,
	/// The country to which the financial symbol belongs
	symbol_country: string,
	/// The sector of the economy to which the financial symbol belongs
	symbol_sector_name: string,
	/// The industry branch or the industry to which the financial symbol belongs
	symbol_industry_name: string,
	/// Basic currency of a symbol
	symbol_currency_base: string,
	/// Profit currency
	symbol_currency_profit: string,
	/// Margin currency
	symbol_currency_margin: string,
	/// Feeder of the current quote
	symbol_bank: string,
	/// Symbol description
	symbol_description: string,
	/// The name of the exchange in which the financial symbol is traded
	symbol_exchange: string,
	/// The formula used for the custom symbol pricing. If the name of a financial symbol used in the formula starts with a digit or contains a special character (&quot;&gt;&quot; &quot;, &quot;.&quot;, &quot;-&quot;, &quot;&amp;&quot;, &quot;#&quot; and so on) quotation marks should be used around this symbol name.
	symbol_formula: string,
	/// The name of a symbol in the ISIN system (International Securities Identification Number). The International Securities Identification Number is a 12-digit alphanumeric code that uniquely identifies a security. The presence of this symbol property is determined on the side of a trade server.
	symbol_isin: string,
	/// The address of the web page containing symbol information. This address will be displayed as a link when viewing symbol properties in the terminal
	symbol_page: string,
	/// Path in the symbol tree
	symbol_path: string,
}

	let instance = new SymbolInfoBridge();
	SymbolInfoInteger(_Symbol, SYMBOL_SUBSCRIPTION_DELAY, instance.symbol_subscription_delay);
	SymbolInfoInteger(_Symbol, SYMBOL_SECTOR, instance.symbol_sector);
	SymbolInfoInteger(_Symbol, SYMBOL_INDUSTRY, instance.symbol_industry);
	SymbolInfoInteger(_Symbol, SYMBOL_CUSTOM, instance.symbol_custom);
	SymbolInfoInteger(_Symbol, SYMBOL_BACKGROUND_COLOR, instance.symbol_background_color);
	SymbolInfoInteger(_Symbol, SYMBOL_CHART_MODE, instance.symbol_chart_mode);
	SymbolInfoInteger(_Symbol, SYMBOL_EXIST, instance.symbol_exist);
	SymbolInfoInteger(_Symbol, SYMBOL_SELECT, instance.symbol_select);
	SymbolInfoInteger(_Symbol, SYMBOL_VISIBLE, instance.symbol_visible);
	SymbolInfoInteger(_Symbol, SYMBOL_SESSION_DEALS, instance.symbol_session_deals);
	SymbolInfoInteger(_Symbol, SYMBOL_SESSION_BUY_ORDERS, instance.symbol_session_buy_orders);
	SymbolInfoInteger(_Symbol, SYMBOL_SESSION_SELL_ORDERS, instance.symbol_session_sell_orders);
	SymbolInfoInteger(_Symbol, SYMBOL_VOLUME, instance.symbol_volume);
	SymbolInfoInteger(_Symbol, SYMBOL_VOLUMEHIGH, instance.symbol_volumehigh);
	SymbolInfoInteger(_Symbol, SYMBOL_VOLUMELOW, instance.symbol_volumelow);
	SymbolInfoInteger(_Symbol, SYMBOL_TIME, instance.symbol_time);
	SymbolInfoInteger(_Symbol, SYMBOL_TIME_MSC, instance.symbol_time_msc);
	SymbolInfoInteger(_Symbol, SYMBOL_DIGITS, instance.symbol_digits);
	SymbolInfoInteger(_Symbol, SYMBOL_SPREAD_FLOAT, instance.symbol_spread_float);
	SymbolInfoInteger(_Symbol, SYMBOL_SPREAD, instance.symbol_spread);
	SymbolInfoInteger(_Symbol, SYMBOL_TICKS_BOOKDEPTH, instance.symbol_ticks_bookdepth);
	SymbolInfoInteger(_Symbol, SYMBOL_TRADE_CALC_MODE, instance.symbol_trade_calc_mode);
	SymbolInfoInteger(_Symbol, SYMBOL_TRADE_MODE, instance.symbol_trade_mode);
	SymbolInfoInteger(_Symbol, SYMBOL_START_TIME, instance.symbol_start_time);
	SymbolInfoInteger(_Symbol, SYMBOL_EXPIRATION_TIME, instance.symbol_expiration_time);
	SymbolInfoInteger(_Symbol, SYMBOL_TRADE_STOPS_LEVEL, instance.symbol_trade_stops_level);
	SymbolInfoInteger(_Symbol, SYMBOL_TRADE_FREEZE_LEVEL, instance.symbol_trade_freeze_level);
	SymbolInfoInteger(_Symbol, SYMBOL_TRADE_EXEMODE, instance.symbol_trade_exemode);
	SymbolInfoInteger(_Symbol, SYMBOL_SWAP_MODE, instance.symbol_swap_mode);
	SymbolInfoInteger(_Symbol, SYMBOL_SWAP_ROLLOVER3DAYS, instance.symbol_swap_rollover3days);
	SymbolInfoInteger(_Symbol, SYMBOL_MARGIN_HEDGED_USE_LEG, instance.symbol_margin_hedged_use_leg);
	SymbolInfoInteger(_Symbol, SYMBOL_EXPIRATION_MODE , instance.symbol_expiration_mode );
	SymbolInfoInteger(_Symbol, SYMBOL_FILLING_MODE, instance.symbol_filling_mode);
	SymbolInfoInteger(_Symbol, SYMBOL_ORDER_MODE, instance.symbol_order_mode);
	SymbolInfoInteger(_Symbol, SYMBOL_ORDER_GTC_MODE, instance.symbol_order_gtc_mode);
	SymbolInfoInteger(_Symbol, SYMBOL_OPTION_MODE, instance.symbol_option_mode);
	SymbolInfoInteger(_Symbol, SYMBOL_OPTION_RIGHT, instance.symbol_option_right);
	SymbolInfoDouble(_Symbol, SYMBOL_BID, instance.symbol_bid);
	SymbolInfoDouble(_Symbol, SYMBOL_BIDHIGH, instance.symbol_bidhigh);
	SymbolInfoDouble(_Symbol, SYMBOL_BIDLOW, instance.symbol_bidlow);
	SymbolInfoDouble(_Symbol, SYMBOL_ASK, instance.symbol_ask);
	SymbolInfoDouble(_Symbol, SYMBOL_ASKHIGH, instance.symbol_askhigh);
	SymbolInfoDouble(_Symbol, SYMBOL_ASKLOW, instance.symbol_asklow);
	SymbolInfoDouble(_Symbol, SYMBOL_LAST, instance.symbol_last);
	SymbolInfoDouble(_Symbol, SYMBOL_LASTHIGH, instance.symbol_lasthigh);
	SymbolInfoDouble(_Symbol, SYMBOL_LASTLOW, instance.symbol_lastlow);
	SymbolInfoDouble(_Symbol, SYMBOL_VOLUME_REAL, instance.symbol_volume_real);
	SymbolInfoDouble(_Symbol, SYMBOL_VOLUMEHIGH_REAL, instance.symbol_volumehigh_real);
	SymbolInfoDouble(_Symbol, SYMBOL_VOLUMELOW_REAL, instance.symbol_volumelow_real);
	SymbolInfoDouble(_Symbol, SYMBOL_OPTION_STRIKE, instance.symbol_option_strike);
	SymbolInfoDouble(_Symbol, SYMBOL_POINT, instance.symbol_point);
	SymbolInfoDouble(_Symbol, SYMBOL_TRADE_TICK_VALUE, instance.symbol_trade_tick_value);
	SymbolInfoDouble(_Symbol, SYMBOL_TRADE_TICK_VALUE_PROFIT, instance.symbol_trade_tick_value_profit);
	SymbolInfoDouble(_Symbol, SYMBOL_TRADE_TICK_VALUE_LOSS, instance.symbol_trade_tick_value_loss);
	SymbolInfoDouble(_Symbol, SYMBOL_TRADE_TICK_SIZE, instance.symbol_trade_tick_size);
	SymbolInfoDouble(_Symbol, SYMBOL_TRADE_CONTRACT_SIZE, instance.symbol_trade_contract_size);
	SymbolInfoDouble(_Symbol, SYMBOL_TRADE_ACCRUED_INTEREST, instance.symbol_trade_accrued_interest);
	SymbolInfoDouble(_Symbol, SYMBOL_TRADE_FACE_VALUE, instance.symbol_trade_face_value);
	SymbolInfoDouble(_Symbol, SYMBOL_TRADE_LIQUIDITY_RATE, instance.symbol_trade_liquidity_rate);
	SymbolInfoDouble(_Symbol, SYMBOL_VOLUME_MIN, instance.symbol_volume_min);
	SymbolInfoDouble(_Symbol, SYMBOL_VOLUME_MAX, instance.symbol_volume_max);
	SymbolInfoDouble(_Symbol, SYMBOL_VOLUME_STEP, instance.symbol_volume_step);
	SymbolInfoDouble(_Symbol, SYMBOL_VOLUME_LIMIT, instance.symbol_volume_limit);
	SymbolInfoDouble(_Symbol, SYMBOL_SWAP_LONG, instance.symbol_swap_long);
	SymbolInfoDouble(_Symbol, SYMBOL_SWAP_SHORT, instance.symbol_swap_short);
	SymbolInfoDouble(_Symbol, SYMBOL_SWAP_SUNDAY, instance.symbol_swap_sunday);
	SymbolInfoDouble(_Symbol, SYMBOL_SWAP_MONDAY, instance.symbol_swap_monday);
	SymbolInfoDouble(_Symbol, SYMBOL_SWAP_TUESDAY, instance.symbol_swap_tuesday);
	SymbolInfoDouble(_Symbol, SYMBOL_SWAP_WEDNESDAY, instance.symbol_swap_wednesday);
	SymbolInfoDouble(_Symbol, SYMBOL_SWAP_THURSDAY, instance.symbol_swap_thursday);
	SymbolInfoDouble(_Symbol, SYMBOL_SWAP_FRIDAY, instance.symbol_swap_friday);
	SymbolInfoDouble(_Symbol, SYMBOL_SWAP_SATURDAY, instance.symbol_swap_saturday);
	SymbolInfoDouble(_Symbol, SYMBOL_MARGIN_INITIAL, instance.symbol_margin_initial);
	SymbolInfoDouble(_Symbol, SYMBOL_MARGIN_MAINTENANCE, instance.symbol_margin_maintenance);
	SymbolInfoDouble(_Symbol, SYMBOL_SESSION_VOLUME, instance.symbol_session_volume);
	SymbolInfoDouble(_Symbol, SYMBOL_SESSION_TURNOVER, instance.symbol_session_turnover);
	SymbolInfoDouble(_Symbol, SYMBOL_SESSION_INTEREST, instance.symbol_session_interest);
	SymbolInfoDouble(_Symbol, SYMBOL_SESSION_BUY_ORDERS_VOLUME, instance.symbol_session_buy_orders_volume);
	SymbolInfoDouble(_Symbol, SYMBOL_SESSION_SELL_ORDERS_VOLUME, instance.symbol_session_sell_orders_volume);
	SymbolInfoDouble(_Symbol, SYMBOL_SESSION_OPEN, instance.symbol_session_open);
	SymbolInfoDouble(_Symbol, SYMBOL_SESSION_CLOSE, instance.symbol_session_close);
	SymbolInfoDouble(_Symbol, SYMBOL_SESSION_AW, instance.symbol_session_aw);
	SymbolInfoDouble(_Symbol, SYMBOL_SESSION_PRICE_SETTLEMENT, instance.symbol_session_price_settlement);
	SymbolInfoDouble(_Symbol, SYMBOL_SESSION_PRICE_LIMIT_MIN, instance.symbol_session_price_limit_min);
	SymbolInfoDouble(_Symbol, SYMBOL_SESSION_PRICE_LIMIT_MAX, instance.symbol_session_price_limit_max);
	SymbolInfoDouble(_Symbol, SYMBOL_MARGIN_HEDGED, instance.symbol_margin_hedged);
	SymbolInfoDouble(_Symbol, SYMBOL_PRICE_CHANGE, instance.symbol_price_change);
	SymbolInfoDouble(_Symbol, SYMBOL_PRICE_VOLATILITY, instance.symbol_price_volatility);
	SymbolInfoDouble(_Symbol, SYMBOL_PRICE_THEORETICAL, instance.symbol_price_theoretical);
	SymbolInfoDouble(_Symbol, SYMBOL_PRICE_DELTA, instance.symbol_price_delta);
	SymbolInfoDouble(_Symbol, SYMBOL_PRICE_THETA, instance.symbol_price_theta);
	SymbolInfoDouble(_Symbol, SYMBOL_PRICE_GAMMA, instance.symbol_price_gamma);
	SymbolInfoDouble(_Symbol, SYMBOL_PRICE_VEGA, instance.symbol_price_vega);
	SymbolInfoDouble(_Symbol, SYMBOL_PRICE_RHO, instance.symbol_price_rho);
	SymbolInfoDouble(_Symbol, SYMBOL_PRICE_OMEGA, instance.symbol_price_omega);
	SymbolInfoDouble(_Symbol, SYMBOL_PRICE_SENSITIVITY, instance.symbol_price_sensitivity);
	SymbolInfoString(_Symbol, SYMBOL_BASIS, instance.symbol_basis);
	SymbolInfoString(_Symbol, SYMBOL_CATEGORY, instance.symbol_category);
	SymbolInfoString(_Symbol, SYMBOL_COUNTRY, instance.symbol_country);
	SymbolInfoString(_Symbol, SYMBOL_SECTOR_NAME, instance.symbol_sector_name);
	SymbolInfoString(_Symbol, SYMBOL_INDUSTRY_NAME, instance.symbol_industry_name);
	SymbolInfoString(_Symbol, SYMBOL_CURRENCY_BASE, instance.symbol_currency_base);
	SymbolInfoString(_Symbol, SYMBOL_CURRENCY_PROFIT, instance.symbol_currency_profit);
	SymbolInfoString(_Symbol, SYMBOL_CURRENCY_MARGIN, instance.symbol_currency_margin);
	SymbolInfoString(_Symbol, SYMBOL_BANK, instance.symbol_bank);
	SymbolInfoString(_Symbol, SYMBOL_DESCRIPTION, instance.symbol_description);
	SymbolInfoString(_Symbol, SYMBOL_EXCHANGE, instance.symbol_exchange);
	SymbolInfoString(_Symbol, SYMBOL_FORMULA, instance.symbol_formula);
	SymbolInfoString(_Symbol, SYMBOL_ISIN, instance.symbol_isin);
	SymbolInfoString(_Symbol, SYMBOL_PAGE, instance.symbol_page);
	SymbolInfoString(_Symbol, SYMBOL_PATH, instance.symbol_path);
