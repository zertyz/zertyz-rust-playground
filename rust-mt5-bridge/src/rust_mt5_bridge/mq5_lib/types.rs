//! Types to make it clear, to Rust, that the MQL5 data needs some tweaking before being used


/// Number of seconds since January 01, 1970./
/// useful with `let datetime = NaiveDateTime::from_timestamp(mq5_datetime as i64, 0);`
pub type MQ5DateTime = u64;

/// First byte is ignored -- remaining bytes are RGB
pub type MQ5Color = i32;

/// A reference to a Metatrader 'string' -- which is, actually, a reference to the string chars themselves, which are zero-terminated and are UTF-16 encoded./
/// Remember that MQL5 uses string references as parameters for function calls -- but not for struct members./
/// Use it with:/
/// ```
/// // given that `param1` is of type `*const u16`
/// let param1 = unsafe { U16CString::from_ptr_str(param1) }
///     .to_string().unwrap_or(String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED for 'param1' »»"));
pub type MQ5StringRef = *const u16;

/// According to the Metatrader documentation at https://docs.mql4.com/basis/types/stringconst (available for MT4 but useful for MT5 as well),
/// an MT5 String (named `MqlString`) consists of 12 bytes / 3 integers: `(size, buffer_ptr, reserved)`. Please refer to [MQ5StringRef] for details
/// on how the bytes in `buffer_ptr` are encoded... but, anyway, you may copy & convert this string to Rust with:
/// ```
/// // note: this code was built to work in both 32 & 64bit binaries, even if the MQLString offers only a 32bit pointer
/// let string_from_mql_string = |mql_string: &MQ5String| -> String {
///     let base_ptr = std::ptr::addr_of!(symbol_info_bridge.symbol_basis) as u64 & (0xFFFFFFFF00000000 as u64);
///     let ptr_64bit: *const u16 = (base_ptr | (mql_string.1 as u64)) as *const u16;
///     unsafe { U16CString::from_ptr_str(ptr_64bit) }
///         .to_string()
///         .unwrap_or_else(|_| String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED»»"))
/// };
pub type MQ5String = (/*size*/u32, /*32bit pointer to the buffer*/u32, /*reserved*/u32);
