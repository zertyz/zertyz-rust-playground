// Defines a structure and function to retrieve symbol information, which is, then, shared as-is with Rust

struct SymbolInfoBridge {
	            ENUM_SYMBOL_SECTOR                     symbol_sector;      // The sector of the economy to which the asset belongs
	          ENUM_SYMBOL_INDUSTRY                   symbol_industry;      // The industry or the economy branch to which the symbol belongs
	                         color           symbol_background_color;      // The color of the background used for the symbol in Market Watch
	        ENUM_SYMBOL_CHART_MODE                 symbol_chart_mode;      // The price type used for generating symbols bars, i.e. Bid or Last
	                          long              symbol_session_deals;      // Number of deals in the current session
	                          long         symbol_session_buy_orders;      // Number of Buy orders at the moment
	                          long        symbol_session_sell_orders;      // Number of Sell orders at the moment
	                          long                     symbol_volume;      // Volume of the last deal
	                          long                 symbol_volumehigh;      // Maximal day volume
	                          long                  symbol_volumelow;      // Minimal day volume
	                      datetime                       symbol_time;      // Time of the last quote
	                          long                   symbol_time_msc;      // Time of the last quote in milliseconds since 1970.01.01
	                           int                     symbol_digits;      // Digits after a decimal point
	                           int                     symbol_spread;      // Spread value in points
	                           int            symbol_ticks_bookdepth;      // Maximal number of requests shown in Depth of Market. For symbols that have no queue of requests, the value is equal to zero.
	         ENUM_SYMBOL_CALC_MODE            symbol_trade_calc_mode;      // Contract price calculation mode
	        ENUM_SYMBOL_TRADE_MODE                 symbol_trade_mode;      // Order execution type
	                           int                                _1;      // for alignment purposes... integers should come in pairs before 8 bytes data
	                      datetime                 symbol_start_time;      // Date of the symbol trade beginning (usually used for futures)
	                      datetime            symbol_expiration_time;      // Date of the symbol trade end (usually used for futures)
	                           int          symbol_trade_stops_level;      // Minimal indention in points from the current close price to place Stop orders
	                           int         symbol_trade_freeze_level;      // Distance to freeze trade operations in points
	   ENUM_SYMBOL_TRADE_EXECUTION              symbol_trade_exemode;      // Deal execution mode
	         ENUM_SYMBOL_SWAP_MODE                  symbol_swap_mode;      // Swap calculation model
	              ENUM_DAY_OF_WEEK         symbol_swap_rollover3days;      // The day of week to charge 3-day swap rollover
	                           int           symbol_expiration_mode ;      // Flags of allowed order expiration modes
	                           int               symbol_filling_mode;      // Flags of allowed order filling modes
	                           int                 symbol_order_mode;      // Flags of allowed order types
	    ENUM_SYMBOL_ORDER_GTC_MODE             symbol_order_gtc_mode;      // Expiration of Stop Loss and Take Profit orders, if SYMBOL_EXPIRATION_MODE=SYMBOL_EXPIRATION_GTC (Good till canceled)
	       ENUM_SYMBOL_OPTION_MODE                symbol_option_mode;      // Option type
	      ENUM_SYMBOL_OPTION_RIGHT               symbol_option_right;      // Option right (Call/Put)
	                           int                                _2;      // for alignment purposes... integers should come in pairs before 8 bytes data
	                        double                        symbol_bid;      // Bid - best sell offer
	                        double                    symbol_bidhigh;      // Maximal Bid of the day
	                        double                     symbol_bidlow;      // Minimal Bid of the day
	                        double                        symbol_ask;      // Ask - best buy offer
	                        double                    symbol_askhigh;      // Maximal Ask of the day
	                        double                     symbol_asklow;      // Minimal Ask of the day
	                        double                       symbol_last;      // Price of the last deal
	                        double                   symbol_lasthigh;      // Maximal Last of the day
	                        double                    symbol_lastlow;      // Minimal Last of the day
	                        double                symbol_volume_real;      // Volume of the last deal
	                        double            symbol_volumehigh_real;      // Maximum Volume of the day
	                        double             symbol_volumelow_real;      // Minimum Volume of the day
	                        double              symbol_option_strike;      // The strike price of an option. The price at which an option buyer can buy (in a Call option) or sell (in a Put option) the underlying asset, and the option seller is obliged to sell or buy the appropriate amount of the underlying asset.
	                        double                      symbol_point;      // Symbol point value
	                        double           symbol_trade_tick_value;      // Value of SYMBOL_TRADE_TICK_VALUE_PROFIT
	                        double    symbol_trade_tick_value_profit;      // Calculated tick price for a profitable position
	                        double      symbol_trade_tick_value_loss;      // Calculated tick price for a losing position
	                        double            symbol_trade_tick_size;      // Minimal price change
	                        double        symbol_trade_contract_size;      // Trade contract size
	                        double     symbol_trade_accrued_interest;      // Accrued interest – accumulated coupon interest, i.e. part of the coupon interest calculated in proportion to the number of days since the coupon bond issuance or the last coupon interest payment
	                        double           symbol_trade_face_value;      // Face value – initial bond value set by the issuer
	                        double       symbol_trade_liquidity_rate;      // Liquidity Rate is the share of the asset that can be used for the margin.
	                        double                 symbol_volume_min;      // Minimal volume for a deal
	                        double                 symbol_volume_max;      // Maximal volume for a deal
	                        double                symbol_volume_step;      // Minimal volume change step for deal execution
	                        double               symbol_volume_limit;      // Maximum allowed aggregate volume of an open position and pending orders in one direction (buy or sell) for the symbol. For example, with the limitation of 5 lots, you can have an open buy position with the volume of 5 lots and place a pending order Sell Limit with the volume of 5 lots. But in this case you cannot place a Buy Limit pending order (since the total volume in one direction will exceed the limitation) or place Sell Limit with the volume more than 5 lots.
	                        double                  symbol_swap_long;      // Long swap value
	                        double                 symbol_swap_short;      // Short swap value
	                        double                symbol_swap_sunday;      // Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from SUNDAY to the next day. There following values are supported:
	                        double                symbol_swap_monday;      // Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Monday to Tuesday
	                        double               symbol_swap_tuesday;      // Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Tuesday to Wednesday
	                        double             symbol_swap_wednesday;      // Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Wednesday to Thursday
	                        double              symbol_swap_thursday;      // Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Thursday to Friday
	                        double                symbol_swap_friday;      // Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Friday to Saturday
	                        double              symbol_swap_saturday;      // Swap calculation ratio (SYMBOL_SWAP_LONG or SYMBOL_SWAP_SHORT) for overnight positions rolled over from Saturday to Sunday
	                        double             symbol_margin_initial;      // Initial margin means the amount in the margin currency required for opening a position with the volume of one lot. It is used for checking a client's assets when he or she enters the market.
	                        double         symbol_margin_maintenance;      // The maintenance margin. If it is set, it sets the margin amount in the margin currency of the symbol, charged from one lot. It is used for checking a client's assets when his/her account state changes. If the maintenance margin is equal to 0, the initial margin is used.
	                        double             symbol_session_volume;      // Summary volume of current session deals
	                        double           symbol_session_turnover;      // Summary turnover of the current session
	                        double           symbol_session_interest;      // Summary open interest
	                        double  symbol_session_buy_orders_volume;      // Current volume of Buy orders
	                        double symbol_session_sell_orders_volume;      // Current volume of Sell orders
	                        double               symbol_session_open;      // Open price of the current session
	                        double              symbol_session_close;      // Close price of the current session
	                        double                 symbol_session_aw;      // Average weighted price of the current session
	                        double   symbol_session_price_settlement;      // Settlement price of the current session
	                        double    symbol_session_price_limit_min;      // Minimal price of the current session
	                        double    symbol_session_price_limit_max;      // Maximal price of the current session
	                        double              symbol_margin_hedged;      // Contract size or margin value per one lot of hedged positions (oppositely directed positions of one symbol). Two margin calculation methods are possible for hedged positions. The calculation method is defined by the broker. 
	                        double               symbol_price_change;      // Change of the current price relative to the end of the previous trading day in %
	                        double           symbol_price_volatility;      // Price volatility in %
	                        double          symbol_price_theoretical;      // Theoretical option price
	                        double                symbol_price_delta;      // Option/warrant delta shows the value the option price changes by, when the underlying asset price changes by 1
	                        double                symbol_price_theta;      // Option/warrant theta shows the number of points the option price is to lose every day due to a temporary breakup, i.e. when the expiration date approaches
	                        double                symbol_price_gamma;      // Option/warrant gamma shows the change rate of delta – how quickly or slowly the option premium changes
	                        double                 symbol_price_vega;      // Option/warrant vega shows the number of points the option price changes by when the volatility changes by 1%
	                        double                  symbol_price_rho;      // Option/warrant rho reflects the sensitivity of the theoretical option price to the interest rate changing by 1%
	                        double                symbol_price_omega;      // Option/warrant omega. Option elasticity shows a relative percentage change of the option price by the percentage change of the underlying asset price
	                        double          symbol_price_sensitivity;      // Option/warrant sensitivity shows by how many points the price of the option's underlying asset should change so that the price of the option changes by one point
	                        string                      symbol_basis;      // The underlying asset of a derivative
	                           int                                _4;
	                        string                   symbol_category;      // The name of the sector or category to which the financial symbol belongs
	                           int                                _5;
	                        string                    symbol_country;      // The country to which the financial symbol belongs
	                           int                                _6;
	                        string                symbol_sector_name;      // The sector of the economy to which the financial symbol belongs
	                           int                                _7;
	                        string              symbol_industry_name;      // The industry branch or the industry to which the financial symbol belongs
	                           int                                _8;
	                        string              symbol_currency_base;      // Basic currency of a symbol
	                           int                                _9;
	                        string            symbol_currency_profit;      // Profit currency
	                           int                               _10;
	                        string            symbol_currency_margin;      // Margin currency
	                           int                               _11;
	                        string                       symbol_bank;      // Feeder of the current quote
	                           int                               _12;
	                        string                symbol_description;      // Symbol description
	                           int                               _13;
	                        string                   symbol_exchange;      // The name of the exchange in which the financial symbol is traded
	                           int                               _14;
	                        string                    symbol_formula;      // The formula used for the custom symbol pricing. If the name of a financial symbol used in the formula starts with a digit or contains a special character (&quot;&gt;&quot; &quot;, &quot;.&quot;, &quot;-&quot;, &quot;&amp;&quot;, &quot;#&quot; and so on) quotation marks should be used around this symbol name.
	                           int                               _15;
	                        string                       symbol_isin;      // The name of a symbol in the ISIN system (International Securities Identification Number). The International Securities Identification Number is a 12-digit alphanumeric code that uniquely identifies a security. The presence of this symbol property is determined on the side of a trade server.
	                           int                               _16;
	                        string                       symbol_page;      // The address of the web page containing symbol information. This address will be displayed as a link when viewing symbol properties in the terminal
	                           int                               _17;
	                        string                       symbol_path;      // Path in the symbol tree
	                           int                               _18;
	                          bool         symbol_subscription_delay;      // Symbol data arrives with a delay. The property can be requested only for symbols selected in MarketWatch (SYMBOL_SELECT = true). The ERR_MARKET_NOT_SELECTED (4302) error is generated for other symbols
	                          bool                     symbol_custom;      // It is a custom symbol – the symbol has been created synthetically based on other symbols from the Market Watch and/or external data sources
	                          bool                      symbol_exist;      // Symbol with this name exists
	                          bool                     symbol_select;      // Symbol is selected in Market Watch
	                          bool                    symbol_visible;      // Symbol is visible in Market Watch.
	                          bool               symbol_spread_float;      // Indication of a floating spread
	                          bool      symbol_margin_hedged_use_leg;      // Calculating hedging margin using the larger leg (Buy or Sell)
};

SymbolInfoBridge InstantiateSymbolInfoBridge(string symbol) {
   SymbolInfoBridge instance;
   // set the alignment markers with easy to spot values -- 08080808 hex
   instance._1                                = 252645135;
   instance._2                                = 252645135;
   instance._4                                = 252645135;
   instance._5                                = 252645135;
   instance._6                                = 252645135;
   instance._7                                = 252645135;
   instance._8                                = 252645135;
   instance._9                                = 252645135;
   instance._10                                = 252645135;
   instance._11                                = 252645135;
   instance._12                                = 252645135;
   instance._13                                = 252645135;
   instance._14                                = 252645135;
   instance._15                                = 252645135;
   instance._16                                = 252645135;
   instance._17                                = 252645135;
   instance._18                                = 252645135;
	instance.symbol_subscription_delay         = SymbolInfoInteger(symbol, SYMBOL_SUBSCRIPTION_DELAY);
	instance.symbol_sector                     = SymbolInfoInteger(symbol, SYMBOL_SECTOR);
	instance.symbol_industry                   = SymbolInfoInteger(symbol, SYMBOL_INDUSTRY);
	instance.symbol_custom                     = SymbolInfoInteger(symbol, SYMBOL_CUSTOM);
	instance.symbol_background_color           = SymbolInfoInteger(symbol, SYMBOL_BACKGROUND_COLOR);
	instance.symbol_chart_mode                 = SymbolInfoInteger(symbol, SYMBOL_CHART_MODE);
	instance.symbol_exist                      = SymbolInfoInteger(symbol, SYMBOL_EXIST);
	instance.symbol_select                     = SymbolInfoInteger(symbol, SYMBOL_SELECT);
	instance.symbol_visible                    = SymbolInfoInteger(symbol, SYMBOL_VISIBLE);
	instance.symbol_session_deals              = SymbolInfoInteger(symbol, SYMBOL_SESSION_DEALS);
	instance.symbol_session_buy_orders         = SymbolInfoInteger(symbol, SYMBOL_SESSION_BUY_ORDERS);
	instance.symbol_session_sell_orders        = SymbolInfoInteger(symbol, SYMBOL_SESSION_SELL_ORDERS);
	instance.symbol_volume                     = SymbolInfoInteger(symbol, SYMBOL_VOLUME);
	instance.symbol_volumehigh                 = SymbolInfoInteger(symbol, SYMBOL_VOLUMEHIGH);
	instance.symbol_volumelow                  = SymbolInfoInteger(symbol, SYMBOL_VOLUMELOW);
	instance.symbol_time                       = SymbolInfoInteger(symbol, SYMBOL_TIME);
	instance.symbol_time_msc                   = SymbolInfoInteger(symbol, SYMBOL_TIME_MSC);
	instance.symbol_digits                     = SymbolInfoInteger(symbol, SYMBOL_DIGITS);
	instance.symbol_spread_float               = SymbolInfoInteger(symbol, SYMBOL_SPREAD_FLOAT);
	instance.symbol_spread                     = SymbolInfoInteger(symbol, SYMBOL_SPREAD);
	instance.symbol_ticks_bookdepth            = SymbolInfoInteger(symbol, SYMBOL_TICKS_BOOKDEPTH);
	instance.symbol_trade_calc_mode            = SymbolInfoInteger(symbol, SYMBOL_TRADE_CALC_MODE);
	instance.symbol_trade_mode                 = SymbolInfoInteger(symbol, SYMBOL_TRADE_MODE);
	instance.symbol_start_time                 = SymbolInfoInteger(symbol, SYMBOL_START_TIME);
	instance.symbol_expiration_time            = SymbolInfoInteger(symbol, SYMBOL_EXPIRATION_TIME);
	instance.symbol_trade_stops_level          = SymbolInfoInteger(symbol, SYMBOL_TRADE_STOPS_LEVEL);
	instance.symbol_trade_freeze_level         = SymbolInfoInteger(symbol, SYMBOL_TRADE_FREEZE_LEVEL);
	instance.symbol_trade_exemode              = SymbolInfoInteger(symbol, SYMBOL_TRADE_EXEMODE);
	instance.symbol_swap_mode                  = SymbolInfoInteger(symbol, SYMBOL_SWAP_MODE);
	instance.symbol_swap_rollover3days         = SymbolInfoInteger(symbol, SYMBOL_SWAP_ROLLOVER3DAYS);
	instance.symbol_margin_hedged_use_leg      = SymbolInfoInteger(symbol, SYMBOL_MARGIN_HEDGED_USE_LEG);
	instance.symbol_expiration_mode            = SymbolInfoInteger(symbol, SYMBOL_EXPIRATION_MODE );
	instance.symbol_filling_mode               = SymbolInfoInteger(symbol, SYMBOL_FILLING_MODE);
	instance.symbol_order_mode                 = SymbolInfoInteger(symbol, SYMBOL_ORDER_MODE);
	instance.symbol_order_gtc_mode             = SymbolInfoInteger(symbol, SYMBOL_ORDER_GTC_MODE);
	instance.symbol_option_mode                = SymbolInfoInteger(symbol, SYMBOL_OPTION_MODE);
	instance.symbol_option_right               = SymbolInfoInteger(symbol, SYMBOL_OPTION_RIGHT);
	instance.symbol_bid                        = SymbolInfoDouble(symbol, SYMBOL_BID);
	instance.symbol_bidhigh                    = SymbolInfoDouble(symbol, SYMBOL_BIDHIGH);
	instance.symbol_bidlow                     = SymbolInfoDouble(symbol, SYMBOL_BIDLOW);
	instance.symbol_ask                        = SymbolInfoDouble(symbol, SYMBOL_ASK);
	instance.symbol_askhigh                    = SymbolInfoDouble(symbol, SYMBOL_ASKHIGH);
	instance.symbol_asklow                     = SymbolInfoDouble(symbol, SYMBOL_ASKLOW);
	instance.symbol_last                       = SymbolInfoDouble(symbol, SYMBOL_LAST);
	instance.symbol_lasthigh                   = SymbolInfoDouble(symbol, SYMBOL_LASTHIGH);
	instance.symbol_lastlow                    = SymbolInfoDouble(symbol, SYMBOL_LASTLOW);
	instance.symbol_volume_real                = SymbolInfoDouble(symbol, SYMBOL_VOLUME_REAL);
	instance.symbol_volumehigh_real            = SymbolInfoDouble(symbol, SYMBOL_VOLUMEHIGH_REAL);
	instance.symbol_volumelow_real             = SymbolInfoDouble(symbol, SYMBOL_VOLUMELOW_REAL);
	instance.symbol_option_strike              = SymbolInfoDouble(symbol, SYMBOL_OPTION_STRIKE);
	instance.symbol_point                      = SymbolInfoDouble(symbol, SYMBOL_POINT);
	instance.symbol_trade_tick_value           = SymbolInfoDouble(symbol, SYMBOL_TRADE_TICK_VALUE);
	instance.symbol_trade_tick_value_profit    = SymbolInfoDouble(symbol, SYMBOL_TRADE_TICK_VALUE_PROFIT);
	instance.symbol_trade_tick_value_loss      = SymbolInfoDouble(symbol, SYMBOL_TRADE_TICK_VALUE_LOSS);
	instance.symbol_trade_tick_size            = SymbolInfoDouble(symbol, SYMBOL_TRADE_TICK_SIZE);
	instance.symbol_trade_contract_size        = SymbolInfoDouble(symbol, SYMBOL_TRADE_CONTRACT_SIZE);
	instance.symbol_trade_accrued_interest     = SymbolInfoDouble(symbol, SYMBOL_TRADE_ACCRUED_INTEREST);
	instance.symbol_trade_face_value           = SymbolInfoDouble(symbol, SYMBOL_TRADE_FACE_VALUE);
	instance.symbol_trade_liquidity_rate       = SymbolInfoDouble(symbol, SYMBOL_TRADE_LIQUIDITY_RATE);
	instance.symbol_volume_min                 = SymbolInfoDouble(symbol, SYMBOL_VOLUME_MIN);
	instance.symbol_volume_max                 = SymbolInfoDouble(symbol, SYMBOL_VOLUME_MAX);
	instance.symbol_volume_step                = SymbolInfoDouble(symbol, SYMBOL_VOLUME_STEP);
	instance.symbol_volume_limit               = SymbolInfoDouble(symbol, SYMBOL_VOLUME_LIMIT);
	instance.symbol_swap_long                  = SymbolInfoDouble(symbol, SYMBOL_SWAP_LONG);
	instance.symbol_swap_short                 = SymbolInfoDouble(symbol, SYMBOL_SWAP_SHORT);
	instance.symbol_swap_sunday                = SymbolInfoDouble(symbol, SYMBOL_SWAP_SUNDAY);
	instance.symbol_swap_monday                = SymbolInfoDouble(symbol, SYMBOL_SWAP_MONDAY);
	instance.symbol_swap_tuesday               = SymbolInfoDouble(symbol, SYMBOL_SWAP_TUESDAY);
	instance.symbol_swap_wednesday             = SymbolInfoDouble(symbol, SYMBOL_SWAP_WEDNESDAY);
	instance.symbol_swap_thursday              = SymbolInfoDouble(symbol, SYMBOL_SWAP_THURSDAY);
	instance.symbol_swap_friday                = SymbolInfoDouble(symbol, SYMBOL_SWAP_FRIDAY);
	instance.symbol_swap_saturday              = SymbolInfoDouble(symbol, SYMBOL_SWAP_SATURDAY);
	instance.symbol_margin_initial             = SymbolInfoDouble(symbol, SYMBOL_MARGIN_INITIAL);
	instance.symbol_margin_maintenance         = SymbolInfoDouble(symbol, SYMBOL_MARGIN_MAINTENANCE);
	instance.symbol_session_volume             = SymbolInfoDouble(symbol, SYMBOL_SESSION_VOLUME);
	instance.symbol_session_turnover           = SymbolInfoDouble(symbol, SYMBOL_SESSION_TURNOVER);
	instance.symbol_session_interest           = SymbolInfoDouble(symbol, SYMBOL_SESSION_INTEREST);
	instance.symbol_session_buy_orders_volume  = SymbolInfoDouble(symbol, SYMBOL_SESSION_BUY_ORDERS_VOLUME);
	instance.symbol_session_sell_orders_volume = SymbolInfoDouble(symbol, SYMBOL_SESSION_SELL_ORDERS_VOLUME);
	instance.symbol_session_open               = SymbolInfoDouble(symbol, SYMBOL_SESSION_OPEN);
	instance.symbol_session_close              = SymbolInfoDouble(symbol, SYMBOL_SESSION_CLOSE);
	instance.symbol_session_aw                 = SymbolInfoDouble(symbol, SYMBOL_SESSION_AW);
	instance.symbol_session_price_settlement   = SymbolInfoDouble(symbol, SYMBOL_SESSION_PRICE_SETTLEMENT);
	instance.symbol_session_price_limit_min    = SymbolInfoDouble(symbol, SYMBOL_SESSION_PRICE_LIMIT_MIN);
	instance.symbol_session_price_limit_max    = SymbolInfoDouble(symbol, SYMBOL_SESSION_PRICE_LIMIT_MAX);
	instance.symbol_margin_hedged              = SymbolInfoDouble(symbol, SYMBOL_MARGIN_HEDGED);
	instance.symbol_price_change               = SymbolInfoDouble(symbol, SYMBOL_PRICE_CHANGE);
	instance.symbol_price_volatility           = SymbolInfoDouble(symbol, SYMBOL_PRICE_VOLATILITY);
	instance.symbol_price_theoretical          = SymbolInfoDouble(symbol, SYMBOL_PRICE_THEORETICAL);
	instance.symbol_price_delta                = SymbolInfoDouble(symbol, SYMBOL_PRICE_DELTA);
	instance.symbol_price_theta                = SymbolInfoDouble(symbol, SYMBOL_PRICE_THETA);
	instance.symbol_price_gamma                = SymbolInfoDouble(symbol, SYMBOL_PRICE_GAMMA);
	instance.symbol_price_vega                 = SymbolInfoDouble(symbol, SYMBOL_PRICE_VEGA);
	instance.symbol_price_rho                  = SymbolInfoDouble(symbol, SYMBOL_PRICE_RHO);
	instance.symbol_price_omega                = SymbolInfoDouble(symbol, SYMBOL_PRICE_OMEGA);
	instance.symbol_price_sensitivity          = SymbolInfoDouble(symbol, SYMBOL_PRICE_SENSITIVITY);
	instance.symbol_basis                      = SymbolInfoString(symbol, SYMBOL_BASIS);
	instance.symbol_category                   = SymbolInfoString(symbol, SYMBOL_CATEGORY);
	instance.symbol_country                    = SymbolInfoString(symbol, SYMBOL_COUNTRY);
	instance.symbol_sector_name                = SymbolInfoString(symbol, SYMBOL_SECTOR_NAME);
	instance.symbol_industry_name              = SymbolInfoString(symbol, SYMBOL_INDUSTRY_NAME);
	instance.symbol_currency_base              = SymbolInfoString(symbol, SYMBOL_CURRENCY_BASE);
	instance.symbol_currency_profit            = SymbolInfoString(symbol, SYMBOL_CURRENCY_PROFIT);
	instance.symbol_currency_margin            = SymbolInfoString(symbol, SYMBOL_CURRENCY_MARGIN);
	instance.symbol_bank                       = SymbolInfoString(symbol, SYMBOL_BANK);
	instance.symbol_description                = SymbolInfoString(symbol, SYMBOL_DESCRIPTION);
	instance.symbol_exchange                   = SymbolInfoString(symbol, SYMBOL_EXCHANGE);
	instance.symbol_formula                    = SymbolInfoString(symbol, SYMBOL_FORMULA);
	instance.symbol_isin                       = SymbolInfoString(symbol, SYMBOL_ISIN);
	instance.symbol_page                       = SymbolInfoString(symbol, SYMBOL_PAGE);
	instance.symbol_path                       = SymbolInfoString(symbol, SYMBOL_PATH);

        Print("symbol_subscription_delay = " + instance.symbol_subscription_delay);
        Print("symbol_sector = " + instance.symbol_sector);
        Print("symbol_industry = " + instance.symbol_industry);
        Print("symbol_custom = " + instance.symbol_custom);
        Print("symbol_background_color = " + instance.symbol_background_color);
        Print("symbol_chart_mode = " + instance.symbol_chart_mode);
        Print("symbol_exist = " + instance.symbol_exist);
        Print("symbol_select = " + instance.symbol_select);
        Print("symbol_visible = " + instance.symbol_visible);
        Print("symbol_session_deals = " + instance.symbol_session_deals);
        Print("symbol_session_buy_orders = " + instance.symbol_session_buy_orders);
        Print("symbol_session_sell_orders = " + instance.symbol_session_sell_orders);
        Print("symbol_volume = " + instance.symbol_volume);
        Print("symbol_volumehigh = " + instance.symbol_volumehigh);
        Print("symbol_volumelow = " + instance.symbol_volumelow);
        Print("symbol_time = " + instance.symbol_time);
        Print("symbol_time_msc = " + instance.symbol_time_msc);
        Print("symbol_digits = " + instance.symbol_digits);
        Print("symbol_spread_float = " + instance.symbol_spread_float);
        Print("symbol_spread = " + instance.symbol_spread);
        Print("symbol_ticks_bookdepth = " + instance.symbol_ticks_bookdepth);
        Print("symbol_trade_calc_mode = " + instance.symbol_trade_calc_mode);
        Print("symbol_trade_mode = " + instance.symbol_trade_mode);
        Print("symbol_start_time = " + instance.symbol_start_time);
        Print("symbol_expiration_time = " + instance.symbol_expiration_time);
        Print("symbol_trade_stops_level = " + instance.symbol_trade_stops_level);
        Print("symbol_trade_freeze_level = " + instance.symbol_trade_freeze_level);
        Print("symbol_trade_exemode = " + instance.symbol_trade_exemode);
        Print("symbol_swap_mode = " + instance.symbol_swap_mode);
        Print("symbol_swap_rollover3days = " + instance.symbol_swap_rollover3days);
        Print("symbol_margin_hedged_use_leg = " + instance.symbol_margin_hedged_use_leg);
        Print("symbol_expiration_mode = " + instance.symbol_expiration_mode);
        Print("symbol_filling_mode = " + instance.symbol_filling_mode);
        Print("symbol_order_mode = " + instance.symbol_order_mode);
        Print("symbol_order_gtc_mode = " + instance.symbol_order_gtc_mode);
        Print("symbol_option_mode = " + instance.symbol_option_mode);
        Print("symbol_option_right = " + instance.symbol_option_right);
        Print("symbol_bid = " + instance.symbol_bid);
        Print("symbol_bidhigh = " + instance.symbol_bidhigh);
        Print("symbol_bidlow = " + instance.symbol_bidlow);
        Print("symbol_ask = " + instance.symbol_ask);
        Print("symbol_askhigh = " + instance.symbol_askhigh);
        Print("symbol_asklow = " + instance.symbol_asklow);
        Print("symbol_last = " + instance.symbol_last);
        Print("symbol_lasthigh = " + instance.symbol_lasthigh);
        Print("symbol_lastlow = " + instance.symbol_lastlow);
        Print("symbol_volume_real = " + instance.symbol_volume_real);
        Print("symbol_volumehigh_real = " + instance.symbol_volumehigh_real);
        Print("symbol_volumelow_real = " + instance.symbol_volumelow_real);
        Print("symbol_option_strike = " + instance.symbol_option_strike);
        Print("symbol_point = " + instance.symbol_point);
        Print("symbol_trade_tick_value = " + instance.symbol_trade_tick_value);
        Print("symbol_trade_tick_value_profit = " + instance.symbol_trade_tick_value_profit);
        Print("symbol_trade_tick_value_loss = " + instance.symbol_trade_tick_value_loss);
        Print("symbol_trade_tick_size = " + instance.symbol_trade_tick_size);
        Print("symbol_trade_contract_size = " + instance.symbol_trade_contract_size);
        Print("symbol_trade_accrued_interest = " + instance.symbol_trade_accrued_interest);
        Print("symbol_trade_face_value = " + instance.symbol_trade_face_value);
        Print("symbol_trade_liquidity_rate = " + instance.symbol_trade_liquidity_rate);
        Print("symbol_volume_min = " + instance.symbol_volume_min);
        Print("symbol_volume_max = " + instance.symbol_volume_max);
        Print("symbol_volume_step = " + instance.symbol_volume_step);
        Print("symbol_volume_limit = " + instance.symbol_volume_limit);
        Print("symbol_swap_long = " + instance.symbol_swap_long);
        Print("symbol_swap_short = " + instance.symbol_swap_short);
        Print("symbol_swap_sunday = " + instance.symbol_swap_sunday);
        Print("symbol_swap_monday = " + instance.symbol_swap_monday);
        Print("symbol_swap_tuesday = " + instance.symbol_swap_tuesday);
        Print("symbol_swap_wednesday = " + instance.symbol_swap_wednesday);
        Print("symbol_swap_thursday = " + instance.symbol_swap_thursday);
        Print("symbol_swap_friday = " + instance.symbol_swap_friday);
        Print("symbol_swap_saturday = " + instance.symbol_swap_saturday);
        Print("symbol_margin_initial = " + instance.symbol_margin_initial);
        Print("symbol_margin_maintenance = " + instance.symbol_margin_maintenance);
        Print("symbol_session_volume = " + instance.symbol_session_volume);
        Print("symbol_session_turnover = " + instance.symbol_session_turnover);
        Print("symbol_session_interest = " + instance.symbol_session_interest);
        Print("symbol_session_buy_orders_volume = " + instance.symbol_session_buy_orders_volume);
        Print("symbol_session_sell_orders_volume = " + instance.symbol_session_sell_orders_volume);
        Print("symbol_session_open = " + instance.symbol_session_open);
        Print("symbol_session_close = " + instance.symbol_session_close);
        Print("symbol_session_aw = " + instance.symbol_session_aw);
        Print("symbol_session_price_settlement = " + instance.symbol_session_price_settlement);
        Print("symbol_session_price_limit_min = " + instance.symbol_session_price_limit_min);
        Print("symbol_session_price_limit_max = " + instance.symbol_session_price_limit_max);
        Print("symbol_margin_hedged = " + instance.symbol_margin_hedged);
        Print("symbol_price_change = " + instance.symbol_price_change);
        Print("symbol_price_volatility = " + instance.symbol_price_volatility);
        Print("symbol_price_theoretical = " + instance.symbol_price_theoretical);
        Print("symbol_price_delta = " + instance.symbol_price_delta);
        Print("symbol_price_theta = " + instance.symbol_price_theta);
        Print("symbol_price_gamma = " + instance.symbol_price_gamma);
        Print("symbol_price_vega = " + instance.symbol_price_vega);
        Print("symbol_price_rho = " + instance.symbol_price_rho);
        Print("symbol_price_omega = " + instance.symbol_price_omega);
        Print("symbol_price_sensitivity = " + instance.symbol_price_sensitivity);
        Print("symbol_basis = " + instance.symbol_basis);
        Print("symbol_category = " + instance.symbol_category);
        Print("symbol_country = " + instance.symbol_country);
        Print("symbol_sector_name = " + instance.symbol_sector_name);
        Print("symbol_industry_name = " + instance.symbol_industry_name);
        Print("symbol_currency_base = " + instance.symbol_currency_base);
        Print("symbol_currency_profit = " + instance.symbol_currency_profit);
        Print("symbol_currency_margin = " + instance.symbol_currency_margin);
        Print("symbol_bank = " + instance.symbol_bank);
        Print("symbol_description = " + instance.symbol_description);
        Print("symbol_exchange = " + instance.symbol_exchange);
        Print("symbol_formula = " + instance.symbol_formula);
        Print("symbol_isin = " + instance.symbol_isin);
        Print("symbol_page = " + instance.symbol_page);
        Print("symbol_path = " + instance.symbol_path);
        
	return instance;
}

