#![no_std]
extern crate alloc;

use alloc::boxed::Box;

pub trait Hardware {
    fn debug(&self, text: &str);
}

pub fn start(hardware: impl Hardware) -> Result<(), Box<dyn core::error::Error>> {
    hardware.debug("Hello tiny");
    
    Ok(())
}
