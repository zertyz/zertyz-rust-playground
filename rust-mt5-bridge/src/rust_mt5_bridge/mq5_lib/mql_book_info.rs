//! Mapping of https://www.mql5.com/en/docs/constants/structures/mqlbookinfo to Rust


use std::fmt::{Debug, Formatter};
use super::types::*;


/// Representation of the Metatrader 5 `MqlBookInfo` structure. From the site:/
/// It provides information about the market depth data.\
/// auto-generated from https://www.mql5.com/en/docs/constants/structures/mqlbookinfo
#[repr(C, packed(4))]
#[derive(/*disable debug on this structure for production*/Debug,Copy,Clone)]
pub struct Mq5MqlBookInfo {
    /// Order type from ENUM_BOOK_TYPE enumeration
    pub book_type: EnumBookType,
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
            book_type: self.book_type,
            price:     self.price,
            volume:    self.volume_real,
        }
    }
}


/// Rust version of the internal Metatrader 5 struct [Mq5MqlBookInfo], with correct alignment and redundant fields removed
#[derive(Debug)]
pub struct MqlBookInfo {
    /// Order type from ENUM_BOOK_TYPE enumeration
    pub book_type: EnumBookType,
    /// Price
    pub price:     f64,
    /// Volume
    pub volume:    f64,
}


/// Shitly, this enum starts with 1 instead of 0. From the site:/
/// To obtain information about the current state of the DOM by MQL5 means, the MarketBookGet() function is used, which places the DOM &quot;screen shot&quot; into the MqlBookInfo array of structures. Each element of the array in the type field contains information about the direction of the order - the value of the ENUM_BOOK_TYPE enumeration./
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/enum_book_type
#[repr(i32)]
#[derive(Debug,Clone,Copy,PartialEq)]
pub enum EnumBookType {
    /// Metatrader says nothing about the value of 0 in this enum
    Undocumented,
    /// Sell order (Offer)
    BookTypeSell/* = 1i32*/,
    /// Buy order (Bid)
    BookTypeBuy,
    /// Sell order by Market
    BookTypeSellMarket,
    /// Buy order by Market
    BookTypeBuyMarket,

    // this will allow Rust not to crash when deserializing the data, if some variants are missing or if some new ones were added to meta trader
    Unmapped4,  Unmapped5,  Unmapped6,  Unmapped7,
    Unmapped8,  Unmapped9,  Unmapped10, Unmapped11, Unmapped12, Unmapped13, Unmapped14, Unmapped15, Unmapped16, Unmapped17, Unmapped18, Unmapped19, Unmapped20,
    Unmapped21, Unmapped22, Unmapped23, Unmapped24, Unmapped25, Unmapped26, Unmapped27, Unmapped28, Unmapped29, Unmapped30, Unmapped31, Unmapped32, Unmapped33,
    Unmapped34, Unmapped35, Unmapped36, Unmapped37, Unmapped38, Unmapped39, Unmapped40, Unmapped41, Unmapped42, Unmapped43, Unmapped44, Unmapped45, Unmapped46,
    Unmapped47, Unmapped48, Unmapped49, Unmapped50, Unmapped51, Unmapped52, Unmapped53, Unmapped54, Unmapped55, Unmapped56, Unmapped57, Unmapped58, Unmapped59,
    Unmapped60, Unmapped61, Unmapped62, Unmapped63, Unmapped64, Unmapped65, Unmapped66, Unmapped67, Unmapped68, Unmapped69, Unmapped70, Unmapped71, Unmapped72,
    Unmapped73, Unmapped74, Unmapped75, Unmapped76, Unmapped77, Unmapped78, Unmapped79, Unmapped80, Unmapped81, Unmapped82, Unmapped83, Unmapped84, Unmapped85,

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
