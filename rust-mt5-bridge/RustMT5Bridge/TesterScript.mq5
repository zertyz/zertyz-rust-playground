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

/*
    test_name = "Testing SymbolInfoBridge serialization...";
    SymbolInfoBridge symbol_info_bridge;
    // set some fields...
    expected = "SymbolInfoBridge { // some values // }";
    test_report_symbol_info(observed, symbol_info_bridge);
    assert(observed, expected, test_name);


    test_name = "Testing AccountInfoBridge serialization...";
    AccountInfoBridge account_info_bridge;
    // set some fields...
    expected = "AccountInfoBridge { // some values // }";
    test_report_account_info(observed, account_info_bridge);
    assert(observed, expected, test_name);


    test_name = "Testing DealPropertiesBridge serialization...";
    DealPropertiesBridge deal_properties_bridge;
    // set some fields...
    expected = "DealPropertiesBridge { // some values // }";
    test_report_deal_properties(observed, deal_properties_bridge);
    assert(observed, expected, test_name);

*/
    test_name = "Testing DealPropertiesBridge serialization...";
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
        Print("  observed: '"+observed+"'");
        Print("  expected: '"+expected+"'");
    }
}