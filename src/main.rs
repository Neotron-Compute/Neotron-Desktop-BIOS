//! # Neotron Desktop BIOS
//!
//! Implement a Neotron BIOS as a Linux/Windows/macOS desktop application.
//!
//! The framebuffer is draw in a window. SD/MMC cards can be passed as files or block devices.

// -----------------------------------------------------------------------------
// Licence Statement
// -----------------------------------------------------------------------------
// Copyright (c) Jonathan 'theJPster' Pallant and the Neotron Developers, 2022
//
// This program is free software: you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free Software
// Foundation, either version 3 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program.  If not, see <https://www.gnu.org/licenses/>.
// -----------------------------------------------------------------------------

// ===========================================================================
// Imports
// ===========================================================================

use std::io::prelude::*;
use std::path::PathBuf;
use std::sync::atomic::AtomicPtr;
use std::sync::{
	atomic::{AtomicU32, AtomicU8, Ordering},
	mpsc, Mutex,
};

use clap::Parser;
use common::video::RGBColour;
use log::{debug, info};
use pix_engine::prelude::*;

use neotron_common_bios as common;

mod font;

// ===========================================================================
// Types
// ===========================================================================

struct MyApp {
	mode: common::video::Mode,
	font8x16: Vec<TextureId>,
	font8x8: Vec<TextureId>,
	sender: mpsc::Sender<AppEvent>,
}

#[derive(Debug, PartialEq, Eq)]
enum AppEvent {
	Started,
	KeyUp(Key),
	KeyDown(Key),
}

/// Our video RAM
struct Framebuffer<const N: usize> {
	contents: std::cell::UnsafeCell<[u8; N]>,
	alt_pointer: AtomicPtr<u32>,
}

/// A Desktop GUI version of a Neotron BIOS
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
	/// Path to the OS library
	#[arg(long)]
	os: PathBuf,
	/// Path to a file to use as a disk image
	#[arg(long)]
	disk: Option<PathBuf>,
	/// Path to NVRAM file
	#[arg(long)]
	nvram: Option<PathBuf>,
}

/// All our emulated hardware
struct Hardware {
	/// When we booted up
	boot_time: std::time::Instant,
	/// Our disk image
	disk_file: Option<std::fs::File>,
}

// ===========================================================================
// Global Variables
// ===========================================================================

/// We only have 'normal' sectored emulated disks
const BLOCK_SIZE: usize = 512;

/// The default VRAM we share in a very hazardous way with the OS.
///
/// Big enough for 640x480 @ 256 colour.
// static mut FRAMEBUFFER: [u8; 307200] = [0u8; 307200];
static FRAMEBUFFER: Framebuffer<{ 640 * 480 }> = Framebuffer::new();

/// Scale the display to make it readable on a modern monitor
const SCALE_FACTOR: f32 = 2.0;

/// When we booted up
static HARDWARE: Mutex<Option<Hardware>> = Mutex::new(None);

/// The functions we export to the OS
static BIOS_API: common::Api = common::Api {
	api_version_get,
	bios_version_get,
	serial_get_info,
	serial_configure,
	serial_write,
	serial_read,
	time_clock_get,
	time_clock_set,
	configuration_get,
	configuration_set,
	video_is_valid_mode,
	video_mode_needs_vram,
	video_set_mode,
	video_get_mode,
	video_get_framebuffer,
	video_wait_for_line,
	memory_get_region,
	hid_get_event,
	hid_set_leds,
	video_get_palette,
	video_set_palette,
	video_set_whole_palette,
	i2c_bus_get_info,
	i2c_write_read,
	audio_mixer_channel_get_info,
	audio_mixer_channel_set_level,
	audio_output_set_config,
	audio_output_get_config,
	audio_output_data,
	audio_output_get_space,
	audio_input_set_config,
	audio_input_get_config,
	audio_input_data,
	audio_input_get_count,
	bus_select,
	bus_get_info,
	bus_write_read,
	bus_exchange,
	time_ticks_get,
	time_ticks_per_second,
	bus_interrupt_status,
	block_dev_get_info,
	block_dev_eject,
	block_write,
	block_read,
	block_verify,
	power_idle,
	power_control,
	compare_and_swap_bool,
};

/// Our standard 256 colour palette
static PALETTE: [AtomicU32; 256] = [
	// Index 000: 0x000 (Black)
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x00, 0x00).as_packed()),
	// Index 001: 0x00a (Blue)
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x00, 0xaa).as_packed()),
	// Index 002: 0x0a0 (Green)
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xaa, 0x00).as_packed()),
	// Index 003: 0x0aa (Cyan)
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xaa, 0xaa).as_packed()),
	// Index 004: 0xa00 (Red)
	AtomicU32::new(RGBColour::from_rgb(0xaa, 0x00, 0x00).as_packed()),
	// Index 005: 0xa0a (Magenta)
	AtomicU32::new(RGBColour::from_rgb(0xaa, 0x00, 0xaa).as_packed()),
	// Index 006: 0xaa0 (Brown)
	AtomicU32::new(RGBColour::from_rgb(0xaa, 0xaa, 0x00).as_packed()),
	// Index 007: 0xaaa (Light Gray)
	AtomicU32::new(RGBColour::from_rgb(0xaa, 0xaa, 0xaa).as_packed()),
	// Index 008: 0x666 (Dark Gray)
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x66, 0x66).as_packed()),
	// Index 009: 0x00f (Light Blue)
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x00, 0xff).as_packed()),
	// Index 010: 0x0f0 (Light Green)
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xff, 0x00).as_packed()),
	// Index 011: 0x0ff (Light Cyan)
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xff, 0xff).as_packed()),
	// Index 012: 0xf00 (Light Red)
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x00, 0x00).as_packed()),
	// Index 013: 0xf0f (Pink)
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x00, 0xff).as_packed()),
	// Index 014: 0xff0 (Yellow)
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xff, 0x00).as_packed()),
	// Index 015: 0xfff (White)
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xff, 0xff).as_packed()),
	// Index 016: 0x003
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x00, 0x33).as_packed()),
	// Index 017: 0x006
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x00, 0x66).as_packed()),
	// Index 018: 0x00c
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x00, 0xcc).as_packed()),
	// Index 019: 0x020
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x22, 0x00).as_packed()),
	// Index 020: 0x023
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x22, 0x33).as_packed()),
	// Index 021: 0x026
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x22, 0x66).as_packed()),
	// Index 022: 0x028
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x22, 0x88).as_packed()),
	// Index 023: 0x02c
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x22, 0xcc).as_packed()),
	// Index 024: 0x02f
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x22, 0xff).as_packed()),
	// Index 025: 0x040
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x44, 0x00).as_packed()),
	// Index 026: 0x043
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x44, 0x33).as_packed()),
	// Index 027: 0x046
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x44, 0x66).as_packed()),
	// Index 028: 0x048
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x44, 0x88).as_packed()),
	// Index 029: 0x04c
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x44, 0xcc).as_packed()),
	// Index 030: 0x04f
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x44, 0xff).as_packed()),
	// Index 031: 0x083
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x88, 0x33).as_packed()),
	// Index 032: 0x086
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x88, 0x66).as_packed()),
	// Index 033: 0x08c
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x88, 0xcc).as_packed()),
	// Index 034: 0x08f
	AtomicU32::new(RGBColour::from_rgb(0x00, 0x88, 0xff).as_packed()),
	// Index 035: 0x0a0
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xaa, 0x00).as_packed()),
	// Index 036: 0x0a3
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xaa, 0x33).as_packed()),
	// Index 037: 0x0a6
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xaa, 0x66).as_packed()),
	// Index 038: 0x0a8
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xaa, 0x88).as_packed()),
	// Index 039: 0x0ac
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xaa, 0xcc).as_packed()),
	// Index 040: 0x0af
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xaa, 0xff).as_packed()),
	// Index 041: 0x0e0
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xee, 0x00).as_packed()),
	// Index 042: 0x0e3
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xee, 0x33).as_packed()),
	// Index 043: 0x0e6
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xee, 0x66).as_packed()),
	// Index 044: 0x0e8
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xee, 0x88).as_packed()),
	// Index 045: 0x0ec
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xee, 0xcc).as_packed()),
	// Index 046: 0x0ef
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xee, 0xff).as_packed()),
	// Index 047: 0x0f3
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xff, 0x33).as_packed()),
	// Index 048: 0x0f6
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xff, 0x66).as_packed()),
	// Index 049: 0x0f8
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xff, 0x88).as_packed()),
	// Index 050: 0x0fc
	AtomicU32::new(RGBColour::from_rgb(0x00, 0xff, 0xcc).as_packed()),
	// Index 051: 0x300
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x00, 0x00).as_packed()),
	// Index 052: 0x303
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x00, 0x33).as_packed()),
	// Index 053: 0x306
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x00, 0x66).as_packed()),
	// Index 054: 0x308
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x00, 0x88).as_packed()),
	// Index 055: 0x30c
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x00, 0xcc).as_packed()),
	// Index 056: 0x30f
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x00, 0xff).as_packed()),
	// Index 057: 0x320
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x22, 0x00).as_packed()),
	// Index 058: 0x323
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x22, 0x33).as_packed()),
	// Index 059: 0x326
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x22, 0x66).as_packed()),
	// Index 060: 0x328
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x22, 0x88).as_packed()),
	// Index 061: 0x32c
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x22, 0xcc).as_packed()),
	// Index 062: 0x32f
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x22, 0xff).as_packed()),
	// Index 063: 0x340
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x44, 0x00).as_packed()),
	// Index 064: 0x343
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x44, 0x33).as_packed()),
	// Index 065: 0x346
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x44, 0x66).as_packed()),
	// Index 066: 0x348
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x44, 0x88).as_packed()),
	// Index 067: 0x34c
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x44, 0xcc).as_packed()),
	// Index 068: 0x34f
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x44, 0xff).as_packed()),
	// Index 069: 0x380
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x88, 0x00).as_packed()),
	// Index 070: 0x383
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x88, 0x33).as_packed()),
	// Index 071: 0x386
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x88, 0x66).as_packed()),
	// Index 072: 0x388
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x88, 0x88).as_packed()),
	// Index 073: 0x38c
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x88, 0xcc).as_packed()),
	// Index 074: 0x38f
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x88, 0xff).as_packed()),
	// Index 075: 0x3a0
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xaa, 0x00).as_packed()),
	// Index 076: 0x3a3
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xaa, 0x33).as_packed()),
	// Index 077: 0x3a6
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xaa, 0x66).as_packed()),
	// Index 078: 0x3a8
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xaa, 0x88).as_packed()),
	// Index 079: 0x3ac
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xaa, 0xcc).as_packed()),
	// Index 080: 0x3af
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xaa, 0xff).as_packed()),
	// Index 081: 0x3e0
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xee, 0x00).as_packed()),
	// Index 082: 0x3e3
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xee, 0x33).as_packed()),
	// Index 083: 0x3e6
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xee, 0x66).as_packed()),
	// Index 084: 0x3e8
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xee, 0x88).as_packed()),
	// Index 085: 0x3ec
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xee, 0xcc).as_packed()),
	// Index 086: 0x3ef
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xee, 0xff).as_packed()),
	// Index 087: 0x3f0
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xff, 0x00).as_packed()),
	// Index 088: 0x3f3
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xff, 0x33).as_packed()),
	// Index 089: 0x3f6
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xff, 0x66).as_packed()),
	// Index 090: 0x3f8
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xff, 0x88).as_packed()),
	// Index 091: 0x3fc
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xff, 0xcc).as_packed()),
	// Index 092: 0x3ff
	AtomicU32::new(RGBColour::from_rgb(0x33, 0xff, 0xff).as_packed()),
	// Index 093: 0x600
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x00, 0x00).as_packed()),
	// Index 094: 0x603
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x00, 0x33).as_packed()),
	// Index 095: 0x606
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x00, 0x66).as_packed()),
	// Index 096: 0x608
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x00, 0x88).as_packed()),
	// Index 097: 0x60c
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x00, 0xcc).as_packed()),
	// Index 098: 0x60f
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x00, 0xff).as_packed()),
	// Index 099: 0x620
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x22, 0x00).as_packed()),
	// Index 100: 0x623
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x22, 0x33).as_packed()),
	// Index 101: 0x626
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x22, 0x66).as_packed()),
	// Index 102: 0x628
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x22, 0x88).as_packed()),
	// Index 103: 0x62c
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x22, 0xcc).as_packed()),
	// Index 104: 0x62f
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x22, 0xff).as_packed()),
	// Index 105: 0x640
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x44, 0x00).as_packed()),
	// Index 106: 0x643
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x44, 0x33).as_packed()),
	// Index 107: 0x646
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x44, 0x66).as_packed()),
	// Index 108: 0x648
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x44, 0x88).as_packed()),
	// Index 109: 0x64c
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x44, 0xcc).as_packed()),
	// Index 110: 0x64f
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x44, 0xff).as_packed()),
	// Index 111: 0x680
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x88, 0x00).as_packed()),
	// Index 112: 0x683
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x88, 0x33).as_packed()),
	// Index 113: 0x686
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x88, 0x66).as_packed()),
	// Index 114: 0x688
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x88, 0x88).as_packed()),
	// Index 115: 0x68c
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x88, 0xcc).as_packed()),
	// Index 116: 0x68f
	AtomicU32::new(RGBColour::from_rgb(0x66, 0x88, 0xff).as_packed()),
	// Index 117: 0x6a0
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xaa, 0x00).as_packed()),
	// Index 118: 0x6a3
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xaa, 0x33).as_packed()),
	// Index 119: 0x6a6
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xaa, 0x66).as_packed()),
	// Index 120: 0x6a8
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xaa, 0x88).as_packed()),
	// Index 121: 0x6ac
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xaa, 0xcc).as_packed()),
	// Index 122: 0x6af
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xaa, 0xff).as_packed()),
	// Index 123: 0x6e0
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xee, 0x00).as_packed()),
	// Index 124: 0x6e3
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xee, 0x33).as_packed()),
	// Index 125: 0x6e6
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xee, 0x66).as_packed()),
	// Index 126: 0x6e8
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xee, 0x88).as_packed()),
	// Index 127: 0x6ec
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xee, 0xcc).as_packed()),
	// Index 128: 0x6ef
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xee, 0xff).as_packed()),
	// Index 129: 0x6f0
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xff, 0x00).as_packed()),
	// Index 130: 0x6f3
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xff, 0x33).as_packed()),
	// Index 131: 0x6f6
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xff, 0x66).as_packed()),
	// Index 132: 0x6f8
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xff, 0x88).as_packed()),
	// Index 133: 0x6fc
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xff, 0xcc).as_packed()),
	// Index 134: 0x6ff
	AtomicU32::new(RGBColour::from_rgb(0x66, 0xff, 0xff).as_packed()),
	// Index 135: 0x803
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x00, 0x33).as_packed()),
	// Index 136: 0x806
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x00, 0x66).as_packed()),
	// Index 137: 0x80c
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x00, 0xcc).as_packed()),
	// Index 138: 0x80f
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x00, 0xff).as_packed()),
	// Index 139: 0x820
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x22, 0x00).as_packed()),
	// Index 140: 0x823
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x22, 0x33).as_packed()),
	// Index 141: 0x826
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x22, 0x66).as_packed()),
	// Index 142: 0x828
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x22, 0x88).as_packed()),
	// Index 143: 0x82c
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x22, 0xcc).as_packed()),
	// Index 144: 0x82f
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x22, 0xff).as_packed()),
	// Index 145: 0x840
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x44, 0x00).as_packed()),
	// Index 146: 0x843
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x44, 0x33).as_packed()),
	// Index 147: 0x846
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x44, 0x66).as_packed()),
	// Index 148: 0x848
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x44, 0x88).as_packed()),
	// Index 149: 0x84c
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x44, 0xcc).as_packed()),
	// Index 150: 0x84f
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x44, 0xff).as_packed()),
	// Index 151: 0x883
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x88, 0x33).as_packed()),
	// Index 152: 0x886
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x88, 0x66).as_packed()),
	// Index 153: 0x88c
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x88, 0xcc).as_packed()),
	// Index 154: 0x88f
	AtomicU32::new(RGBColour::from_rgb(0x88, 0x88, 0xff).as_packed()),
	// Index 155: 0x8a0
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xaa, 0x00).as_packed()),
	// Index 156: 0x8a3
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xaa, 0x33).as_packed()),
	// Index 157: 0x8a6
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xaa, 0x66).as_packed()),
	// Index 158: 0x8a8
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xaa, 0x88).as_packed()),
	// Index 159: 0x8ac
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xaa, 0xcc).as_packed()),
	// Index 160: 0x8af
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xaa, 0xff).as_packed()),
	// Index 161: 0x8e0
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xee, 0x00).as_packed()),
	// Index 162: 0x8e3
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xee, 0x33).as_packed()),
	// Index 163: 0x8e6
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xee, 0x66).as_packed()),
	// Index 164: 0x8e8
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xee, 0x88).as_packed()),
	// Index 165: 0x8ec
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xee, 0xcc).as_packed()),
	// Index 166: 0x8ef
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xee, 0xff).as_packed()),
	// Index 167: 0x8f0
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xff, 0x00).as_packed()),
	// Index 168: 0x8f3
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xff, 0x33).as_packed()),
	// Index 169: 0x8f6
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xff, 0x66).as_packed()),
	// Index 170: 0x8f8
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xff, 0x88).as_packed()),
	// Index 171: 0x8fc
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xff, 0xcc).as_packed()),
	// Index 172: 0x8ff
	AtomicU32::new(RGBColour::from_rgb(0x88, 0xff, 0xff).as_packed()),
	// Index 173: 0xc00
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x00, 0x00).as_packed()),
	// Index 174: 0xc03
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x00, 0x33).as_packed()),
	// Index 175: 0xc06
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x00, 0x66).as_packed()),
	// Index 176: 0xc08
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x00, 0x88).as_packed()),
	// Index 177: 0xc0c
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x00, 0xcc).as_packed()),
	// Index 178: 0xc0f
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x00, 0xff).as_packed()),
	// Index 179: 0xc20
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x22, 0x00).as_packed()),
	// Index 180: 0xc23
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x22, 0x33).as_packed()),
	// Index 181: 0xc26
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x22, 0x66).as_packed()),
	// Index 182: 0xc28
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x22, 0x88).as_packed()),
	// Index 183: 0xc2c
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x22, 0xcc).as_packed()),
	// Index 184: 0xc2f
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x22, 0xff).as_packed()),
	// Index 185: 0xc40
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x44, 0x00).as_packed()),
	// Index 186: 0xc43
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x44, 0x33).as_packed()),
	// Index 187: 0xc46
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x44, 0x66).as_packed()),
	// Index 188: 0xc48
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x44, 0x88).as_packed()),
	// Index 189: 0xc4c
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x44, 0xcc).as_packed()),
	// Index 190: 0xc4f
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x44, 0xff).as_packed()),
	// Index 191: 0xc80
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x88, 0x00).as_packed()),
	// Index 192: 0xc83
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x88, 0x33).as_packed()),
	// Index 193: 0xc86
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x88, 0x66).as_packed()),
	// Index 194: 0xc88
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x88, 0x88).as_packed()),
	// Index 195: 0xc8c
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x88, 0xcc).as_packed()),
	// Index 196: 0xc8f
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0x88, 0xff).as_packed()),
	// Index 197: 0xca0
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xaa, 0x00).as_packed()),
	// Index 198: 0xca3
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xaa, 0x33).as_packed()),
	// Index 199: 0xca6
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xaa, 0x66).as_packed()),
	// Index 200: 0xca8
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xaa, 0x88).as_packed()),
	// Index 201: 0xcac
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xaa, 0xcc).as_packed()),
	// Index 202: 0xcaf
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xaa, 0xff).as_packed()),
	// Index 203: 0xce0
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xee, 0x00).as_packed()),
	// Index 204: 0xce3
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xee, 0x33).as_packed()),
	// Index 205: 0xce6
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xee, 0x66).as_packed()),
	// Index 206: 0xce8
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xee, 0x88).as_packed()),
	// Index 207: 0xcec
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xee, 0xcc).as_packed()),
	// Index 208: 0xcef
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xee, 0xff).as_packed()),
	// Index 209: 0xcf0
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xff, 0x00).as_packed()),
	// Index 210: 0xcf3
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xff, 0x33).as_packed()),
	// Index 211: 0xcf6
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xff, 0x66).as_packed()),
	// Index 212: 0xcf8
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xff, 0x88).as_packed()),
	// Index 213: 0xcfc
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xff, 0xcc).as_packed()),
	// Index 214: 0xcff
	AtomicU32::new(RGBColour::from_rgb(0xcc, 0xff, 0xff).as_packed()),
	// Index 215: 0xf03
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x00, 0x33).as_packed()),
	// Index 216: 0xf06
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x00, 0x66).as_packed()),
	// Index 217: 0xf08
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x00, 0x88).as_packed()),
	// Index 218: 0xf0c
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x00, 0xcc).as_packed()),
	// Index 219: 0xf20
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x22, 0x00).as_packed()),
	// Index 220: 0xf23
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x22, 0x33).as_packed()),
	// Index 221: 0xf26
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x22, 0x66).as_packed()),
	// Index 222: 0xf28
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x22, 0x88).as_packed()),
	// Index 223: 0xf2c
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x22, 0xcc).as_packed()),
	// Index 224: 0xf2f
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x22, 0xff).as_packed()),
	// Index 225: 0xf40
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x44, 0x00).as_packed()),
	// Index 226: 0xf43
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x44, 0x33).as_packed()),
	// Index 227: 0xf46
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x44, 0x66).as_packed()),
	// Index 228: 0xf48
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x44, 0x88).as_packed()),
	// Index 229: 0xf4c
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x44, 0xcc).as_packed()),
	// Index 230: 0xf4f
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x44, 0xff).as_packed()),
	// Index 231: 0xf80
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x88, 0x00).as_packed()),
	// Index 232: 0xf83
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x88, 0x33).as_packed()),
	// Index 233: 0xf86
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x88, 0x66).as_packed()),
	// Index 234: 0xf88
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x88, 0x88).as_packed()),
	// Index 235: 0xf8c
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x88, 0xcc).as_packed()),
	// Index 236: 0xf8f
	AtomicU32::new(RGBColour::from_rgb(0xff, 0x88, 0xff).as_packed()),
	// Index 237: 0xfa0
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xaa, 0x00).as_packed()),
	// Index 238: 0xfa3
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xaa, 0x33).as_packed()),
	// Index 239: 0xfa6
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xaa, 0x66).as_packed()),
	// Index 240: 0xfa8
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xaa, 0x88).as_packed()),
	// Index 241: 0xfac
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xaa, 0xcc).as_packed()),
	// Index 242: 0xfaf
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xaa, 0xff).as_packed()),
	// Index 243: 0xfe0
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xee, 0x00).as_packed()),
	// Index 244: 0xfe3
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xee, 0x33).as_packed()),
	// Index 245: 0xfe6
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xee, 0x66).as_packed()),
	// Index 246: 0xfe8
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xee, 0x88).as_packed()),
	// Index 247: 0xfec
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xee, 0xcc).as_packed()),
	// Index 248: 0xfef
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xee, 0xff).as_packed()),
	// Index 249: 0xff3
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xff, 0x33).as_packed()),
	// Index 250: 0xff6
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xff, 0x66).as_packed()),
	// Index 251: 0xff8
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xff, 0x88).as_packed()),
	// Index 252: 0xffc
	AtomicU32::new(RGBColour::from_rgb(0xff, 0xff, 0xcc).as_packed()),
	// Index 253: 0xbbb
	AtomicU32::new(RGBColour::from_rgb(0xbb, 0xbb, 0xbb).as_packed()),
	// Index 254: 0x333
	AtomicU32::new(RGBColour::from_rgb(0x33, 0x33, 0x33).as_packed()),
	// Index 255: 0x777
	AtomicU32::new(RGBColour::from_rgb(0x77, 0x77, 0x77).as_packed()),
];

static VIDEO_MODE: AtomicU8 = AtomicU8::new(0);

/// HID events come from here
static EV_QUEUE: Mutex<Option<mpsc::Receiver<AppEvent>>> = Mutex::new(None);

/// Where the OS config is read from or written to.
static CONFIG_FILE_PATH: Mutex<Option<PathBuf>> = Mutex::new(None);

// ===========================================================================
// Macros
// ===========================================================================

// None

// ===========================================================================
// Functions
// ===========================================================================

/// The entry point to our program.
///
/// We set up a game window using PixEngine. The event loop pumps in this thread.
///
/// We then load the OS from the `so` file given, and jump to it in a new thread.
fn main() {
	env_logger::init();

	let args = Args::parse();

	// Let's go!
	info!("Netron Desktop BIOS");

	{
		let mut hw = HARDWARE.lock().unwrap();
		*hw = Some(Hardware {
			boot_time: std::time::Instant::now(),
			disk_file: args
				.disk
				.map(|path| std::fs::File::open(path).expect("open disk file")),
		});
	}

	let white_on_black = common::video::Attr::new(
		common::video::TextForegroundColour::White,
		common::video::TextBackgroundColour::Black,
		false,
	);
	for char_idx in 0..(80 * 60) {
		// Blank
		FRAMEBUFFER.write_at(char_idx * 2, b' ');
		// White on Black
		FRAMEBUFFER.write_at((char_idx * 2) + 1, white_on_black.as_u8());
	}

	// Process args
	info!("Loading OS from: {}", args.os.display());
	let lib = unsafe { libloading::Library::new(args.os).expect("library to load") };
	println!("Loaded!");

	if let Some(config_path) = args.nvram {
		info!("Loading OS config from: {}", config_path.display());
		*CONFIG_FILE_PATH.lock().unwrap() = Some(config_path);
	}

	// Make a window
	let mut engine = Engine::builder()
		.dimensions(640, 480)
		.title("Neotron Desktop BIOS")
		.show_frame_rate()
		.target_frame_rate(60)
		.build()
		.unwrap();
	let (sender, receiver) = mpsc::channel();
	let mut app = MyApp {
		mode: unsafe { common::video::Mode::from_u8(0) },
		font8x16: Vec::new(),
		font8x8: Vec::new(),
		sender,
	};

	EV_QUEUE.lock().unwrap().replace(receiver);

	// Run the OS
	std::thread::spawn(move || unsafe {
		// Wait for Started message
		let queue = EV_QUEUE.lock().unwrap();
		let ev = queue.as_ref().unwrap().recv().unwrap();
		assert_eq!(ev, AppEvent::Started);
		drop(queue);
		info!("Video init complete. OS starting...");
		let main_func: libloading::Symbol<unsafe extern "C" fn(api: &'static common::Api) -> !> =
			lib.get(b"os_main").expect("os_main() not found");
		main_func(&BIOS_API);
	});

	engine.run(&mut app).unwrap();
}

/// Returns the version number of the BIOS API.
extern "C" fn api_version_get() -> common::Version {
	debug!("api_version_get()");
	common::API_VERSION
}

/// Returns a pointer to a static string slice containing the BIOS Version.
///
/// This string contains the version number and build string of the BIOS.
/// For C compatibility this string is null-terminated and guaranteed to
/// only contain ASCII characters (bytes with a value 127 or lower). We
/// also pass the length (excluding the null) to make it easy to construct
/// a Rust string. It is unspecified as to whether the string is located
/// in Flash ROM or RAM (but it's likely to be Flash ROM).
extern "C" fn bios_version_get() -> common::FfiString<'static> {
	debug!("bios_version_get()");
	common::FfiString::new("Neotron Desktop BIOS\0")
}

/// Get information about the Serial ports in the system.
///
/// Serial ports are ordered octet-oriented pipes. You can push octets
/// into them using a 'write' call, and pull bytes out of them using a
/// 'read' call. They have options which allow them to be configured at
/// different speeds, or with different transmission settings (parity
/// bits, stop bits, etc) - you set these with a call to
/// `SerialConfigure`. They may physically be a MIDI interface, an RS-232
/// port or a USB-Serial port. There is no sense of 'open' or 'close' -
/// that is an Operating System level design feature. These APIs just
/// reflect the raw hardware, in a similar manner to the registers exposed
/// by a memory-mapped UART peripheral.
extern "C" fn serial_get_info(_device: u8) -> common::FfiOption<common::serial::DeviceInfo> {
	debug!("serial_get_info()");
	common::FfiOption::None
}

/// Set the options for a given serial device. An error is returned if the
/// options are invalid for that serial device.
extern "C" fn serial_configure(
	_device: u8,
	_config: common::serial::Config,
) -> common::ApiResult<()> {
	debug!("serial_configure()");
	Err(common::Error::Unimplemented).into()
}

/// Write bytes to a serial port. There is no sense of 'opening' or
/// 'closing' the device - serial devices are always open. If the return
/// value is `Ok(n)`, the value `n` may be less than the size of the given
/// buffer. If so, that means not all of the data could be transmitted -
/// only the first `n` bytes were.
extern "C" fn serial_write(
	_device: u8,
	_data: common::FfiByteSlice,
	_timeout: common::FfiOption<common::Timeout>,
) -> common::ApiResult<usize> {
	debug!("serial_write()");
	Err(common::Error::Unimplemented).into()
}

/// Read bytes from a serial port. There is no sense of 'opening' or
/// 'closing' the device - serial devices are always open. If the return value
///  is `Ok(n)`, the value `n` may be less than the size of the given buffer.
///  If so, that means not all of the data could be received - only the
///  first `n` bytes were filled in.
extern "C" fn serial_read(
	_device: u8,
	_data: common::FfiBuffer,
	_timeout: common::FfiOption<common::Timeout>,
) -> common::ApiResult<usize> {
	debug!("serial_read()");
	Err(common::Error::Unimplemented).into()
}

/// Get the current wall time.
///
/// The Neotron BIOS does not understand time zones, leap-seconds or the
/// Gregorian calendar. It simply stores time as an incrementing number of
/// seconds since some epoch, and the number of milliseconds since that second
/// began. A day is assumed to be exactly 86,400 seconds long. This is a lot
/// like POSIX time, except we have a different epoch - the Neotron epoch is
/// 2000-01-01T00:00:00Z. It is highly recommend that you store UTC in the BIOS
/// and use the OS to handle time-zones.
///
/// If the BIOS does not have a battery-backed clock, or if that battery has
/// failed to keep time, the system starts up assuming it is the epoch.
extern "C" fn time_clock_get() -> common::Time {
	debug!("time_clock_get()");
	// 946684800 seconds between 2000-01-01 and 1970-01-01
	let epoch = std::time::SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(946684800);
	let difference = epoch.elapsed().unwrap_or_default();
	// We're good until 2068, when I shall be retired.
	assert!(difference.as_secs() <= u64::from(u32::MAX));
	common::Time {
		secs: difference.as_secs() as u32,
		nsecs: difference.subsec_nanos(),
	}
}

/// Set the current wall time.
///
/// See `time_get` for a description of now the Neotron BIOS should handle
/// time.
///
/// You only need to call this whenever you get a new sense of the current
/// time (e.g. the user has updated the current time, or if you get a GPS
/// fix). The BIOS should push the time out to the battery-backed Real
/// Time Clock, if it has one.
extern "C" fn time_clock_set(time: common::Time) {
	debug!("time_clock_set({:?})", time);
}

/// Get the configuration data block.
///
/// Configuration data is, to the BIOS, just a block of bytes of a given
/// length. How it stores them is up to the BIOS - it could be EEPROM, or
/// battery-backed SRAM.
extern "C" fn configuration_get(mut os_buffer: common::FfiBuffer) -> common::ApiResult<usize> {
	let file_path = CONFIG_FILE_PATH.lock().unwrap().clone();
	let Some(os_buffer) = os_buffer.as_mut_slice() else {
		return common::ApiResult::Err(common::Error::DeviceError);
	};
	match file_path.as_ref() {
		Some(path) => match std::fs::read(path) {
			Ok(read_data) => {
				for (src, dest) in read_data.iter().zip(os_buffer.iter_mut()) {
					*dest = *src;
				}
				common::ApiResult::Ok(read_data.len())
			}
			Err(_e) => {
				println!("Failed to get config from {:?}", path);
				common::ApiResult::Err(common::Error::DeviceError)
			}
		},
		None => common::ApiResult::Err(common::Error::Unimplemented),
	}
}

/// Set the configuration data block.
///
/// See `configuration_get`.
extern "C" fn configuration_set(buffer: common::FfiByteSlice) -> common::ApiResult<()> {
	let file_path = CONFIG_FILE_PATH.lock().unwrap().clone();
	match file_path.as_ref() {
		Some(path) => match std::fs::write(path, buffer.as_slice()) {
			Ok(_) => common::ApiResult::Ok(()),
			Err(_e) => {
				println!("Failed to write config to {:?}", path);
				common::ApiResult::Err(common::Error::DeviceError)
			}
		},
		None => common::ApiResult::Err(common::Error::Unimplemented),
	}
}

/// Does this Neotron BIOS support this video mode?
extern "C" fn video_is_valid_mode(mode: common::video::Mode) -> bool {
	debug!("video_is_valid_mode()");
	mode == common::video::Mode::new(
		common::video::Timing::T640x480,
		common::video::Format::Text8x16,
	)
}

/// Switch to a new video mode.
///
/// The contents of the screen are undefined after a call to this function.
extern "C" fn video_set_mode(mode: common::video::Mode, fb: *mut u32) -> common::ApiResult<()> {
	info!("video_set_mode({:?})", mode);
	match mode.timing() {
		common::video::Timing::T640x480 => {
			// OK
		}
		common::video::Timing::T640x400 => {
			// OK
		}
		_ => {
			return common::ApiResult::Err(common::Error::UnsupportedConfiguration);
		}
	}
	match mode.format() {
		common::video::Format::Text8x16 => {
			// OK
		}
		common::video::Format::Text8x8 => {
			// OK
		}
		_ => {
			return common::ApiResult::Err(common::Error::UnsupportedConfiguration);
		}
	}

	// We know this is a valid video mode because it was set with `video_set_mode`.
	let mode_value = mode.as_u8();
	VIDEO_MODE.store(mode_value, Ordering::Relaxed);
	FRAMEBUFFER.alt_pointer.store(fb, Ordering::Relaxed);
	common::ApiResult::Ok(())
}

/// Returns the video mode the BIOS is currently in.
///
/// The OS should call this function immediately after start-up and note
/// the value - this is the `default` video mode which can always be
/// serviced without supplying extra RAM.
extern "C" fn video_get_mode() -> common::video::Mode {
	debug!("video_get_mode()");
	let mode_value = VIDEO_MODE.load(Ordering::Relaxed);
	// We know this is a valid video mode because it was set with `video_set_mode`.
	unsafe { common::video::Mode::from_u8(mode_value) }
}

/// Get the framebuffer address.
///
/// We can write through this address to the video framebuffer. The
/// meaning of the data we write, and the size of the region we are
/// allowed to write to, is a function of the current video mode (see
/// `video_get_mode`).
extern "C" fn video_get_framebuffer() -> *mut u32 {
	let p = FRAMEBUFFER.get_pointer() as *mut u32;
	debug!("video_get_framebuffer() -> {:p}", p);
	p
}

/// Find out whether the given video mode needs more VRAM than we currently have.
///
/// The answer is no for any currently supported video mode (which is just the four text modes right now).
extern "C" fn video_mode_needs_vram(_mode: common::video::Mode) -> bool {
	debug!("video_mode_needs_vram()");
	false
}

/// Find out how large a given region of memory is.
///
/// The first region is the 'main application region' and is defined to always
/// start at address `0x2000_0000` on a standard Cortex-M system. This
/// application region stops just before the BIOS reserved memory, at the top of
/// the internal SRAM. The OS will have been linked to use the first 1 KiB of
/// this region.
///
/// Other regions may be located at other addresses (e.g. external DRAM or
/// PSRAM).
///
/// The OS will always load non-relocatable applications into the bottom of
/// Region 0. It can allocate OS specific structures from any other Region (if
/// any), or from the top of Region 0 (although this reduces the maximum
/// application space available). The OS will prefer lower numbered regions
/// (other than Region 0), so faster memory should be listed first.
///
/// If the region number given is invalid, the function returns `(null, 0)`.
extern "C" fn memory_get_region(region: u8) -> common::FfiOption<common::MemoryRegion> {
	static mut MEMORY_BLOCK: (*mut u8, usize) = (std::ptr::null_mut(), 0);
	match region {
		0 => {
			if unsafe { MEMORY_BLOCK.0.is_null() } {
				// Allocate 256 KiB of storage space for the OS to use
				let mut data = Box::new([0u8; 256 * 1024]);
				unsafe {
					MEMORY_BLOCK.0 = data.as_mut_ptr() as *mut u8;
					MEMORY_BLOCK.1 = std::mem::size_of_val(&*data);
				}
				std::mem::forget(data);
			}
			common::FfiOption::Some(common::MemoryRegion {
				start: unsafe { MEMORY_BLOCK.0 },
				length: unsafe { MEMORY_BLOCK.1 },
				kind: common::FfiMemoryKind::from(common::MemoryKind::Ram),
			})
		}
		_ => common::FfiOption::None,
	}
}

/// Get the next available HID event, if any.
///
/// This function doesn't block. It will return `Ok(None)` if there is no event ready.
extern "C" fn hid_get_event() -> common::ApiResult<common::FfiOption<common::hid::HidEvent>> {
	let queue = EV_QUEUE.lock().unwrap();
	match queue.as_ref().unwrap().try_recv() {
		Ok(AppEvent::KeyUp(key)) => {
			let code = common::hid::HidEvent::KeyRelease(convert_keycode(key));
			debug!("hid_get_event() -> {:?}", code);
			common::ApiResult::Ok(common::FfiOption::Some(code))
		}
		Ok(AppEvent::KeyDown(key)) => {
			let code = common::hid::HidEvent::KeyPress(convert_keycode(key));
			debug!("hid_get_event() -> {:?}", code);
			common::ApiResult::Ok(common::FfiOption::Some(code))
		}
		_ => common::ApiResult::Ok(common::FfiOption::None),
	}
}

/// Convert a pix-engine keycode into a Neotron BIOS keycode
fn convert_keycode(key: Key) -> common::hid::KeyCode {
	match key {
		Key::Backspace => common::hid::KeyCode::Backspace,
		Key::Tab => common::hid::KeyCode::Tab,
		Key::Return => common::hid::KeyCode::Return,
		Key::Escape => common::hid::KeyCode::Escape,
		Key::Space => common::hid::KeyCode::Spacebar,
		// Key::Exclaim => common::hid::KeyCode::Exclaim,
		// Key::Quotedbl => common::hid::KeyCode::Quotedbl,
		Key::Hash => common::hid::KeyCode::Oem7,
		// Key::Dollar => common::hid::KeyCode::Dollar,
		// Key::Percent => common::hid::KeyCode::Percent,
		// Key::Ampersand => common::hid::KeyCode::Ampersand,
		Key::Quote => common::hid::KeyCode::Oem3,
		// Key::LeftParen => common::hid::KeyCode::LeftParen,
		// Key::RightParen => common::hid::KeyCode::RightParen,
		// Key::Asterisk => common::hid::KeyCode::Asterisk,
		// Key::Plus => common::hid::KeyCode::Plus,
		Key::Comma => common::hid::KeyCode::OemComma,
		Key::Minus => common::hid::KeyCode::OemMinus,
		Key::Period => common::hid::KeyCode::OemPeriod,
		Key::Slash => common::hid::KeyCode::Oem2,
		Key::Num0 => common::hid::KeyCode::Key0,
		Key::Num1 => common::hid::KeyCode::Key1,
		Key::Num2 => common::hid::KeyCode::Key2,
		Key::Num3 => common::hid::KeyCode::Key3,
		Key::Num4 => common::hid::KeyCode::Key4,
		Key::Num5 => common::hid::KeyCode::Key5,
		Key::Num6 => common::hid::KeyCode::Key6,
		Key::Num7 => common::hid::KeyCode::Key7,
		Key::Num8 => common::hid::KeyCode::Key8,
		Key::Num9 => common::hid::KeyCode::Key9,
		// Key::Colon => common::hid::KeyCode::Colon,
		Key::Semicolon => common::hid::KeyCode::Oem1,
		// Key::Less => common::hid::KeyCode::Less,
		Key::Equals => common::hid::KeyCode::OemPlus,
		// Key::Greater => common::hid::KeyCode::Greater,
		// Key::Question => common::hid::KeyCode::Question,
		// Key::At => common::hid::KeyCode::At,
		Key::LeftBracket => common::hid::KeyCode::Oem4,
		Key::Backslash => common::hid::KeyCode::Oem5,
		Key::RightBracket => common::hid::KeyCode::Oem6,
		// Key::Caret => common::hid::KeyCode::Caret,
		// Key::Underscore => common::hid::KeyCode::Underscore,
		Key::Backquote => common::hid::KeyCode::Oem8,
		Key::A => common::hid::KeyCode::A,
		Key::B => common::hid::KeyCode::B,
		Key::C => common::hid::KeyCode::C,
		Key::D => common::hid::KeyCode::D,
		Key::E => common::hid::KeyCode::E,
		Key::F => common::hid::KeyCode::F,
		Key::G => common::hid::KeyCode::G,
		Key::H => common::hid::KeyCode::H,
		Key::I => common::hid::KeyCode::I,
		Key::J => common::hid::KeyCode::J,
		Key::K => common::hid::KeyCode::K,
		Key::L => common::hid::KeyCode::L,
		Key::M => common::hid::KeyCode::M,
		Key::N => common::hid::KeyCode::N,
		Key::O => common::hid::KeyCode::O,
		Key::P => common::hid::KeyCode::P,
		Key::Q => common::hid::KeyCode::Q,
		Key::R => common::hid::KeyCode::R,
		Key::S => common::hid::KeyCode::S,
		Key::T => common::hid::KeyCode::T,
		Key::U => common::hid::KeyCode::U,
		Key::V => common::hid::KeyCode::V,
		Key::W => common::hid::KeyCode::W,
		Key::X => common::hid::KeyCode::X,
		Key::Y => common::hid::KeyCode::Y,
		Key::Z => common::hid::KeyCode::Z,
		Key::Delete => common::hid::KeyCode::Delete,
		Key::CapsLock => common::hid::KeyCode::CapsLock,
		Key::F1 => common::hid::KeyCode::F1,
		Key::F2 => common::hid::KeyCode::F2,
		Key::F3 => common::hid::KeyCode::F3,
		Key::F4 => common::hid::KeyCode::F4,
		Key::F5 => common::hid::KeyCode::F5,
		Key::F6 => common::hid::KeyCode::F6,
		Key::F7 => common::hid::KeyCode::F7,
		Key::F8 => common::hid::KeyCode::F8,
		Key::F9 => common::hid::KeyCode::F9,
		Key::F10 => common::hid::KeyCode::F10,
		Key::F11 => common::hid::KeyCode::F11,
		Key::F12 => common::hid::KeyCode::F12,
		Key::PrintScreen => common::hid::KeyCode::PrintScreen,
		Key::ScrollLock => common::hid::KeyCode::ScrollLock,
		Key::Pause => common::hid::KeyCode::PauseBreak,
		Key::Insert => common::hid::KeyCode::Insert,
		Key::Home => common::hid::KeyCode::Home,
		Key::PageUp => common::hid::KeyCode::PageUp,
		Key::End => common::hid::KeyCode::End,
		Key::PageDown => common::hid::KeyCode::PageDown,
		Key::Right => common::hid::KeyCode::ArrowRight,
		Key::Left => common::hid::KeyCode::ArrowLeft,
		Key::Down => common::hid::KeyCode::ArrowDown,
		Key::Up => common::hid::KeyCode::ArrowUp,
		Key::NumLock => common::hid::KeyCode::NumpadLock,
		Key::KpDivide => common::hid::KeyCode::NumpadDivide,
		Key::KpMultiply => common::hid::KeyCode::NumpadMultiply,
		Key::KpMinus => common::hid::KeyCode::NumpadSubtract,
		Key::KpPlus => common::hid::KeyCode::NumpadAdd,
		Key::KpEnter => common::hid::KeyCode::NumpadEnter,
		Key::Kp1 => common::hid::KeyCode::Numpad1,
		Key::Kp2 => common::hid::KeyCode::Numpad2,
		Key::Kp3 => common::hid::KeyCode::Numpad3,
		Key::Kp4 => common::hid::KeyCode::Numpad4,
		Key::Kp5 => common::hid::KeyCode::Numpad5,
		Key::Kp6 => common::hid::KeyCode::Numpad6,
		Key::Kp7 => common::hid::KeyCode::Numpad7,
		Key::Kp8 => common::hid::KeyCode::Numpad8,
		Key::Kp9 => common::hid::KeyCode::Numpad9,
		Key::Kp0 => common::hid::KeyCode::Numpad0,
		Key::KpPeriod => common::hid::KeyCode::NumpadPeriod,
		// Key::KpEquals => common::hid::KeyCode::KpEquals,
		// Key::KpComma => common::hid::KeyCode::KpComma,
		Key::LCtrl => common::hid::KeyCode::LControl,
		Key::LShift => common::hid::KeyCode::LShift,
		Key::LAlt => common::hid::KeyCode::LAlt,
		Key::LGui => common::hid::KeyCode::LWin,
		Key::RCtrl => common::hid::KeyCode::RControl,
		Key::RShift => common::hid::KeyCode::RShift,
		Key::RAlt => common::hid::KeyCode::RAltGr,
		Key::RGui => common::hid::KeyCode::RWin,
		_ => common::hid::KeyCode::X,
	}
}

/// Control the keyboard LEDs.
extern "C" fn hid_set_leds(_leds: common::hid::KeyboardLeds) -> common::ApiResult<()> {
	debug!("hid_set_leds()");
	Err(common::Error::Unimplemented).into()
}

/// Wait for the next occurence of the specified video scan-line.
///
/// In general we must assume that the video memory is read top-to-bottom
/// as the picture is being drawn on the monitor (e.g. via a VGA video
/// signal). If you modify video memory during this *drawing period*
/// there is a risk that the image on the monitor (however briefly) may
/// contain some parts from before the modification and some parts from
/// after. This can given rise to the *tearing effect* where it looks
/// like the screen has been torn (or ripped) across because there is a
/// discontinuity part-way through the image.
///
/// This function busy-waits until the video drawing has reached a
/// specified scan-line on the video frame.
///
/// There is no error code here. If the line you ask for is beyond the
/// number of visible scan-lines in the current video mode, it waits util
/// the last visible scan-line is complete.
///
/// If you wait for the last visible line until drawing, you stand the
/// best chance of your pixels operations on the video RAM being
/// completed before scan-lines start being sent to the monitor for the
/// next frame.
///
/// You can also use this for a crude `16.7 ms` delay but note that
/// some video modes run at `70 Hz` and so this would then give you a
/// `14.3ms` second delay.
extern "C" fn video_wait_for_line(_line: u16) {
	debug!("video_wait_for_line()");
	// TODO
}

extern "C" fn video_get_palette(index: u8) -> common::FfiOption<common::video::RGBColour> {
	debug!("video_get_palette({})", index);
	let entry = PALETTE.get(usize::from(index));
	let entry_value =
		entry.map(|raw| common::video::RGBColour::from_packed(raw.load(Ordering::Relaxed)));
	match entry_value {
		Some(rgb) => common::FfiOption::Some(rgb),
		None => common::FfiOption::None,
	}
}

extern "C" fn video_set_palette(index: u8, rgb: common::video::RGBColour) {
	debug!("video_set_palette({}, #{:6x})", index, rgb.as_packed());
	if let Some(e) = PALETTE.get(usize::from(index)) {
		e.store(rgb.as_packed(), Ordering::Relaxed);
	}
}

unsafe extern "C" fn video_set_whole_palette(
	palette: *const common::video::RGBColour,
	length: usize,
) {
	debug!("video_set_whole_palette({:p}, {})", palette, length);
	let slice = std::slice::from_raw_parts(palette, length);
	for (entry, new_rgb) in PALETTE.iter().zip(slice) {
		entry.store(new_rgb.as_packed(), Ordering::Relaxed);
	}
}

extern "C" fn i2c_bus_get_info(_i2c_bus: u8) -> common::FfiOption<common::i2c::BusInfo> {
	debug!("i2c_bus_get_info");
	common::FfiOption::None
}

extern "C" fn i2c_write_read(
	_i2c_bus: u8,
	_i2c_device_address: u8,
	_tx: common::FfiByteSlice,
	_tx2: common::FfiByteSlice,
	_rx: common::FfiBuffer,
) -> common::ApiResult<()> {
	debug!("i2c_write_read");
	common::ApiResult::Err(common::Error::Unimplemented)
}

extern "C" fn audio_mixer_channel_get_info(
	_audio_mixer_id: u8,
) -> common::FfiOption<common::audio::MixerChannelInfo> {
	debug!("audio_mixer_channel_get_info");
	common::FfiOption::None
}

extern "C" fn audio_mixer_channel_set_level(
	_audio_mixer_id: u8,
	_level: u8,
) -> common::ApiResult<()> {
	debug!("audio_mixer_channel_set_level");
	common::ApiResult::Err(common::Error::Unimplemented)
}

extern "C" fn audio_output_set_config(_config: common::audio::Config) -> common::ApiResult<()> {
	debug!("audio_output_set_config");
	common::ApiResult::Err(common::Error::Unimplemented)
}

extern "C" fn audio_output_get_config() -> common::ApiResult<common::audio::Config> {
	debug!("audio_output_get_config");
	common::ApiResult::Err(common::Error::Unimplemented)
}

unsafe extern "C" fn audio_output_data(_samples: common::FfiByteSlice) -> common::ApiResult<usize> {
	debug!("audio_output_data");
	common::ApiResult::Err(common::Error::Unimplemented)
}

extern "C" fn audio_output_get_space() -> common::ApiResult<usize> {
	debug!("audio_output_get_space");
	common::ApiResult::Err(common::Error::Unimplemented)
}

extern "C" fn audio_input_set_config(_config: common::audio::Config) -> common::ApiResult<()> {
	debug!("audio_input_set_config");
	common::ApiResult::Err(common::Error::Unimplemented)
}

extern "C" fn audio_input_get_config() -> common::ApiResult<common::audio::Config> {
	debug!("audio_input_get_config");
	common::ApiResult::Err(common::Error::Unimplemented)
}

extern "C" fn audio_input_data(_samples: common::FfiBuffer) -> common::ApiResult<usize> {
	debug!("audio_input_data");
	common::ApiResult::Err(common::Error::Unimplemented)
}

extern "C" fn audio_input_get_count() -> common::ApiResult<usize> {
	debug!("audio_input_get_count");
	common::ApiResult::Err(common::Error::Unimplemented)
}

extern "C" fn bus_select(_periperal_id: common::FfiOption<u8>) {
	debug!("bus_select");
}

extern "C" fn bus_get_info(_periperal_id: u8) -> common::FfiOption<common::bus::PeripheralInfo> {
	debug!("bus_get_info");
	common::FfiOption::None
}

extern "C" fn bus_write_read(
	_tx: common::FfiByteSlice,
	_tx2: common::FfiByteSlice,
	_rx: common::FfiBuffer,
) -> common::ApiResult<()> {
	debug!("bus_write_read");
	common::ApiResult::Err(common::Error::Unimplemented)
}

extern "C" fn bus_exchange(_buffer: common::FfiBuffer) -> common::ApiResult<()> {
	debug!("bus_exchange");
	common::ApiResult::Err(common::Error::Unimplemented)
}

extern "C" fn time_ticks_get() -> common::Ticks {
	let mut hw_guard = HARDWARE.lock().unwrap();
	let hw = hw_guard.as_mut().unwrap();
	let boot_time = hw.boot_time;
	let difference = boot_time.elapsed();
	debug!("time_ticks_get() -> {}", difference.as_millis());
	common::Ticks(difference.as_millis() as u64)
}

/// We simulate a 1 kHz tick
extern "C" fn time_ticks_per_second() -> common::Ticks {
	debug!("time_ticks_per_second()");
	common::Ticks(1000)
}

extern "C" fn bus_interrupt_status() -> u32 {
	debug!("bus_interrupt_status()");
	0
}

extern "C" fn block_dev_get_info(dev_id: u8) -> common::FfiOption<common::block_dev::DeviceInfo> {
	debug!("block_dev_get_info(dev_id: {})", dev_id);
	let mut hw_guard = HARDWARE.lock().unwrap();
	let hw = hw_guard.as_mut().unwrap();
	if dev_id == 0 {
		match &mut hw.disk_file {
			Some(file) => common::FfiOption::Some(common::block_dev::DeviceInfo {
				name: common::FfiString::new("File0"),
				device_type: common::block_dev::DeviceType::HardDiskDrive.into(),
				block_size: BLOCK_SIZE as u32,
				num_blocks: file.metadata().unwrap().len() / (BLOCK_SIZE as u64),
				ejectable: false,
				removable: false,
				media_present: true,
				read_only: false,
			}),
			None => common::FfiOption::None,
		}
	} else {
		common::FfiOption::None
	}
}

extern "C" fn block_dev_eject(dev_id: u8) -> common::ApiResult<()> {
	debug!("block_dev_eject(dev_id: {})", dev_id);
	common::ApiResult::Ok(())
}

extern "C" fn block_write(
	dev_id: u8,
	block_idx: common::block_dev::BlockIdx,
	num_blocks: u8,
	buffer: common::FfiByteSlice,
) -> common::ApiResult<()> {
	debug!(
		"block_write(dev_id: {}, block_id: {}, num_blocks: {}, buffer_len: {})",
		dev_id, block_idx.0, num_blocks, buffer.data_len
	);
	let mut hw_guard = HARDWARE.lock().unwrap();
	let hw = hw_guard.as_mut().unwrap();
	if dev_id == 0 {
		match &mut hw.disk_file {
			Some(file) => {
				if file
					.seek(std::io::SeekFrom::Start(block_idx.0 * BLOCK_SIZE as u64))
					.is_err()
				{
					return common::ApiResult::Err(common::Error::BlockOutOfBounds);
				}
				let buffer_slice = &buffer.as_slice()[0..usize::from(num_blocks) * BLOCK_SIZE];
				if let Err(e) = file.write_all(buffer_slice) {
					log::warn!("Failed to write to disk image: {:?}", e);
					return common::ApiResult::Err(common::Error::DeviceError);
				}
				common::ApiResult::Ok(())
			}
			None => common::ApiResult::Err(common::Error::DeviceError),
		}
	} else {
		common::ApiResult::Err(common::Error::InvalidDevice)
	}
}

extern "C" fn block_read(
	dev_id: u8,
	block_idx: common::block_dev::BlockIdx,
	num_blocks: u8,
	mut buffer: common::FfiBuffer,
) -> common::ApiResult<()> {
	debug!(
		"block_read(dev_id: {}, block_id: {}, num_blocks: {}, buffer_len: {})",
		dev_id, block_idx.0, num_blocks, buffer.data_len
	);
	let mut hw_guard = HARDWARE.lock().unwrap();
	let hw = hw_guard.as_mut().unwrap();
	if dev_id == 0 {
		match &mut hw.disk_file {
			Some(file) => {
				if file
					.seek(std::io::SeekFrom::Start(block_idx.0 * BLOCK_SIZE as u64))
					.is_err()
				{
					return common::ApiResult::Err(common::Error::BlockOutOfBounds);
				}
				if let Some(buffer_slice) = buffer.as_mut_slice() {
					let buffer_slice = &mut buffer_slice[0..usize::from(num_blocks) * BLOCK_SIZE];
					if let Err(e) = file.read_exact(buffer_slice) {
						log::warn!("Failed to read from disk image: {:?}", e);
						return common::ApiResult::Err(common::Error::DeviceError);
					}
				}
				common::ApiResult::Ok(())
			}
			None => common::ApiResult::Err(common::Error::DeviceError),
		}
	} else {
		common::ApiResult::Err(common::Error::InvalidDevice)
	}
}

extern "C" fn block_verify(
	dev_id: u8,
	block_idx: common::block_dev::BlockIdx,
	num_blocks: u8,
	buffer: common::FfiByteSlice,
) -> common::ApiResult<()> {
	debug!(
		"block_read(dev_id: {}, block_id: {}, num_blocks: {}, buffer_len: {})",
		dev_id, block_idx.0, num_blocks, buffer.data_len
	);
	let mut hw_guard = HARDWARE.lock().unwrap();
	let hw = hw_guard.as_mut().unwrap();
	if dev_id == 0 {
		match &mut hw.disk_file {
			Some(file) => {
				if file
					.seek(std::io::SeekFrom::Start(block_idx.0 * BLOCK_SIZE as u64))
					.is_err()
				{
					return common::ApiResult::Err(common::Error::BlockOutOfBounds);
				}
				let buffer_slice = &buffer.as_slice()[0..usize::from(num_blocks) * BLOCK_SIZE];
				let mut read_buffer = vec![0u8; buffer_slice.len()];
				if let Err(e) = file.read_exact(&mut read_buffer) {
					log::warn!("Failed to write to disk image: {:?}", e);
					return common::ApiResult::Err(common::Error::DeviceError);
				}
				if read_buffer.as_slice() == buffer_slice {
					common::ApiResult::Ok(())
				} else {
					common::ApiResult::Err(common::Error::DeviceError)
				}
			}
			None => common::ApiResult::Err(common::Error::DeviceError),
		}
	} else {
		common::ApiResult::Err(common::Error::InvalidDevice)
	}
}

extern "C" fn power_idle() {
	std::thread::sleep(std::time::Duration::from_millis(1));
}

extern "C" fn power_control(mode: common::FfiPowerMode) -> ! {
	println!("Got power mode {:?}, but quitting...", mode);
	std::process::exit(0);
}

extern "C" fn compare_and_swap_bool(
	item: &std::sync::atomic::AtomicBool,
	old_value: bool,
	new_value: bool,
) -> bool {
	item.compare_exchange(old_value, new_value, Ordering::Relaxed, Ordering::Relaxed)
		.is_ok()
}

// ===========================================================================
// Impl Blocks
// ===========================================================================

impl MyApp {
	const NUM_FG: usize = 16;

	/// Generate an RGBA texture for each glyph, in each foreground colour.
	///
	/// We have 256 glyphs, in each of 16 colours, so this is expensive and
	/// slow. But it makes rendering text acceptably fast.
	fn render_font(
		font: &font::Font,
		texture_buffer: &mut Vec<TextureId>,
		s: &mut PixState,
	) -> PixResult<()> {
		let mut slot = 0;
		for glyph in 0..=255 {
			for palette_entry in PALETTE.iter().take(Self::NUM_FG) {
				let fg = RGBColour::from_packed(palette_entry.load(Ordering::Relaxed));
				info!("Drawing {glyph} in {:06x}", fg.as_packed());
				let texture_id = if texture_buffer.len() > slot {
					texture_buffer[slot]
				} else {
					let id = s.create_texture(8, font.height as u32, PixelFormat::Rgba)?;
					texture_buffer.push(id);
					id
				};
				slot += 1;
				s.set_texture_target(texture_id)?;
				s.background(Color::TRANSPARENT);
				s.clear()?;
				s.stroke(rgb!(fg.red(), fg.green(), fg.blue(), 255));
				for font_y in 0..(font.height as i32) {
					let mut font_line =
						font.data[((glyph as usize) * font.height) + font_y as usize];
					for font_x in 0..8i32 {
						if (font_line & 0x80) != 0 {
							s.point(Point::new([font_x, font_y]))?;
						};
						font_line <<= 1;
					}
				}
				s.clear_texture_target();
			}
		}
		Ok(())
	}

	/// Generate an RGBA texture for each glyph, in each foreground colour, in
	/// each font.
	fn render_glyphs(&mut self, s: &mut PixState) -> PixResult<()> {
		Self::render_font(&font::font16::FONT, &mut self.font8x16, s)?;
		Self::render_font(&font::font8::FONT, &mut self.font8x8, s)?;
		Ok(())
	}
}

impl PixEngine for MyApp {
	/// Perform application initialisation.
	fn on_start(&mut self, s: &mut PixState) -> PixResult<()> {
		self.render_glyphs(s)?;
		// Let the rest of the OS start now
		self.sender.send(AppEvent::Started).unwrap();
		Ok(())
	}

	/// Terminate the process to ensure the OS thread dies too.
	fn on_stop(&mut self, _s: &mut PixState) -> PixResult<()> {
		std::process::exit(0);
	}

	/// Called whenever the app has an event to process.
	///
	/// We send key up and key down events into a queue for the OS to process later.
	fn on_event(&mut self, _s: &mut PixState, event: &Event) -> PixResult<bool> {
		match event {
			Event::KeyUp {
				key: Some(key),
				keymod: _,
				repeat: _,
			} => {
				self.sender.send(AppEvent::KeyUp(*key)).unwrap();
				Ok(true)
			}
			Event::KeyDown {
				key: Some(key),
				keymod: _,
				repeat: _,
			} => {
				self.sender.send(AppEvent::KeyDown(*key)).unwrap();
				Ok(true)
			}
			_ => Ok(false),
		}
	}

	/// Called in a tight-loop to update the application.
	///
	/// We convert the contents of `FRAMEBUFFER` into pixels on the canvas.
	fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
		let mode_value = VIDEO_MODE.load(Ordering::Relaxed);
		let new_mode = unsafe { common::video::Mode::from_u8(mode_value) };
		if new_mode != self.mode {
			self.mode = new_mode;
			let width = (new_mode.horizontal_pixels() as f32) * SCALE_FACTOR;
			let height = (new_mode.vertical_lines() as f32) * SCALE_FACTOR;
			info!("Window set to {} x {}", width, height);
			s.set_window_dimensions((width as u32, height as u32))?;
			s.scale(SCALE_FACTOR, SCALE_FACTOR)?;
		}

		s.blend_mode(BlendMode::Blend);

		let (font, font_height) = match self.mode.format() {
			common::video::Format::Text8x16 => (&self.font8x16, 16),
			common::video::Format::Text8x8 => (&self.font8x8, 8),
			_ => {
				// Unknown mode - do nothing
				return Ok(());
			}
		};

		let num_cols = self.mode.text_width().unwrap();
		let num_rows = self.mode.text_height().unwrap();
		// FRAMEBUFFER is an num_cols x num_rows size array of (u8_glyph, u8_attr).
		for row in 0..num_rows {
			let y = row * font_height;
			for col in 0..num_cols {
				let cell_no = (row * num_cols) + col;
				let byte_offset = usize::from(cell_no) * 2;
				let x = col * 8;
				let glyph = FRAMEBUFFER.get_at(byte_offset);
				let attr = common::video::Attr(FRAMEBUFFER.get_at(byte_offset + 1));
				let fg_idx = attr.fg().make_ffi_safe().0;
				let bg_idx = attr.bg().make_ffi_safe().0;
				let bg =
					RGBColour::from_packed(PALETTE[usize::from(bg_idx)].load(Ordering::Relaxed));
				let glyph_box = rect!(i32::from(x), i32::from(y), 8i32, font_height as i32,);
				s.fill(rgb!(bg.red(), bg.green(), bg.blue()));
				s.rect(glyph_box)?;
				let slot = (usize::from(glyph) * Self::NUM_FG) + usize::from(fg_idx);
				s.texture(font[slot], None, Some(glyph_box))?;
			}
		}

		Ok(())
	}
}

impl<const N: usize> Framebuffer<N> {
	/// Create a new blank Framebuffer.
	///
	/// Everything is zero initialised.
	const fn new() -> Framebuffer<N> {
		Framebuffer {
			contents: std::cell::UnsafeCell::new([0u8; N]),
			alt_pointer: AtomicPtr::new(core::ptr::null_mut()),
		}
	}

	/// Set a byte in the framebuffer.
	///
	/// Panics if you try and write out of bounds.
	///
	/// Uses volatile writes.
	fn write_at(&self, offset: usize, value: u8) {
		unsafe {
			let array_ptr = self.get_pointer() as *mut u8;
			let byte_ptr = array_ptr.add(offset);
			byte_ptr.write_volatile(value);
		}
	}

	/// Get a byte from the framebuffer.
	///
	/// Panics if you try and read out of bounds.
	///
	/// Uses volatile reads.
	fn get_at(&self, offset: usize) -> u8 {
		unsafe {
			let array_ptr = self.get_pointer() as *const u8;
			let byte_ptr = array_ptr.add(offset);
			byte_ptr.read_volatile()
		}
	}

	/// Get a pointer to the framebuffer you can give to the OS.
	fn get_pointer(&self) -> *mut u32 {
		let mut p = self.alt_pointer.load(Ordering::Relaxed);
		if p.is_null() {
			p = self.contents.get() as *mut u32;
		}
		p
	}
}

unsafe impl<const N: usize> Sync for Framebuffer<N> {}

// ===========================================================================
// End of File
// ===========================================================================
