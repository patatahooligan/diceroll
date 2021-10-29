use rand::prelude::*;

#[derive(Copy, Clone)]
pub enum Sign {
    Plus,
    Minus,
}

impl Sign {
    fn from_char(c: char) -> Result<Sign, &'static str> {
        match c {
            '+' => Ok(Sign::Plus),
            '-' => Ok(Sign::Minus),
            _ => Err("Sign character should be '+' or '-'"),
        }
    }

    pub fn to_char(self) -> char {
        match self {
            Sign::Plus => '+',
            Sign::Minus => '-',
        }
    }

    pub fn to_multiplier(self) -> i32 {
        match self {
            Sign::Plus => 1,
            Sign::Minus => -1,
        }
    }
}

pub struct SingleDieRoll {
    pub max: i32,
    pub roll: i32,
    pub sign: Sign,
}

pub struct DiceRoll {
    pub dice: Vec<SingleDieRoll>,
    pub modifier: i32,
}

fn roll_single_die(max: i32) -> i32 {
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

    // To make parsing more uniform, add a '+' to the beginning of the string if
    // no sign exists there.
    if !roll_string.starts_with(&['+', '-'][..]) {
        roll_string.insert(0, '+');
    }

    // Do the parsing in two steps, first split on '+'/'-' (keeping the sign),
    // then parse the substrings.
    let mut roll_substrings = Vec::new();
    while let Some(next_sign_pos) = roll_string.rfind(&['+', '-'][..]) {
        roll_substrings.push(roll_string.split_off(next_sign_pos));
    }

    roll_substrings.reverse();

    let mut die_roll = DiceRoll {
        dice: Vec::new(),
        modifier: 0,
    };
    for substring in roll_substrings {
        // Rolls are '{x}d{y}', modifiers are just '{x}'

        match substring.contains('d') {
            true => {
                let sign =
                    Sign::from_char(substring.as_bytes()[0] as char).unwrap();
                let substring = &substring[1..];
                let elements: Vec<&str> = substring.split('d').collect();
                assert_eq!(elements.len(), 2);
                let amount = elements[0].parse::<i32>().unwrap();
                let max = elements[1].parse::<i32>().unwrap();

                for _i in 0..amount {
                    die_roll.dice.push(SingleDieRoll { max, roll: 0, sign });
                }
            }
            false => {
                die_roll.modifier += substring.parse::<i32>().unwrap();
            }
        }
    }

    die_roll
}

// Helper function
fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}
