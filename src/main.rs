// SPDX-FileCopyrightText: 2024 Ledger SAS
//
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate shield;
use shield::println;
use sentry_uapi::syscall;
use sentry_uapi::systypes::{SleepDuration, SleepMode};


#[cfg(target_os = "none")]
shield::shield_main!();

fn main() {
    loop {
        let duration = SleepDuration::ArbitraryMs(1000);
        println!("Hello, World from Rust app !");
        let _ = syscall::sleep(duration, SleepMode::Deep);
    }

}
