#![feature(alloc)]
#![no_std]

extern crate alloc;
extern crate tock;

use alloc::fmt::Write;
use tock::console::Console;
use tock::ipc::ble_ess::{self, ReadingType};
use tock::sensors::*;
use tock::ipc::Client;
use alloc::String;

fn main() {
    let mut console = Console::new();
    let mut client = Client::new(
            String::from("org.tockos.services.ble-ess")).unwrap();
    let buffer = client.share(5).unwrap();

    write!(&mut console, "Hi James :)\n").unwrap();

    loop {
        if let Ok(_) = client.ping() {
            write!(&mut console, "OK! :)\n").unwrap();
        } else {
            write!(&mut console, "BAD! :)\n").unwrap();
        }

        tock::timer::delay_ms(5000);
    }
}

