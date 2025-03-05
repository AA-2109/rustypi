# LAN_check

`LAN_check` is a command-line tool that checks LAN connectivity for a given IP address and attempts to restart the network if it is down on Debian-based systems.

## Table of Contents

1. [Installation](#installation)
2. [Usage](#usage)

## Installation

To install `LAN_check`, you need to have Rust installed on your system. If you don't have Rust installed, you can download and install it from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository and build the project:

```sh
git clone https://github.com/yourusername/LAN_check.git
cd LAN_check
cargo build --release
```
## Usage

To check the LAN connectivity for a specific IP address, use the following command:
```sh
./target/release/LAN_check --ip=<IP_ADDRESS>
```
