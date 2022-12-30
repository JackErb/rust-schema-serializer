use std::iter;
use std::str;

#[derive(Debug)]
pub enum Symbol {
    OpenCurlyBrace,   // {
    CloseCurlyBrace,  // }
    OpenBrace,        // [
    CloseBrace,       // ]
    Comma,            // ,
    Colon,            // :
}

#[derive(Debug)]
pub enum Token {
    Identifier(String), // A literal identifier (e.g. field name or an enum variant)
    Integer(i64),
    Decimal(f64),
    String(String),
    Punctuation(Symbol),
}

impl Symbol {
    pub fn from_char(ch: char) -> Option<Symbol> {
        match ch {
            '{' => Some(Symbol::OpenCurlyBrace),
            '}' => Some(Symbol::CloseCurlyBrace),
            '[' => Some(Symbol::OpenBrace),
            ']' => Some(Symbol::CloseBrace),
            ',' => Some(Symbol::Comma),
            ':' => Some(Symbol::Colon),
            _ =>   None
        }
    }
}

fn parse_number(chars: &mut iter::Peekable<str::Chars>) -> Result<Token, &'static str> {
    let mut accumulator= String::new();
    let mut is_decimal= false;

    while let Some(next_char)= chars.next() {
        println!("Next char: {}", next_char);
        match next_char {
            '0'..='9' => accumulator.push(next_char),
            '.' => {
                accumulator.push(next_char);
                if is_decimal {
                    return Err("Hit a second period while parsing decimal number.");
                }
                is_decimal= true;
            }
            _ => {
                // Hit a non number character. Stop parsing this number
                break;
            }
        }
    }

    if is_decimal {
        let number= accumulator.parse::<f64>();
        match number {
            Ok(number) => return Ok(Token::Decimal(number)),
            Err(_) => return Err("Failed to parse decimal number")
        }
    } else {
        let number= accumulator.parse::<i64>();
        match number {
            Ok(number) => return Ok(Token::Integer(number)),
            Err(_) => return Err("Failed to parse integer")
        }
    }
}

fn parse_identifier(chars: &mut iter::Peekable<str::Chars>) -> Token {
    let mut accumulator= String::new();

    while let Some(next_char)= chars.next() {
        println!("Next char: {}", next_char);
        match next_char {
            'a'..='z' | 'A'..='Z' | '_' => accumulator.push(next_char),
            _ => {
                // Hit a non alphabetic character, stop parsing this identifier
                break;
            }
        }
    }
    Token::Identifier(String::from(accumulator))
}

pub fn string_to_tokens(contents: &str) -> Result<Vec<Token>, &'static str> {
    let mut chars= contents.chars().peekable();
    let mut tokens= Vec::new();

    while let Some(next_char)= chars.peek() {
        println!("Next char: {}", next_char);
        // Based on the next character, match the next token, ignoring any whitespace.
        let next_token= match next_char {
            '0'..='9' => {
                let mut chars_clone= chars.clone();
                let token= parse_number(&mut chars_clone)?;
                chars.clone_from(&chars_clone);

                Some(token)
            },
            'a'..='z' | 'A'..='Z' => {
                let mut chars_clone= chars.clone();
                let token= parse_identifier(&mut chars_clone);
                chars.clone_from(&chars_clone);

                Some(token)
            },
            _ => {
                let token= if let Some(symbol)= Symbol::from_char(*next_char) {
                    Some(Token::Punctuation(symbol))
                } else if next_char.is_whitespace() {
                    // Ignore whitespace
                    None
                } else {
                    // Hit an unexpected symbol. Fail the parsing.
                    return Err("Hit unexpected character while parsing block definition");
                };

                chars.next();
                token
            }
        };

        match next_token {
            Some(token) => tokens.push(token),
            None => ()
        }
    }

    println!("{:?}", tokens);

    Ok(tokens)
}
