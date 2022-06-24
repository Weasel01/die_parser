//! # 🎲 die_parser
//!
//! This crate parses the notation for die rolls as used in tabletop games like D&D.
//!
//! It aims to do so in the most *simple*, *easy* and *lightweight* way possible.
//!
//!     Input:
//!     1.) "2d6"         (Roll 2 six-sided dice.)
//!     2.) "4d20 - 5"    (Roll 4 twenty-sided dice and subtract 5 from the result.)
//!
//!     Output:
//!     1.)    Roll {
//!             number_of_sides: 6
//!             number_of_dice: 2
//!             modifier: 0
//!            }
//!     2.)    Roll {
//!             number_of_sides: 20
//!             number_of_dice: 4
//!             modifier: -5
//!            }
//!
//! ## ❓ Getting started:
//! **Try [Roll::parse_roll()]!**

use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, map_res};
use nom::sequence::separated_pair;
use nom::{branch, IResult};
use std::fmt::{Display, Formatter};

use std::str::FromStr;

/// Holds information about a die roll.
#[derive(Debug, PartialEq)]
pub struct Roll {
    /// The type of die.
    pub number_of_sides: u16,
    /// How many dice are to be rolled.
    pub number_of_dice: u16,
    /// A modifier to be added to the result of the die rolls.
    pub modifier: i32,
}
impl Roll {
    /// A convenience function that allows you to manually create a new [Roll].
    pub fn new(number_of_sides: u16, number_of_dice: u16, modifier: i32) -> Self {
        Self {
            number_of_sides,
            number_of_dice,
            modifier,
        }
    }

    /// Parses a given input string with no regard to validity.
    fn parse_modified_roll(input: &str) -> Result<Roll, RollError> {
        // Remove whitespaces.
        let whitespaceless = input.replace(" ", "");

        // Parse type of die and amount of dice.
        let (remainder, (number_of_dice, number_of_sides)) =
            match parse_simple_roll(&whitespaceless) {
                Ok(v) => v,
                Err(_) => return Err(RollError::ParsingError),
            };

        // Parse the modifier
        let (_, modifier) = match parse_modifier(remainder) {
            Ok(v) => v,
            Err(_) => return Err(RollError::ParsingError),
        };

        // Success!
        Ok(Roll {
            number_of_dice,
            number_of_sides,
            modifier,
        })
    }

    /// Checks if a give roll is using a valid type of die and amount of dice.
    fn check_roll_validity(&self, max_dice: u16) -> Result<(), RollError> {
        // Check for die type.
        match self.number_of_sides {
            2 => (),
            4 => (),
            6 => (),
            8 => (),
            10 => (),
            12 => (),
            20 => (),
            100 => (),
            _ => return Err(RollError::DieTypeInvalid),
        }

        // Check for amount of dice. If max_dice == 0 ~> no limit.
        if self.number_of_dice > max_dice && !max_dice != 0 {
            return Err(RollError::DiceExceedLimit);
        } else if self.number_of_dice <= 0 {
            return Err(RollError::NoDiceToRoll);
        }

        // Checks passed.
        Ok(())
    }

    /// **Tries to parse input as roll notation (e.g. `4d20 + 5`).**
    ///
    /// * Whitespaces are ignored.
    /// * Checks for validity of roll.[^1]
    ///     * Enforces a limit of 100 dice per roll.[^2]
    ///
    /// # Examples
    /// ```
    /// use die_parser::Roll;
    /// use die_parser::RollError;
    ///
    /// let roll = Roll::parse_roll("3d10 - 5");
    /// assert_eq!(roll, Ok(Roll::new(10, 3, -5)));
    ///
    /// let invalid_roll = Roll::parse_roll("101d20");
    /// assert_eq!(invalid_roll, Err(RollError::DiceExceedLimit));
    /// ```
    /// [^1]: Valid die types are: d2, d4, d6, d8, d10, d12, d20, d100
    ///
    /// [^2]: If you wish to allow more (or only allow less) than 100 dice per roll, use [`Roll::parse_roll_with_limit()`] instead.
    pub fn parse_roll(input: &str) -> Result<Roll, RollError> {
        let result = Roll::parse_modified_roll(input)?;

        return match result.check_roll_validity(100) {
            Ok(()) => Ok(result),
            Err(e) => Err(e),
        };
    }

    /// **Tries to parse input as roll notation (e.g. `4d20 + 5`).**
    ///
    /// * Whitespaces are ignored.
    /// * Checks for validity of roll.[^1]
    ///     * Enforces a custom limit of how many dice are allowed per roll `(0 = no limit)`.
    ///
    /// # Examples
    /// ```
    /// use die_parser::Roll;
    /// use die_parser::RollError;
    ///
    /// let roll = Roll::parse_roll_with_limit("3d10 - 5", 1000);
    /// assert_eq!(roll, Ok(Roll::new(10, 3, -5)));
    ///
    /// let invalid_roll = Roll::parse_roll_with_limit("15d20", 10);
    /// assert_eq!(invalid_roll, Err(RollError::DiceExceedLimit));
    /// ```
    ///
    /// [^1]: Valid die types are: d2, d4, d6, d8, d10, d12, d20, d100
    pub fn parse_roll_with_limit(input: &str, max_dice: u16) -> Result<Roll, RollError> {
        let result = Roll::parse_modified_roll(input)?;

        // Check if the roll is valid using the users max_dice value.
        return match result.check_roll_validity(max_dice) {
            Ok(()) => Ok(result),
            Err(e) => Err(e),
        };
    }
}

/// The different types of errors that may occur trying to construct a [Roll] from a given input string.
#[derive(Debug, PartialEq)]
pub enum RollError {
    /// Signifies that the inputted die type did not match any of the valid types.
    ///
    /// Valid die types are: `d2`, `d4`, `d6`, `d8`, `d10`, `d12`, `d20`, `d100`
    /// # Example
    /// ```
    /// use die_parser::{Roll, RollError};
    ///
    /// let invalid_roll = Roll::parse_roll("1d50");
    /// assert_eq!(invalid_roll, RollError::DieTypeInvalid);
    /// ```
    DieTypeInvalid,
    /// Signifies that the requested amount of dice exceeded the set limit.
    /// # Example
    /// ```
    /// use die_parser::{Roll, RollError};
    ///
    /// let invalid_roll = Roll::parse_roll("9001d20");
    /// assert_eq!(invalid_roll, RollError::DiceExceedLimit);
    /// ```
    DiceExceedLimit,
    /// Signifies that the requested amount of dice was less than 1.
    /// # Example
    /// ```
    /// use die_parser::{Roll, RollError};
    ///
    /// let invalid_roll = Roll::parse_roll("0d20");
    /// assert_eq!(invalid_roll, RollError::NoDiceToRoll);
    /// ```
    NoDiceToRoll,
    /// Signifies that the input string was malformed.
    /// # Example
    /// ```
    /// use die_parser::{Roll, RollError};
    ///
    /// let invalid_roll = Roll::parse_roll("4d2invalid_characters+5");
    /// assert_eq!(invalid_roll, RollError::ParsingError);
    ///
    /// ```
    ParsingError,
}
impl Display for RollError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DieTypeInvalid => write!(f, "The requested type of die is invalid."),
            Self::DiceExceedLimit => write!(f, "Amount of dice exceeds the specified limit."),
            Self::NoDiceToRoll => write!(f, "Can't roll less than 1 die."),
            Self::ParsingError => write!(f, "Failed to parse the input string."),
        }
    }
}
impl std::error::Error for RollError {}

/// Parse a `u16` from the start of the input string.
fn parse_numbers(input: &str) -> IResult<&str, u16> {
    map_res(digit1, u16::from_str)(input)
}

/// Tries to parse die type and amount of dice from a notated die roll (e.g. `4d20`).
fn parse_simple_roll(s: &str) -> IResult<&str, (u16, u16)> {
    let parser = separated_pair(parse_numbers, char('d'), parse_numbers);
    map(parser, |(number_of_dice, number_of_sides)| {
        (number_of_dice, number_of_sides)
    })(s)
}

/// Looks for modifiers operator.
fn parse_operator(s: &str) -> IResult<&str, &str> {
    branch::alt((tag("+"), tag("-"), tag("")))(s)
}

/// Tries to parse the modifier part of a notated die roll (e.g. `+5`).
fn parse_modifier(s: &str) -> IResult<&str, i32> {
    // Split operator and modifier.
    let (remainder, operator) = parse_operator(s).unwrap();

    // Generate i32.
    match operator {
        "+" => map(parse_numbers, |modifier| modifier as i32)(remainder),
        "-" => map(parse_numbers, |modifier| modifier as i32 * -1)(remainder),
        // Return 0 as modifier if no operator signalling a modifier was found.
        _ => Ok((remainder, 0)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_simple_roll() {
        let tests = [
            ("4d20", (4, 20), ""),
            ("4d20remainder_text", (4, 20), "remainder_text"),
        ];

        for (input, expected_output, expected_remaining_input) in tests {
            let (remaining_input, output) = parse_simple_roll(input).unwrap();
            assert_eq!(remaining_input, expected_remaining_input);
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_parse_modifier() {
        let tests_positive = [
            ("+5", 5, ""),
            ("+5remainder_text", 5, "remainder_text"),
            ("", 0, ""),
            ("random_unparsable", 0, "random_unparsable"),
        ];
        let tests_negative = [
            ("-5", -5, ""),
            ("-5remainder_text", -5, "remainder_text"),
            ("", 0, ""),
            ("random_unparsable", 0, "random_unparsable"),
        ];
        for (input, expected_output, expected_remaining_input) in tests_positive {
            let (remaining_input, output) = parse_modifier(input).unwrap();
            assert_eq!(remaining_input, expected_remaining_input);
            assert_eq!(output, expected_output);
        }
        for (input, expected_output, expected_remaining_input) in tests_negative {
            let (remaining_input, output) = parse_modifier(input).unwrap();
            assert_eq!(remaining_input, expected_remaining_input);
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_parse_modified_roll() {
        let tests = [
            ("4d10+3", Roll::new(10, 4, 3)),
            ("4d10-3", Roll::new(10, 4, -3)),
            ("4 d 10  + 3", Roll::new(10, 4, 3)),
            ("4 d 10  - 3", Roll::new(10, 4, -3)),
            ("4d10+3 random_stuff", Roll::new(10, 4, 3)),
            ("4d10-3 random_stuff", Roll::new(10, 4, -3)),
        ];

        for (input, expected_output) in tests {
            let output = Roll::parse_modified_roll(input).unwrap();
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_err_modified_roll() {
        let tests = [
            ("4d10+unparsable_modifier", RollError::ParsingError),
            ("4d10-unparsable_modifier", RollError::ParsingError),
            ("4d10  + unparsable_modifier", RollError::ParsingError),
            ("4d10  - unparsable_modifier", RollError::ParsingError),
            ("4dinvalid_die_type", RollError::ParsingError),
            ("invalid_die_amountd20", RollError::ParsingError),
        ];

        for (input, expected_output) in tests {
            let output = Roll::parse_modified_roll(input).unwrap_err();
            assert_eq!(output, expected_output);
        }
    }

    #[test]
    fn test_err_parse_roll() {
        let tests = [
            ("4d5", RollError::DieTypeInvalid),
            ("0d20", RollError::NoDiceToRoll),
            ("0d5", RollError::DieTypeInvalid),
            ("9001d20", RollError::DiceExceedLimit),
        ];

        for (input, expected_output) in tests {
            let output = Roll::parse_roll(input).unwrap_err();
            assert_eq!(output, expected_output);
        }
    }
}
