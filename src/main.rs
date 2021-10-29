use std::env;

use diceroll::*;

fn main() {
    for arg in env::args().skip(1) {
        let mut dice_roll = parse_roll_string(arg);
        assert!(!dice_roll.dice.is_empty());

        roll_dice(&mut dice_roll);

        print!("{} (d{})", dice_roll.dice[0].roll, dice_roll.dice[0].max);

        for die in dice_roll.dice.iter().skip(1) {
            print!(" {0} {1} (d{2})", die.sign.to_char(), die.roll, die.max);
        }

        if dice_roll.modifier > 0 {
            print!(" + {}", dice_roll.modifier);
        }
        else if dice_roll.modifier < 0 {
            print!(" - {}", -dice_roll.modifier);
        }

        let result = dice_roll
            .dice
            .iter()
            .fold(0, |sum, die| sum + die.roll * die.sign.to_multiplier())
            + dice_roll.modifier;

        println!(" = {}", result);
    }
}
