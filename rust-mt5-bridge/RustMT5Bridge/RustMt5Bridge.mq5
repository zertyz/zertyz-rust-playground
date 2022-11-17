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

#include "SymbolInfoBridge.mqh"
#include "AccountInfoBridge.mqh"
#include "DealPropertiesBridge.mqh"

// some possibly useful properties available for the program:
//   * https://www.mql5.com/en/docs/constants/environment_state/mql5_programm_info
//   * https://www.mql5.com/en/docs/marketinformation/symbolinfosessiontrade

#import "rust_mt5_bridge.dll"
int    register_trading_expert_advisor_for_production(string account_token, string rust_algorithm, string symbol);
void   unregister_trading_expert_advisor(int handle, int reason_id);
void   on_tick(int handle, MqlTick& tick);
void   report_symbol_info(int handle, SymbolInfoBridge& symbol_info);
void   report_account_info(int handle, AccountInfoBridge& account_info);
void   report_deal_properties(int handle, DealPropertiesBridge& deal_properties);
int    register_trading_expert_advisor_for_testing(string account_token, string rust_algorithm, string symbol);
void   on_tester_pass(int handle);
void   on_trade(int handle, int pending_orders_count, int open_positions_count);
double on_tester(int handle);

/*void report_buyer_initiated_trade(int handle, long date_time, double price, int volume);
void report_seller_initiated_trade(int handle, long date_time, double price, int volume);
void report_book_ask_update(int handle, long date_time, double ask_price, long volume);
void report_book_bid_update(int handle, long date_time, double bid_price, long volume);
void report_book_volume_update(int handle, long date_time, long volume);*/
#import

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
        report_symbol_info(rust_handle, symbol_info);
        // per-session information (reported only by the first expert advisor to start)
        if (rust_handle == 0) {
            AccountInfoBridge account_info = InstantiateAccountInfoBridge();
            report_account_info(rust_handle, account_info);
            // report all deals since time immemorial
            HistorySelect(0, TimeCurrent());
            for (uint i=0; i<HistoryDealsTotal(); i++) {
                long ticket_number = HistoryDealGetTicket(i);
                if (ticket_number > 0) {
                    DealPropertiesBridge deal_properties = InstantiateDealPropertiesBridge(ticket_number);
                    report_deal_properties(rust_handle, deal_properties);
                }
            }
        }
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

// any further tricks to get from https://www.mql5.com/en/docs/event_handlers/ontick ?
void OnTick() {
   MqlTick last_tick;
   SymbolInfoTick(_Symbol, last_tick);
   on_tick(rust_handle, last_tick);
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

// any further tricks to get from https://www.mql5.com/en/docs/event_handlers/ontrade ?
void OnTrade() {
    on_trade(rust_handle,
             OrdersTotal(),
             PositionsTotal());
}

// TODO: learn more from https://www.mql5.com/en/docs/event_handlers/ontester
double OnTester() {
    double ignored_ret_for_now = on_tester(rust_handle);
    return wakeup_spread * (min_profit - 1);
}

// more events to consider: see on the DLL
