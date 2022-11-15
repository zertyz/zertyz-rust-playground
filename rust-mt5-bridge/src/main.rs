mod rust_mt5_bridge;

use std::collections::HashMap;
use std::error::Error;
use rust_mt5_bridge::types::MqlTick;

use std::ffi::{c_char, CStr, CString};
use std::io;
use std::io::{BufRead, Write};
use std::process::{ExitCode, Termination};
use widestring::{U16CString, WideCString, WideString};
use regex::Regex;

fn main() -> Result<ExitCode, Box<dyn Error>> {

    println!("rust-mt5-bridge manual testing tool");
    println!("===================================");
    println!("Use it to replay production log files, helping in the event data interpretation evolutions");
    println!("");
    println!("Example:");
    println!("    ./target/release/main < rust_mt5_bridge.log");
    println!("");


    // initialize the DLL, just like Metatrader 5 would
    rust_mt5_bridge::DllMain(0 as *const (),
                             1,
                             0 as *const ());

    let mut handle_for_symbol = HashMap::<String, i32>::new();

    let stdin = io::stdin();

    // recognized log lines
    let on_init_regex = Regex::new(r#"OnInit: registering trading expert advisor for PRODUCTION.*account_token: "(.+?)", algorithm: "(.+?)", symbol: "(.+?)".*"#).expect("OnInit() regex compilation");
    let on_tick = Regex::new(r#"OnTick\(\d+\): ([^:]+): * MqlTick \{ time: ([^,]+), bid: ([^,]+), ask: ([^,]+), last: ([^,]+), volume: ([^,]+), time_msc: ([^,]+), flags: ([^,]+), volume_real: ([^ ]+) \}"#).expect("OnTick() regex compilation");

    // read all log lines sent to stdin
    let mut line_number = 0;
    for line in stdin.lock().lines() {
        line_number += 1;
        let line = line.expect(r#"&format!("at line #{line_number}")"#);

        if let Some(captures) = on_init_regex.captures(line.as_str()) {
            let account_token = captures.get(1).expect(&format!("'account_token' wasn't captured for line #{line_number}: '{}'", line)).as_str();
            let algorithm = captures.get(2).expect(&format!("'algorithm' wasn't captured for line #{line_number}: '{}'", line)).as_str();
            let symbol = captures.get(3).expect(&format!("'symbol' wasn't captured for line #{line_number}: '{}'", line)).as_str();

            // register the handle for the expert advisor
            let handle = rust_mt5_bridge::register_trading_expert_advisor_for_production(U16CString::from_str(account_token).unwrap().as_ptr(),
                                                                                              U16CString::from_str(algorithm).unwrap().as_ptr(),
                                                                                              U16CString::from_str(symbol).unwrap().as_ptr());
            handle_for_symbol.insert(symbol.to_string(), handle);

        } else if let Some(captures) = on_tick.captures(line.as_str()) {

            let symbol = captures.get(1).expect(r#"&format!("'symbol' wasn't captured for line #{line_number}: '{}'", line)"#).as_str();
            let handle = handle_for_symbol.get(symbol).expect(r#"&format!("handle not found for symbol '{symbol}' at line #{line_number}: '{}'", line)"#);
            let mql_tick = MqlTick {
                time: captures.get(2).expect(r#"&format!("'time' wasn't captured for line #{line_number}: '{}'", line)"#).as_str().parse::<u64>().expect(r#"&format!("could not parse 'time' as u64 at line #{line_number}: '{}'", line)"#),
                bid:  captures.get(3).expect(r#"&format!("'bid' wasn't captured for line #{line_number}: '{}'", line)"#).as_str().parse::<f64>().expect(r#"&format!("could not parse 'bid' as f64 at line #{line_number}: '{}'", line)"#),
                ask: captures.get(4).expect(r#"&format!("'ask' wasn't captured for line #{line_number}: '{}'", line)"#).as_str().parse::<f64>().expect(r#"&format!("could not parse 'ask' as f64 at line #{line_number}: '{}'", line)"#),
                last: captures.get(5).expect(r#"&format!("'last' wasn't captured for line #{line_number}: '{}'", line)"#).as_str().parse::<f64>().expect(r#"&format!("could not parse 'last' as f64 at line #{line_number}: '{}'", line)"#),
                volume: captures.get(6).expect(r#"&format!("'volume' wasn't captured for line #{line_number}: '{}'", line)"#).as_str().parse::<u64>().expect(r#"&format!("could not parse 'volume' as u64 at line #{line_number}: '{}'", line)"#),
                time_msc: captures.get(7).expect(r#"&format!("'time_msc' wasn't captured for line #{line_number}: '{}'", line)"#).as_str().parse::<i64>().expect(r#"&format!("could not parse 'time_msc' as i64 at line #{line_number}: '{}'", line)"#),
                flags: captures.get(8).expect(r#"&format!("'flags' wasn't captured for line #{line_number}: '{}'", line)"#).as_str().parse::<u32>().expect(r#"&format!("could not parse 'flags' as u32 at line #{line_number}: '{}'", line)"#),
                volume_real: captures.get(9).expect(r#"&format!("'volume_real' wasn't captured for line #{line_number}: '{}'", line)"#).as_str().parse::<f64>().expect(r#"&format!("could not parse 'volume_real' as f64 at line #{line_number}: '{}'", line)"#),
            };
            rust_mt5_bridge::on_tick(*handle, &mql_tick as *const MqlTick);

        }

        if line_number % 8192 == 0 {
            print!("\r{line_number} lines reprocessed");
            io::stdout().flush().unwrap();
        }
    }

    println!("\r{line_number} lines reprocessed.");

    // deinitialize the DLL, just as Metatrader 5 would
    rust_mt5_bridge::DllMain(0 as *const (),
                             3,
                             0 as *const ());

    Ok(ExitCode::SUCCESS)
}