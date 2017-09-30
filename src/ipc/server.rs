#![allow(unused_imports)]

use core::cell::Cell;
use core::mem;
use syscalls;
use alloc::heap::Heap;
use alloc::String;
use alloc::boxed::Box;
use alloc::raw_vec::RawVec;
use alloc::allocator::{Alloc, Layout};
use ipc::DRIVER_NUM;

pub struct Server {

}
