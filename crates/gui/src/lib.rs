#![warn(clippy::print_stdout)]

rust_i18n::i18n!("locales", fallback = "en");

pub mod app;
pub mod expression_parser;
pub mod gst_utils;
pub mod path_compare;
pub mod third_party_licenses;
pub mod widgets;
