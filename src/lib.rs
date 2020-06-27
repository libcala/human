// human
//
// Copyright (c) 2020 Jeron Aldaron Lau
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// https://apache.org/licenses/LICENSE-2.0>, or the Zlib License, <LICENSE-ZLIB
// or http://opensource.org/licenses/Zlib>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! # Getting Started
//! Add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies.human]
//! version = "0.1"
//! ```
//!
//! ```rust
//! // TODO
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/libcala/human/master/res/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/libcala/human/master/res/logo.svg",
    html_root_url = "https://docs.rs/human"
)]
#![forbid(unsafe_code)]
#![warn(
    anonymous_parameters,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unused_extern_crates,
    unused_qualifications,
    variant_size_differences
)]

mod mode;
mod input;

pub use mode::Mode;
pub use input::{TextInput, GameInput, UiInput, Input, input, rumble, renumber};
