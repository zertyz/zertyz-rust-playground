#property copyright   "Copyright 2022, OgreRobot.com"
#property link        "https://OgreRobot.com"
#property version     "1.00"
#property description "Market Data Provider Bridge to Rust, sharing trade & book events while"
#property description "allowing low to none `OnTick()` (trade) events to be lost"

#include "RustDll.mqh"

string   account_token  = "SjDud7s53Hvx7643Gtta7352Jdgx7453Hfzt635";                // The account for the one attempting to run the algorithm
string   rust_algorithm = "Market Data Provider";                                   // Don't trade: just provide Market Data
int      rust_handle    = -1;                                                       // The handler to identify this instance on the Rust side -- to be passed to almost all Rust functions


int OnInit() {
    rust_handle = register_trading_expert_advisor_for_production(account_token, rust_algorithm, _Symbol);
    if (rust_handle >= 0) {
        Print(StringFormat("MarketDataProvider: PRODUCTION trading EA for symbol '%s' was successfully registered with rust_handle=%d for using Rust algorithm '%s' and account token '%s'",
                           _Symbol, rust_handle, rust_algorithm, account_token));
        // per-session information (reported only by the first Expert Advisor DLL client to start on this Metatrader Terminal instance)
        if (rust_handle == 0) {
            #include "EnumReporter.mqh"
            collect_and_report_account_info(rust_handle);
            collect_and_report_all_deals_properties(rust_handle);
        }
        collect_and_report_symbol_info(rust_handle);
        MarketBookAdd(_Symbol);
        // check any DLL errors that could prevent this EA from running well
        string error_message; // pre-allocated buffer for any error messages
        StringReserve(error_message, 4096);
        if (has_fatal_error(rust_handle, error_message)) {
            Print("QUITTING DUE TO ERROR: " + error_message);
            return INIT_FAILED;
        } else {
            return INIT_SUCCEEDED;
        }
    } else {
        Print(StringFormat("MarketDataProvider: FAILED registering PRODUCTION trading EA for symbol '%s' with Error Code #%d -- attempted Rust algorithm was '%s' and account token '%s'",
                           _Symbol, rust_handle, rust_algorithm, account_token));
        return INIT_FAILED;
    }
}

void OnDeinit(const int reason) {
    unregister_trading_expert_advisor(rust_handle, reason);
    Print(StringFormat("MarketDataProvider: PRODUCTION trading EA for symbol '%s' (rust_handle=%d; rust_algorithm='%s'; account_token='%s') was unregistered due to MT5 request: reason #%d",
                       _Symbol, rust_handle, rust_algorithm, account_token, reason));
}

// any further tricks to get from https://www.mql5.com/en/docs/event_handlers/ontick ?
MqlTick last_tick;
void OnTick() {
   SymbolInfoTick(_Symbol, last_tick);
   on_tick(rust_handle, last_tick);
}

// If `OnTick()` events are being lost, consider moving this to its own EA --
// a single EA may receive those events (for different symbols), without losing any one, as they are enqueued.
// Requires subscription with 'MarketBookAdd()'
MqlBookInfo  book_info[];    // kept global to optimize allocations
void OnBookEvent(const string&  symbol) {
    MarketBookGet(symbol, book_info);
    // the Rust part will compute deltas to issue book additions / updates / removal events to subscribers
    on_book(rust_handle, book_info, ArraySize(book_info));
}