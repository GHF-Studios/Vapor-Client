#!/usr/bin/env -S rust-script --force

//! Patch the super-workspace RustRover project to use app-local Vapor tools.
//!
//! ```cargo
//! [package]
//! edition = "2024"
//! ```

#[path = "../../_shared/vapor_tools.rs"]
mod vapor_tools;

fn main() {
    let _cache_buster = "standalone-home-v4";
    vapor_tools::exit(vapor_tools::patch_rustrover_main());
}
