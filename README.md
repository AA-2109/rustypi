# Rustypi

`rustypi` is a command-line tool that checks LAN network, mounted share and updates Pi-hole Gravity database.
Ideal for running periodically with cron, automating Raspberry Pi and Pi-Hole maintenance.

## Table of Contents

1. [Installation](#installation)
2. [Usage](#usage)

## Installation

To install `rustypi`, you need to have Rust installed on your system. If you don't have Rust installed, you can download and install it from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository and build the project:

```sh
git clone https://github.com/yourusername/rustypi.git
cd rustypi
cargo build --release
```
Alternatively, download compiled binary files from Release tab.

## Usage
Run `rustypi` with path to mounted share as an argument:
```sh
./target/release/rustypi /mnt/share
```
To run periodically - use crontab -e:
```sh
*/60 * * * * /home/pi/Dokumenty/rustypi/target/release/rustypi /mnt/share
```