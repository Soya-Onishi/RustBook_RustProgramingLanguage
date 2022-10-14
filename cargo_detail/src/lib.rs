//! # Cargo Detail
//! `cargo detail`ではCargo.tomlやコメントに関する解説を行う

pub mod kinds;
pub mod utils;

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let five = 5;
/// 
/// assert_eq!(6, cargo_detail::add_one(five));
/// ```
pub fn add_one(x: i32) -> i32 {
  x + 1
}