// Tests our DLL through special feedback functions exposed for that very purpose

#property copyright "Copyright OgreRobot.com"
#property link      "https://www.OgreRobot.com"
#property version   "1.00"
#property strict

#include "RustDll.mqh"
#include "RustToMQLMethodCall.mqh"


void OnStart() {

    string test_name;
    string expected;
    string observed;
    StringInit(observed, 1024, 0);  // this will contain the Rust ==> MT5 test results. The `string` should have enough capacity, since it cannot be allocated on the Rust side
                                    // -- unfortunately, if not enough size is provided, memory corruption will happen, since Rust don't receive the buffer size when strings
                                    //    are passed as function parameters (just the pointer to the buffer is passed)

    // Makes all MQL Enum variant values known to Rust, so they may be converted properly (MQL Variants are not ordered nor sequential, unfortunately)
    #include "EnumReporter.mqh"
    // check that all went fine
    if (has_fatal_error(-1, observed)) {
        Print("I'D QUIT NOW (BUT I WON'T) DUE TO DLL ERROR: " + observed);
    }


    Print("");
    test_name = "Testing Mt5MqlTick::flags constants...";
    expected = "MqlTick::flags { TICK_FLAG_BID: "+TICK_FLAG_BID+", TICK_FLAG_ASK: "+TICK_FLAG_ASK+", TICK_FLAG_LAST: "+TICK_FLAG_LAST+", TICK_FLAG_VOLUME: "+TICK_FLAG_VOLUME+", TICK_FLAG_BUY: "+TICK_FLAG_BUY+", TICK_FLAG_SELL: "+TICK_FLAG_SELL+" }";
    dump_mql_tick_flag_constants(observed);
    assert(observed, expected, test_name);
    
    
    Print("");
    test_name = "Testing OnDeinit(reason) codes...";
    expected = "OnDeinit::reasons { REASON_PROGRAM: "+REASON_PROGRAM+", REASON_REMOVE: "+REASON_REMOVE+", REASON_RECOMPILE: "+REASON_RECOMPILE+", REASON_CHARTCHANGE: "+REASON_CHARTCHANGE+", REASON_CHARTCLOSE: "+REASON_CHARTCLOSE+", REASON_PARAMETERS: "+REASON_PARAMETERS+", REASON_ACCOUNT: "+REASON_ACCOUNT+", REASON_TEMPLATE: "+REASON_TEMPLATE+", REASON_INITFAILED: "+REASON_INITFAILED+", REASON_CLOSE: "+REASON_CLOSE+" }";
    dump_on_deinit_reasons(observed);
    assert(observed, expected, test_name);


    Print("");
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
    dump_mql_tick(observed, mql_tick);
    assert(observed, expected, test_name);


    Print("");
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
    dump_symbol_info_bridge(observed, symbol_info_bridge);
    assert(observed, expected, test_name);


    Print("");
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
    dump_account_info_bridge(observed, account_info_bridge);
    assert(observed, expected, test_name);


    Print("");
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
    dump_deal_properties_bridge(observed, deal_properties_bridge);
    assert(observed, expected, test_name);


    Print("");
    test_name = "Testing 'MqlBookInfo' serialization...";
    MqlBookInfo  book_info[2];
    book_info[0].price       = 1.1;
    book_info[0].type        = BOOK_TYPE_BUY_MARKET;
    book_info[0].volume      = 3;
    book_info[0].volume_real = 4.4;
    book_info[1].price       = 5.5;
    book_info[1].type        = BOOK_TYPE_SELL_MARKET;
    book_info[1].volume      = 7;
    book_info[1].volume_real = 8.8;
    dump_mql_book_info(observed, book_info, ArraySize(book_info));
    expected = "[MqlBookInfo { book_type: BookTypeBuyMarket, price: 1.1, volume: 4.4 }, MqlBookInfo { book_type: BookTypeSellMarket, price: 5.5, volume: 8.8 }]";
    assert(observed, expected, test_name);

    
    Print("");
    test_name = "Testing 'MqlTradeTransaction' serialization for OnTradeTransaction()...";
    MqlTradeTransaction trade_transaction;
    trade_transaction.deal = 1;
    trade_transaction.order = 2;
    trade_transaction.symbol = "sYmBoL";
    trade_transaction.type = TRADE_TRANSACTION_HISTORY_ADD;
    trade_transaction.order_type = ORDER_TYPE_BUY_STOP;
    trade_transaction.order_state = ORDER_STATE_REJECTED;
    trade_transaction.deal_type = DEAL_TYPE_BONUS;
    trade_transaction.time_type = ORDER_TIME_SPECIFIED_DAY;
    trade_transaction.time_expiration = 8;
    trade_transaction.price = 9.9;
    trade_transaction.price_trigger = 10.10;
    trade_transaction.price_sl = 11.11;
    trade_transaction.price_tp = 12.12;
    trade_transaction.volume = 13.13;
    trade_transaction.position = 14;
    trade_transaction.position_by = 15;
    expected = "MqlTradeTransaction { " +
    "deal: 1, " +
    "order: 2, " +
    "symbol: \"sYmBoL\", " +
    "transaction_type: TradeTransactionHistoryAdd, " +
    "order_type: OrderTypeBuyStop, " +
    "order_state: OrderStateRejected, " +
    "deal_type: DealTypeBonus, " +
    "time_type: OrderTimeSpecifiedDay, " +
    "time_expiration: 1970-01-01T00:00:08, " +
    "price: 9.9, " +
    "price_trigger: 10.1, " +
    "price_sl: 11.11, " +
    "price_tp: 12.12, " +
    "volume: 13.13, " +
    "position: 14, " +
    "position_by: 15 }";
    dump_mql_trade_transaction(observed, trade_transaction);
    assert(observed, expected, test_name);

    
    Print("");
    test_name = "Testing 'MqlTradeRequest' serialization for OnTradeTransaction()...";
    MqlTradeRequest trade_request;
    trade_request.action = TRADE_ACTION_DEAL;
    trade_request.magic = 2;
    trade_request.order = 3;
    trade_request.symbol = "SyMbOl";
    trade_request.volume = 5.5;
    trade_request.price = 6.6;
    trade_request.stoplimit = 7.7;
    trade_request.sl = 8.8;
    trade_request.tp = 9.9;
    trade_request.deviation = 10;
    trade_request.type = ORDER_TYPE_BUY_STOP_LIMIT;
    trade_request.type_filling = ORDER_FILLING_IOC;
    trade_request.type_time = ORDER_TIME_GTC;
    trade_request.expiration = 14;
    trade_request.comment = "cOmMeNt";
    trade_request.position = 16;
    trade_request.position_by = 17;
    expected = "MqlTradeRequest { " +
    "action: TradeActionDeal, " +
    "magic: 2, " +
    "order: 3, " +
    "symbol: \"SyMbOl\", " +
    "volume: 5.5, " +
    "price: 6.6, " +
    "stoplimit: 7.7, " +
    "sl: 8.8, " +
    "tp: 9.9, " +
    "deviation: 10, " +
    "order_type: OrderTypeBuyStopLimit, " +
    "order_type_filling: OrderFillingIoc, " +
    "order_type_time: OrderTimeGtc, " +
    "expiration: 1970-01-01T00:00:14, " +
    "comment: \"cOmMeNt\", " +
    "position: 16, " +
    "position_by: 17 }";
    dump_mql_trade_request(observed, trade_request);
    assert(observed, expected, test_name);
    

    Print("");
    test_name = "Testing 'MqlTradeResult' serialization for OnTradeTransaction()...";
    MqlTradeResult trade_result;
    trade_result.retcode = 10039;
    trade_result.deal = 2;
    trade_result.order = 3;
    trade_result.volume = 4.4;
    trade_result.price = 5.5;
    trade_result.bid = 6.6;
    trade_result.ask = 7.7;
    trade_result.comment = "CoMmEnT";
    trade_result.request_id = 9;
    trade_result.retcode_external = 10;
    expected = "MqlTradeResult { " +
    "retcode: TradeRetcodeCloseOrderExist, " +
    "deal: 2, " +
    "order: 3, " +
    "volume: 4.4, " +
    "price: 5.5, " +
    "bid: 6.6, " +
    "ask: 7.7, " +
    "comment: \"CoMmEnT\", " +
    "request_id: 9, " +
    "retcode_external: 10 }";
    dump_mql_trade_result(observed, trade_result);
    assert(observed, expected, test_name);
    
    
    Print("");
    init_rust_to_mql_method_calling_interface();
    Print("Testing 'RustToMQLMethodCall':");

    test_name = "    Alert(msg)";
    expected = "{\"fn_to_call\": \"Alert\", \"params\": [\"Please, show this message to the user\"]}";
    test_schedule_mql5_function_call(0, expected);
    next_mql5_function_to_call(0, observed);
    assert(observed, expected,test_name);
    test_schedule_mql5_function_call(0, expected);
    execute_pending_functions(0);

    test_name = "    Print(msg)";
    expected = "{\"fn_to_call\": \"Print\", \"params\": [\"Please, print this message on the MT5 Terminal\"]}";
    test_schedule_mql5_function_call(0, expected);
    next_mql5_function_to_call(0, observed);
    assert(observed, expected,test_name);
    test_schedule_mql5_function_call(0, expected);
    execute_pending_functions(0);
    
    test_name = "    Comment(msg)";
    expected = "{\"fn_to_call\": \"Comment\", \"params\": [\"Are on the Symbol's Graph Top-Left corner??\"]}";
    test_schedule_mql5_function_call(0, expected);
    next_mql5_function_to_call(0, observed);
    assert(observed, expected,test_name);
    test_schedule_mql5_function_call(0, expected);
    execute_pending_functions(0);
    
    test_name = "    OrderCalcMargin(...)";
    expected = "{\"fn_to_call\": \"OrderCalcMargin\", \"params\": [\"enum_order_type_action\": "+TRADE_ACTION_DEAL+", \"symbol\": \"PETR4\", \"volume\": 100, \"price\": 32.02]}";
    test_schedule_mql5_function_call(0, expected);
    next_mql5_function_to_call(0, observed);
    assert(observed, expected,test_name);
    test_schedule_mql5_function_call(0, expected);
    execute_pending_functions(0);
    
    test_name = "    collect_and_report_account_info()";
    expected = "{\"fn_to_call\": \"collect_and_report_account_info\", \"params\": []}";
    test_schedule_mql5_function_call(0, expected);
    next_mql5_function_to_call(0, observed);
    assert(observed, expected,test_name);
    test_schedule_mql5_function_call(0, expected);
    execute_pending_functions(0);
    
    test_name = "    collect_and_report_symbol_info()";
    expected = "{\"fn_to_call\": \"collect_and_report_symbol_info\", \"params\": []}";
    test_schedule_mql5_function_call(0, expected);
    next_mql5_function_to_call(0, observed);
    assert(observed, expected,test_name);
    test_schedule_mql5_function_call(0, expected);
    execute_pending_functions(0);

    test_name = "    collect_and_report_all_deals_properties()";
    expected = "{\"fn_to_call\": \"collect_and_report_all_deals_properties\", \"params\": []}";
    test_schedule_mql5_function_call(0, expected);
    next_mql5_function_to_call(0, observed);
    assert(observed, expected,test_name);
    test_schedule_mql5_function_call(0, expected);
    execute_pending_functions(0);

    
   //StringSetLength(observed, 0);  // would free the string, but don't...
   //StringInit(observed, 0, 0);    // not even this resolves the "1 leaked strings left" / "4096 bytes of leaked memory" messages... MQL5 bug on scripts? EAs don't suffer from this
}

void assert(string observed, string expected, string message) {
    if (observed == expected) {
        Print(message + " OK -- "+observed.Length()+" chars");
    } else {
        Print(message + " FAILED");
        Print("  observed ( "+observed.Length()+" chars): '"+observed+"'");
        Print("  expected ( "+expected.Length()+" chars): '"+expected+"'");
    }
}