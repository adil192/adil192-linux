# adil192-linux

This is a collection of scripts that apply my personal preferences to a Linux system.
It installs apps, drivers and themes.

It is generally intended to be used with the [Fedora COSMIC Spin](https://fedoraproject.org/spins/cosmic/).

This project is for my personal use:
no guarantees of any kind are made.

## Installation

```bash
# Cd into a directory where you want to clone the repository
mkdir -p ~/Documents/GitHub/ && cd ~/Documents/GitHub/

# Clone the repository
git clone https://github.com/adil192/adil192-linux.git
cd adil192-linux

# Install Rust if you haven't yet
./bootstrap/install_rust.sh

# Run the scripts and follow the yes/no prompts (press Enter to accept defaults)
cargo run
```

## Uninstallation

Run `cargo run -- --uninstall` and follow the yes/no prompts.

Note that most changes are not uninstallable,
so you may have to manually reverse them.
