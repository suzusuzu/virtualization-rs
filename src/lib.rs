#![allow(improper_ctypes)]

//! virtualization-rs provides the API of the Apple [Virtualization.framework](https://developer.apple.com/documentation/virtualization?language=objc) in Rust language.
//!
//! # Examples
//! See the [simplevm](https://github.com/suzusuzu/virtualization-rs/blob/main/examples/simplevm.rs) for more details.
//!
//! The example is inspired from [SimpleVM](https://github.com/KhaosT/SimpleVM).

extern crate block;
extern crate objc;

pub mod base;
pub mod virtualization;
