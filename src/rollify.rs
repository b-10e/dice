use std::future;

use crate::tokenize::Token;

pub struct Roll {
    pub dice_count: i32,
    pub face_count: i32,
    pub modifier: i32,
    pub filter: Token,
    pub filter_amount: i32
}

// turn a list of tokens into a list of rolls
pub fn to_rolls(tokens: &Vec<Token>) -> Result<Vec<Roll>, String> 
{
    for i in 0..tokens.len()
    {
        
    }

    Ok(vec![])
}