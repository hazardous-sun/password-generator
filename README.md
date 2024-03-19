# Password Generator

This is a simple random password generator written in Rust to explore some fundamental language concepts. It provides
secure password creation with customizable options.

## Installation

### Running with Cargo:

The recommended way to run the application is using Cargo, Rust's package manager. Ensure you have Rust
installed (https://www.rust-lang.org/tools/install). Then, navigate to the project directory and run:

```shell
cargo run
```

### Optional Installation Script:

For easier access from the command line, you can optionally use the provided [installer.sh script](INSTALLER.md).
However, run this script with caution as it involves using sudo and modifying system paths.

Here's how to use it (if you choose to):

1. Open a terminal in the project directory.
2. Run the following command, providing your password when prompted:

```shell
sudo sh installer.sh
```

This will add the password generator command to your system's /usr/bin/ directory, allowing you to run it from any
location using passgen.

## Options

The password generator offers various options to customize the generated password:

* -u, --upper: Includes uppercase letters (A-Z) in the password.
* -l, --lower: Includes lowercase letters (a-z) in the password.
* -n, --numbers: Includes numbers (0-9) in the password.
* -b, --basic-sym: Includes basic mathematical symbols (+, -, *, /, etc.) in the password.
* -e, --extra-sym: Includes additional symbols (?!@#$%&, etc.) in the password.
* -r, --check-rep: Ensures no character repeats consecutively within the password (prevents weak patterns).

## Usage Example

To generate a password with 16 characters, including uppercase letters, lowercase letters, and numbers, run:

```shell
passgen 16 -u -l -n
```

## Security Considerations

* While this tool helps generate random passwords, consider using a dedicated password manager for enhanced security and
  management of multiple passwords.
* Always choose strong passwords with a mix of character types and sufficient length.

## Contribution

Feel free to contribute to this project by creating pull requests on
our [GitHub repository](https://github.com/hazardous-sun/password-generator.git).