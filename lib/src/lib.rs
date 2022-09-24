#![doc = include_str!("../docs/src/01_getting_started.md")]
// #![doc = include_str!("01_getting_started.md")]


/// A module to work with nanopublications
///
/// ## Usage
///
/// ```
/// use nanopub_rs::nanopub::Nanopub;
/// let np = Nanopub::new("<http://s> <http://p> <http://o> .");
/// ```
pub mod nanopub;