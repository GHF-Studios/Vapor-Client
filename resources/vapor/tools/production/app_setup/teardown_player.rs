#!/usr/bin/env -S rust-script --force

//! Remove installer-managed player-mode Vapor-home state.
//!
//! ```cargo
//! [package]
//! edition = "2024"
//! ```

#[path = "../../_shared/vapor_tools.rs"]
mod vapor_tools;

fn main() {
    let _cache_buster = "standalone-home-v4";
    vapor_tools::exit(vapor_tools::teardown_player_main());
}
