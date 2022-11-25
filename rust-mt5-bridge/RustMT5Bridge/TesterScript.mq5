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
/*    
[
	MqlBookInfo { book_type: BookTypeBuy, price: 23.47, volume: 58400.0 }, 
	MqlBookInfo { book_type: BookTypeBuy, price: 23.46, volume: 125400.0 }, 
	MqlBookInfo { book_type: BookTypeBuy, price: 23.45, volume: 54200.0 }, 
	MqlBookInfo { book_type: BookTypeBuy, price: 23.44, volume: 57700.0 }, 
	MqlBookInfo { book_type: BookTypeBuy, price: 23.43, volume: 16700.0 }, 
	MqlBookInfo { book_type: BookTypeSellMarket, price: 23.42, volume: 4100.0 }, 
	MqlBookInfo { book_type: BookTypeSellMarket, price: 23.41, volume: 42300.0 }, 
	MqlBookInfo { book_type: BookTypeSellMarket, price: 23.4, volume: 51700.0 }, 
	MqlBookInfo { book_type: BookTypeSellMarket, price: 23.39, volume: 61300.0 }, 
	MqlBookInfo { book_type: BookTypeSellMarket, price: 23.38, volume: 55900.0 }
]
*/
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