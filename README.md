# SSH Brute-Force Attack using Rust

This project demonstrates how to perform a brute-force attack on an SSH server using Rust. The code attempts to connect to an SSH server with a list of potential passwords, identifying the correct password through multiple login attempts.

## Disclaimer

**This code is for educational purposes only. Unauthorized access to computer systems is illegal and unethical. Use this code responsibly and only on systems for which you have explicit permission.**

## Features

- Connects to an SSH server and attempts to authenticate using a list of passwords.
- Detects successful login and stops further attempts.

## Requirements

- Rust (Install from [rust-lang.org](https://www.rust-lang.org/))
- SSH2 library (libssh2)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/Yukibhl/SSH-Brute-Force-attack.git
   cd SSH-Brute-Force-attack
2. Install dependencies:

Ensure that you have libssh2 installed on your system.

      On Debian/Ubuntu:
        sudo apt-get install libssh2-1-dev
      On macOS:
        brew install libssh2
3. Build the project:
cargo build --release
