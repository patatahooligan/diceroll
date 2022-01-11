use std::env;

use diceroll::*;

fn print_help() {
    print!("Usage: {} ", env::args().next().unwrap_or("".to_string()));
    println!("DICEROLL [DICEROLL [...]]");
    println!();
    println!("You can specify any number space-separated dicerolls. Each ");
    println!("one can be comprised of multiple dice using the conventional ");
    println!("D&D syntax. Examples:");
    println!();
    println!("  3d20: the sum of 3 d20 rolls");
    println!("  1d20+1d4: the sum of a d20 and a d4");
    println!("  2d8-1: the sum of 2 d8 rolls with a -1 modifier")
}

fn main() {
    if env::args().skip(1).any(|x| x == "-h") {
        print_help();
        return;
    }

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
