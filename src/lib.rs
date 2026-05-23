#![cfg_attr(not(feature = "std"), no_std)]
//! Turkish language analysis for Pizza search engine.
//!
//! Provides a Turkish analyzer with locale-aware lowercasing
//! (dotted/dotless I handling), suffix-stripping stemmer, and stop words.
//!
//! # Components
//!
//! - [`TurkishApostropheFilter`] — Strips suffixes after apostrophe (İstanbul'un→İstanbul)
//! - [`TurkishLowercaseFilter`] — Turkish-specific lowercasing (İ→i, I→ı)
//! - [`TurkishStemFilter`] — Turkish suffix-stripping stemmer
//! - [`TurkishStopFilter`] — Turkish stop words filter
extern crate alloc;
mod apostrophe;
mod lowercase;
mod stem;
mod stop;

pub mod register;

pub use apostrophe::TurkishApostropheFilter;
pub use lowercase::TurkishLowercaseFilter;
pub use register::register_all;
pub use stem::TurkishStemFilter;
pub use stop::TurkishStopFilter;
