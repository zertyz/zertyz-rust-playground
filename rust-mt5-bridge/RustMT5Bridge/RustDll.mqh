// functions exported by our bridge dll


#include "SymbolInfoBridge.mqh"
#include "AccountInfoBridge.mqh"
#include "DealPropertiesBridge.mqh"


#import "rust_mt5_bridge.dll"
void   set_enum_variant_value(string rust_enum_name, string rust_variant_name, int mql_variant_value);
int    register_trading_expert_advisor_for_production(string account_token, string rust_algorithm, string symbol);
bool   has_fatal_error(int handle, string& error_message_buffer);
void   report_fatal_error(int handle, string& error_message);
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
void   on_trade_transaction(int handle, const MqlTradeTransaction& transaction, const MqlTradeRequest& request, const MqlTradeResult& result);
int    next_mql5_function_to_call(int handle_id, string& buffer);
void   report_mql5_function_called(int rust_handle, string& calling_buffer);

// struct & constants dumping functions to allow some sort of automated testing
// (struct's alignment considerations and field positions may yield devastatingly wrong results)
// -- the test logic consists in the Rust side receiving the struct, serializing it and putting it back into 'buffer', so it can be
//    compared on the MT5 side. See `TesterScript.mq5`
void dump_mql_tick_flag_constants(string& buffer);
void dump_on_deinit_reasons(string& buffer);
void dump_mql_tick(string& buffer, MqlTick& tick);
void dump_symbol_info_bridge(string& buffer, SymbolInfoBridge& symbol_info);
void dump_account_info_bridge(string& buffer, AccountInfoBridge& account_info);
void dump_deal_properties_bridge(string& buffer, DealPropertiesBridge& deal_properties);
void dump_mql_book_info(string& buffer, MqlBookInfo& book_info[], int array_len);
void dump_mql_trade_transaction(string& buffer, MqlTradeTransaction& transaction);
void dump_mql_trade_request(string& buffer, MqlTradeRequest& request);
void dump_mql_trade_result(string& buffer, MqlTradeResult& result);
uint test_schedule_mql5_function_call(int executing_handle_id, string& function_call_descriptor);


/*void report_buyer_initiated_trade(int handle, long date_time, double price, int volume);
void report_seller_initiated_trade(int handle, long date_time, double price, int volume);
void report_book_ask_update(int handle, long date_time, double ask_price, long volume);
void report_book_bid_update(int handle, long date_time, double bid_price, long volume);
void report_book_volume_update(int handle, long date_time, long volume);*/
#import
