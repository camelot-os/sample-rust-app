// SPDX-FileCopyrightText: 2024 Ledger SAS
//
// SPDX-License-Identifier: Apache-2.0

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

extern crate shield;
use sentry_uapi::systypes::Status;
use shield::print;
use shield::println;
use sentry_uapi::syscall;
use sentry_uapi::systypes;
use sentry_uapi::systypes::SleepDuration;
use sentry_uapi::systypes::SleepMode;

#[cfg(target_os = "none")]
shield::shield_main!();

const C_HELLO_APP_ID: u32 = 0xC001F002;

const MESSAGES: [&str; 8] = [
    "Short msg",
    "This is a bit longer message",
    "yet another message to send via IPC",
    "Tiny",
    "Medium length message for testing",
    "another basic medium message to send",
    "Short again",
    "Final message to conclude the test cases",
];

fn send_message(handle: systypes::TaskHandle, idx : usize) {
    println!("Sending message: {}", MESSAGES[idx]);
    let status  = syscall::send_ipc(handle, MESSAGES[idx].len() as u8);
    match status {
        Status::Ok => {}
        _ => {
            let _ = print!("Failed to send message, status: {}\n", status as u32);
        }
    }
}


fn main() {
    let mut idx = 0;
    //println!("Hello, World from Rust app !");
    let mut handle: systypes::TaskHandle = 0;
    let _status = syscall::get_process_handle(C_HELLO_APP_ID);
    let _ = sentry_uapi::copy_from_kernel(&mut handle);
    loop {
        send_message(handle, idx);
        idx = (idx + 1) % MESSAGES.len();
        let duration = SleepDuration::ArbitraryMs(1000);
        let _ = syscall::sleep(duration, SleepMode::Deep);
    }
}
