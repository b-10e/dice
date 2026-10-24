use std::io::{self, Write};

use rand::prelude::*;
use colored::Colorize;

use crate::{rollify::Roll, tokenize::Token};

pub fn display_rolls(rolls: &Vec<Roll>) 
{
    // initialize rng thread
    let mut rng = rand::rng();
    let mut total_sum: i32 = 0;

    // loop through rolls and print
    for roll in rolls 
    {
        let mut dice_results: Vec<(i32,bool)> = Vec::with_capacity(roll.dice_count as usize);
        let mut roll_sum: i32 = 0;

        // loop each die and store result
        for _ in 0..roll.dice_count 
        {
            let random_number = rng.random_range(1..=roll.face_count);
            dice_results.push(
                (
                    random_number,
                    true
                )
            )
        }

        // filter dice, if needed
        dice_results.sort_by(|a,b| a.0.cmp(&b.0));
        let mut changes_remaining: i32 = roll.filter_amount;

        match roll.filter {
            Some(Token::KeepHighest) => {
                // set all elements to drop
                for result in dice_results.iter_mut() {
                    result.1 = false;
                }
                // set x highest elements to drop
                for result in dice_results.iter_mut().rev() {
                    result.1 = true;
                    changes_remaining -= 1;
                    if changes_remaining <= 0 {
                        break;
                    }
                }
            }
            Some(Token::KeepLowest) => {
                // set all elements to drop
                for result in dice_results.iter_mut() {
                    result.1 = false;
                }
                // set x lowest elements to keep
                for result in dice_results.iter_mut() {
                    result.1 = true;
                    changes_remaining -= 1;
                    if changes_remaining <= 0 {
                        break;
                    }
                }
            }
            Some(Token::DropHighest) => {
                // set x lowest elements to keep
                for result in dice_results.iter_mut().rev() {
                    result.1 = false;
                    changes_remaining -= 1;
                    if changes_remaining <= 0 {
                        break;
                    }
                }
            }
            Some(Token::DropLowest) => {
                // set x lowest elements to drop
                for result in dice_results.iter_mut() {
                    result.1 = false;
                    changes_remaining -= 1;
                    if changes_remaining <= 0 {
                        break;
                    }
                }
            }
            _ => (),    
        }

        // reshuffle results
        dice_results.shuffle(&mut rng);

        // print face count if multiple roll sets
        if rolls.len() > 1 {
            print!("{}",format!("d{}: ",roll.face_count).bright_black());
        }

        // print dice
        for die in dice_results {
            if die.1 {
                print!("{}",format!("[{}] ",die.0));
                // add die to sum if not dropped
                roll_sum += die.0;
            } else {
                print!("{}",format!("[{}] ",die.0).bright_black());
            }

        }

        // print modifier
        roll_sum += roll.modifier;
        if roll.modifier >= 1 {
            print!("{}",format!("+{} ",roll.modifier).green());
        } else if roll.modifier <= -1 {
            print!("{}",format!("{} ",roll.modifier).red());
        }

        // print sum if multiple dice, multiple rolls, and/or a modifier
        if rolls.len() > 1 || roll.dice_count > 1 || roll.modifier != 0 {
            println!("= {}",format!("{}",roll_sum).blue());
        }

        total_sum += roll_sum;
    }

    // print total sum if multiple roll sets
    if rolls.len() > 1 {
        println!("{} {}",format!("Total:").bright_black(),format!("{}",total_sum).blue());
    }

    // fancy spacing
    println!();
    if rolls.len() == 1 && rolls[0].dice_count == 1 && rolls[0].modifier == 0 {
        println!();
    }
}