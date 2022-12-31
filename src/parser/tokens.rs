use std::iter;
use std::str;
use super::ParseResult;

#[derive(PartialEq)]
pub enum Symbol {
    OpenCurlyBrace,   // {
    CloseCurlyBrace,  // }
    OpenBrace,        // [
    CloseBrace,       // ]
    Comma,            // ,
    Colon,            // :
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

#[derive(PartialEq, Debug)]
pub enum Token {
    Identifier(String), // A literal identifier (e.g. field name or an enum variant)
    String(String),
    Integer(i64),
    Decimal(f64),
    Punctuation(Symbol),
}


fn parse_number(chars: &mut iter::Peekable<str::Chars>) -> ParseResult<Token> {
    let mut accumulator= String::new();
    let mut is_decimal= false;

    while let Some(next_char)= chars.peek() {
        match next_char {
            '0'..='9' => accumulator.push(chars.next().unwrap()),
            '.' => {
                accumulator.push(chars.next().unwrap());
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

fn parse_identifier(chars: &mut iter::Peekable<str::Chars>) -> ParseResult<Token> {
    let mut accumulator= String::new();

    while let Some(next_char)= chars.peek() {
        match next_char {
            'a'..='z' | 'A'..='Z' | '_' => accumulator.push(chars.next().unwrap()),
            _ => {
                // Hit a non alphabetic character, stop parsing this identifier
                break;
            }
        }
    }

    Ok(Token::Identifier(accumulator))
}

fn parse_string(chars: &mut iter::Peekable<str::Chars>) -> ParseResult<Token> {
    chars.next(); // consume the quotation mark that initiated this parse
    let mut accumulator= String::new();

    while let Some(next_char)= chars.next() {
        match next_char {
            '"' => return Ok(Token::String(accumulator)),
            _ => accumulator.push(next_char),
        }
    }

    Err("Hit end of file while reading string, expected closing quotation mark.")
}

pub fn string_to_tokens(contents: &str) -> ParseResult<Vec<Token>> {
    let mut chars= contents.chars().peekable();
    let mut tokens= Vec::new();

    while let Some(next_char)= chars.peek() {
        // Based on the next character, match the next token, ignoring any whitespace.
        let next_token= match next_char {
            '0'..='9' => Some(parse_number(&mut chars)?),
            'a'..='z' | 'A'..='Z' => Some(parse_identifier(&mut chars)?),
            '"' => Some(parse_string(&mut chars)?),
            _ => {
                let token= if let Some(symbol)= Symbol::from_char(*next_char) {
                    Some(Token::Punctuation(symbol))
                } else if next_char.is_whitespace() {
                    // Ignore whitespace
                    None
                } else {
                    // Hit an unexpected symbol. Fail the parsing.
                    println!("Found unexpected character {} while reading block definition", next_char);
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

    Ok(tokens)
}
