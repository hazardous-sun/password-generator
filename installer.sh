#!/bin/bash

if [[ $SUDO_USER ]]; then
    ORIGINAL_USER="$SUDO_USER"
else
    echo "The script requires sudo to run."
    echo "Usage:"
    echo "    Option 1: installer.sh"
    echo "    Option 2: installer.sh [CARGO_BUILD_LOCATION] [CARGO_HOME_LOCATION] [RUSTUP_HOME_LOCATION]"
    exit 1
fi

CURRENT_DIRECTORY="$(pwd)"
CARGO_HOME=""
RUSTUP_HOME=""
CARGO_BUILD=""

if [[ $1 ]]; then
  CARGO_BUILD=$1
else
  CARGO_BUILD="/home/$ORIGINAL_USER/.cargo/bin/cargo build"
fi

if [[ $2 ]]; then
  CARGO_HOME=$2
else
  CARGO_HOME="/home/$ORIGINAL_USER/.cargo"
fi

if [[ $3 ]]; then
  RUSTUP_HOME=$3
else
  RUSTUP_HOME="/home/$ORIGINAL_USER/.rustup"
fi

# Use the -E option to preserve the environment when running cargo with sudo
sudo -E -u "$ORIGINAL_USER" sh -c "export CARGO_HOME='$CARGO_HOME' && export RUSTUP_HOME='$RUSTUP_HOME' && $CARGO_BUILD"

# Move the binary to the user's bin directory
mv "$CURRENT_DIRECTORY"/target/debug/passgen /usr/bin/ --force
