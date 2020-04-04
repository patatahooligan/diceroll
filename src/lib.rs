use rand::prelude::*;

pub struct SingleDieRoll {
    pub max: u32,
    pub roll: u32,
}

pub struct DiceRoll {
    pub dice: Vec<SingleDieRoll>,
    pub modifier: u32,
}

fn roll_single_die(max: u32) -> u32 {
    // gen_range takes [min, max) but dice are usually notated as 1dX <-> (0, X]
    // so increment the max
    thread_rng().gen_range(1, max + 1)
}

pub fn roll_dice(dice_roll: &mut DiceRoll) {
    for die in &mut dice_roll.dice {
        die.roll = roll_single_die(die.max);
    }
}

pub fn parse_roll_string(mut roll_string: String) -> DiceRoll {
    // Expect the notation "1d20 + 2d8 + 3, ..."
    remove_whitespace(&mut roll_string);

    // Here single_roll means roll in the d&d so it might contain more than one
    // die and modifiers
    let mut die_roll = DiceRoll {dice: Vec::new(), modifier: 0};
    for part in roll_string.split('+') {
        // Rolls are '{x}d{y}', modifiers are just '{x}'
        match part.contains('d') {
            true => {
                let elements: Vec<&str> = part.split('d').collect();
                assert_eq!(elements.len(), 2);
                let amount = elements[0].parse::<u32>().unwrap();
                let max = elements[1].parse::<u32>().unwrap();

                for _i in 0..amount {
                    die_roll.dice.push(SingleDieRoll{max, roll: 0});
                }
            }
            false => {
                die_roll.modifier += part.parse::<u32>().unwrap();
            }
        }
    }

    die_roll
}

// Helper function
fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}
