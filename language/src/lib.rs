//! This is an implementation of [Loren Sherman's Galligreyan
//! alphabet](https://github.com/JosephGonzalez03/gallifreyan/blob/main/Gallifreyan.pdf). It
//! provides the alphabet's letters as well as methods to decompose them into vectors of f32 cartesian points.  

pub mod letter_parts;
/// The `GallifreyanLetter` implementation.
pub mod letters;
/// Helper functions to calculate an `GallifreyanLetter`'s points.
pub mod math_util;
