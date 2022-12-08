//! Experimental observations showed that enum variants in Metatrader follows
//! arbitrary numbering. This module exists to translate MQL <--> Rust values.
//!
//! 1) Modules in `mq5_lib` should expose their enums by calling `register_enum(enum_descriptor: MqlRustEnumDescriptor)`
//! 2) Each MQL program that loads this DLL should call `set_enum_variant_value(...)` for the mapping to be done
//! 3) Each function exposed by this DLL to Metatrader that receives a struct containing enum fields should map those to `i32`
//!    (before that, they were mapped to the Enums directly, but the discrepancies in the values -- and segfaults for values that
//!     didn't exist -- were what motivated this module)
//! 4) When converting the Metatrader struct to Rust struct, `i32` fields should be resolved with
//!    `resolve_rust_variant(self: &enum_descriptor, mql_variant_value: i32) -> RustEnumType`
//! 5) For the opposite operation (to return a Rust struct containing Enums to Metatrader):
//!    `resolve_mql_variant(self: &enum_descriptor, rust_variant: RustEnumType) -> i32`
//!
//!
//! # Implementation notes:
//!
//! The Metatrader script should call this module once, when the DLL is loaded, by a single thread -- as we use statics
//! to hold the enum names and no mutexes in them. This is easily achieved by running the MQL code when `handle_id` 0
//! is given out by this DLL.

use strum::{EnumString, FromRepr};
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;
use once_cell::sync::{Lazy};
use log::{warn};

/// Thread-unsafe repository for all registered enums mapped by this module
/// (see [module](super::mql_rust_enum) docs for details)
static mut MQL_RUST_ENUM_DESCRIPTORS: Lazy<HashMap<String, MqlRustEnumDescriptor>> = Lazy::new(|| HashMap::with_capacity(32));


/// Rust code that is mapping MQL enums should register such enums using this structure.\
/// Example:
/// ```
///     let mql_rust_enum_descriptor = MqlRustEnumDescriptor::new("MqlEnumMappedToRust", &MqlEnumMappedToRust::from_str);
/// ```
/// NOTE: the enum should be declared like this:
/// ```
///     #[derive(Debug,PartialEq,EnumString,FromRepr)]
///     enum MqlEnumMappedToRust {
///         Soft,
///         Hard,
///         UnknownMqlVariantValue,
///     }
///     // `strum` could implement `IntoRepr` -- but it, currently, doesn't
///     // so all Mql enums mapped to Rust must do this:
///     impl Into<i32> for MqlEnumMappedToRust {
///         fn into(self) -> i32 {
///             self as i32
///         }
///     }
///     // `strum`'s docs says `FromRepr` also implements the TryFrom trait, but it doesn't.
///     impl From<i32> for MqlEnumMappedToRust {
///         fn from(variant_value: i32) -> Self {
///             if variant_value != -1 {
///                 if let Some(variant) = Self::from_repr(variant_value as usize) {
///                     return variant;
///                 }
///             }
///             Self::UnknownMqlVariantValue
///         }
///     }
pub struct MqlRustEnumDescriptor {
    /// The enum name, as known in Rust -- the MQL name can be automatically determined from this:
    /// MQL enum names are upper cased snake case
    rust_enum_name: String,
    /// MQL variant values as indexes and the Rust variant values as the values stored in this vector
    mql_to_rust_variants: Vec<i32>,
    /// Rust variant values as indexes and the MQL variant values as the values stored in this vector
    rust_to_mql_variants: Vec<i32>,
    /// The function pointer to `YourEnum::from_str(...)` to convert the Rust variant name `&str` to the Rust variant value `i32`/
    /// NOTE: When the Enum is declared with `#[derive(strum::EnumString)]` the `from_str()` function will be available
    variant_name_to_i32: Box<dyn Fn(&str) -> Option<i32> + Sync + Send>,
}

impl MqlRustEnumDescriptor {

    /// Instantiates & non-reentrantly registers (on the static mapping) the given enum descriptor, returning a reference to it
    pub fn new<IntoString: Into<String>, RustEnumType: Into<i32>>(rust_enum_name: IntoString, from_str_fn: impl Fn(&str) -> Result<RustEnumType, strum::ParseError> + Sync + Send + 'static) -> &'static Self {
        let descriptor = Self {
            rust_enum_name: rust_enum_name.into(),
            mql_to_rust_variants: Vec::new(),
            rust_to_mql_variants: Vec::new(),
            variant_name_to_i32: Box::new(move |str| if let Ok(enum_value) = from_str_fn(str) {
                                                               Some(enum_value.into())
                                                           } else {
                                                                None
                                                           }),
        };
        register_mql_rust_enum_descriptor(descriptor)
    }

    /// Given the `mql_variant_value` (previously registered with [set_enum_variant_value()]), will return the Rust Variant or `RustEnumType::UnknownMqlVariantValue`
    /// if the given value is unknown
    pub fn resolve_rust_variant<RustEnumType: From<i32>>(&self, mql_variant_value: i32) -> RustEnumType {
log::debug!("### out of MQL value of {}, RESOLVING Rust enum from {}", mql_variant_value, self.debug());
        if let Some(rust_variant_value) = self.mql_to_rust_variants.get(mql_variant_value as usize) {
            RustEnumType::from(*rust_variant_value)
        } else {
            RustEnumType::from(-1)  // remember: all Rust mappings of MQL enums must have a 'UnknownMqlVariantValue' variant, which should be mapped to -1
        }
    }

    pub fn name(&self) -> &str {
        &self.rust_enum_name
    }

    pub fn debug(&self) -> String {
        let Self { rust_enum_name, mql_to_rust_variants, rust_to_mql_variants, .. } = self;
        format!("MqlRustEnumDescriptor {{ rust_enum_name: '{rust_enum_name}'; mql_to_rust_variants: {:?}; rust_to_mql_variants: {:?} }}",
                 mql_to_rust_variants, rust_to_mql_variants)
    }

}

/// Non-reentrantly registers the given enum descriptor, returning a reference to it
fn register_mql_rust_enum_descriptor(mql_rust_enum_descriptor: MqlRustEnumDescriptor) -> &'static MqlRustEnumDescriptor {
    unsafe {
        MQL_RUST_ENUM_DESCRIPTORS.entry(mql_rust_enum_descriptor.rust_enum_name.clone())
            .and_modify(|previous_descriptor| {
                warn!("IMPLEMENTATION ISSUE: mql_rust_enum_descriptor for enum '{}' was registered twice! FIX THIS!", previous_descriptor.rust_enum_name);
            })
            .or_insert(mql_rust_enum_descriptor)
    }
}

/// Consults the static mapping for the given enum name, returning a reference to it or `None`, if it isn't found./
/// This is thread-safe as long as [MqlRustEnumDescriptor::new()] isn't called
pub fn get_mql_rust_enum_descriptor(rust_enum_name: &str) -> Option<&'static mut MqlRustEnumDescriptor> {
    unsafe {
        MQL_RUST_ENUM_DESCRIPTORS.get_mut(rust_enum_name)
    }
}

/// MQL will call this to inform the variant values.\
/// If `Err`, a message is returned that should be shown in the Metatrader terminal and cause the MQL Program to fail to init
pub fn set_enum_variant_value(rust_enum_name: &str, rust_variant_name: &str, mql_variant_value: i32) -> Result<(), String>{
    if let Some(enum_descriptor) = get_mql_rust_enum_descriptor(rust_enum_name) {
        if let Some(rust_variant_value) = (enum_descriptor.variant_name_to_i32)(rust_variant_name) {
            // set mql=>rust mapping
            while enum_descriptor.mql_to_rust_variants.len() <= mql_variant_value as usize {
                enum_descriptor.mql_to_rust_variants.push(-1);
            }
            enum_descriptor.mql_to_rust_variants[mql_variant_value as usize] = rust_variant_value;
            // set rust=>mql mapping
            while enum_descriptor.rust_to_mql_variants.len() <= rust_variant_value as usize {
                enum_descriptor.rust_to_mql_variants.push(-1);
            }
            enum_descriptor.rust_to_mql_variants[rust_variant_value as usize] = mql_variant_value;
            Ok(())
        } else {
            let message = format!("Attempted to set the variant `{rust_variant_name} = mql_value {mql_variant_value}` for enum '{rust_enum_name}' -- but that variant isn't known on the Rust side. FIX IT!");
            Err(message)
        }
    } else {
        let message = format!("Attempted to set the variant `{rust_variant_name} = mql_value {mql_variant_value}` for enum '{rust_enum_name}' -- but that enum isn't registered. FIX IT! Known ENUMs are: {:?}",
                                     unsafe{MQL_RUST_ENUM_DESCRIPTORS.iter()}.map(|(k, _v)| k).collect::<Vec<_>>());
        Err(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[derive(Debug,PartialEq,EnumString,FromRepr)]
    enum MqlEnumMappedToRust {
        Soft,
        Hard,
        UnknownMqlVariantValue,
    }
    // strum could implement IntoRepr -- but it, currently, doesn't
    // so all Mql enums mapped to Rust must do this:
    impl Into<i32> for MqlEnumMappedToRust {
        fn into(self) -> i32 {
            self as i32
        }
    }
    // `strum`'s docs says `FromRepr` also implements the TryFrom trait, but it doesn't.
    impl From<i32> for MqlEnumMappedToRust {
        fn from(variant_value: i32) -> Self {
            if variant_value != -1 {
                if let Some(variant) = Self::from_repr(variant_value as usize) {
                    return variant;
                }
            }
            Self::UnknownMqlVariantValue
        }
    }

    /// Dummy test, implementation dependent... for sheer inspection
    #[test]
    fn spikes() {
        let enum_name = "MqlEnumMappedToRust__spikes";
        // inputs
        let mql_soft: i32 = 8;
        let mql_hard: i32 = 80;
        let rust_soft: i32 = MqlEnumMappedToRust::Soft.into();
        let rust_hard: i32 = MqlEnumMappedToRust::Hard.into();
        // enum registration
        let mql_rust_enum_descriptor = MqlRustEnumDescriptor::new(enum_name, &MqlEnumMappedToRust::from_str);
        // mql variant values registration
        set_enum_variant_value(enum_name, "Soft", 8)
            .expect("Setting the MQL variant value of a Rust-known variant for a previously registered enum");
        set_enum_variant_value(enum_name, "Hard", 80)
            .expect("Setting the MQL variant value of a Rust-known variant for a previously registered enum");
        // checks on the internal structures
        assert_eq!(mql_rust_enum_descriptor.mql_to_rust_variants.get(mql_soft as usize), Some(&rust_soft), "Mql=>Rust: 'Soft' variant wasn't correctly mapped");
        assert_eq!(mql_rust_enum_descriptor.mql_to_rust_variants.get(mql_hard as usize), Some(&rust_hard), "Mql=>Rust: 'Hard' variant wasn't correctly mapped");
        assert_eq!(mql_rust_enum_descriptor.rust_to_mql_variants.get(rust_soft as usize), Some(&mql_soft), "Rust=>Mql: 'Soft' variant wasn't correctly mapped");
        assert_eq!(mql_rust_enum_descriptor.rust_to_mql_variants.get(rust_hard as usize), Some(&mql_hard), "Rust=>Mql: 'Hard' variant wasn't correctly mapped");
        dbg!(&mql_rust_enum_descriptor.rust_enum_name);
        dbg!(&mql_rust_enum_descriptor.mql_to_rust_variants);
        dbg!(&mql_rust_enum_descriptor.rust_to_mql_variants);
    }

    /// The happy-path
    #[test]
    fn simple_mapping() {
        let enum_name = "MqlEnumMappedToRust__simple_mapping";
        // inputs
        let mql_soft: i32 = 8;
        let mql_hard: i32 = 80;
        // enum registration
        let mql_rust_enum_descriptor = MqlRustEnumDescriptor::new(enum_name, &MqlEnumMappedToRust::from_str);
        // mql variant values registration
        set_enum_variant_value(enum_name, "Soft", 8)
            .expect("Setting the MQL variant value of a Rust-known variant for a previously registered enum");
        set_enum_variant_value(enum_name, "Hard", 80)
            .expect("Setting the MQL variant value of a Rust-known variant for a previously registered enum");
        // resolving from MQL enum values to Rust variants
        let resolved_soft: MqlEnumMappedToRust = mql_rust_enum_descriptor.resolve_rust_variant(mql_soft);
        let resolved_hard: MqlEnumMappedToRust = mql_rust_enum_descriptor.resolve_rust_variant(mql_hard);
        assert_eq!(resolved_soft, MqlEnumMappedToRust::Soft, "Resolving a MQL variant value as Rust enum variant didn't work!");
        assert_eq!(resolved_hard, MqlEnumMappedToRust::Hard, "Resolving a MQL variant value as Rust enum variant didn't work!");
    }

    /// some foreseen erroneous usage patterns and the meaningful feedback they should yield
    #[test]
    fn error_cases() {

        // unregistered enum
        assert!(get_mql_rust_enum_descriptor("MqlEnumMappedToRust").is_none(),
                "Attempting to resolve the descriptor of an unregistered enum should result in `None`");

        // mql variant not mapped to Rust
        MqlRustEnumDescriptor::new("MqlEnumMappedToRust", &MqlEnumMappedToRust::from_str);
        let mql_rust_enum_descriptor = get_mql_rust_enum_descriptor("MqlEnumMappedToRust")
            .expect("Retrieving the descriptor of a previously registered Enum");
        assert_eq!(mql_rust_enum_descriptor.resolve_rust_variant::<MqlEnumMappedToRust>(969696), MqlEnumMappedToRust::UnknownMqlVariantValue,
                   "An unregistered MQL variant value should resolve to the `::UnknownMqlVariantValue` variant");
        assert_eq!(mql_rust_enum_descriptor.resolve_rust_variant::<MqlEnumMappedToRust>(-1),     MqlEnumMappedToRust::UnknownMqlVariantValue,
                   "As a convention, the MQL variant value of -1 is reserved to the Rust `::UnknownMqlVariantValue` variant");

        // setting a value for an unknown-to-Rust variant
        match set_enum_variant_value("MqlEnumMappedToRust", "ThisVariantDoesNotExistInRust", 696969) {
            Ok(())                    => panic!("Setting a value for an unknown variant was just attempted. This should have failed!"),
            Err(error_message) => assert_eq!(error_message, "Attempted to set the variant `ThisVariantDoesNotExistInRust = mql_value 696969` for enum 'MqlEnumMappedToRust' -- but that variant isn't known on the Rust side. FIX IT!",
                                                    "WRONG ERROR MESSAGE"),
        }

        // setting a value for a variant of an unregistered enum
        match set_enum_variant_value("ThisEnumWasNotRegistered", "WhatSoEver", 696969) {
            Ok(())                    => panic!("Setting a varian'ts value for an unregistered enum was just attempted. This should have failed!"),
            Err(error_message) => assert!(error_message.starts_with("Attempted to set the variant `WhatSoEver = mql_value 696969` for enum 'ThisEnumWasNotRegistered' -- but that enum isn't registered. FIX IT!"),
                                                    "WRONG ERROR MESSAGE"),
        }

    }
}


