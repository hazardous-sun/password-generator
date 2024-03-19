# Shell Installation Script Documentation

The [installer.sh](installer.sh) script installs the compiled password generator binary (passgen) into the user's
system-wide bin directory (/usr/bin/).

## Requirements:

* Requires sudo privileges to run.
* Assumes the Rust compiler and Cargo are already installed.

## Usage

* Option 1: ./installer.sh (Recommended)
    * This option uses default locations for Cargo and Rustup.
* Option 2: ./installer.sh [CARGO_BUILD_LOCATION] [CARGO_HOME_LOCATION] [RUSTUP_HOME_LOCATION]
    * This option allows specifying custom installation paths for Cargo, Cargo's home directory, and Rustup's home
      directory (advanced usage).

### Steps

1. Check for sudo:

* The script verifies if the user is running it with sudo. If not, it displays an error message explaining usage options
  and exits.

2. Get Script Location:

* The script gets the current working directory using pwd and stores it in the CURRENT_DIRECTORY variable.

3. Define Installation Paths (Optional):

* It defines variables for Cargo build command (CARGO_BUILD), Cargo home directory (CARGO_HOME), and Rustup home
  directory (RUSTUP_HOME).
* If arguments are provided during execution:
    * The first argument is used for CARGO_BUILD (defaults to /home/$ORIGINAL_USER/.cargo/bin/cargo build).
    * The second argument is used for CARGO_HOME (defaults to /home/$ORIGINAL_USER/.cargo).
    * The third argument is used for RUSTUP_HOME (defaults to /home/$ORIGINAL_USER/.rustup).

4. Run Cargo Build with sudo:
    * The script uses sudo -E -u "$ORIGINAL_USER" sh -c "..." to run the Cargo build command with elevated privileges
      while preserving the user's environment variables.
        * -E: Preserves environment variables.
        * -u "$ORIGINAL_USER": Runs the command as the original user who provided sudo (important for proper path
          handling).
        * sh -c "...": Executes the provided shell command within the script (the actual cargo build command with
          environment variable exports).
    * This ensures Cargo uses the potentially custom installation paths if provided.
5. Move Binary:
    * After successful compilation, the script uses mv to move the generated binary (passgen) from the project's target
      directory ("$CURRENT_DIRECTORY"/target/debug/passgen) to the system-wide bin directory (/usr/bin/) using the
      --force flag to overwrite any existing file.

## Notes

* This script modifies system paths, so use it with caution and ensure you understand the implications.
* The recommended usage is to run the script without arguments, which uses default locations.
* Custom paths are for advanced users who have specific installation setups for Cargo and Rustup.