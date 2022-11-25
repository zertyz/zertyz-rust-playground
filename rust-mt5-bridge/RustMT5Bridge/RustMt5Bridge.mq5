#property copyright   "Copyright 2022, OgreRobot.com"
#property link        "https://OgreRobot.com"
#property version     "1.00"
#property description "Bridge to Rust, allowing implementing trading & market data providers Expert Advisors there."
#property description "The parameters present here are for a fictitious trading algorithm"
#property description "that takes into account the last minimum (for its buying decisions)"
#property description "and the minimum acceptable profit, for its selling ones."
#property description "Exposing parameters to Metatrader is interesting because we can use its testing facilities."
input double min_profit        = 0.03;  // The minimum profit at which the decision maker is allowed to close a position -- $0.03 starts protecting you from delays in the execution
input double wakeup_spread     = 0.03;  // The spread between the last lower price and now that, when reached
input double wakeup_grace_secs = 1.0;   // After the wakeup_spread has been reached, prices must not go down for (at least) that many seconds before a purchase can be done

#include "RustDll.mqh"

// some possibly useful properties available for the program:
//   * https://www.mql5.com/en/docs/constants/environment_state/mql5_programm_info
//   * https://www.mql5.com/en/docs/marketinformation/symbolinfosessiontrade

string account_token  = "SjDud7s53Hvx7643Gtta7352Jdgx7453Hfzt635";                  // The account for the one attempting to run the algorithm
string rust_algorithm = StringFormat("Fictitious(min_profit: %G, wakeup_spread: %G, wakeup_grace_secs: )",
                                     min_profit, wakeup_spread, wakeup_grace_secs); // Which Algorithm to run on the Rust side
int    rust_handle   = -1;                                                          // The handler to identify this instance on the Rust side -- to be passed to almost all Rust functions
datetime testing_start;                                                             // If testing was started, remembers when
string report;

int OnInit() {
    rust_handle = register_trading_expert_advisor_for_production(account_token, rust_algorithm, _Symbol);
    if (rust_handle >= 0) {
        Print(StringFormat("RustMtBridge: PRODUCTION trading EA for symbol '%s' was successfully registered with rust_handle=%d for using Rust algorithm '%s' and account token '%s'",
                           _Symbol, rust_handle, rust_algorithm, account_token));
        SymbolInfoBridge symbol_info = InstantiateSymbolInfoBridge(_Symbol);
        //Print(StringFormat("RustMtBridge(%d): '%s': Reporting symbol info...", rust_handle, _Symbol));
        report_symbol_info(rust_handle, symbol_info);
        // per-session information (reported only by the first expert advisor to start)
        if (rust_handle == 0) {
            AccountInfoBridge account_info = InstantiateAccountInfoBridge();
            //Print(StringFormat("RustMtBridge(%d): '%s': Reporting account info...", rust_handle, _Symbol));
            report_account_info(rust_handle, account_info);
            // report all deals since time immemorial
            HistorySelect(0, TimeCurrent());
            for (uint i=0; i<HistoryDealsTotal(); i++) {
                long ticket_number = HistoryDealGetTicket(i);
                if (ticket_number > 0) {
                    DealPropertiesBridge deal_properties = InstantiateDealPropertiesBridge(ticket_number);
                    //Print(StringFormat("RustMtBridge(%d): '%s': Reporting deal properties...", rust_handle, _Symbol));
                    report_deal_properties(rust_handle, deal_properties);
                }
            }
        }
        //Print(StringFormat("RustMtBridge(%d): '%s': Initialization completed", rust_handle, _Symbol));
        return INIT_SUCCEEDED;
    } else {
        Print(StringFormat("RustMtBridge: FAILED registering PRODUCTION trading EA for symbol '%s' with Error Code #%d -- attempted Rust algorithm was '%s' and account token '%s'",
                           _Symbol, rust_handle, rust_algorithm, account_token));
        return INIT_FAILED;
    }
}

void OnDeinit(const int reason) {
    unregister_trading_expert_advisor(rust_handle, reason);
    Print(StringFormat("RustMtBridge: PRODUCTION trading EA for symbol '%s' (rust_handle=%d; rust_algorithm='%s'; account_token='%s') was unregistered due to MT5 request: reason #%d",
                       _Symbol, rust_handle, rust_algorithm, account_token, reason));
}

/// TODO: get all tricks from https://www.mql5.com/en/docs/event_handlers/ontesterinit
int OnTesterInit() {
    rust_handle = register_trading_expert_advisor_for_testing(account_token, rust_algorithm, _Symbol);
    if (rust_handle >= 0) {
        Print(StringFormat("RustMtBridge: TESTING trading EA for symbol '%s' was successfully registered with rust_handle=%d for using Rust algorithm '%s' and account token '%s'",
                           _Symbol, rust_handle, rust_algorithm, account_token));
        return INIT_SUCCEEDED;
    } else {
        Print(StringFormat("RustMtBridge: FAILED registering TESTING trading EA for symbol '%s' with Error Code #%d -- attempted Rust algorithm was '%s' and account token '%s'",
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
    Print(StringFormat("RustMtBridge: TESTING trading EA for symbol '%s' (rust_handle=%d; rust_algorithm='%s'; account_token='%s') was unregistered due to MT5 request",
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
    return wakeup_spread * (min_profit - 1);
}

// any further tricks to get from https://www.mql5.com/en/docs/event_handlers/ontick ?
void OnTick() {
   MqlTick last_tick;
   SymbolInfoTick(_Symbol, last_tick);
   on_tick(rust_handle, last_tick);
}

// Events bellow this line are likely to be moved to other EAs, so not to interfere with the `OnTick()` events
//////////////////////////////////////////////////////////////////////////////////////////////////////////////

// any further tricks to get from https://www.mql5.com/en/docs/event_handlers/ontrade ?
void OnTrade() {
    on_trade(rust_handle,
             OrdersTotal(),
             PositionsTotal());
}

// A single EA may receive those events (for different symbols), without losing any one, as they are enqueued.
// Subscribe with 'MarketBookAdd()'
MqlBookInfo  book_info[];    // kept global to optimize allocations
void OnBookEvent(const string&  symbol) {
    MarketBookGet(symbol, book_info);
    // the Rust part will compute deltas to issue book additions / editions / removal events to subscribers
    on_book(rust_handle, book_info, ArraySize(book_info));
}