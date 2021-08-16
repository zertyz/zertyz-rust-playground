//! Contains conditional compilation definitions attending to:
//!   - "features" definitions, client project's Cargo "[dependencies]" declarations
//!   - Release / Debug compilations

#![warn(dead_code)]

use std::io::{stdout,stderr,Write};

#[cfg(debug_assertions)]
/// busy waiting constant for debug & release compilation
pub const BUSY_LOOP_DELAY: u32 = 9999;
#[cfg(not(debug_assertions))]
/// busy waiting constant for debug & release compilation
pub const BUSY_LOOP_DELAY: u32 = 9999999;

// if features = stdout
pub const OUTPUT: fn(&str) = stdout_write;

fn stdout_write(buf: &str) {
    stdout().flush().unwrap();
    stderr().flush().unwrap();
    print!("{}", buf);
    stdout().flush().unwrap();
    stderr().flush().unwrap();
}

fn stderr_write(buf: &str) {
    stdout().flush().unwrap();
    stderr().flush().unwrap();
    eprint!("{}", buf);
    stdout().flush().unwrap();
    stderr().flush().unwrap();
}

fn null_write(_buf: &str) {
    // release compilations will optimize out this call for '_buf' is not used
}

