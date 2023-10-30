#!/bin/bash

if [[ $SUDO_USER ]]; then
    ORIGINAL_USER="$SUDO_USER"
else
    echo "The script requires sudo to run."
    exit 1
fi

CURRENT_DIRECTORY="$(pwd)"
CARGO_HOME="/home/$ORIGINAL_USER/.cargo"
RUSTUP_HOME="/home/$ORIGINAL_USER/.rustup"

# Use the -E option to preserve the environment when running cargo with sudo
sudo -E -u "$ORIGINAL_USER" CARGO_HOME="$CARGO_HOME" RUSTUP_HOME="$RUSTUP_HOME" /home/"$ORIGINAL_USER"/.cargo/bin/cargo build

# Move the binary to the user's bin directory
sudo mv "$CURRENT_DIRECTORY"/target/debug/password_generator /bin/
