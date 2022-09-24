#![doc = include_str!("../docs/src/01_getting_started.md")]
// # extern crate sophia;
// #![warn(missing_docs)]
// https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html
// #![doc(html_favicon_url = "https://raw.github.com/MaastrichtU-IDS/knowledge-collaboratory/main/frontend/app/assets/icon.png")]
// #![doc(html_logo_url = "https://raw.github.com/MaastrichtU-IDS/knowledge-collaboratory/main/frontend/app/assets/icon.png")]
// #![doc(issue_tracker_base_url = "https://github.com/vemonet/nanopub-rs/issues/")]
// #![ no_implicit_prelude ]

mod constants;
/// A module to work with nanopublications
///
/// ## Usage
///
/// ```
/// use nanopub_rs::nanopub::Nanopub;
/// let np = Nanopub::new(
///     "<http://s> <http://p> <http://o> .",
///     "PUBKEY",
///     "PRIVATE_KEY",
///     "https://orcid.org/0000-0000-0000-0000",
///     None,
///     None,
/// );
/// ```
pub mod nanopub;
