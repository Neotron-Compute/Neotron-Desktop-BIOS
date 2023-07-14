# Neotron Desktop BIOS

This is the [Neotron](https://github.com/neotron-compute) BIOS that lets you run the Neotron OS as a Linux, macOS or Windows application!

![Build Status](https://github.com/neotron-compute/neotron-desktop-bios/workflows/Build/badge.svg "Github Action Build Status")

![Format Status](https://github.com/neotron-compute/neotron-desktop-bios/workflows/Format/badge.svg "Github Action Format Check Status")

## Compatibility

This BIOS uses [pix-engine](https://crates.io/crates/pix-engine), so should run on any platform that pix-engine supports.

If you have a Mac, run:

```console
$ brew install sdl2
$ brew install sdl2_mixer
$ brew install sdl2_image
$ export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib" 
```

You will need to re-run the `export` command before you re-build the application.

## Building on Linux

Build and run this BIOS (and use it to boot Neotron OS) with...

```console
~ $ git checkout https://github.com/neotron-compute/Neotron-Desktop-BIOS.git
~ $ cd Neotron-Desktop-BIOS
~/Neotron-Desktop-BIOS $ RUST_LOG=debug cargo run -- --serial=/dev/ttyS0 --peripheral=sdmmc,./disk.img --os=./libneotron_os.so
```

Press `Esc` with the GUI window selected to quit the BIOS.

The file `libneotron_os.so` is not supplied. You can build it with:

```console
~ $ git checkout https://github.com/neotron-compute/neotron-os.git
~ $ cd neotron-os
~/neotron-os $ cargo build --release --lib
~/neotron-os $ ls ./target/release/*.so
./target/release/libneotron_os.so
~/neotron-os $ cp ./target/release/libneotron_os.so ~/Neotron-Desktop-BIOS
```

## Building on Windows

1. Install and bootstrap [vcpkg](https://github.com/microsoft/vcpkg)
2. Install the SDL2 libraries with vcpkg:

  ```console
  C:\Users\user\Documents\vcpkg> ./vcpkg.exe install sdl2-ttf:x64-windows sdl2:x64-windows sdl2-mixer:x64-windows sdl2-gfx:x64-windows sdl2-ttf:x64-windows sdl2-image:x64-windows
  ```

3. Set your PATH, INCLUDE and LIB to include the directories in your vcpkg install folder:

  ```console
  C:\Users\user\Documents> set PATH=%PATH%;C:\Users\user\Documents\vcpkg\installed\x64-windows\bin
  C:\Users\user\Documents> set INCLUDE=%INCLUDE%;C:\Users\user\Documents\vcpkg\installed\x64-windows\include
  C:\Users\user\Documents> set LIB=%LIB%;C:\Users\user\Documents\vcpkg\installed\x64-windows\lib
  ```

## Features

* GUI window with pixel-perfect video rendering
* TODO: UART support
* TODO: SD/MMC emulation support
* TODO: Audio support
* TODO: Human Interface Device support

## Changelog

### Unreleased Changes ([Source](https://github.com/neotron-compute/Neotron-Desktop-BIOS/tree/main))

* First release

## Licence

	Neotron-Desktop-BIOS Copyright (c) Jonathan 'theJPster' Pallant, 2022

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
