// Tests our DLL through special feedback functions exposed for that very purpose

#property copyright "Copyright OgreRobot.com"
#property link      "https://www.OgreRobot.com"
#property version   "1.00"
#property strict

#include "RustDll.mqh"


void OnStart() {

    string test_name;
    string expected;
    string observed;
    StringReserve(observed, 16384);  // this will contain the Rust ==> MT5 test results. The `string` should have enough capacity, since it cannot be allocated on the Rust side
                                     // -- unfortunately, if not enough size is provided, memory corruption will happen, since Rust don't receive the buffer size when strings
                                     //    are passed as function parameters (just the pointer to the buffer is passed)

    // Makes all MQL Enum variant values known to Rust, so they may be converted properly (MQL Variants are not ordered nor sequential, unfortunately)
    #include "EnumReporter.mqh"
    // check that all went fine
    if (has_fatal_error(-1, observed)) {
        Print("I'D QUIT NOW (BUT I WON'T) DUE TO DLL ERROR: " + observed);
    }

    test_name = "Testing MqlTick serialization...";
    MqlTick mql_tick;
    mql_tick.time        = 12344321;
    mql_tick.bid         = 97.58;
    mql_tick.ask         = 11.75;
    mql_tick.last        = 11.71;   
    mql_tick.volume      = 9814989;
    mql_tick.time_msc    = 4321001;
    mql_tick.flags       = 82;
    mql_tick.volume_real = 3.14159;
    expected = "Mq5MqlTick { time: 12344321, bid: 97.58, ask: 11.75, last: 11.71, volume: 9814989, time_msc: 4321001, flags: 82, volume_real: 3.14159 }";
    test_on_tick(observed, mql_tick);
    assert(observed, expected, test_name);

    Print("MqlTick constants -- assure those values are exposed to Rust:");
    Print("// flags for `Mt5MqlTick::flags`");
    Print("////////////////////////////////");
    Print("// from https://www.mql5.com/en/docs/constants/structures/mqltick");
    expose_constant("TICK_FLAG_BID",    TICK_FLAG_BID,    "tick has changed a Bid price");
    expose_constant("TICK_FLAG_ASK",    TICK_FLAG_ASK,    "a tick has changed an Ask price");
    expose_constant("TICK_FLAG_LAST",   TICK_FLAG_LAST,   "a tick has changed the last deal price");
    expose_constant("TICK_FLAG_VOLUME", TICK_FLAG_VOLUME, "a tick has changed a volume");
    expose_constant("TICK_FLAG_BUY",    TICK_FLAG_BUY,    "a tick is a result of a buy deal");
    expose_constant("TICK_FLAG_SELL",   TICK_FLAG_SELL,   "a tick is a result of a sell deal");


    test_name = "Testing SymbolInfoBridge serialization...";
    SymbolInfoBridge symbol_info_bridge;
    symbol_info_bridge.symbol_sector = SECTOR_HEALTHCARE;
    symbol_info_bridge.symbol_industry = INDUSTRY_ADVERTISING;
    symbol_info_bridge.symbol_background_color = 8421504;   // #808080
    symbol_info_bridge.symbol_chart_mode = SYMBOL_CHART_MODE_BID;
    symbol_info_bridge.symbol_session_deals = 547;
    symbol_info_bridge.symbol_session_buy_orders = 745;
    symbol_info_bridge.symbol_session_sell_orders = 574;
    symbol_info_bridge.symbol_volume = 112233;
    symbol_info_bridge.symbol_volumehigh = 332211;
    symbol_info_bridge.symbol_volumelow = 111111;
    symbol_info_bridge.symbol_time = 987654321;
    symbol_info_bridge.symbol_time_msc = 987654321001;
    symbol_info_bridge.symbol_digits = 3;
    symbol_info_bridge.symbol_spread = 47;
    symbol_info_bridge.symbol_ticks_bookdepth = 74;
    symbol_info_bridge.symbol_trade_calc_mode = SYMBOL_CALC_MODE_EXCH_FUTURES;
    symbol_info_bridge.symbol_trade_mode = SYMBOL_TRADE_MODE_CLOSEONLY;
    symbol_info_bridge.symbol_start_time = 7447;
    symbol_info_bridge.symbol_expiration_time = 32777;
    symbol_info_bridge.symbol_trade_stops_level = 9182;
    symbol_info_bridge.symbol_trade_freeze_level = 1928;
    symbol_info_bridge.symbol_trade_exemode = SYMBOL_TRADE_EXECUTION_INSTANT;
    symbol_info_bridge.symbol_swap_mode = SYMBOL_SWAP_MODE_INTEREST_CURRENT;
    symbol_info_bridge.symbol_swap_rollover3days = FRIDAY;
    symbol_info_bridge.symbol_expiration_mode  = 1982;
    symbol_info_bridge.symbol_filling_mode = 1289;
    symbol_info_bridge.symbol_order_mode = 9821;
    symbol_info_bridge.symbol_order_gtc_mode = SYMBOL_ORDERS_DAILY_EXCLUDING_STOPS;
    symbol_info_bridge.symbol_option_mode = SYMBOL_OPTION_MODE_AMERICAN;
    symbol_info_bridge.symbol_option_right = SYMBOL_OPTION_RIGHT_PUT;
    symbol_info_bridge.symbol_bid = 3.14159;
    symbol_info_bridge.symbol_bidhigh = 3.14160;
    symbol_info_bridge.symbol_bidlow = 3.14161;
    symbol_info_bridge.symbol_ask = 3.14162;
    symbol_info_bridge.symbol_askhigh = 3.14163;
    symbol_info_bridge.symbol_asklow = 3.14164;
    symbol_info_bridge.symbol_last = 3.14165;
    symbol_info_bridge.symbol_lasthigh = 3.14166;
    symbol_info_bridge.symbol_lastlow = 3.14167;
    symbol_info_bridge.symbol_volume_real = 3.14168;
    symbol_info_bridge.symbol_volumehigh_real = 3.14169;
    symbol_info_bridge.symbol_volumelow_real = 3.14170;
    symbol_info_bridge.symbol_option_strike = 3.14171;
    symbol_info_bridge.symbol_point = 3.14172;
    symbol_info_bridge.symbol_trade_tick_value = 3.14173;
    symbol_info_bridge.symbol_trade_tick_value_profit = 3.14174;
    symbol_info_bridge.symbol_trade_tick_value_loss = 3.14175;
    symbol_info_bridge.symbol_trade_tick_size = 3.14176;
    symbol_info_bridge.symbol_trade_contract_size = 3.14177;
    symbol_info_bridge.symbol_trade_accrued_interest = 3.14178;
    symbol_info_bridge.symbol_trade_face_value = 3.14179;
    symbol_info_bridge.symbol_trade_liquidity_rate = 3.14180;
    symbol_info_bridge.symbol_volume_min = 3.14181;
    symbol_info_bridge.symbol_volume_max = 3.14182;
    symbol_info_bridge.symbol_volume_step = 3.14183;
    symbol_info_bridge.symbol_volume_limit = 3.14184;
    symbol_info_bridge.symbol_swap_long = 3.14185;
    symbol_info_bridge.symbol_swap_short = 3.14186;
    symbol_info_bridge.symbol_swap_sunday = 3.14187;
    symbol_info_bridge.symbol_swap_monday = 3.14188;
    symbol_info_bridge.symbol_swap_tuesday = 3.14189;
    symbol_info_bridge.symbol_swap_wednesday = 3.14190;
    symbol_info_bridge.symbol_swap_thursday = 3.14191;
    symbol_info_bridge.symbol_swap_friday = 3.14192;
    symbol_info_bridge.symbol_swap_saturday = 3.14193;
    symbol_info_bridge.symbol_margin_initial = 3.14194;
    symbol_info_bridge.symbol_margin_maintenance = 3.14195;
    symbol_info_bridge.symbol_session_volume = 3.14196;
    symbol_info_bridge.symbol_session_turnover = 3.14197;
    symbol_info_bridge.symbol_session_interest = 3.14198;
    symbol_info_bridge.symbol_session_buy_orders_volume = 3.14199;
    symbol_info_bridge.symbol_session_sell_orders_volume = 3.14200;
    symbol_info_bridge.symbol_session_open = 3.14201;
    symbol_info_bridge.symbol_session_close = 3.14202;
    symbol_info_bridge.symbol_session_aw = 3.14203;
    symbol_info_bridge.symbol_session_price_settlement = 3.14204;
    symbol_info_bridge.symbol_session_price_limit_min = 3.14205;
    symbol_info_bridge.symbol_session_price_limit_max = 3.14206;
    symbol_info_bridge.symbol_margin_hedged = 3.14207;
    symbol_info_bridge.symbol_price_change = 3.14208;
    symbol_info_bridge.symbol_price_volatility = 3.14209;
    symbol_info_bridge.symbol_price_theoretical = 3.14210;
    symbol_info_bridge.symbol_price_delta = 3.14211;
    symbol_info_bridge.symbol_price_theta = 3.14212;
    symbol_info_bridge.symbol_price_gamma = 3.14213;
    symbol_info_bridge.symbol_price_vega = 3.14214;
    symbol_info_bridge.symbol_price_rho = 3.14215;
    symbol_info_bridge.symbol_price_omega = 3.14216;
    symbol_info_bridge.symbol_price_sensitivity = 3.14217;
    symbol_info_bridge.symbol_basis = "a";
    symbol_info_bridge.symbol_category = "b";
    symbol_info_bridge.symbol_country = "c";
    symbol_info_bridge.symbol_sector_name = "d";
    symbol_info_bridge.symbol_industry_name = "e";
    symbol_info_bridge.symbol_currency_base = "f";
    symbol_info_bridge.symbol_currency_profit = "g";
    symbol_info_bridge.symbol_currency_margin = "h";
    symbol_info_bridge.symbol_bank = "i";
    symbol_info_bridge.symbol_description = "j";
    symbol_info_bridge.symbol_exchange = "k";
    symbol_info_bridge.symbol_formula = "l";
    symbol_info_bridge.symbol_isin = "m";
    symbol_info_bridge.symbol_page = "n";
    symbol_info_bridge.symbol_path = "o";
    symbol_info_bridge.symbol_subscription_delay = true;
    symbol_info_bridge.symbol_custom = false;
    symbol_info_bridge.symbol_exist = true;
    symbol_info_bridge.symbol_select = false;
    symbol_info_bridge.symbol_visible = true;
    symbol_info_bridge.symbol_spread_float = false;
    symbol_info_bridge.symbol_margin_hedged_use_leg = true;
    expected = "SymbolInfoRust { " +
    "symbol_sector: SectorHealthcare, " +
    "symbol_industry: IndustryAdvertising, " +
    "symbol_background_color: (128, 128, 128), " +
    "symbol_chart_mode: SymbolChartModeBid, " +
    "symbol_session_deals: 547, " +
    "symbol_session_buy_orders: 745, " +
    "symbol_session_sell_orders: 574, " +
    "symbol_volume: 112233, " +
    "symbol_volumehigh: 332211, " +
    "symbol_volumelow: 111111, " +
    "symbol_time: 2001-04-19T04:25:21.001, " +
    "symbol_digits: 3, " +
    "symbol_spread: 47, " +
    "symbol_ticks_bookdepth: 74, " +
    "symbol_trade_calc_mode: SymbolCalcModeExchFutures, " +
    "symbol_trade_mode: SymbolTradeModeCloseonly, " +
    "symbol_start_time: 1970-01-01T02:04:07, " +
    "symbol_expiration_time: 1970-01-01T09:06:17, " +
    "symbol_trade_stops_level: 9182, " +
    "symbol_trade_freeze_level: 1928, " +
    "symbol_trade_exemode: SymbolTradeExecutionInstant, " +
    "symbol_swap_mode: SymbolSwapModeInterestCurrent, " +
    "symbol_swap_rollover3days: Friday, " +
    "symbol_expiration_mode: 1982, " +
    "symbol_filling_mode: 1289, " +
    "symbol_order_mode: 9821, " +
    "symbol_order_gtc_mode: SymbolOrdersDailyExcludingStops, " +
    "symbol_option_mode: SymbolOptionModeAmerican, " +
    "symbol_option_right: SymbolOptionRightPut, " +
    "symbol_bid: 3.14159, " +
    "symbol_bidhigh: 3.1416, " +
    "symbol_bidlow: 3.14161, " +
    "symbol_ask: 3.14162, " +
    "symbol_askhigh: 3.14163, " +
    "symbol_asklow: 3.14164, " +
    "symbol_last: 3.14165, " +
    "symbol_lasthigh: 3.14166, " +
    "symbol_lastlow: 3.14167, " +
    "symbol_volume_real: 3.14168, " +
    "symbol_volumehigh_real: 3.14169, " +
    "symbol_volumelow_real: 3.1417, " +
    "symbol_option_strike: 3.14171, " +
    "symbol_point: 3.14172, " +
    "symbol_trade_tick_value: 3.14173, " +
    "symbol_trade_tick_value_profit: 3.14174, " +
    "symbol_trade_tick_value_loss: 3.14175, " +
    "symbol_trade_tick_size: 3.14176, " +
    "symbol_trade_contract_size: 3.14177, " +
    "symbol_trade_accrued_interest: 3.14178, " +
    "symbol_trade_face_value: 3.14179, " +
    "symbol_trade_liquidity_rate: 3.1418, " +
    "symbol_volume_min: 3.14181, " +
    "symbol_volume_max: 3.14182, " +
    "symbol_volume_step: 3.14183, " +
    "symbol_volume_limit: 3.14184, " +
    "symbol_swap_long: 3.14185, " +
    "symbol_swap_short: 3.14186, " +
    "symbol_swap_sunday: 3.14187, " +
    "symbol_swap_monday: 3.14188, " +
    "symbol_swap_tuesday: 3.14189, " +
    "symbol_swap_wednesday: 3.1419, " +
    "symbol_swap_thursday: 3.14191, " +
    "symbol_swap_friday: 3.14192, " +
    "symbol_swap_saturday: 3.14193, " +
    "symbol_margin_initial: 3.14194, " +
    "symbol_margin_maintenance: 3.14195, " +
    "symbol_session_volume: 3.14196, " +
    "symbol_session_turnover: 3.14197, " +
    "symbol_session_interest: 3.14198, " +
    "symbol_session_buy_orders_volume: 3.14199, " +
    "symbol_session_sell_orders_volume: 3.142, " +
    "symbol_session_open: 3.14201, " +
    "symbol_session_close: 3.14202, " +
    "symbol_session_aw: 3.14203, " +
    "symbol_session_price_settlement: 3.14204, " +
    "symbol_session_price_limit_min: 3.14205, " +
    "symbol_session_price_limit_max: 3.14206, " +
    "symbol_margin_hedged: 3.14207, " +
    "symbol_price_change: 3.14208, " +
    "symbol_price_volatility: 3.14209, " +
    "symbol_price_theoretical: 3.1421, " +
    "symbol_price_delta: 3.14211, " +
    "symbol_price_theta: 3.14212, " +
    "symbol_price_gamma: 3.14213, " +
    "symbol_price_vega: 3.14214, " +
    "symbol_price_rho: 3.14215, " +
    "symbol_price_omega: 3.14216, " +
    "symbol_price_sensitivity: 3.14217, " +
    "symbol_basis: \"a\", " +
    "symbol_category: \"b\", " +
    "symbol_country: \"c\", " +
    "symbol_sector_name: \"d\", " +
    "symbol_industry_name: \"e\", " +
    "symbol_currency_base: \"f\", " +
    "symbol_currency_profit: \"g\", " +
    "symbol_currency_margin: \"h\", " +
    "symbol_bank: \"i\", " +
    "symbol_description: \"j\", " +
    "symbol_exchange: \"k\", " +
    "symbol_formula: \"l\", " +
    "symbol_isin: \"m\", " +
    "symbol_page: \"n\", " +
    "symbol_path: \"o\", " +
    "symbol_subscription_delay: true, " +
    "symbol_custom: false, " +
    "symbol_exist: true, " +
    "symbol_select: false, " +
    "symbol_visible: true, " +
    "symbol_spread_float: false, " +
    "symbol_margin_hedged_use_leg: true }";
    test_report_symbol_info(observed, symbol_info_bridge);
    assert(observed, expected, test_name);


    test_name = "Testing AccountInfoBridge serialization...";
    AccountInfoBridge account_info_bridge;
    account_info_bridge.account_balance = 1.1;
    account_info_bridge.account_credit = 2.2;
    account_info_bridge.account_profit = 3.3;
    account_info_bridge.account_equity = 4.4;
    account_info_bridge.account_margin = 5.5;
    account_info_bridge.account_margin_free = 6.6;
    account_info_bridge.account_margin_level = 7.7;
    account_info_bridge.account_margin_so_call = 8.8;
    account_info_bridge.account_margin_so_so = 9.9;
    account_info_bridge.account_margin_initial = 10.10;
    account_info_bridge.account_margin_maintenance = 11.11;
    account_info_bridge.account_assets = 12.12;
    account_info_bridge.account_liabilities = 13.13;
    account_info_bridge.account_commission_blocked = 14.14;
    account_info_bridge.account_login = 15.15;
    account_info_bridge.account_leverage = 16.16;
    account_info_bridge.account_name = "17.17";
    account_info_bridge.account_server = "18.18";
    account_info_bridge.account_currency = "19.19";
    account_info_bridge.account_company = "20.20";
    account_info_bridge.account_trade_mode = ACCOUNT_TRADE_MODE_CONTEST;
    account_info_bridge.account_limit_orders = 22;
    account_info_bridge.account_margin_so_mode = ACCOUNT_STOPOUT_MODE_MONEY;
    account_info_bridge.account_margin_mode = ACCOUNT_MARGIN_MODE_RETAIL_HEDGING;
    account_info_bridge.account_currency_digits = 25;
    account_info_bridge.account_trade_allowed = false;
    account_info_bridge.account_trade_expert = true;
    account_info_bridge.account_fifo_close = false;
    account_info_bridge.account_hedge_allowed = true;
    expected = "AccountInfoRust { " +
    "account_balance: 1.1, " +
    "account_credit: 2.2, " +
    "account_profit: 3.3, " +
    "account_equity: 4.4, " +
    "account_margin: 5.5, " +
    "account_margin_free: 6.6, " +
    "account_margin_level: 7.7, " +
    "account_margin_so_call: 8.8, " +
    "account_margin_so_so: 9.9, " +
    "account_margin_initial: 10.1, " +
    "account_margin_maintenance: 11.11, " +
    "account_assets: 12.12, " +
    "account_liabilities: 13.13, " +
    "account_commission_blocked: 14.14, " +
    "account_login: 15, " +
    "account_leverage: 16, " +
    "account_name: \"17.17\", " +
    "account_server: \"18.18\", " +
    "account_currency: \"19.19\", " +
    "account_company: \"20.20\", " +
    "account_trade_mode: AccountTradeModeContest, " +
    "account_limit_orders: 22, " +
    "account_margin_so_mode: AccountStopoutModeMoney, " +
    "account_margin_mode: AccountMarginModeRetailHedging, " +
    "account_currency_digits: 25, " +
    "account_trade_allowed: false, " +
    "account_trade_expert: true, " +
    "account_fifo_close: false, " +
    "account_hedge_allowed: true }";
    test_report_account_info(observed, account_info_bridge);
    assert(observed, expected, test_name);


    test_name = "Testing DealPropertiesBridge serialization...";
    DealPropertiesBridge deal_properties_bridge;
    deal_properties_bridge.deal_volume = 1.1;
    deal_properties_bridge.deal_price = 2.2;
    deal_properties_bridge.deal_commission = 3.3;
    deal_properties_bridge.deal_swap = 4.4;
    deal_properties_bridge.deal_profit = 5.5;
    deal_properties_bridge.deal_fee = 6.6;
    deal_properties_bridge.deal_sl = 7.7;
    deal_properties_bridge.deal_tp = 8.8;
    deal_properties_bridge.deal_symbol = "SyMbOl";
    deal_properties_bridge.deal_comment = "cOmMeNt";
    deal_properties_bridge.deal_external_id = "iD";
    deal_properties_bridge.deal_ticket = 9;
    deal_properties_bridge.deal_order = 10;
    deal_properties_bridge.deal_time_msc = 11223344;
    deal_properties_bridge.deal_magic = 55;
    deal_properties_bridge.deal_position_id = 66;
    deal_properties_bridge.deal_time = 11223;
    deal_properties_bridge.deal_type = DEAL_TYPE_COMMISSION_AGENT_DAILY;
    deal_properties_bridge.deal_entry = DEAL_ENTRY_OUT_BY;
    deal_properties_bridge.deal_reason = DEAL_REASON_SO;
    expected = "DealPropertiesRust { " +
    "deal_volume: 1.1, " +
    "deal_price: 2.2, " +
    "deal_commission: 3.3, " +
    "deal_swap: 4.4, " +
    "deal_profit: 5.5, " +
    "deal_fee: 6.6, " +
    "deal_sl: 7.7, " +
    "deal_tp: 8.8, " +
    "deal_ticket: 9, " +
    "deal_order: 10, " +
    "deal_magic: 55, " +
    "deal_position_id: 66, " +
    "deal_time: 1970-01-01T03:07:03.344, " +
    "deal_symbol: \"SyMbOl\", " +
    "deal_comment: \"cOmMeNt\", " +
    "deal_external_id: \"iD\", " +
    "deal_type: DealTypeCommissionAgentDaily, " +
    "deal_entry: DealEntryOutBy, " +
    "deal_reason: DealReasonSo }";
    test_report_deal_properties(observed, deal_properties_bridge);
    assert(observed, expected, test_name);

    test_name = "Testing 'MqlTradeTransaction', 'MqlTradeRequest', & 'MqlTradeResult' serialization for OnTradeTransaction()...";
    MqlTradeTransaction trade_transaction;
    trade_transaction.deal = 0;
    trade_transaction.order = 0;
    trade_transaction.symbol = 0;
    trade_transaction.type = 0;
    trade_transaction.order_type = 0;
    trade_transaction.order_state = 0;
    trade_transaction.deal_type = 0;
    trade_transaction.time_type = 0;
    trade_transaction.time_expiration = 0;
    trade_transaction.price = 0;
    trade_transaction.price_trigger = 0;
    trade_transaction.price_sl = 0;
    trade_transaction.price_tp = 0;
    trade_transaction.volume = 0;
    trade_transaction.position = 0;
    trade_transaction.position_by = 0;
    MqlTradeRequest trade_request;
    trade_request.action = TRADE_ACTION_DEAL;
    trade_request.magic = 0;
    trade_request.order = 0;
    trade_request.symbol = 0;
    trade_request.volume = 0;
    trade_request.price = 0;
    trade_request.stoplimit = 0;
    trade_request.sl = 0;
    trade_request.tp = 0;
    trade_request.deviation = 0;
    trade_request.type = 0;
    trade_request.type_filling = 0;
    trade_request.type_time = 0;
    trade_request.expiration = 0;
    trade_request.comment = 0;
    trade_request.position = 0;
    trade_request.position_by = 0;
    MqlTradeResult trade_result;
    trade_result.retcode = 0;
    trade_result.deal = 0;
    trade_result.order = 0;
    trade_result.volume = 0;
    trade_result.price = 0;
    trade_result.bid = 0;
    trade_result.ask = 0;
    trade_result.comment = 0;
    trade_result.request_id = 0;
    trade_result.retcode_external = 0;
    expected = "MqlTradeTransaction { // some values // }; MqlTradeRequest { // some other values// }; MqlTradeResult { // yet some other values // }";
    test_on_trade_transaction(observed, trade_transaction, trade_request, trade_result);
    assert(observed, expected, test_name);

    test_name = "Testing 'MqlBookInfo' serialization...";
    MarketBookAdd(_Symbol);
    Sleep(1500);
    MqlBookInfo  book_info[];
    MarketBookGet(_Symbol, book_info);
    Print("book_info len is " + ArraySize(book_info));
    test_on_book(observed, book_info, ArraySize(book_info));
    Print("Returned string length is " + observed.Length());
    Print("    & buffer is " + observed.BufferSize());
    Print(observed);
    
    Print("Verify ENUM values for 'ENUM_BOOK_TYPE'-- assure those values start with 0:");
    Print("BOOK_TYPE_SELL="+BOOK_TYPE_SELL);
    Print("BOOK_TYPE_BUY="+BOOK_TYPE_BUY);
    Print("BOOK_TYPE_SELL_MARKET="+BOOK_TYPE_SELL_MARKET);
    Print("BOOK_TYPE_BUY_MARKET="+BOOK_TYPE_BUY_MARKET);
    
    Print("ENUM_DAY_OF_THE_WEEK::SUNDAY = " + SUNDAY);
    Print("ENUM_SYMBOL_CALC_MODE::SYMBOL_CALC_MODE_FOREX = " + SYMBOL_CALC_MODE_FOREX);
    Print("ENUM_SYMBOL_CHART_MODE::SYMBOL_CHART_MODE_BID = " + SYMBOL_CHART_MODE_BID);
    Print("ENUM_SYMBOL_INDUSTRY::INDUSTRY_UNDEFINED = " + INDUSTRY_UNDEFINED);
    Print("ENUM_SYMBOL_OPTION_MODE::SYMBOL_OPTION_MODE_EUROPEAN = " + SYMBOL_OPTION_MODE_EUROPEAN);
    Print("ENUM_SYMBOL_OPTION_RIGHT::SYMBOL_OPTION_RIGHT_CALL = " + SYMBOL_OPTION_RIGHT_CALL);
    Print("ENUM_SYMBOL_ORDER_GTC_MODE::SYMBOL_ORDERS_GTC = " + SYMBOL_ORDERS_GTC);
    Print("ENUM_SYMBOL_SECTOR::SECTOR_UNDEFINED = " + SECTOR_UNDEFINED);
    Print("ENUM_SYMBOL_SWAP_MODE::SYMBOL_SWAP_MODE_DISABLED = " + SYMBOL_SWAP_MODE_DISABLED);
    Print("ENUM_SYMBOL_TRADE_EXECUTION::SYMBOL_TRADE_EXECUTION_REQUEST = " + SYMBOL_TRADE_EXECUTION_REQUEST);
    Print("ENUM_SYMBOL_TRADE_MODE::SYMBOL_TRADE_MODE_DISABLED = " + SYMBOL_TRADE_MODE_DISABLED);
    Print("ENUM_ACCOUNT_MARGIN_MODE::ACCOUNT_MARGIN_MODE_RETAIL_NETTING = " + ACCOUNT_MARGIN_MODE_RETAIL_NETTING);
    Print("ENUM_ACCOUNT_STOPOUT_MODE::ACCOUNT_STOPOUT_MODE_PERCENT = " + ACCOUNT_STOPOUT_MODE_PERCENT);
    Print("ENUM_ACCOUNT_TRADE_MODE_DEMO::ACCOUNT_TRADE_MODE_DEMO = " + ACCOUNT_TRADE_MODE_DEMO);
    Print("ENUM_DEAL_ENTRY::DEAL_ENTRY_IN = " + DEAL_ENTRY_IN);
    Print("ENUM_DEAL_REASON::DEAL_REASON_CLIENT = " + DEAL_REASON_CLIENT);
    Print("ENUM_DEAL_TYPE::DEAL_TYPE_BUY = " + DEAL_TYPE_BUY);

    Print("EnumTradeTransactionType::TRADE_TRANSACTION_ORDER_ADD = " + TRADE_TRANSACTION_ORDER_ADD);
    Print("EnumOrderType::ORDER_TYPE_BUY = " + ORDER_TYPE_BUY);
    Print("EnumOrderState::ORDER_STATE_STARTED = " + ORDER_STATE_STARTED);
    Print("EnumDealType::DEAL_TYPE_BUY = " + DEAL_TYPE_BUY);
    Print("EnumOrderTypeTime::ORDER_TIME_GTC = " + ORDER_TIME_GTC);
    Print("EnumOrderTypeFilling::ORDER_FILLING_FOK = " + ORDER_FILLING_FOK);
    Print("EnumTradeRequestActions::TRADE_ACTION_DEAL = " + TRADE_ACTION_DEAL);
    Print("EnumOrderType::ORDER_TYPE_BUY = " + ORDER_TYPE_BUY);


}

void expose_constant(string constant_name, long constant_value, string comment) {
    Print("/// " + comment);
    Print("pub const " + constant_name + ": u32 = " + constant_value + ";");
}

void assert(string observed, string expected, string message) {
    Print("");
    if (observed == expected) {
        Print(message + " OK");
    } else {
        Print(message + " FAILED");
        Print("  observed ( "+observed.Length()+"chars): '"+observed+"'");
        Print("  expected ( "+expected.Length()+"chars): '"+expected+"'");
    }
}