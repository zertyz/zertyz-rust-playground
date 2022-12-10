use super::{
    types::*,
    mql_rust_enum,
    mq5_lib::types::MQ5StringRef,
};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs;
use std::io::Write;
use std::iter::Iterator;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;
use widestring::{U16CString};
use parking_lot::RawMutex;
use parking_lot::lock_api::RawMutex as _RawMutex;
use log::{debug, info, warn, error};


const MAX_HANDLES: i32 = 1024;

// Runtime (static) data
////////////////////////

static HANDLE_COUNT: AtomicI32 = AtomicI32::new(0);
/// Keeps track of `handle_id`s conceived to clients (MT5 scripts)
static mut HANDLES: Vec<Handle> = Vec::new();
/// Guard for writes to HANDLES (reads are unguarded)
static HANDLES_GUARD: RawMutex = RawMutex::INIT;
/// If present, indicates any fatal errors that should cause MQL Programs to quit in order to avoid undefined behavior
static mut FATAL_ERROR: Option<String> = None;

/// See the docs docs for this function in https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain
#[no_mangle]
pub extern "system" fn DllMain(_: *const (), fdw_reason: u32, _: *const ()) -> u32 {
    match fdw_reason {
        0 => warn!("DllMain() called for reason 0: DLL_PROCESS_DETACH -- the DLL is being completely unloaded for the process is about to cleanly exit"),
        1 => {
            init(Some("rust_mt5_bridge.log"));
            warn!("'rust_mt5_bridge.dll' was loaded and started -- allowing up to {MAX_HANDLES} handles (Expert Advisors, Indicators, Testers, etc.) to be created -- removing them won't free resources (restarting Metatrader will)");
            warn!("DllMain() called for reason 1: DLL_PROCESS_ATTACH -- DLL was loaded!");
        },
        2 => debug!("DllMain() called for reason 2: DLL_THREAD_ATTACH -- host process just created another thread"),
        3 => debug!("DllMain() called for reason 3: DLL_THREAD_DETACH -- host process just ended one of its threads"),
        n => debug!("DllMain() called for unknown reason {n}"),
    }
    1   // = TRUE: the DLL is good with the reported event
}

/// Called by the `OnInit()` to expose, to Rust, the MQL values for the mapped enums.\
/// If there are problems registering the enum variant -- for instance, if this DLL doesn't
/// know about it, this DLL instance will be marked as having experienced a "fatal error"
/// -- for which the MQL Program should respond by quitting, as undefined behavior is likely
/// to happen (See [has_fatal_error()])
#[no_mangle]
pub extern fn set_enum_variant_value(rust_enum_name: MQ5StringRef, rust_variant_name: MQ5StringRef, mql_variant_value: i32) {
    let rust_enum_name     = unsafe { U16CString::from_ptr_str(    rust_enum_name) }.to_string().unwrap_or(String::from("ERROR CONVERTING `rust_enum_name` -- a supposedly UTF-16 Metatrader 5 String reference to a UTF-8 Rust String"));
    let rust_variant_name  = unsafe { U16CString::from_ptr_str( rust_variant_name) }.to_string().unwrap_or(String::from("ERROR CONVERTING `rust_variant_name` -- a supposedly UTF-16 Metatrader 5 String reference to a UTF-8 Rust String"));
    match mql_rust_enum::set_enum_variant_value(&rust_enum_name, &rust_variant_name, mql_variant_value) {
        Ok(()) => {
            info!("set_enum_variant_value: rust_enum_name: '{rust_enum_name}'; rust_variant_name: '{rust_variant_name}'; mql_variant_value: {mql_variant_value}");
        },
        Err(error_message) => {
            error!("set_enum_variant_value: {} -- MQL Program should quit, otherwise UNDEFINED BEHAVIOR will happen", error_message);
            unsafe { FATAL_ERROR = Some(error_message.clone()); }
        },
    }
}

/// "fatal errors" are DLL errors that should cause any MQL programs to quit, as attempting to continue
/// is likely to cause undefined behavior -- which is sure to be disastrous.\
/// MQL Programs should check on this function before returning from `OnInit()`, and quit in case `true` is returned -- in this case,
/// `pre_allocated_error_message_buffer` will contain a brief explanation of the problem -- which should be shown to the
/// Metatrader Terminal User.\
/// NOTE: `pre_allocated_error_message_buffer` should be allocated on the MQL side and
///       have enough space to hold the error messages -- 256 bytes should be enough.\
/// NOTE 2: the MQL Program must be asked to quit through the "Rust=>MQL calling interface" as well, if errors
///         were detected past `OnInit()`
#[no_mangle]
pub extern fn has_fatal_error(handle_id: i32, pre_allocated_error_message_buffer: *mut u16) -> bool {
    if let Some(fatal_error) = unsafe { &FATAL_ERROR } {
        warn!("Informing handle_id {handle_id} that it must quit due to the fatal error '{fatal_error}'");
        unchecked_convert_rust_to_mql5_string(fatal_error.clone(), pre_allocated_error_message_buffer);
        true
    } else {
        false
    }
}

/// Causes this DLL to, as soon as possible, cease its operations and quit all MQL5 programs using it due to an
/// unrecoverable error prone to cause undefined behavior -- stopping the operations, then, is imposed to avoid disaster
#[no_mangle]
pub extern fn report_fatal_error(handle_id: i32, error_message: MQ5StringRef) {
    let error_message = unsafe { U16CString::from_ptr_str(error_message) }.to_string().unwrap_or(String::from("ERROR CONVERTING `error_message` -- a supposedly UTF-16 Metatrader 5 String reference to a UTF-8 Rust String"));
    let handle = unsafe { &HANDLES[handle_id as usize] };
    let symbol = &handle.symbol;
    error!("report_fatal_error({handle_id}): {symbol}: a FATAL error was reported: '{error_message}' -- all MQL5 programs using this DLL should quit as soon as possible!");
    unsafe {
        FATAL_ERROR = Some(error_message);
    }
}

/// Called by the `OnInit()` to inform the market data for the symbol being considered
/// (as well the session information for operation) and to get the `handle` to be passed
/// to all the other functions here -- if negative, it indicates an error code and the
/// loading of the MT5 script must be cancelled
#[no_mangle]
pub extern fn register_trading_expert_advisor_for_production(account_token: MQ5StringRef, algorithm: MQ5StringRef, symbol: MQ5StringRef) -> i32 {
    let account_token = unsafe { U16CString::from_ptr_str(account_token) }.to_string().unwrap_or(String::from("ERROR CONVERTING `account_token` -- a supposedly UTF-16 Metatrader 5 String reference to a UTF-8 Rust String"));
    let algorithm = unsafe { U16CString::from_ptr_str(algorithm) }.to_string().unwrap_or(String::from("ERROR CONVERTING `algorithm` -- a supposedly UTF-16 Metatrader 5 String reference to a UTF-8 Rust String"));
    let symbol = unsafe { U16CString::from_ptr_str(symbol) }.to_string().unwrap_or(String::from("ERROR CONVERTING `symbol` -- a supposedly UTF-16 Metatrader 5 String reference to a UTF-8 Rust String"));

    let handle_id = register(account_token, algorithm, symbol);

    // sleep a little -- relative to the conceived `handle_id` -- to allow the UI to be responsive when we approach the limit of 100 EA's in the Metatrader 5 Terminal
    std::thread::sleep(std::time::Duration::from_millis(100 * handle_id as u64));

    let handle = unsafe { &HANDLES[handle_id as usize] };
    if handle_id == -1 {
        error!("OnInit: FAILED registering trading expert advisor for PRODUCTION: {:?} -- exhausted handles {handle_id} where the max is {MAX_HANDLES}", handle);
    } else {
        info!("OnInit: registering trading expert advisor for PRODUCTION: {:?} -- attributed handle_id: {handle_id}", handle);
    }
    handle_id
}

/// Called by `OnDeinit()` or `OnTesterDeinit()` when the MT5 script is ending.\
/// IMPORTANT: tradeoff decision: this function doesn't clean resources -- this way we don't lose speed requiring a Mutex for `HANDLES`.
///            As a consequence, MT5 must be restarted every day.\
///            (In the future, we may mark the slot as vacant and search for vacant ones when registering)
#[no_mangle]
pub extern fn unregister_trading_expert_advisor(handle_id: i32, _reason_id: i32) {
    let handle = unsafe { &HANDLES[handle_id as usize] };
    info!("OnDeinit/OnTesterDeinit: unregistering trading expert advisor for `handle_id` #{handle_id}: {:?}", handle);
}

/// Called by the `OnInit()` to inform the market data for the symbol being considered
/// (as well the session information for operation) and to get the `handle` to be passed
/// to all the other functions here -- if negative, it indicates an error code and the
/// loading of the MT5 script must be cancelled
#[no_mangle]
pub extern fn register_trading_expert_advisor_for_testing(account_token: MQ5StringRef, algorithm: MQ5StringRef, symbol: MQ5StringRef) -> i32 {
    let account_token = unsafe { U16CString::from_ptr_str(account_token) }.to_string().unwrap_or(String::from("ERROR CONVERTING `account_token` -- a supposedly UTF-16 Metatrader 5 String reference to a UTF-8 Rust String"));
    let algorithm = unsafe { U16CString::from_ptr_str(algorithm) }.to_string().unwrap_or(String::from("ERROR CONVERTING `algorithm` -- a supposedly UTF-16 Metatrader 5 String reference to a UTF-8 Rust String"));
    let symbol = unsafe { U16CString::from_ptr_str(symbol) }.to_string().unwrap_or(String::from("ERROR CONVERTING `symbol` -- a supposedly UTF-16 Metatrader 5 String reference to a UTF-8 Rust String"));
    let handle_id = register(account_token, algorithm, symbol);

    // sleep a little -- relative to the conceived `handle_id` -- to allow the UI to be responsive when we approach the limit of 100 EA's in the Metatrader 5 Terminal
    std::thread::sleep(std::time::Duration::from_millis(100 * handle_id as u64));

    let handle = unsafe { &HANDLES[handle_id as usize] };
    if handle_id == -1 {
        error!("OnTesterInit: FAILED registering trading expert advisor for TESTING: {:?} -- exhausted handles {handle_id} where the max is {MAX_HANDLES}", handle);
    } else {
        info!("OnInit: registering trading expert advisor for TESTING: {:?} -- attributed handle_id: {handle_id}", handle);
    }
    handle_id
}

/// Called to inform details over the symbol under negotiation./
/// Typically consulted once per session per symbol, at the start.
#[no_mangle]
pub extern fn report_symbol_info(handle_id: i32, symbol_info: *const SymbolInfoBridge) {
    let handle = unsafe { &HANDLES[handle_id as usize] };
    let symbol_info = SymbolInfoBridge::from_ptr_to_internal(symbol_info);
    info!("report_symbol_info({handle_id}): {}: {:?}", handle.symbol, symbol_info);
}

/// Called to inform details for the account used to make the negotiations./
/// Typically consulted after every order issued / executed / edited / cancelled.
#[no_mangle]
pub extern fn report_account_info(handle_id: i32, account_info: *const AccountInfoBridge) {
    let handle = unsafe { &HANDLES[handle_id as usize] };
    let account_info = AccountInfoBridge::from_ptr_to_internal(account_info);
    info!("report_account_info({handle_id}): {}: {:?}", handle.symbol, account_info);
}

/// Called to inform details for a "deal" (an executed order)./
/// Typically consulted after every order issued / executed / edited / cancelled and dumped (all deals)
/// at the start of the session.
#[no_mangle]
pub extern fn report_deal_properties(handle_id: i32, deal_properties: *const DealPropertiesBridge) {
    let handle = unsafe { &HANDLES[handle_id as usize] };
    let deal_properties = DealPropertiesBridge::from_ptr_to_internal(deal_properties);
    info!("report_deal_properties({handle_id}): {}: {:?}", handle.symbol, deal_properties);
}


/// Called when there are new Quotes available.\
/// This function should return as fast as possible, or else new ticks will be lost (they are not enqueued).
/// See the docs https://www.mql5.com/en/docs/event_handlers/ontick
#[no_mangle]
pub extern fn on_tick(handle_id: i32, mt5_tick: *const Mq5MqlTick) {
    // this will be logged
    let handle = unsafe { &HANDLES[handle_id as usize] };
    let mt5_tick = unsafe { &*mt5_tick };
    info!("OnTick({handle_id}): {}: {:?}", handle.symbol, mt5_tick);
    // this will be enqueued
    let rust_tick = mt5_tick.to_internal(&handle.symbol);
    match rust_tick.to_event() {
        TickEvent::Trade(trade_event)    => info!("OnTick({handle_id}): {}:   {:?}", handle.symbol, trade_event),
        TickEvent::Spread(spread_event) => info!("OnTick({handle_id}): {}:  {:?}", handle.symbol, spread_event),
    }
}

/// Called when:
///   1) one of our orders is executed
///   2) our enqueued orders change in the broker (new one added, any removed or due to an update)
///   3) positions changes
/// See the docs https://www.mql5.com/en/docs/runtime/event_fire#trade
///              https://www.mql5.com/en/docs/event_handlers/ontrade
/// # Arguments
///
/// * `handle` - A string slice that holds the name of the person
/// * `pending_orders_count` - the result of calling `OrdersTotal()`: the number of pending orders awaiting execution -- see https://www.mql5.com/en/docs/trading/orderstotal
/// * `open_positions_count` - the result of calling `PositionsTotal()`: the number of symbols for which we may issue a sell order -- https://www.mql5.com/en/docs/trading/positionstotal
#[no_mangle]
pub extern fn on_trade(handle_id:            i32,
                       pending_orders_count: u32,
                       open_positions_count: u32) {
    let handle = unsafe { &HANDLES[handle_id as usize] };
    let symbol = &handle.symbol;
    info!("OnTrade: handle_id: {handle_id}, symbol: '{symbol}', pending_orders_count: {pending_orders_count}, open_positions_count: {open_positions_count}");
}

/// Called on book updates -- notice, however, that many book events may be skipped: this function is only
/// useful for reporting the new book state, which is shared in the `book_info_array_ptr` with a limit of
/// 5 entries the buying book and 5 entries for the selling book.
#[no_mangle]
pub extern fn on_book(handle_id:           i32,
                      book_info_array_ptr: *const Mq5MqlBookInfo,
                      array_len:           i32) {
    let handle = unsafe { &mut HANDLES[handle_id as usize] };
    let book_info_array = unsafe { std::slice::from_raw_parts(book_info_array_ptr, array_len as usize) };
    // this should be logged
    info!("OnBook({handle_id}): {}: {:?}", handle.symbol, book_info_array);
    let delta_events = compute_book_delta_events(&handle.books, book_info_array);
    // these will be enqueued for later processing
    debug!("OnBook({handle_id}): {}: {:?}", handle.symbol, delta_events);
    apply_book_delta_events(&mut handle.books, &delta_events);
    debug!("OnBook({handle_id}): {}: {:?}", handle.symbol, handle.books);
}

#[no_mangle]
pub extern fn on_trade_transaction(handle_id:   i32,
                                   transaction: *const Mq5MqlTradeTransaction,
                                   request:     *const Mq5MqlTradeRequest,
                                   result:      *const Mq5MqlTradeResult) {

    let handle = unsafe { &mut HANDLES[handle_id as usize] };
    let transaction = unsafe { &*transaction };
    let request       = unsafe { &*request };
    let result          = unsafe { &*result };
    info!("OnTradeTransaction({handle_id}): {}: {:?}; {:?}; {:?}", handle.symbol, transaction, request, result);
}

/// Called when a testing session ends -- returns the genetic evaluation function result, for which the genetic engine (built into Metatrader)
/// will try to find the maximum value possible, by running several times this EA and tuning the input parameters on each try.\
/// See the docs https://www.mql5.com/en/docs/event_handlers/ontester
#[no_mangle]
pub extern fn on_tester(handle_id: u32) -> f64 {
    let handle = unsafe { &HANDLES[handle_id as usize] };
    let symbol = &handle.symbol;
    info!("OnTester: handle_id: {handle_id}, symbol: '{symbol}'");
    -1.0
}

/// Usage and utility of this function is not yet clear --
/// more study and experiments are needed https://www.mql5.com/en/docs/event_handlers/ontesterpass
#[no_mangle]
pub extern fn on_tester_pass(handle_id: u32) {
    let handle = unsafe { &HANDLES[handle_id as usize] };
    let symbol = &handle.symbol;
    info!("OnTester: handle_id: {handle_id}, symbol: '{symbol}'");
}

/// If the returned value >= 0, places (in the pre-allocated `buffer`) the JSON function call descriptor that Rust wants MQL5 to do.\
/// The JSON in the form `{"fn_to_call": "MqlFunction", "params": [10, "yes!", 9]}` -- see `RustToMQLMethodCall.mqh`
#[no_mangle]
pub extern fn next_mql5_function_to_call(handle_id: i32, buffer: *mut u16) -> i32 {
    let next_function_call = consume_next_mql5_function_call(handle_id);
    if let Some(next_function_call) = next_function_call {
        let handle = unsafe { &HANDLES[handle_id as usize] };
        let symbol = &handle.symbol;
        debug!("ExecuteMQL5Function({handle_id}): {symbol}: {next_function_call}");
        unchecked_convert_rust_to_mql5_string(next_function_call, buffer);
        1
    } else {
        -1
    }
}

/// Called after a Rust triggered MQL5 function call was completed -- `function_called_json_descriptor` is a JSON with calling results in the form:
/// `{"fn_called": "MqlFunction", "returns": [1, "done!", 2]}`
#[no_mangle]
pub extern fn report_mql5_function_called(handle_id: i32, function_called_json_descriptor: *mut u16) {
    let function_called_json_descriptor = unsafe { U16CString::from_ptr_str(    function_called_json_descriptor) }.to_string().unwrap_or(String::from("ERROR CONVERTING `function_called_json_descriptor` -- a supposedly UTF-16 Metatrader 5 String reference to a UTF-8 Rust String"));
    let handle = unsafe { &HANDLES[handle_id as usize] };
    let symbol = &handle.symbol;
    debug!("ExecutedMQL5Function({handle_id}): {symbol}: {function_called_json_descriptor}");
}


// Automated testing functions
//////////////////////////////
// the following functions are for tests executed on the MT5 side, whose main purpose is to
// validate that structs shared between the languages are right: alignment, data types and field order
// must match...
// ... but other checks are done as well, such as scheduling MQL function executions (which is normally
// triggered by Rust logic)

/// Dumps the Rust internal values of the constants used in `MqlTick::flags` -- both to the log and
/// back to the MQL program, via the pre-allocated MQL String `buffer`, so that the MQL Tester
/// program may validate them
#[no_mangle]
pub extern fn dump_mql_tick_flag_constants(buffer: *mut u16) {
    let constants = serialize_mql_tick_flag_constants();
    info!("dump_mql_tick_flag_constants(): {:?}", constants);
    unchecked_convert_rust_to_mql5_string(constants, buffer);
}

#[no_mangle]
pub extern fn dump_on_deinit_reasons(buffer: *mut u16) {
    let constants = serialize_on_deinit_reasons();
    info!("dump_on_deinit_reasons(): {:?}", constants);
    unchecked_convert_rust_to_mql5_string(constants, buffer);
}

/// Dumps how Rust reads the [Mq5MqlTick] structure -- both to the log and
/// back to the MQL program, via the pre-allocated MQL String `buffer`
#[no_mangle]
pub extern fn dump_mql_tick(buffer: *mut u16, tick: *const Mq5MqlTick) {
    info!("dump_mql_tick(): {:?}", unsafe { &*tick });
    serialize_mql5_struct(buffer, tick);
}

/// Dumps how Rust reads the [SymbolInfoBridge] structure -- both to the log and
/// back to the MQL program, via the pre-allocated MQL String `buffer`
#[no_mangle]
pub extern fn dump_symbol_info_bridge(buffer: *mut u16, symbol_info: *const SymbolInfoBridge) {
    let symbol_info = SymbolInfoBridge::from_ptr_to_internal(symbol_info);
    info!("dump_symbol_info_bridge(): {:?}", symbol_info);
    serialize_mql5_struct(buffer, std::ptr::addr_of!(symbol_info));
}

/// Dumps how Rust reads the [AccountInfoBridge] structure -- both to the log and
/// back to the MQL program, via the pre-allocated MQL String `buffer`
#[no_mangle]
pub extern fn dump_account_info_bridge(buffer: *mut u16, account_info: *const AccountInfoBridge) {
    let account_info = AccountInfoBridge::from_ptr_to_internal(account_info);
    info!("dump_account_info_bridge(): {:?}", account_info);
    serialize_mql5_struct(buffer, std::ptr::addr_of!(account_info));
}

/// Dumps how Rust reads the [DealPropertiesBridge] structure -- both to the log and
/// back to the MQL program, via the pre-allocated MQL String `buffer`
#[no_mangle]
pub extern fn dump_deal_properties_bridge(buffer: *mut u16, deal_properties: *const DealPropertiesBridge) {
    let deal_properties = DealPropertiesBridge::from_ptr_to_internal(deal_properties);
    info!("dump_deal_properties_bridge(): {:?}", deal_properties);
    serialize_mql5_struct(buffer, std::ptr::addr_of!(deal_properties));
}

/// Dumps how Rust reads the [Mq5MqlBookInfo] structure -- both to the log and
/// back to the MQL program, via the pre-allocated MQL String `buffer`
#[no_mangle]
pub extern fn dump_mql_book_info(buffer: *mut u16, book_info_array: *const Mq5MqlBookInfo, array_len: i32) {
    let mut owned_book_info = Vec::with_capacity(array_len as usize);
    let mut book_info_cursor = book_info_array;
    for _ in 0..array_len as usize {
        let mt5_book_info = unsafe { &*book_info_cursor };
        let rust_book_info = mt5_book_info.to_internal();
        owned_book_info.push(rust_book_info);
        book_info_cursor = (book_info_cursor as usize + std::mem::size_of::<Mq5MqlBookInfo>()) as *mut Mq5MqlBookInfo;
    }
    info!("dump_mql_book_info(): {:?}", owned_book_info);
    serialize_mql5_array(buffer, owned_book_info);
}

/// Dumps how Rust reads the [Mq5MqlTradeTransaction] structure -- both to the log and
/// back to the MQL program, via the pre-allocated MQL String `buffer`
#[no_mangle]
pub extern fn dump_mql_trade_transaction(buffer:      *mut u16,
                                         transaction: *const Mq5MqlTradeTransaction) {
    let transaction = Mq5MqlTradeTransaction::from_ptr_to_internal(transaction);
    info!("dump_mql_trade_transaction(): {:?}", transaction);
    serialize_mql5_struct(buffer, &transaction);
}

/// Dumps how Rust reads the [Mq5MqlTradeRequest] structure -- both to the log and
/// back to the MQL program, via the pre-allocated MQL String `buffer`
#[no_mangle]
pub extern fn dump_mql_trade_request(buffer:  *mut u16,
                                     request: *const Mq5MqlTradeRequest) {
    let request = Mq5MqlTradeRequest::from_ptr_to_internal(request);
    info!("dump_mql_trade_request(): {:?}", request);
    serialize_mql5_struct(buffer, &request);
}

/// Dumps how Rust reads the [Mq5MqlTradeResult] structure -- both to the log and
/// back to the MQL program, via the pre-allocated MQL String `buffer`
#[no_mangle]
pub extern fn dump_mql_trade_result(buffer: *mut u16,
                                    result: *const Mq5MqlTradeResult) {
    let result = Mq5MqlTradeResult::from_ptr_to_internal(result);
    info!("dump_mql_trade_result(): {:?}", result);
    serialize_mql5_struct(buffer, &result);
}

/// calls [schedule_mql5_function_call()] for testing purposes
#[no_mangle]
pub extern fn test_schedule_mql5_function_call(executing_handle_id: i32, function_call_descriptor: MQ5StringRef) -> u32 {
    let function_call_descriptor = unsafe { U16CString::from_ptr_str(    function_call_descriptor) }.to_string().unwrap_or(String::from("ERROR CONVERTING `function_call_descriptor` -- a supposedly UTF-16 Metatrader 5 String reference to a UTF-8 Rust String"));
    schedule_mql5_function_call(executing_handle_id, function_call_descriptor)
}

/// Converts & copies `rust_string` into `pre_allocated_mql5_string`./
/// `pre_allocated_mql5_string` should have enough space to hold the data + \0, or else
/// Undefined Behavior will happen due to the corruption to the Metatrader heap
fn unchecked_convert_rust_to_mql5_string(rust_string: String, pre_allocated_mql5_string: *mut u16) {
    let u16_string = U16CString::from_str(rust_string).unwrap();
    let u16_string_ptr = u16_string.as_ptr();
    unsafe {
        std::ptr::copy_nonoverlapping(u16_string_ptr, pre_allocated_mql5_string, (u16_string.len()) * 2);
        // write the end of the string char \0
        std::ptr::write((pre_allocated_mql5_string as usize + u16_string.len()*2) as *mut u16, 0);
    }
}

/// Puts the Debug output of `struct_ptr` into `pre_allocated_mql5_string` -- which should have enough size or else
/// memory corruption will occur
fn serialize_mql5_struct<StructType: Debug>(pre_allocated_mql5_string: *mut u16, struct_ptr: *const StructType) {
    let strct = unsafe { &*struct_ptr };
    unchecked_convert_rust_to_mql5_string(format!("{:?}", strct), pre_allocated_mql5_string);
}

/// Dumps the Debug output of `array` into `pre_allocated_mql5_string` -- which should have enough size or else
/// memory corruption will occur
fn serialize_mql5_array<StructType: Debug>(pre_allocated_mql5_string: *mut u16, array: Vec<StructType>) {
    unchecked_convert_rust_to_mql5_string(format!("{:?}", array), pre_allocated_mql5_string);
}

/// Prepares the environment for this library's functions to work
fn init(log_file_path: Option<&str>) {
    let mut config = simple_log::LogConfigBuilder::builder();
    if let Some(log_file_path) = log_file_path {
        config = config
            .path(log_file_path)
            .output_file();
    } else {
        config = config
            .output_console();
    }
    config = config
        .size(MAX_LOG_FILE_SIZE_MB)
        .roll_count(MAX_LOG_FILES)
        .time_format("%H:%M:%S.%f")
        .level(LOG_LEVEL);

    simple_log::new(config.build())
        .expect("instantiating simplelog file writer");
    unsafe {
        for _i in 0..MAX_HANDLES {
            // place holders -- see `register()` for the real data to be put in `HANDLES`
            HANDLES.push(Handle {
                client_type:           ClientType::ProductionExpertAdvisor,
                account_token:         "".to_string(),
                algorithm:             "".to_string(),
                symbol:                "".to_string(),
                books:                 OrderBooks {
                                           sell_orders: VecDeque::with_capacity(0),
                                           buy_orders: VecDeque::with_capacity(0),
                                       },
                mql_functions_to_call: VecDeque::with_capacity(0)
            });
        }
    }
    symbol_info_bridge::init();
    account_info_bridge::init();
    deal_properties_bridge::init();
    mql_book_info::init();
    mql_trade_request::init();
    mql_trade_transaction::init();
}

/// Reserves a slot, inits it & returns the `handle_id` that is required by, almost, every function in this DLL./
/// A returned value of `-1` means a slot could not be obtained (all possible slots were already used)./
/// `handle_id` may be used to access the handle as in `let handle = unsafe { &HANDLES[handle_id as usize] };`
fn register(account_token: String, algorithm: String, symbol: String) -> i32 {
    let books = OrderBooks {
        sell_orders: VecDeque::with_capacity(5),
        buy_orders:  VecDeque::with_capacity(5),
    };
    let mql_functions_to_call = VecDeque::with_capacity(16);
    let handle = Handle {
        client_type: ClientType::ProductionExpertAdvisor,
        account_token,
        algorithm,
        symbol,
        books,
        mql_functions_to_call,
    };
    let handle_id = HANDLE_COUNT.fetch_add(1, Relaxed);
    if handle_id >= MAX_HANDLES {
        -1
    } else {
        unsafe {
            // although most hardware is capable of working OK without these guards, we use a "better safe than sorry" approach here
            // -- and this is out of the hot-path anyway
            HANDLES_GUARD.lock();
            HANDLES[handle_id as usize] = handle;
            HANDLES_GUARD.unlock();
        }
        handle_id
    }
}

/// applies `delta_events` to `rolling_books` in order to update the order books
/// -- or, in other words, "reconstruct the book"./
/// [compute_book_delta_events()] is the opposite operation
fn apply_book_delta_events(rolling_books: &mut OrderBooks, delta_events: &[BookEvents]) {

    // if >= 0, the return value indicates that the price point was found at that index
    fn search(book: &VecDeque<MqlBookInfo>, price: f64) -> i32 {
        // the books are sorted, so `book.binary_search()` could be used if the books were allowed to grow past 5 elements
        // -- for only 5, a full search is likely to be more efficient
        for i in 0..book.len() {
            if book[i].price == price {
                return i as i32;
            }
        }
        -1
    }
    let del = |book: &mut VecDeque<MqlBookInfo>, price: f64| {
        let i = search(book, price);
        book.remove(i as usize);
    };
    let update = |book: &mut VecDeque<MqlBookInfo>, price: f64, delta_quantity: f64| {
        let i = search(book, price);
        book[i as usize].volume += delta_quantity;
    };
    let add = |book: &mut VecDeque<MqlBookInfo>, new_book_order: MqlBookInfo| {
        // whatever the book, they are always sorted descendingly by price
        let i = book.partition_point(|order| order.price > new_book_order.price);
        book.insert(i, new_book_order);
    };

    for delta_event in delta_events {
        match delta_event {
            BookEvents::Add    { book, price, quantity } => {
                let book_deque = match book {
                    BookParties::Sellers => &mut rolling_books.sell_orders,
                    BookParties::Buyers => &mut rolling_books.buy_orders,
                };
                add(book_deque, MqlBookInfo { book_type: book.to_mt5_enum_book(), price: *price, volume: *quantity })
            },
            BookEvents::Del    { book, price, quantity: _quantity } => {
                let book_deque = match book {
                    BookParties::Sellers => &mut rolling_books.sell_orders,
                    BookParties::Buyers => &mut rolling_books.buy_orders,
                };
                del(book_deque, *price)
            },
            BookEvents::Update { book, price, delta_quantity } => {
                let book_deque = match book {
                    BookParties::Sellers => &mut rolling_books.sell_orders,
                    BookParties::Buyers => &mut rolling_books.buy_orders,
                };
                update(book_deque, *price, *delta_quantity)
            }
        }
    }
}

/// returns the delta events that would turn `old_books` into `new_books`, where:
///   - `new_books` is the Metatrader array received by the `OnBook()` event
///   - `old_books` is our internally kept structure to allow us to compute the event deltas.\
/// [apply_book_delta_events()] should be used to advance the book.
fn compute_book_delta_events(old_books: &OrderBooks, new_books: &[Mq5MqlBookInfo]) -> Vec<BookEvents> {
    let mut delta_events = Vec::<BookEvents>::with_capacity(new_books.len());
    let mut old_books_iter = old_books.iter().peekable();
    let mut new_books_iter = new_books.iter().peekable();
    // The two iterators want to walk together, building the 'delta_events' whatever one lags behind the other
    'next: loop {
        let mut peeked_old = old_books_iter.peek();
        let mut peeked_new = new_books_iter.peek();
        're_evaluate: loop {
            match (peeked_old, peeked_new) {
                (Some(old), Some(new)) => {
                    if old.book_type != ENUM_BOOK_TYPE.resolve_rust_variant(new.book_type) {
                        if old.book_type.is_sell() {
                            peeked_new = None;
                            continue 're_evaluate
                        } else {
                            peeked_old = None;
                            continue 're_evaluate
                        }
                    }
                    if old.price < new.price {
                        delta_events.push(BookEvents::Add { book: BookParties::from_mt5_enum_book(ENUM_BOOK_TYPE.resolve_rust_variant(new.book_type)), price: new.price, quantity: new.volume_real });
                        peeked_old = None;
                    } else if old.price > new.price {
                        delta_events.push(BookEvents::Del { book: BookParties::from_mt5_enum_book(old.book_type), price: old.price, quantity: old.volume });
                        peeked_new = None;
                    } else if old.volume != new.volume_real {
                        delta_events.push(BookEvents::Update { book: BookParties::from_mt5_enum_book(old.book_type), price: new.price, delta_quantity: new.volume_real-old.volume });
                    }
                },
                (Some(old), None) => {
                    delta_events.push(BookEvents::Del { book: BookParties::from_mt5_enum_book(old.book_type), price: old.price, quantity: old.volume });
                },
                (None, Some(new)) => {
                    delta_events.push(BookEvents::Add { book: BookParties::from_mt5_enum_book(ENUM_BOOK_TYPE.resolve_rust_variant(new.book_type)), price: new.price, quantity: new.volume_real });
                },
                (None, None) => break 'next,
            }
            break 're_evaluate
        }
        // advance the cursors
        if let Some(_old) = peeked_old {
            old_books_iter.next();
        }
        if let Some(_new) = peeked_new {
            new_books_iter.next();
        }
    }
    delta_events
}


/// Schedules a function to be executed by one of the MQL5 programs.\
/// `function_call` is a JSON in the form `{"fn_to_call": "MqlFunction", "params": [10, "yes!", 9]}` -- see `RustToMQLMethodCall.mqh`.\
/// Returns the number of pending functions to call after the scheduling is done
fn schedule_mql5_function_call(executing_handle_id: i32, function_call: String) -> u32 {
    unsafe {
        let mut mql_functions_to_call = &mut HANDLES[executing_handle_id as usize].mql_functions_to_call;
        HANDLES_GUARD.lock();
        mql_functions_to_call.push_back(function_call);
        HANDLES_GUARD.unlock();
        mql_functions_to_call.len() as u32
    }
}

/// Consumes any next MQL5 function to be called for the given `handle_id`.\
/// Returns the JSON call descriptor -- see [schedule_mql5_function_execution()]\
/// Implementation note: occasional false negatives are acceptable for the sake of execution speed (.len() check is out of the mutex section)
fn consume_next_mql5_function_call(handle_id: i32) -> Option<String> {
    unsafe {
        if HANDLES[handle_id as usize].mql_functions_to_call.len() > 0 {
            HANDLES_GUARD.lock();
            let function_call = HANDLES[handle_id as usize].mql_functions_to_call.pop_front();
            HANDLES_GUARD.unlock();
            function_call
        } else {
            None
        }
    }
}

/// are we compiled in DEBUG or RELEASE mode?
#[cfg(debug_assertions)]
pub const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
pub const DEBUG: bool = false;

/// Keep those levels in sync with Cargo.toml's `log` crate levels defined in features.
/// Example: features = ["max_level_debug", "release_max_level_info"]
const LOG_LEVEL: &str           = if DEBUG { "debug" } else { "info" };
const MAX_LOG_FILE_SIZE_MB: u64 = 2*1024;
const MAX_LOG_FILES: u32        = 22;

/// to be called when debugging logging issues
fn _internal_logger(path: &str, contents: &str) {
    fs::File::options().create(true).append(true)
        .open(path).expect("open internal log file for appending")
        .write_all(contents.as_bytes()).expect("write to internal log file");
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::mq5_lib::EnumBookType::*;

    #[ctor::ctor]
    fn suite_setup() {
        init(None);
    }

    /// tests both [apply_book_delta_events()] & [compute_book_delta_events()]
    /// -- sharing the same test since they are complementary
    #[test]
    fn on_book() {

        ENUM_BOOK_TYPE.debug();
        mql_rust_enum::set_enum_variant_value("EnumBookType", "BookTypeBuy", BookTypeBuy as i32)
            .expect("Setting the MQL variant value of a Rust-known variant for a previously registered enum");
        mql_rust_enum::set_enum_variant_value("EnumBookType", "BookTypeSell", BookTypeSell as i32)
            .expect("Setting the MQL variant value of a Rust-known variant for a previously registered enum");
        mql_rust_enum::set_enum_variant_value("EnumBookType", "BookTypeBuyMarket", BookTypeBuyMarket as i32)
            .expect("Setting the MQL variant value of a Rust-known variant for a previously registered enum");
        mql_rust_enum::set_enum_variant_value("EnumBookType", "BookTypeSellMarket", BookTypeSellMarket as i32)
            .expect("Setting the MQL variant value of a Rust-known variant for a previously registered enum");
        let handle_id = register(format!("acnt_tkn"), format!("algo"), format!("SYMBL"));
        let _handle = unsafe { &HANDLES[handle_id as usize] };

        // "original" book used for delta computation --
        // Metatrader, as production data shows as of 2022-11-24, will always yield books like this:
        //   a) Sell orders comes first, in the opposite order -- higher prices first instead of lowers first
        //   b) Buy orders comes next (after all sell orders) and they are ordered descendingly (by price)
        //   c) Only the book top 5 seems to be shared.
        //   OBS: this kind of ordering in (a) eases the visualization of the spread, but isn't helpful for our internal algorithms
        let base_books_generator = || OrderBooks {
            sell_orders: VecDeque::from([
                MqlBookInfo { book_type: BookTypeSell, price: 23.47, volume: 58400.00 },
                MqlBookInfo { book_type: BookTypeSell, price: 23.46, volume: 125400.00 },
                MqlBookInfo { book_type: BookTypeSell, price: 23.45, volume: 54200.00 },
                MqlBookInfo { book_type: BookTypeSell, price: 23.44, volume: 57700.00 },
                MqlBookInfo { book_type: BookTypeSell, price: 23.43, volume: 16700.00 }]),
            buy_orders:  VecDeque::from([
                MqlBookInfo { book_type: BookTypeBuy, price: 23.42, volume: 4100.00 },
                MqlBookInfo { book_type: BookTypeBuy, price: 23.41, volume: 42300.00 },
                MqlBookInfo { book_type: BookTypeBuy, price: 23.40, volume: 51700.00 },
                MqlBookInfo { book_type: BookTypeBuy, price: 23.39, volume: 61300.00 },
                MqlBookInfo { book_type: BookTypeBuy, price: 23.38, volume: 55900.00 }])
        };

        // scenario containing the new books (to be received by Metatrader) and the expected generated book event deltas
        ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        // those events are such that, when applied to `base_books` with `apply_book_delta_events()`, will turn it into the given books;
        // and also the other way around: when both books are given to `compute_book_delta_events()`, those events should be returned.

        let consumed_buy_price_point = (
            [
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.47, volume: 0, volume_real: 58400.00 },
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.46, volume: 0, volume_real: 125400.00 },
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.45, volume: 0, volume_real: 54200.00 },
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.44, volume: 0, volume_real: 57700.00 },
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.43, volume: 0, volume_real: 16700.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.41, volume: 0, volume_real: 42300.00 },     // 23.42 was consumed
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.40, volume: 0, volume_real: 51700.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.39, volume: 0, volume_real: 61300.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.38, volume: 0, volume_real: 55900.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.37, volume: 0, volume_real: 1400.00 },      // this is the new price point
            ],
            [
                BookEvents::Del { book: BookParties::Buyers, price: 23.42, quantity: 4100.00 },
                BookEvents::Add { book: BookParties::Buyers, price: 23.37, quantity: 1400.00 },
            ]);

        let consumed_sell_price_point = (
            [
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.48, volume: 0, volume_real: 76100.00 },    // this is the new price point
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.47, volume: 0, volume_real: 58400.00 },
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.46, volume: 0, volume_real: 125400.00 },
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.45, volume: 0, volume_real: 54200.00 },
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.44, volume: 0, volume_real: 57700.00 },    // 23.43 was consumed
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.42,  volume: 0, volume_real: 4100.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.41,  volume: 0, volume_real: 42300.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.40,  volume: 0, volume_real: 51700.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.39,  volume: 0, volume_real: 61300.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.38,  volume: 0, volume_real: 55900.00 },
            ],
            [
                BookEvents::Add { book: BookParties::Sellers, price: 23.48, quantity: 76100.00 },
                BookEvents::Del { book: BookParties::Sellers, price: 23.43, quantity: 16700.00 },
            ]);

        let consumed_buy_and_sell_price_points = (
            [
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.48, volume: 0, volume_real: 76100.00 },
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.47, volume: 0, volume_real: 58400.00 },
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.46, volume: 0, volume_real: 125400.00 },
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.45, volume: 0, volume_real: 54200.00 },
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.44, volume: 0, volume_real: 57700.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.41,  volume: 0, volume_real: 42300.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.40,  volume: 0, volume_real: 51700.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.39,  volume: 0, volume_real: 61300.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.38,  volume: 0, volume_real: 55900.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.37,  volume: 0, volume_real: 1400.00 },
            ],
            [
                BookEvents::Add { book: BookParties::Sellers, price: 23.48, quantity: 76100.00 },
                BookEvents::Del { book: BookParties::Sellers, price: 23.43, quantity: 16700.00 },
                BookEvents::Del { book: BookParties::Buyers, price: 23.42, quantity: 4100.00 },
                BookEvents::Add { book: BookParties::Buyers, price: 23.37, quantity: 1400.00 },
            ]);

        let price_gaps = (
            [
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.48, volume: 0, volume_real: 58401.00 },    // price gap here: 23.47 is now missing
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.46, volume: 0, volume_real: 125400.00 },
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.45, volume: 0, volume_real: 54200.00 },
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.44, volume: 0, volume_real: 57700.00 },
                Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.43, volume: 0, volume_real: 16700.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.42,  volume: 0, volume_real: 4100.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.41,  volume: 0, volume_real: 42300.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.40,  volume: 0, volume_real: 51700.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.39,  volume: 0, volume_real: 61300.00 },
                Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.37,  volume: 0, volume_real: 55901.00 },     // price gap here: 23.38 is now missing
            ],
            [
                BookEvents::Add { book: BookParties::Sellers, price: 23.48, quantity: 58401.00 },
                BookEvents::Del { book: BookParties::Sellers, price: 23.47, quantity: 58400.00 },
                BookEvents::Del { book: BookParties::Buyers, price: 23.38, quantity: 55900.00 },
                BookEvents::Add { book: BookParties::Buyers, price: 23.37, quantity: 55901.00 },
            ]);

        // asserts the behavior of the functions under test both individually and as the opposite operation of one another
        // "delta events" will be generated (and asserted) to transform the `old_books` into `new_books` (which is also asserted)
        let assert = |prefix_message, old_books, new_books: Vec<Mq5MqlBookInfo>, expected_book_events| {
            let observed_book_events = compute_book_delta_events(&old_books, &new_books);
//println!("observed_book_events: {:#?}", observed_book_events);
            assert_eq!(&observed_book_events, &expected_book_events, "{prefix_message} at 'compute_book_delta_events(...)': wrong 'delta events' were generated");
            // assert `old_books` really turns into `new_books` when the returned `observed_book_events` are applied to it.
            // `work_books`, therefore, is to be considered the "rolling books", constantly being updated with new book events
            let mut work_books = OrderBooks {
                sell_orders: VecDeque::from_iter(old_books.sell_orders.iter().map(|e_ref| MqlBookInfo { book_type: e_ref.book_type, price: e_ref.price, volume: e_ref.volume })),
                buy_orders: VecDeque::from_iter(old_books.buy_orders.iter().map(|e_ref| MqlBookInfo { book_type: e_ref.book_type, price: e_ref.price, volume: e_ref.volume }))
            };
            let converted_new_books = new_books.iter().map(|mq5_mql_book_event| mq5_mql_book_event.to_internal()).collect::<Vec<_>>();
            apply_book_delta_events(&mut work_books, &observed_book_events);
//println!("work_books: {:#?}", work_books);
            assert_eq!(work_books.iter().collect::<Vec<_>>(),
                       converted_new_books.iter().collect::<Vec<_>>(),
                       "{prefix_message} at 'apply_book_delta_events(...)': `old_books` could not be transformed to `new_books` when applying the 'delta events'");
        };


        // test cases
        /////////////

        // exercise 'Add' events
        assert("Check 1 (empty `old_books` / full `new_books`)",
               OrderBooks { sell_orders: Default::default(), buy_orders: Default::default() },
               base_books_generator().iter().map(|e| Mq5MqlBookInfo { book_type: e.book_type as i32, price: e.price, volume: 0, volume_real: e.volume }).collect::<Vec<_>>(),
               base_books_generator().iter()
                   .map(|mql_book_info| match mql_book_info.book_type {
                       BookTypeSell | BookTypeSellMarket => BookEvents::Add { book: BookParties::Sellers, price: mql_book_info.price, quantity: mql_book_info.volume },
                       BookTypeBuy  | BookTypeBuyMarket  => BookEvents::Add { book: BookParties::Buyers,  price: mql_book_info.price, quantity: mql_book_info.volume },
                       _ => panic!("Unknown book_type '{:?}'", mql_book_info.book_type),
                   })
                   .collect::<Vec<_>>());

        // exercise 'Del' events
        assert("Check 2 (full `old_books` / empty `new_books`)",
               base_books_generator(),
               vec![],
               base_books_generator().iter()
                   .map(|mql_book_info| match mql_book_info.book_type {
                       BookTypeSell | BookTypeSellMarket => BookEvents::Del { book: BookParties::Sellers, price: mql_book_info.price, quantity: mql_book_info.volume },
                       BookTypeBuy  | BookTypeBuyMarket  => BookEvents::Del { book: BookParties::Buyers,  price: mql_book_info.price, quantity: mql_book_info.volume },
                       _ => panic!("Unknown book_type '{:?}'", mql_book_info.book_type),
                   })
                   .collect::<Vec<_>>());

        // exercise 'Update' events -- quantities += 100
        assert("Check 3 (quantity changes)",
               base_books_generator(),
               base_books_generator().iter()
                   .map(|mql_book_info| Mq5MqlBookInfo { book_type: mql_book_info.book_type as i32, price: mql_book_info.price, volume: 0, volume_real: mql_book_info.volume + 100.0, })
                   .collect::<Vec<_>>(),
               base_books_generator().iter()
                   .map(|mql_book_info| match mql_book_info.book_type {
                       BookTypeSell | BookTypeSellMarket => BookEvents::Update { book: BookParties::Sellers, price: mql_book_info.price, delta_quantity: 100.0 },
                       BookTypeBuy  | BookTypeBuyMarket  => BookEvents::Update { book: BookParties::Buyers,  price: mql_book_info.price, delta_quantity: 100.0 },
                       _ => panic!("Unknown book_type '{:?}'", mql_book_info.book_type),
                   })
                   .collect::<Vec<_>>());

        assert("Check 4 ('consumed_buy_price_point': a price point is consumed from the buying orders book -- events: delete that price point + added the next price point at the bottom)",
               base_books_generator(),
               Vec::from(consumed_buy_price_point.0),
               Vec::from(consumed_buy_price_point.1));

        assert("Check 5 ('consumed_sell_price_point': a price point is consumed from the selling orders book -- events: delete that price point + added the next price point at the top)",
               base_books_generator(),
               Vec::from(consumed_sell_price_point.0),
               Vec::from(consumed_sell_price_point.1));

        assert("Check 6 ('consumed_buy_and_sell_price_points': a mix of 'consumed_buy_price_point' && 'consumed_sell_price_point' -- 4 events to be generated in total)",
               base_books_generator(),
               Vec::from(consumed_buy_and_sell_price_points.0),
               Vec::from(consumed_buy_and_sell_price_points.1));

        assert("Check 7 ('price_gaps': price gaps opened up in other places other than the book tops)",
               base_books_generator(),
               Vec::from(price_gaps.0),
               Vec::from(price_gaps.1));


        // production cases
        ///////////////////
        // (that had issues at some point)

        assert("Production case 1 (a Buying price point moves to Selling)",
               OrderBooks {
                   sell_orders: VecDeque::from([
                       MqlBookInfo { book_type: BookTypeSell, price: 23.76, volume: 39500.0 },
                       MqlBookInfo { book_type: BookTypeSell, price: 23.75, volume: 34700.0 },
                       MqlBookInfo { book_type: BookTypeSell, price: 23.74, volume: 32200.0 },
                       MqlBookInfo { book_type: BookTypeSell, price: 23.73, volume: 27400.0 },
                       MqlBookInfo { book_type: BookTypeSell, price: 23.72, volume: 10700.0 }
                   ]), buy_orders: VecDeque::from([
                       MqlBookInfo { book_type: BookTypeBuy, price: 23.71, volume: 9100.0 },
                       MqlBookInfo { book_type: BookTypeBuy, price: 23.70, volume: 267800.0 },
                       MqlBookInfo { book_type: BookTypeBuy, price: 23.69, volume: 29300.0 },
                       MqlBookInfo { book_type: BookTypeBuy, price: 23.68, volume: 50700.0 },
                       MqlBookInfo { book_type: BookTypeBuy, price: 23.67, volume: 31600.0 }
                   ])
               },
               vec![
                   Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.75, volume: 34600, volume_real: 34600.0 },
                   Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.74, volume: 33000, volume_real: 33000.0 },
                   Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.73, volume: 29700, volume_real: 29700.0 },
                   Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.72, volume: 28200, volume_real: 28200.0 },
                   Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 23.71, volume: 9900, volume_real: 9900.0 },
                   Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.69, volume: 14900, volume_real: 14900.0 },
                   Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.68, volume: 30700, volume_real: 30700.0 },
                   Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.67, volume: 15100, volume_real: 15100.0 },
                   Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.66, volume: 24500, volume_real: 24500.0 },
                   Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 23.65, volume: 89500, volume_real: 89500.0 }
               ],
               vec![
                   BookEvents::Del { book: BookParties::Sellers, price: 23.76, quantity: 39500.0 },
                   BookEvents::Update { book: BookParties::Sellers, price: 23.75, delta_quantity: -100.0 },
                   BookEvents::Update { book: BookParties::Sellers, price: 23.74, delta_quantity: 800.0 },
                   BookEvents::Update { book: BookParties::Sellers, price: 23.73, delta_quantity: 2300.0 },
                   BookEvents::Update { book: BookParties::Sellers, price: 23.72, delta_quantity: 17500.0 },
                   BookEvents::Add    { book: BookParties::Sellers, price: 23.71, quantity: 9900.0 },
                   BookEvents::Del    { book: BookParties::Buyers, price: 23.71, quantity: 9100.0 },
                   BookEvents::Del    { book: BookParties::Buyers, price: 23.7, quantity: 267800.0 },
                   BookEvents::Update { book: BookParties::Buyers, price: 23.69, delta_quantity: -14400.0 },
                   BookEvents::Update { book: BookParties::Buyers, price: 23.68, delta_quantity: -20000.0 },
                   BookEvents::Update { book: BookParties::Buyers, price: 23.67, delta_quantity: -16500.0 },
                   BookEvents::Add    { book: BookParties::Buyers, price: 23.66, quantity: 24500.0 },
                   BookEvents::Add    { book: BookParties::Buyers, price: 23.65, quantity: 89500.0 }
               ]);

        assert("Production case 2 (double entries, in the buyers book, for the same price point)",
               OrderBooks {
                   sell_orders: VecDeque::from([
                       MqlBookInfo { book_type: BookTypeSell, price: 45.9, volume: 2600.0 },
                       MqlBookInfo { book_type: BookTypeSell, price: 45.89, volume: 500.0 },
                       MqlBookInfo { book_type: BookTypeSell, price: 45.88, volume: 1100.0 },
                       MqlBookInfo { book_type: BookTypeSell, price: 45.87, volume: 400.0 },
                       MqlBookInfo { book_type: BookTypeSell, price: 45.86, volume: 500.0 }
                   ]), buy_orders: VecDeque::from([
                       MqlBookInfo { book_type: BookTypeBuy, price: 45.84, volume: 400.0 },
                       MqlBookInfo { book_type: BookTypeBuy, price: 45.83, volume: 600.0 },
                       MqlBookInfo { book_type: BookTypeBuy, price: 45.82, volume: 800.0 },
                       MqlBookInfo { book_type: BookTypeBuy, price: 45.81, volume: 1200.0 },
                       MqlBookInfo { book_type: BookTypeBuy, price: 45.8, volume: 1600.0 }
                   ])
               },
               vec![
                   Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 45.91, volume: 800, volume_real: 800.0 },
                   Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 45.9, volume: 2800, volume_real: 2800.0 },
                   Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 45.89, volume: 400, volume_real: 400.0 },
                   Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 45.88, volume: 700, volume_real: 700.0 },
                   Mq5MqlBookInfo { book_type: BookTypeSell as i32, price: 45.87, volume: 400, volume_real: 400.0 },
                   Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 45.85, volume: 500, volume_real: 500.0 },
                   Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 45.84, volume: 500, volume_real: 500.0 },
                   Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 45.83, volume: 700, volume_real: 700.0 },
                   Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 45.82, volume: 800, volume_real: 800.0 },
                   Mq5MqlBookInfo { book_type: BookTypeBuy as i32,  price: 45.81, volume: 1600, volume_real: 1600.0 }
               ],
               vec![
                   BookEvents::Add    { book: BookParties::Sellers, price: 45.91, quantity: 800.0 },
                   BookEvents::Update { book: BookParties::Sellers, price: 45.9, delta_quantity: 200.0 },
                   BookEvents::Update { book: BookParties::Sellers, price: 45.89, delta_quantity: -100.0 },
                   BookEvents::Update { book: BookParties::Sellers, price: 45.88, delta_quantity: -400.0 },
                   BookEvents::Del    { book: BookParties::Sellers, price: 45.86, quantity: 500.0 },
                   BookEvents::Add    { book: BookParties::Buyers, price: 45.85, quantity: 500.0 },
                   BookEvents::Update { book: BookParties::Buyers, price: 45.84, delta_quantity: 100.0 },
                   BookEvents::Update { book: BookParties::Buyers, price: 45.83, delta_quantity: 100.0 },
                   BookEvents::Update { book: BookParties::Buyers, price: 45.81, delta_quantity: 400.0 },
                   BookEvents::Del    { book: BookParties::Buyers, price: 45.80, quantity: 1600.0 },
               ]);

    }
}