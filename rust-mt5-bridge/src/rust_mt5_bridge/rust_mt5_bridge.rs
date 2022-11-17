use super::types::*;

use std::ffi::{c_char, CStr, CString, OsStr, OsString};
use std::fs;
use std::io::Write;
use std::iter::Iterator;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering::Relaxed;

use once_cell::sync::Lazy;
use widestring::{U16CString, WideCStr, WideCString, WideString};
use log::{error, info, LevelFilter};


const MAX_HANDLES: i32 = 1024;

// Runtime (static) data
////////////////////////

static HANDLE_COUNT: AtomicI32 = AtomicI32::new(0);
/// Keeps track of `handle_id`s conceived to clients (MT5 scripts)
static mut HANDLES: Vec<Handle> = Vec::new();

/// See the docs docs for this function in https://learn.microsoft.com/en-us/windows/win32/dlls/dllmain
#[no_mangle]
pub extern "system" fn DllMain(_: *const (), fdw_reason: u32, _: *const ()) -> u32 {
    match fdw_reason {
        0 => info!("DllMain() called for reason 0: DLL_PROCESS_DETACH -- loading failed and the DLL is being completely unloaded"),
        1 => {
            let log_file_path = "rust_mt5_bridge.log";
            let config = simple_log::LogConfigBuilder::builder()
                .path(log_file_path)
                .size(MAX_LOG_FILE_SIZE_MB)
                .roll_count(MAX_LOG_FILES)
                .time_format("%H:%M:%S.%f")
                .level(LOG_LEVEL)
                .output_file()
                .build();

            simple_log::new(config)
                .expect("instantiating simplelog file writer");
            info!("'rust_mt5_bridge.dll' was loaded and started -- allowing up to {MAX_HANDLES} handles (Expert Advisors, Indicators, Testers, etc.) to be created -- removing them won't free resources (restarting Metatrader will)");
            info!("DllMain() called for reason 1: DLL_PROCESS_ATTACH -- DLL was loaded!");
            unsafe {
                // ensures our `HANDLES` is thread-safe and don't need a mutex
                for i in 0..MAX_HANDLES {
                    HANDLES.push(Handle {
                        client_type: ClientType::ProductionExpertAdvisor,
                        account_token: "".to_string(),
                        algorithm: "".to_string(),
                        symbol: "".to_string()
                    });
                }
            }
        },
        2 => info!("DllMain() called for reason 2: DLL_THREAD_ATTACH -- host process just created another thread"),
        3 => info!("DllMain() called for reason 3: DLL_THREAD_DETACH -- host process just ended one of its threads"),
        n => info!("DllMain() called for unknown reason {n}"),
    }
    1   // = TRUE: the DLL is good with the reported event
}

/// Called by the `OnInit()` to inform the market data for the symbol being considered
/// (as well the session information for operation) and to get the `handle` to be passed
/// to all the other functions here -- if negative, it indicates an error code and the
/// loading of the MT5 script must be cancelled
#[no_mangle]
pub extern fn register_trading_expert_advisor_for_production(account_token: *const u16, algorithm: *const u16, symbol: *const u16) -> i32 {
    let account_token = unsafe { U16CString::from_ptr_str(account_token) }.to_string().unwrap_or(String::from("ERROR CONVERTING `account_token` -- a supposedly UTF-16 Metatrader 5 String to a UTF-8 Rust String"));
    let algorithm = unsafe { U16CString::from_ptr_str(algorithm) }.to_string().unwrap_or(String::from("ERROR CONVERTING `algorithm` -- a supposedly UTF-16 Metatrader 5 String to a UTF-8 Rust String"));
    let symbol = unsafe { U16CString::from_ptr_str(symbol) }.to_string().unwrap_or(String::from("ERROR CONVERTING `symbol` -- a supposedly UTF-16 Metatrader 5 String to a UTF-8 Rust String"));

    let handle = Handle {
        client_type: ClientType::ProductionExpertAdvisor,
        account_token,
        algorithm,
        symbol,
    };

    let handle_id = HANDLE_COUNT.fetch_add(1, Relaxed);
    if handle_id >= MAX_HANDLES {
        error!("OnInit: FAILED registering trading expert advisor for PRODUCTION: {:?} -- exhausted handles {handle_id} where the max is {MAX_HANDLES}", handle);
        -1
    } else {
        info!("OnInit: registering trading expert advisor for PRODUCTION: {:?} -- attributed handle_id: {handle_id}", handle);
        unsafe { HANDLES[handle_id as usize] = handle; }
        handle_id
    }
}

/// Called by `OnDeinit()` or `OnTesterDeinit()` when the MT5 script is ending.\
/// IMPORTANT: tradeoff decision: this function doesn't clean resources -- this way we don't lose speed requiring a Mutex for `HANDLES`.
///            As a consequence, MT5 must be restarted every day.\
///            (In the future, we may mark the slot as vacant and search for vacant ones when registering)
#[no_mangle]
pub extern fn unregister_trading_expert_advisor(handle_id: i32, reason_id: i32) {
    let handle = unsafe { &HANDLES[handle_id as usize] };
    info!("OnDeinit/OnTesterDeinit: unregistering trading expert advisor for `handle_id` #{handle_id}: {:?}", handle);
}

/// Called by the `OnInit()` to inform the market data for the symbol being considered
/// (as well the session information for operation) and to get the `handle` to be passed
/// to all the other functions here -- if negative, it indicates an error code and the
/// loading of the MT5 script must be cancelled
#[no_mangle]
pub extern fn register_trading_expert_advisor_for_testing(account_token: *const u16, algorithm: *const u16, symbol: *const u16) -> i32 {
    let account_token = unsafe { U16CString::from_ptr_str(account_token) }.to_string().unwrap_or(String::from("ERROR CONVERTING `account_token` -- a supposedly UTF-16 Metatrader 5 String to a UTF-8 Rust String"));
    let algorithm = unsafe { U16CString::from_ptr_str(algorithm) }.to_string().unwrap_or(String::from("ERROR CONVERTING `algorithm` -- a supposedly UTF-16 Metatrader 5 String to a UTF-8 Rust String"));
    let symbol = unsafe { U16CString::from_ptr_str(symbol) }.to_string().unwrap_or(String::from("ERROR CONVERTING `symbol` -- a supposedly UTF-16 Metatrader 5 String to a UTF-8 Rust String"));

    let handle = Handle {
        client_type: ClientType::TestingExpertAdvisor,
        account_token,
        algorithm,
        symbol,
    };

    let handle_id = HANDLE_COUNT.fetch_add(1, Relaxed);
    if handle_id >= MAX_HANDLES {
        error!("OnTesterInit: FAILED registering trading expert advisor for TESTING: {:?} -- exhausted handles {handle_id} where the max is {MAX_HANDLES}", handle);
        -1
    } else {
        info!("OnInit: registering trading expert advisor for TESTING: {:?} -- attributed handle_id: {handle_id}", handle);
        unsafe { HANDLES[handle_id as usize] = handle; }
        handle_id
    }
}

/// Called to inform details over the symbol under negotiation./
/// Typically consulted once per session per symbol, at the start.
#[no_mangle]
pub extern fn report_symbol_info(handle_id: i32, symbol_info: *const SymbolInfoBridge) {
    let handle = unsafe { &HANDLES[handle_id as usize] };
    let symbol_info = SymbolInfoBridge::from_ptr_to_internal(symbol_info);
    info!("report_symbol_info({handle_id}): {}: {:#?}", handle.symbol, symbol_info);
}

/// Called to inform details for the account used to make the negotiations./
/// Typically consulted after every order issued / executed / edited / cancelled.
#[no_mangle]
pub extern fn report_account_info(handle_id: i32, account_info: *const AccountInfoBridge) {
    let handle = unsafe { &HANDLES[handle_id as usize] };
    let account_info = AccountInfoBridge::from_ptr_to_internal(account_info);
    info!("report_account_info({handle_id}): {}: {:#?}", handle.symbol, account_info);
}


/// Called when there are new Quotes available.\
/// This function should return as fast as possible, or else new ticks will be lost (they are not enqueued).
/// See the docs https://www.mql5.com/en/docs/event_handlers/ontick
#[no_mangle]
pub extern fn on_tick(handle_id: i32, tick: *const MqlTick) {
    let handle = unsafe { &HANDLES[handle_id as usize] };
    let tick = unsafe { &*tick };
info!("OnTick({handle_id}): {}: {:?}", handle.symbol, tick);
    match tick.to_event() {
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
pub extern fn on_trade(handle_id:            u32,
                       pending_orders_count: u32,
                       open_positions_count: u32) {
    let handle = unsafe { &HANDLES[handle_id as usize] };
    let symbol = &handle.symbol;
    info!("OnTrade: handle_id: {handle_id}, symbol: '{symbol}', pending_orders_count: {pending_orders_count}, open_positions_count: {open_positions_count}");
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

// additional events to consider implementing in the future:
//
//  * https://www.mql5.com/en/docs/basis/function/events
//  * https://www.mql5.com/en/docs/event_handlers/ontradetransaction
//    -- can be used to sync the states of executed orders, opened positions, etc
// void  OnTradeTransaction()
//    const MqlTradeTransaction&    trans,     // trade transaction structure
//    const MqlTradeRequest&        request,   // request structure
//    const MqlTradeResult&         result     // response structure
//    );
//


/// are we compiled in DEBUG or RELEASE mode?
#[cfg(debug_assertions)]
pub const DEBUG: bool = true;
#[cfg(not(debug_assertions))]
pub const DEBUG: bool = false;

/// Keep those levels in sync with Cargo.toml's `log` crate levels defined in features.
/// Example: features = ["max_level_debug", "release_max_level_info"]
const LOG_LEVEL: &str = if DEBUG {
    "debug"
} else {
    "info"
};
const MAX_LOG_FILE_SIZE_MB: u64 = 2*1024;
const MAX_LOG_FILES: u32 = 22;

/// to be called when debugging logging issues
fn internal_logger(path: &str, contents: &str) {
    fs::File::options().create(true).append(true)
        .open(path).expect("open internal log file for appending")
        .write_all(contents.as_bytes()).expect("write to internal log file");
}