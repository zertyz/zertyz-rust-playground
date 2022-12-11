/// Allows Rust to call MQL functions.
/// Since this is true -- https://www.mql5.com/en/forum/127282 -- (MQL5 doesn't allow DLLs to call its functions) our only option is to
// ask Rust what functions it would like to have called -- which Rust maintains in a synchronized Ring Buffer -- and let the MQL code do the call.
/// The communications are made in JSON format:
///   - JSON when Rust wants MQL5 to call a function: {"fn_to_call": "MqlFunction", "params": [10, "yes!", 9]}
///   - JSON when MQL5 reports back to Rust the returns of a called function: {"fn_called": "MqlFunction", "returns": [1, "done!", 2]}


#include "JAson.mqh"


string calling_buffer;  // pre-allocated buffer to exchange communications between MQL & Rust regarding calling & return information of MQL5 functions


/// Call this from EA's `OnInit()`
void init_rust_to_mql_method_calling_interface() {
   // pre-allocates the communications buffer (as Rust doesn't have a way of growing it)
   StringReserve(calling_buffer, 4096);
}


CJAVal calling_json;
CJAVal returning_json;
/// Inquires Rust for the next functions to call, returning true if at least one has been called and false otherwise.
/// Callers are advised to call this function from 2 places:
///   1) EA's `OnTick()` event handler's last statement, to reduce the "execution request queue waiting time"
///   2) EA's `OnTimer()` event, configured to run once every 200ms -- as a fall back
bool execute_pending_functions(int rust_handle) {
   int call_id = next_mql5_function_to_call(rust_handle, calling_buffer);
   if (call_id >= 0) {
      calling_json.Deserialize(calling_buffer);
      string fn_name = calling_json["fn_to_call"].ToStr();
      CJAVal returns = call_mql_function(rust_handle, fn_name, calling_json["params"]);
      returning_json["fn_called"] = fn_name;
      returning_json["returns"] = returns;
      calling_buffer = "";
      returning_json.Serialize(calling_buffer);
      report_mql5_function_called(rust_handle, calling_buffer);
      returning_json.Clear();
      return true;
   }
   return false;
}


/// Do the actual call of MQL5 functions, recording any returned results and other meaningful state after the command completion
CJAVal call_mql_function(int rust_handle, string function_name, CJAVal& params) {
   CJAVal returns;

   // https://www.mql5.com/en/docs/common/comment
   if (function_name == "Alert") {
      Alert(params[0].ToStr());
   } else if (function_name == "Print") {
      Print(params[0].ToStr());
   } else if (function_name == "Comment") {
      Comment(params[0].ToStr());

   // our internally defined functions
   } else if (function_name == "collect_and_report_account_info") {
      collect_and_report_account_info(rust_handle);
   } else if (function_name == "collect_and_report_symbol_info") {
      collect_and_report_symbol_info(rust_handle);
   } else if (function_name == "collect_and_report_all_deals_properties") {
      collect_and_report_all_deals_properties(rust_handle);

   // uh-oh -- EAs will exit :(
   } else {
      string message = "RustToMQLMethodCall.mqh: don't know how to call function `"+function_name+"` -- MQL code needs updating? Marking the DLL as Not Good to Continue (all EAs are likely to immediately exit)";
      Print(message);
      Alert(message);
      report_fatal_error(rust_handle, message);
   }
   return returns;
}