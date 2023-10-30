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
CARGO_BUILD="/home/$ORIGINAL_USER/.cargo/bin/cargo build"

# Use the -E option to preserve the environment when running cargo with sudo
sudo -E -u "$ORIGINAL_USER" sh -c "export CARGO_HOME='$CARGO_HOME' && export RUSTUP_HOME='$RUSTUP_HOME' && $CARGO_BUILD"

# Rename password_generator to passgen
mv "$CURRENT_DIRECTORY"/target/debug/password_generator "$CURRENT_DIRECTORY"/target/debug/passgen

# Move the binary to the user's bin directory
mv "$CURRENT_DIRECTORY"/target/debug/passgen /usr/local/bin/ --force
