# Password Generator

This is a simple random password generator written in Rust as a way of studying some basic concepts of the language.

## Installation

Apart from just running the application with Cargo, you can use the "installer.sh" as sudo to add the command
to `/usr/bin/` in order to easy running the command
on a CLI.

## Options

* -u    --upper
  * Allows the usage of upper case letters in the password
  * `ABCDEFGHIJKLMNOPQRSTUVWXYZ`
* -l    --lower
  * Allows the usage of lower case letters in the password
  * `abcdefghijklmnopqrstuvwxyz`
* -n    --numbers
  * Allows the usage of numbers in the password
  * `0123456789`
* -b    --basic-sym
  * Allows the usage of math symbols in the password
  * `-+=*/><[]{}()`
* -e    --extra-sym
  * Allows the usage of extra symbols in the password
  * `?!@#$%&_|;:`
* -r    --check-rep
  * Checks for character repetitions in the password. If the character is equal to two of the last three digits,
    the program rerolls the character and inserts a different one.
