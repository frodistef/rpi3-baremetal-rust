# Raspberry Pi 3+ Baremetal with Rust


This project is my first project published on GitHub.

I started with this after seeing a [YouTube video](https://www.youtube.com/watch?v=jZT8APrzvc4) of [lowlevellearning](https://github.com/lowlevellearning).


Goals of this project:

* learn using Rust for baremetal programming.
* learn using GitHub


## Credits: 

Credits go to:

[lowlevellearning](https://github.com/lowlevellearning) for:
* Getting me interested in the first place;
* Giving clear instructions on how to start;
* Sharing knowledge/code.

[Paul Simon](https://www.youtube.com/channel/UCPzYEjoLx1Y7fikoVriGC8g) as I took some of his remarks in the comments of the YouTube video.

## Source

I took some source files from 

Linker script

As base, I took the linker script from [lowlevellearning](https://github.com/lowlevellearning):
https://github.com/lowlevellearning/raspberry-pi-baremetal-c


## Setup

Hardware:
* PC, running Debian (Bullseye);
* Raspberry Pi 3+, as I had one available at the time of starting;
* Power supply;
* 16GB SD card, formatted as FAT32, as storage device;
* USB-SD convertor to write files to SD.

Software:
* Debian Bullseye;
* Rust compiler, toolchain etc. etc.;
* Microsoft Visual Studio Code;
* GIT.


### OS

I'm using a Debian (Bullseye) distrubution with Rust installed using:

```
curl https://sh.rustup.rs -sSf | sh
```

### Install Rust toolchain for RPI:

```
rustup toolchain add armv7a-none-eabi
```

### Install binutils for Cargo

```
cargo install cargo-binutils

rustup component add llvm-tools-preview
```

### Download RPi files

Some extra files are needed, next to `kernel7.img` to run the code on the RPI.

From: https://github.com/raspberrypi/firmware/tree/master/boot

Download: `bootcode.bin`, `start.elf` and `fixup.dat`

These are placed in the folder `to_put_on_sd`.


## Build

### To build the code:

```
cargo build
```

### Make the RPI kernel image

``` 
cargo objcopy -- -O binary ./to_put_on_sd/kernel7.img
```


## Run

Mount SD card.

Copy the files in `./to_put_on_sd/` in the root of the SDCard:

* `kernel7.img`
* `bootcode.bin`
* `start.elf`
* `fixup.dat`
* `config.txt`

Unmount SD card.

Place SD in Raspberry Pi.

Power Raspberry Pi.

## Usefull commands

To check if the addresses (for example the start of the code) is correct:

```
arm-none-eabi-objdump -D target/armv7a-none-eabi/debug/my-rusty-pi|less
```

To disassemble a 'release' binary:

```
cargo objdump --release -- --disassemble --no-show-raw-insn
```