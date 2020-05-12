# diceroll

Small cli dicerolling program written in Rust.

# Usage

Pass the required dice rolls as arguments along with any modifiers. For dice use the common NdX notation where N is the number of dice and X is the type, eg 2d20. Modifiers are plain integers. Combine multiple dice and modifiers with '+' signs.

For example,

    $ ./diceroll 1d20+2d8+3

might output

> 18 (d20) + 5 (d8) + 5 (d8) + 3 = 31

Don't put spaces between elements of a single roll. Use spaces to separate independent rolls instead. For example you could roll attack and damage together.

    $ ./diceroll 1d20+7 1d8+6

> 18 (d20) + 7 = 25  
> 6 (d8) + 6 = 12

If you absolutely want extra spaces for readability then wrapping the arguments in quotes would work.

    $ ./diceroll '1d20 + 7' '1d8 + 6'

# Building

This project is managed by cargo, rust's package manager. Make sure you have it installed, then inside of the project dir, run

    $ cargo build

or for the optimized release build

    $ cargo build --release

# TODO

* Cleaner error messages. Now it just outputs the rust assertion failures.
* man page & --help option
* Maybe some indication of lowest/highest rolls, eg color-coding. This is useful for automatic failures & criticals.
* Add advantage/disadvantage
* Some form of preconfigured rolls. Could be as simple as a configuration file with named rolls

      attack: 1d20+7

  useable as

      diceroll attack

  Or maybe a rudimentary character sheet from which the rolls can be automatically generated.
