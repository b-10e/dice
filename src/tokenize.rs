#[derive(Debug)]
pub enum Token {
    Number(i32),
    Dice,
    Addition,
    Subtraction,
    DropHighest,
    DropLowest,
    KeepHighest,
    KeepLowest,
}

// convert a string to a list of tokens
pub fn to_tokens(input: &str) -> Result<Vec<Token>, String> 
{
    let mut tokens: Vec<Token> = Vec::new();

    // create mutable list of input chars
    let mut input_chars: Vec<char> = input.chars().collect();

    // add end indicator
    input_chars.push(' ');

    // loop through chars and convert to tokens
    let mut temp_token_chars: Vec<char> = Vec::new();

    for i in 0..input_chars.len() {
        // store current character
        let current_char: char = input_chars[i];

        // hit end of input, cleanup and finish
        if current_char == ' ' {
            // check if just escaped number, push stored number to token list if so
            if temp_token_chars.len() > 0 && temp_token_chars.last().expect("No temp_toke_chars found").is_ascii_digit()
            {
                if let Ok(num) = temp_token_chars.iter().collect::<String>().parse::<i32>()
                {
                    tokens.push(Token::Number(num));
                    temp_token_chars.clear();
                } 
                else 
                {
                    return Err(format!("Failed to parse \"{}\" as a number",temp_token_chars.iter().collect::<String>()));
                }
            }
            break;
        }

        // store next char
        let next_char: char = input_chars[i + 1];

        // handle numbers
        if current_char.is_ascii_digit()
        {
            temp_token_chars.push(current_char);
        } 
        // check if just escaped number, push stored number to token list if so
        else if temp_token_chars.len() > 0 && temp_token_chars.last().expect("No temp_toke_chars found").is_ascii_digit()
        {
            if let Ok(num) = temp_token_chars.iter().collect::<String>().parse::<i32>()
            {
                tokens.push(Token::Number(num));
                temp_token_chars.clear();
            } else {
                return Err(format!("Failed to parse \"{}\" as a number",temp_token_chars.iter().collect::<String>()));
            }
        }
        
        // handle 'd'/"dh"/"dl" (dice, drop highest, or drop lowest)
        if current_char == 'd'
        {
            match next_char {
                'k' => return Err(format!("Invalid \'{next_char}\' found after \'d\'")),
                'd' => return Err(format!("Invalid \'{next_char}\' found after \'d\'")),
                '+' => return Err(format!("Invalid \'{next_char}\' found after \'d\'")),
                '-' => return Err(format!("Invalid \'{next_char}\' found after \'d\'")),
                ' ' => return Err(format!("End of line found after \'d\'")),
                'h' => tokens.push(Token::DropHighest),
                'l' => tokens.push(Token::DropLowest),
                _ => tokens.push(Token::Dice),
            }
            continue;
        }
        if current_char == 'l' || current_char == 'h' {continue;}
        if current_char == 'k'
        {
            match next_char {
                'h' => tokens.push(Token::KeepHighest),
                'l' => tokens.push(Token::KeepLowest),
                ' ' => return Err(format!("End of line found after \'k\'")),
                _ => return Err(format!("Invalid \'{next_char}\' found after \'k\'")),
            }
        }

        // handle operators
        if current_char == '+'
        {
            tokens.push(Token::Addition);
            continue;
        }
        if current_char == '-'
        {
            tokens.push(Token::Subtraction);
            continue;           
        }
    }
    
    Ok(tokens)
}