use std::ffi::{c_char, CStr, CString};
use widestring::{U16CString, WideCString, WideString};

mod rust_mt5_bridge;
use rust_mt5_bridge::types::MqlTick;

fn main() {
    println!("Calling DLLMain...");
    rust_mt5_bridge::DllMain(0 as *const (),
                             1,
                             0 as *const ());
    println!("Calling dll register function...");
    rust_mt5_bridge::register_trading_expert_advisor_for_production(U16CString::from_str("coco?").unwrap().as_ptr(),
                                                                    U16CString::from_str("coco?").unwrap().as_ptr(),
                                                                    U16CString::from_str("coco?").unwrap().as_ptr());
    println!("Calling dll on_tick() function...");
    let mql_tick = MqlTick::default();
    rust_mt5_bridge::on_tick(0, &mql_tick as *const MqlTick);
    println!("Calling dll on_tick() function again...");
    rust_mt5_bridge::on_tick(0, &mql_tick as *const MqlTick);
    println!("Done. Where are the logs?");
}