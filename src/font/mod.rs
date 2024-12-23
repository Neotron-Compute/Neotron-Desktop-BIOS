//! # Fonts for the Neotron Desktop BIOS
//!

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

// -----------------------------------------------------------------------------
// Sub-modules
// -----------------------------------------------------------------------------

pub mod font16;
pub mod font8;

// -----------------------------------------------------------------------------
// Imports
// -----------------------------------------------------------------------------

// None

// -----------------------------------------------------------------------------
// Types
// -----------------------------------------------------------------------------

/// A font
pub struct Font<'a> {
	pub name: &'static str,
	pub height: usize,
	pub data: &'a [u8],
}

// -----------------------------------------------------------------------------
// Functions
// -----------------------------------------------------------------------------

// None

// -----------------------------------------------------------------------------
// End of file
// -----------------------------------------------------------------------------
