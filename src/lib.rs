#![no_std]

use core::ptr::NonNull;

extern crate alloc;

pub struct MyDriver {
    mmio: NonNull<u8>,
}

impl MyDriver {
    pub fn new(mmio: NonNull<u8>) -> Self {
        Self { mmio }
    }

    pub fn initialize(&mut self) {
        // Initialization code here
        log::debug!("Driver initialized with MMIO address: {:#p}", self.mmio);
    }
}
