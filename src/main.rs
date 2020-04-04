use std::env;

use diceroll::*;

fn main() {
    for arg in env::args().skip(1) {
        let mut dice_roll = parse_roll_string(arg);
        roll_dice(&mut dice_roll);

        let result =
            dice_roll.dice.iter().fold(0, |sum, die| sum + die.roll) +
            dice_roll.modifier;

        println!("{}", result);
    }
}
