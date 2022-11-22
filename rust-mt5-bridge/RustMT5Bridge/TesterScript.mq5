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


}

void assert(string observed, string expected, string message) {
    if (observed == expected) {
        Print(message + " OK");
    } else {
        Print(message + " FAILED");
        Print("  observed: '"+observed+"'");
        Print("  expected: '"+expected+"'");
    }
}