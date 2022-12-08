//! Mapping of https://www.mql5.com/en/docs/constants/structures/mqlbookinfo to Rust


use super::{
    super::mql_rust_enum::{MqlRustEnumDescriptor},
};
use std::str::FromStr;
use std::fmt::{Debug};
use once_cell::sync::Lazy;
use strum::{EnumString,FromRepr};


/// Representation of the Metatrader 5 `MqlBookInfo` structure. From the site:/
/// It provides information about the market depth data.\
/// auto-generated from https://www.mql5.com/en/docs/constants/structures/mqlbookinfo
#[repr(C, packed(4))]
#[derive(/*disable debug on this structure for production since it will cause a copy due to 'packed(4)' above*/Debug,Copy,Clone)]
pub struct Mq5MqlBookInfo {
    /// Order type from ENUM_BOOK_TYPE enumeration
    pub book_type: i32, //EnumBookType,
    /// Price -- just an `f64`, but outside Rust's 8-byte alignment requirements for the type
    pub price: f64,
    /// Volume
    pub volume: i64,
    /// Volume with greater accuracy
    pub volume_real: f64,
}
impl Mq5MqlBookInfo {
    /// Copies the reference to the original [Mq5MqlTick] struct to its Rust optimized version -- which may
    /// be worked on later, as the original reference must be returned (as fast as possible) to be reused
    pub fn to_internal(&self) -> MqlBookInfo {
        MqlBookInfo {
            book_type: ENUM_BOOK_TYPE.resolve_rust_variant(self.book_type),
            price:     self.price,
            volume:    self.volume_real,
        }
    }
}


/// Rust version of the internal Metatrader 5 struct [Mq5MqlBookInfo], with correct alignment and redundant fields removed
#[derive(Debug,PartialEq)]
pub struct MqlBookInfo {
    /// Order type from ENUM_BOOK_TYPE enumeration
    pub book_type: EnumBookType,
    /// Price
    pub price:     f64,
    /// Volume
    pub volume:    f64,
}


/// To obtain information about the current state of the DOM by MQL5 means, the MarketBookGet() function is used, which places the DOM &quot;screen shot&quot; into the MqlBookInfo array of structures. Each element of the array in the type field contains information about the direction of the order - the value of the ENUM_BOOK_TYPE enumeration./
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/enum_book_type
#[repr(i32)]
#[derive(Debug,PartialEq,EnumString,FromRepr,Clone,Copy)]
pub enum EnumBookType {
    /// Sell order (Offer)
    BookTypeSell,
    /// Buy order (Bid)
    BookTypeBuy,
    /// Sell order by Market
    BookTypeSellMarket,
    /// Buy order by Market
    BookTypeBuyMarket,

    /// in case MQL Code is out of sync with the DLL version...
    UnknownMqlVariantValue = -1,
}
impl Into<i32> for EnumBookType {
    fn into(self) -> i32 {
        self as i32
    }
}
impl From<i32> for EnumBookType {
    fn from(variant_value: i32) -> Self {
        if let Some(variant) = Self::from_repr(variant_value) {
            return variant;
        }
        Self::UnknownMqlVariantValue
    }
}
impl EnumBookType {
    pub fn is_sell(&self) -> bool {
        match self {
            EnumBookType::BookTypeSell | EnumBookType::BookTypeSellMarket => true,
            _ => false,
        }
    }
    pub fn is_buy(&self) -> bool {
        match self {
            EnumBookType::BookTypeBuy | EnumBookType::BookTypeBuyMarket => true,
            _ => false,
        }
    }
}

pub static ENUM_BOOK_TYPE: Lazy<&MqlRustEnumDescriptor> = Lazy::new(|| MqlRustEnumDescriptor::new("EnumBookType", &EnumBookType::from_str));

/// called when the program starts -- to register the MQL<=>Rust Enums
pub fn init() {
    log::info!("Internally registering ENUM '{}'", ENUM_BOOK_TYPE.name());
}