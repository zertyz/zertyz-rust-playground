//! Contains conditional compilation definitions attending to:
//!   - "features" definitions, client project's Cargo "[dependencies]" declarations
//!   - Release / Debug compilations

use std::io::{stdout, Write, stderr};
use std::io;

#[cfg(debug_assertions)]
/// busy waiting constant for debug & release compilation
pub const BUSY_LOOP_DELAY: u32 = 9999;
#[cfg(not(debug_assertions))]
/// busy waiting constant for debug & release compilation
pub const BUSY_LOOP_DELAY: u32 = 9999999;

// if features = stdout
pub const OUTPUT: fn(&str) = stdout_write;

fn stdout_write(buf: &str) {
    stdout().write(buf.as_bytes());
    io::stdout().flush().unwrap();
}

fn stderr_write(buf: &str) {
    stderr().write(buf.as_bytes());
    io::stdout().flush().unwrap();
}

fn null_write(_buf: &str) {
    // release compilations will optimize out this call for '_buf' is not used
}

