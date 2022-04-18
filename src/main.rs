//! # Neotron XXX BIOS
//!
//! This is the BIOS for the Neotron XXX.

#![no_std]
#![no_main]

// Make sure we halt the CPU on panic
extern crate panic_halt;

use core::fmt::Write;
use cortex_m_rt::entry;

/// BIOS Version
static BIOS_VERSION: &'static str = "v0.1.0";

/// Describes the hardware in the system
struct Hardware {
    _cp: cortex_m::Peripherals,
    // Add your CPU specific hardware here
}

/// Entry point for the BIOS. This is called by the startup code.
#[entry]
fn main() -> ! {
    // Set up the hardware
    let mut h = hardware_setup();

    // Print the BIOS version
    writeln!(h, "Neotron XXX BIOS {}", BIOS_VERSION).unwrap();
    writeln!(h, "Finding OS...").unwrap();

    // Here is where we find the OS and jump to it. We might:
    // * load it from the first sector of an SD card,
    // * search in Flash for it, or
    // * jump to a specific location in Flash.

    // Uh-oh - tell the user a bad thing happened
    panic!("No operating system found.");
}

/// Configure the hardware
fn hardware_setup() -> Hardware {
    Hardware {
        _cp: cortex_m::Peripherals::take().expect("Couldn't get hardware"),
    }
}

impl core::fmt::Write for Hardware {
    fn write_str(&mut self, _msg: &str) -> core::fmt::Result {
        // Write _msg to the screen, converting to the video character set as
        // required.
        Ok(())
    }
}

// End of file
