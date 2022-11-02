use rand::prelude::*;

/// The sign of a die roll. We need this because a die might actually be
/// subtractive. For example a College of Lord Bard can use Cutting
/// Words to subtract an inspiration die from an enemy roll. You end up
/// with something like 1d20 - 1d6. Having this enum allows us to pack
/// the sign in [SingleDieRoll] and eliminate the need for tracking
/// additions/subtractions outside of the struct.
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

    /// Convert the sign to a multiplier, `1` or `-1` to simplify
    /// aggregating [SingleDieRoll]s.
    pub fn to_multiplier(self) -> i32 {
        match self {
            Sign::Plus => 1,
            Sign::Minus => -1,
        }
    }
}

/// A single die roll. Used to break a [CompositeRoll] to its elements.
/// For example each part of the right hand side below is a separate
/// [SingleDieRoll] object.
///  - 1d20 + 2d6 -> +1d20, +1d6, +1d6
///
/// The object holds both the type of die, in its equivalent
/// represantation of [SingleDieRoll::max] (1d20 <-> max = 20), and the
/// result if it has been rolled [SingleDieRoll::roll].
// TODO: look into decoupling the max from the roll. Having them in the
// same struct is a bit convenient and it allows the reuse of memory,
// but it also doesn't prevent us from making mistakes like forgetting
// to roll the dice, or rolling them more than once. I think the clean
// Rust way would be to consume Die objects and produce RollResult or
// something.
pub struct SingleDieRoll {
    pub max: i32,
    pub roll: i32,
    pub sign: Sign,
}

/// A sum of arbitrary amount of [SingleDieRoll] and an integer
/// modifier. So something like "1d20 + 1d6 - 3" is represented by a
/// single [CompositeRoll] containing two [SingleDieRoll]s (+1d20, +1d6)
/// and a modifier (-3).
pub struct CompositeRoll {
    pub dice: Vec<SingleDieRoll>,
    pub modifier: i32,
}

/// Return the result of a single die roll. This is a tiny wrapper
/// around an RNG to shift the results to the [1, max] range.
fn roll_single_die(max: i32) -> i32 {
    // gen_range takes [min, max) but dice are usually notated as 1dX <-> (0, X]
    // so increment the max
    thread_rng().gen_range(1, max + 1)
}

/// Convenience wrapper to roll every die inside a [CompositeRoll].
pub fn roll_dice(dice_roll: &mut CompositeRoll) {
    for die in &mut dice_roll.dice {
        die.roll = roll_single_die(die.max);
    }
}

/// Attempt to parse a string and return an equivalent [CompositeRoll].
/// The roll string must be a composite of any number of dice (eg +1d20,
/// -2d4), and optionally modifiers (+2, -1). The function ignores all
/// whitespace from the string.
pub fn parse_roll_string(
    mut roll_string: String,
) -> Result<CompositeRoll, String> {
    // Expect the notation "1d20 + 2d8 + 3, ..."
    remove_whitespace(&mut roll_string);

    // To make parsing more uniform, add a '+' to the beginning of the string if
    // no sign exists there.
    if !roll_string.starts_with(&['+', '-'][..]) {
        roll_string.insert(0, '+');
    }

    // Do the parsing in two steps, first split on '+'/'-' (keeping the sign),
    // then parse the substrings.
    let roll_substrings = {
        let mut roll_substrings = Vec::new();
        while let Some(next_sign_pos) = roll_string.rfind(&['+', '-'][..]) {
            roll_substrings.push(roll_string.split_off(next_sign_pos));
        }

        roll_substrings.reverse();
        roll_substrings
    };

    let mut die_roll = CompositeRoll {
        dice: Vec::new(),
        modifier: 0,
    };
    for substring in roll_substrings {
        // Rolls are '{x}d{y}', modifiers are just '{x}'

        match substring.contains('d') {
            true => {
                // This one uses unwrap because it shouldn't be possible to end
                // up here with something that doesn't start with a valid sign.
                // Therefore, if this fails, the program is invalid and there is
                // little use for a pretty error message.
                let sign =
                    Sign::from_char(substring.as_bytes()[0] as char).unwrap();
                let substring = &substring[1..];
                let elements: Vec<&str> = substring.split('d').collect();
                assert_eq!(elements.len(), 2);
                let amount = match elements[0].parse::<i32>() {
                    Ok(x) => x,
                    Err(err_str) => return Err(err_str.to_string()),
                };

                let max = match elements[1].parse::<i32>() {
                    Ok(x) => x,
                    Err(err_str) => return Err(err_str.to_string()),
                };

                for _i in 0..amount {
                    die_roll.dice.push(SingleDieRoll { max, roll: 0, sign });
                }
            }
            false => match substring.parse::<i32>() {
                Ok(x) => die_roll.modifier += x,
                Err(err_str) => return Err(err_str.to_string()),
            },
        }
    }

    Ok(die_roll)
}

/// Remove all whitespace from a string.
fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}
