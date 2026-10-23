use std::future;

use crate::tokenize::Token::{self, Number};

pub struct Roll {
    pub dice_count: i32,
    pub face_count: i32,
    pub modifier: i32,
    pub filter: Option<Token>,
    pub filter_amount: i32
}

// turn a list of tokens into a list of rolls
pub fn to_rolls(tokens: &Vec<Token>) -> Result<Vec<Roll>, String> 
{
// collect indicies of the start of every roll
    let roll_indicies = tokens
    .iter()
    .enumerate()
    .filter(|(_,t)| t == &&Token::Dice)
    .map(
        |(index, _)| 
        {
            if index > 0 {
                if let Token::Number(_) = tokens[index-1] {
                    return index - 1
                }
            }
            return index
        }
    )
    .collect::<Vec<usize>>();

// create rolls
    let mut rolls: Vec<Roll> = Vec::with_capacity(roll_indicies.len());

    for i in 0..roll_indicies.len()
    {
    // determine this roll's token bounds
        let start = roll_indicies[i]; {
            
        }
        let end = {
            if i == roll_indicies.len() - 1 {
                roll_indicies.len()
            } else {
                roll_indicies[i+1]
            }
        };
        let mut roll_tokens = tokens[start..end].to_owned();
        if roll_tokens.last().expect("roll_tokens has no tokens") == &Token::Addition {
            roll_tokens.pop();
        }
        roll_tokens.push(Token::End);

        let mut roll: Roll = Roll {
            dice_count: 1,
            face_count: 6,
            modifier: 0,
            filter: None,
            filter_amount: 1
        };
    // create roll from roll_tokens

        // check for dice count, if given
        if let Number(num) = roll_tokens[0] {
            roll.dice_count = num;
            roll_tokens.remove(0);
        }

        // remove the Dice token
        if roll_tokens[0] == Token::Dice {
            roll_tokens.remove(0);
        } else {
            return Err("Could not find Dice Token".to_string())
        }

        // get face count
        if let Number(num) = roll_tokens[0] {
            roll.face_count = num;
            roll_tokens.remove(0);
        } else {
            return Err("Could not find face count".to_string())
        }
        
    }

    Ok(vec![])
}