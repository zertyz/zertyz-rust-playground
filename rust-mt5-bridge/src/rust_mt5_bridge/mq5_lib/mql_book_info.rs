//! Mapping of https://www.mql5.com/en/docs/constants/structures/mqlbookinfo to Rust


use super::types::*;


/// It provides information about the market depth data.\
/// auto-generated from https://www.mql5.com/en/docs/constants/structures/mqlbookinfo
#[repr(C)]
#[derive(Debug)]
pub struct MqlBookInfo {
    /// Order type from ENUM_BOOK_TYPE enumeration
    pub type_: EnumBookType,
    /// Price
    pub price: f64,
    /// Volume
    pub volume: i64,
    /// Volume with greater accuracy
    pub volume_real: f64,
}

/// To obtain information about the current state of the DOM by MQL5 means, the MarketBookGet() function is used, which places the DOM &quot;screen shot&quot; into the MqlBookInfo array of structures. Each element of the array in the type field contains information about the direction of the order - the value of the ENUM_BOOK_TYPE enumeration./
/// auto-generated from https://www.mql5.com/en/docs/constants/tradingconstants/enum_book_type
#[repr(i32)]
#[derive(Debug,Clone,Copy)]
pub enum EnumBookType {
    /// Sell order (Offer)
    BookTypeSell,
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

