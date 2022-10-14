use crate::kinds;

/// Combine two primary colors in equal amounts to create 
/// a secondary color.
pub fn mix(_c1: kinds::PrimaryColor, c2: kinds::SecondaryColor) -> kinds::SecondaryColor {
  c2
}