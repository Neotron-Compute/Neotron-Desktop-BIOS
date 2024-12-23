use neotron_common_bios::video::RGBColour;
use std::sync::atomic::AtomicU32;

pub(crate) const fn make_default_palette() -> [std::sync::atomic::AtomicU32; 256] {
	[
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
	]
}
