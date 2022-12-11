#property copyright   "Copyright 2022, OgreRobot.com"
#property link        "https://OgreRobot.com"
#property version     "1.00"
#property description "Trading Bridge to Rust, informing Market Data and allowing Trades to happen."
#property description "Notice, however, that the Market Data reporting done here has a lower priority than"
#property description "the reports done by the dedicated `MarketDataProvider` EA, if available."
input string rust_algorithm = "{\"algorithm\": \"NaiveTrader\", \"stop_win\": 0.02}";  // Rust algorithm and parameters

#include "RustDll.mqh"
#include "RustToMQLMethodCall.mqh"


// some possibly useful properties available for the program:
//   * https://www.mql5.com/en/docs/constants/environment_state/mql5_programm_info
//   * https://www.mql5.com/en/docs/marketinformation/symbolinfosessiontrade

string   account_token  = "SjDud7s53Hvx7643Gtta7352Jdgx7453Hfzt635";                // The account for the one attempting to run the algorithm
int      rust_handle   = -1;                                                        // The handler to identify this instance on the Rust side -- to be passed to almost all Rust functions
datetime testing_start;                                                             // If testing was started, remembers when
string   report;

int OnInit() {
    init_rust_to_mql_method_calling_interface();
    rust_handle = register_trading_expert_advisor_for_production(account_token, rust_algorithm, _Symbol);
    if (rust_handle >= 0) {
        Print(StringFormat("Trader: PRODUCTION trading EA for symbol '%s' was successfully registered with rust_handle=%d for using Rust algorithm '%s' and account token '%s'",
                           _Symbol, rust_handle, rust_algorithm, account_token));
        // per-session information (reported only by the first expert advisor to start)
        if (rust_handle == 0) {
            #include "EnumReporter.mqh"
            collect_and_report_account_info(rust_handle);
            collect_and_report_all_deals_properties(rust_handle);
        }
        collect_and_report_symbol_info(rust_handle);
        //Print(StringFormat("RustMtBridge(%d): '%s': Initialization completed", rust_handle, _Symbol));
        MarketBookAdd(_Symbol);
        // check any DLL errors that could prevent this EA from running well
        string error_message; // pre-allocated buffer for any error messages
        StringReserve(error_message, 4096);
        if (has_fatal_error(rust_handle, error_message)) {
            Print("QUITTING DUE TO ERROR: " + error_message);
            return INIT_FAILED;
        } else {
            EventSetMillisecondTimer(500);
            return INIT_SUCCEEDED;
        }
    } else {
        Print(StringFormat("Trader: FAILED registering PRODUCTION trading EA for symbol '%s' with Error Code #%d -- attempted Rust algorithm was '%s' and account token '%s'",
                           _Symbol, rust_handle, rust_algorithm, account_token));
        return INIT_FAILED;
    }
}

void OnDeinit(const int reason) {
    EventKillTimer();
    unregister_trading_expert_advisor(rust_handle, reason);
    Print(StringFormat("Trader: PRODUCTION trading EA for symbol '%s' (rust_handle=%d; rust_algorithm='%s'; account_token='%s') was unregistered due to MT5 request: reason #%d",
                       _Symbol, rust_handle, rust_algorithm, account_token, reason));
}

/// TODO: get all tricks from https://www.mql5.com/en/docs/event_handlers/ontesterinit
int OnTesterInit() {
    rust_handle = register_trading_expert_advisor_for_testing(account_token, rust_algorithm, _Symbol);
    if (rust_handle >= 0) {
        Print(StringFormat("Trader: TESTING trading EA for symbol '%s' was successfully registered with rust_handle=%d for using Rust algorithm '%s' and account token '%s'",
                           _Symbol, rust_handle, rust_algorithm, account_token));
        return INIT_SUCCEEDED;
    } else {
        Print(StringFormat("Trader: FAILED registering TESTING trading EA for symbol '%s' with Error Code #%d -- attempted Rust algorithm was '%s' and account token '%s'",
                           _Symbol, rust_handle, rust_algorithm, account_token));
        return INIT_FAILED;
    }
//    testing_start = TimeLocal();
//    report = StringFormat("%s: optimization launched at %s",
//                          __FUNCTION__, TimeToString(testing_start, TIME_MINUTES|TIME_SECONDS) );
//    Print(report);
//    Comment(report);
}

void OnTesterDeinit() {
    unregister_trading_expert_advisor(rust_handle, 0);
    Print(StringFormat("Trader: TESTING trading EA for symbol '%s' (rust_handle=%d; rust_algorithm='%s'; account_token='%s') was unregistered due to MT5 request",
                       _Symbol, rust_handle, rust_algorithm, account_token));

//    string log_message = StringFormat("%s: optimization took %d seconds",
//                                      __FUNCTION__, TimeLocal()-testing_start);
//    PrintFormat(log_message);
//    report = report + "\r\n" + log_message;
//    Comment(report);
}

// TODO: not sure how to use this function -- https://www.mql5.com/en/docs/event_handlers/ontesterpass
void OnTesterPass() {
    on_tester_pass(rust_handle);
}

// TODO: learn more from https://www.mql5.com/en/docs/event_handlers/ontester
double OnTester() {
    double ignored_ret_for_now = on_tester(rust_handle);
    return 1;
}

void OnTick() {
    MqlTick last_tick;
    SymbolInfoTick(_Symbol, last_tick);
    on_tick(rust_handle, last_tick);
    execute_pending_functions(rust_handle);
}

void OnTrade() {
    on_trade(rust_handle,
             OrdersTotal(),
             PositionsTotal());
    execute_pending_functions(rust_handle);
}

MqlBookInfo  book_info[];
void OnBookEvent(const string&  symbol) {
    MarketBookGet(symbol, book_info);
    // the Rust part will compute deltas to issue book additions / editions / removal events to subscribers
    on_book(rust_handle, book_info, ArraySize(book_info));
    execute_pending_functions(rust_handle);
}

// Called when an order issue by us gets processed by the exchange
void OnTradeTransaction(const MqlTradeTransaction& transaction, const MqlTradeRequest&  request, const MqlTradeResult& result) {
    on_trade_transaction(rust_handle, transaction, request, result);
    execute_pending_functions(rust_handle);
}

void OnTimer() {
   execute_pending_functions(rust_handle);
}
