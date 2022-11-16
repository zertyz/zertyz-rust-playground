//! Mapping of https://www.mql5.com/en/docs/constants/structures/mqltick to Rust


use super::types::*;


/// Rust version of the Metatrader 5 `MqlTick` structure. From the site:/
/// This is a structure for storing the latest prices of the symbol. It is designed for fast retrieval of the most requested information about current prices.\
/// auto-generated from https://www.mql5.com/en/docs/constants/structures/mqltick
#[repr(C)]
#[derive(Debug)]
pub struct MqlTick {
	/// Time of the last prices update
	pub time: MQ5DateTime,
	/// Current Bid price
	pub bid: f64,
	/// Current Ask price
	pub ask: f64,
	/// Price of the last deal (Last)
	pub last: f64,
	/// Volume for the current Last price
	pub volume: u64,
	/// Time of a price last update in milliseconds
	pub time_msc: i64,
	/// Tick flags
	pub flags: u32,
	/// Volume for the current Last price with greater accuracy
	pub volume_real: f64,
}

