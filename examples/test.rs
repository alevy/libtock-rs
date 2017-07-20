#![feature(alloc)]
#![no_std]

extern crate tock;
extern crate alloc;

use alloc::boxed::Box;

fn main() {
    Box::new(123);
}
