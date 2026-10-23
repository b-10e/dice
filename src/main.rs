use rand::prelude::*;
use colored::Colorize;

mod tokenize;
mod rollify;

use std::io::{self, Write};

use crate::{rollify::{Roll, to_rolls}, tokenize::Token};

/*
```
1d6     one six-sided die     1–6
3d6     three d6, summed     3–18
1d20+5     a d20 plus five     6–25
2d6-1     two d6, minus one     1–11
4d6kh3     four d6, keep the best three     3–18
4d6dl1     the same roll, written as a drop     3–18
2d20kh1     advantage     1–20
2d20kl1     disadvantage     1–20
1d8+1d6     two different dice, summed     2–14
```
*/

pub const VALID_CHARACTERS: [char; 16] = ['0','1','2','3','4','5','6','7','8','9','d','k','l','h','+','-'];

fn main() -> io::Result<()> {

    loop {

    // get user input
        let mut input: String = input("> ")?.trim().to_ascii_lowercase().split_whitespace().collect();

    // check for exit
        if input == "quit" || input == "exit" 
        {
            return Ok(());
        }

    // basic input validation
        if input.is_empty() {
            continue;
        }
        if !input.chars().all(|c| VALID_CHARACTERS.contains(&c)) {
            println!("{} Invalid characters detected", "!".yellow());
            continue;
        }
    
    // tokenize input
        let tokens: Vec<Token>;
        match tokenize::to_tokens(&input) 
        {
            Ok(t) => {
                println!("Tokens:\n    {:?}",t);
                tokens = t;
            },
            Err(e) => {
                println!("{} {e}","!".yellow());
                continue;
            },
        }

    // convert tokens to list of rolls
        let rolls: Vec<Roll>;
        match to_rolls(&tokens)
        {
            Ok(r) => {
                println!("got rolls");
                rolls = r;
            },
            Err(e) => {
                println!("{} {e}","!".yellow());
                continue;
            },
        }
    }
}

// read input into string and return
fn input(prompt: &str) -> io::Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    // print prompt
    print!("{prompt}");
    let _ = io::stdout().flush();

    // read line and return
    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}
