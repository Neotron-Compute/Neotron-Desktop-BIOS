# Neotron-XXX-BIOS

> Template repository for new BIOS implementations. You should delete these
> quoted blocks and correct the rest of the text as required.

This is the [Neotron](https://github.com/neotron-compute) BIOS for the FooBar XYZ development board.

![Build Status](https://github.com/$GITHUB_USERNAME/$GITHUB_REPO/workflows/Build/badge.svg "Github Action Build Status")

![Format Status](https://github.com/$GITHUB_USERNAME/$GITHUB_REPO/workflows/Format/badge.svg "Github Action Format Check Status")

## Compatibility

This BIOS will run on the official FooBar XYZ Developer Kit, and also the FooBar XYZ Explorer board. Both feature the same FooBar XYZ SoC. Other boards with the same SoC can be supported with a minor change to the pin configurations.

## Features

> Replace this with the specs of your board!

The FooBar XYZ Developer Kit offers:

* 128 KiB RAM
* 512 KiB Flash
* Cortex-M4 clocked at 64 MHz
* SD/MMC Slot, with DMA
* 3-wire TTL UART
* Hardware accelerated graphics blitter
* 8-colour VGA output on pins P0, P1, P2, P4 and P5.
* Stereo sound on the on-board 3.5mm jack

The FooBar XYZ Explorer Board adds:

* On-board LCD with 480x272 resolution (60x17 text mode) in 16 colours
* RS-232 port

## Changelog

> Your repo should implement a Changelog in this format. Add a new section
> every time you tag a release. Don't forget to change `$GITHUB_USERNAME` and
> `$GITHUB_REPO` to the appropriate values for your repository!

### Unreleased Changes ([Source](https://github.com/$GITHUB_USERNAME/$GITHUB_REPO/tree/master) | [Changes](https://github.com/$GITHUB_USERNAME/$GITHUB_REPO/compare/v0.2.0...master))

* None

### v0.2.0 ([Source](https://github.com/$GITHUB_USERNAME/$GITHUB_REPO/tree/v0.2.0) | [Changes](https://github.com/$GITHUB_USERNAME/$GITHUB_REPO/compare/v0.1.0...v0.2.0))

* Fixed changelog in README.

### v0.1.0 ([Source](https://github.com/$GITHUB_USERNAME/$GITHUB_REPO/tree/v0.1.0))

* First release

## Licence

> This is an example licence. You can change it if you so desire, but please
> do pay attention to the licenses of any components you use in your BIOS.

	Neotron-XXX-BIOS Copyright (c) Some Developer, 2019

	This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you shall be licensed as above, without
any additional terms or conditions.
