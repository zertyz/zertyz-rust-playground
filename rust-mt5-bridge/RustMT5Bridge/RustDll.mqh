// functions exported by our bridge dll


#include "SymbolInfoBridge.mqh"
#include "AccountInfoBridge.mqh"
#include "DealPropertiesBridge.mqh"


#import "rust_mt5_bridge.dll"
int    register_trading_expert_advisor_for_production(string account_token, string rust_algorithm, string symbol);
void   unregister_trading_expert_advisor(int handle, int reason_id);
void   report_symbol_info(int handle, SymbolInfoBridge& symbol_info);
void   report_account_info(int handle, AccountInfoBridge& account_info);
void   report_deal_properties(int handle, DealPropertiesBridge& deal_properties);
int    register_trading_expert_advisor_for_testing(string account_token, string rust_algorithm, string symbol);
void   on_tester_pass(int handle);
double on_tester(int handle);
void   on_tick(int handle, MqlTick& tick);
void   on_trade(int handle, int pending_orders_count, int open_positions_count);
void   on_book(int handle, MqlBookInfo& book_info[], int array_len);

// variants of the above functions to allow some sort of automated testing
// (actually, we care only for structs, whose alignment considerations and field positions may yield devastatingly wrong results)
// -- the test logic consists in the Rust side receiving the struct, serializing it and putting it back into 'buffer', so it can be
//    compared on the MT5 side
void test_on_tick(string& buffer, MqlTick& tick);
void test_report_symbol_info(string& buffer, SymbolInfoBridge& symbol_info);
void test_report_account_info(string& buffer, AccountInfoBridge& account_info);
void test_report_deal_properties(string& buffer, DealPropertiesBridge& deal_properties);
void test_on_book(string& buffer, MqlBookInfo& book_info[], int array_len);
//void test_


/*void report_buyer_initiated_trade(int handle, long date_time, double price, int volume);
void report_seller_initiated_trade(int handle, long date_time, double price, int volume);
void report_book_ask_update(int handle, long date_time, double ask_price, long volume);
void report_book_bid_update(int handle, long date_time, double bid_price, long volume);
void report_book_volume_update(int handle, long date_time, long volume);*/
#import
