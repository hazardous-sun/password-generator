# Password Generator

This is a simple random password generator written in Rust as a way of studying some basic concepts of the language.

## Installation

Apart from just running with application with Cargo, you can use the "installer.sh" as sudo to add the command to "
/bin/" in order to easy running the command
on a CLI.

## Usage

### Cargo

`[OPTIONS] cargo run [PASSWORD_LEN]`

#### Environment variables

* UPPER_CASE: Allows the usage of upper case letters in the password -> "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
* LOWER_CASE: Allows the usage of lower case letters in the password -> "abcdefghijklmnopqrstuvwxyz"
* NUMBERS: Allows the usage of numbers in the password -> "0123456789"
* MATH_SYM: Allows the usage of math symbols in the password -> "-+=*/><[]{}()"
* EXTRA_SYM: Allows the usage of extra symbols in the password -> "?!@#$%&_|;:"
* CHECK_REP: Checks for character repetitions in the password. If the character is equal to 2 of the last 3 inserted, 
the program inserts a different one.
