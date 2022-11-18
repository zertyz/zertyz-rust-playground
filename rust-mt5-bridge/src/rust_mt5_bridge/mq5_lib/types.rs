//! Types to make it clear, to Rust, that the MQL5 data needs some tweaking before being used


use widestring::U16CString;

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
/// an MT5 String (named `MqlString`) consists of 12 bytes / 3 integers: `(allocated size, low_buffer_ptr, high_buffer_ptr)`. Please refer to [MQ5StringRef] for details
/// on how the bytes in `buffer_ptr` are encoded... but, anyway, you may copy & convert this string to Rust with:
/// ```
/// // provided `mql_string` is defined as `mql_string: MQ5String`:
/// let string = string_from_mql_string(&mql_string);
pub type MQ5String = (/*allocated buffer size*/u32, /*least significant part of the 64bits pointer to the buffer*/u32, /*most significant part of the 64 bits pointer*/u32);

pub fn string_from_mql_string(mql_string: &MQ5String) -> String {
    let ptr_64bit = (((mql_string.2 as u64) << 32) | mql_string.1 as u64) as *const u16;
    // log::debug!("### mql_string ({:?}) -- ({:x}, {:x}, {:x}) was determined as having its buffer at pointer 0x{:x}",
    //             mql_string, mql_string.0, mql_string.1, mql_string.2, alternative_pointer as u64);
    unsafe { U16CString::from_ptr_str(ptr_64bit) }
        .to_string()
        .unwrap_or_else(|_| String::from("««Metatrader's UTF-16 to Rust's UTF-8 conversion FAILED»»"))
}
