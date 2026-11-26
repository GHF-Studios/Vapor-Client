#!/usr/bin/env -S rust-script --force

//! Clone source repositories into a Vapor SuperWorkspace.
//!
//! ```cargo
//! [package]
//! edition = "2024"
//! ```

#[path = "../../_shared/vapor_tools.rs"]
mod vapor_tools;

fn main() {
    let _cache_buster = "standalone-home-v4";
    vapor_tools::exit(vapor_tools::clone_source_main());
}
