use colored::Colorize;
use rand::prelude::*;

use crate::{rollify::Roll, tokenize::Token};

const DICE_PRINT_LIMIT: usize = 24;

pub struct Die {
    value: i32,
    enabled: bool,
}

impl Die {
    pub fn new(value: i32, enabled: bool) -> Die {
        Die { value, enabled }
    }
}

pub fn display_rolls(rolls: &Vec<Roll>) {
    // initialize rng thread
    let mut rng = rand::rng();
    let mut total_sum: i32 = 0;

    // calculate longest notation character length for formatting
    let mut longest_notation_char_count = 0;
    for roll in rolls {
        let notation_len = notation_len(roll);
        if notation_len > longest_notation_char_count {
            longest_notation_char_count = notation_len;
        }
    }

    // loop through rolls and print
    for roll in rolls {
        let mut dice_results: Vec<Die> = Vec::with_capacity(roll.dice_count as usize);
        let mut roll_sum: i32 = 0;

        // loop each die and store result
        for _ in 0..roll.dice_count {
            let random_number = rng.random_range(1..=roll.face_count);
            dice_results.push(Die::new(random_number, true));
        }

        // filter dice, if needed
        dice_results.sort_by(|a, b| a.value.cmp(&b.value));
        let mut changes_remaining: i32 = roll.filter_amount;

        match roll.filter {
            Some(Token::KeepHighest) => {
                // set all elements to drop
                for result in dice_results.iter_mut() {
                    result.enabled = false;
                }
                // set x highest elements to drop
                for result in dice_results.iter_mut().rev() {
                    result.enabled = true;
                    changes_remaining -= 1;
                    if changes_remaining <= 0 {
                        break;
                    }
                }
            }
            Some(Token::KeepLowest) => {
                // set all elements to drop
                for result in dice_results.iter_mut() {
                    result.enabled = false;
                }
                // set x lowest elements to keep
                for result in dice_results.iter_mut() {
                    result.enabled = true;
                    changes_remaining -= 1;
                    if changes_remaining <= 0 {
                        break;
                    }
                }
            }
            Some(Token::DropHighest) => {
                // set x lowest elements to keep
                for result in dice_results.iter_mut().rev() {
                    result.enabled = false;
                    changes_remaining -= 1;
                    if changes_remaining <= 0 {
                        break;
                    }
                }
            }
            Some(Token::DropLowest) => {
                // set x lowest elements to drop
                for result in dice_results.iter_mut() {
                    result.enabled = false;
                    changes_remaining -= 1;
                    if changes_remaining <= 0 {
                        break;
                    }
                }
            }
            _ => (),
        }

        // print notation if multiple roll sets
        if rolls.len() > 1 {
            print!(
                "{}",
                format!("{}d{}: ", roll.dice_count, roll.face_count).bright_black()
            );

            // add extra spacing if necessary to line up all notationes
            let spacing_difference = (longest_notation_char_count + 2) - (notation_len(&roll) + 2);
            for _ in 0..spacing_difference {
                print!(" ");
            }
        }

        // print dice
        for i in 0..dice_results.len() {
            // add enabled die to sum
            if dice_results[i].enabled {
                roll_sum += dice_results[i].value;
            }

            // prevent printing too many dice
            if i >= DICE_PRINT_LIMIT {
                if i == DICE_PRINT_LIMIT {
                    print!("{}", "... ".bright_black());
                }
                continue;
            }

            if dice_results[i].enabled {
                print!("{}", format!("[{}] ", dice_results[i].value));
            } else {
                print!("{}", format!("[{}] ", dice_results[i].value).bright_black());
            }
        }

        // print modifier
        roll_sum += roll.modifier;
        if roll.modifier >= 1 {
            print!("{}", format!("+{} ", roll.modifier).green());
        } else if roll.modifier <= -1 {
            print!("{}", format!("{} ", roll.modifier).red());
        }

        // print sum if multiple dice, multiple rolls, and/or a modifier
        if rolls.len() > 1 || roll.dice_count > 1 || roll.modifier != 0 {
            println!("= {}", format!("{roll_sum}").blue());
        }

        total_sum += roll_sum;
    }

    // print total sum if multiple roll sets
    if rolls.len() > 1 {
        // print spacer bar
        for _ in 0..longest_notation_char_count + 2 {
            print!("{}", "-".bright_black());
        }
        println!();
        println!("= {}", format!("{}", total_sum).blue());
    }

    // fancy spacing
    println!();
    if rolls.len() == 1 && rolls[0].dice_count == 1 && rolls[0].modifier == 0 {
        println!();
    }
}

fn notation_len(roll: &Roll) -> usize {
    let mut len = 0;

    // add number of digits in dice count
    if roll.dice_count > 1 {
        len += format!("{}", roll.dice_count).as_bytes().len();
    }

    len += 1; // add one for 'd'

    // add number of digits in face count
    len += format!("{}", roll.face_count).as_bytes().len();

    len
}
