use super::Token;
use super::Symbol;

use std::fmt;

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Identifier(str) => write!(f, "{}", str),
            Token::String(str) => write!(f, "\"{}\"", str),
            Token::Integer(num) => write!(f, "{}", num),
            Token::Decimal(num) => write!(f, "{}", num),
            Token::Punctuation(symbol) => write!(f, "{:?}", symbol),
        }
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let char= match self {
            Symbol::OpenCurlyBrace => '{',
            Symbol::CloseCurlyBrace => '}',
            Symbol::OpenBrace => '[',
            Symbol::CloseBrace => ']',
            Symbol::Comma => ',',
            Symbol::Colon => ':',
        };

        write!(f, "{}", char)
    }
}

fn print_newline(tabs: i32) {
    println!();
    for _ in 0..tabs {
        print!("  ");
    }
}

pub fn print_tokens(tokens: &Vec<Token>, error_index: usize) {
    let mut tabs= 0;

    println!("Parsing of schema failed:");
    for index in 0..tokens.len() {
        let token= &tokens[index];
        match token {
            Token::Identifier(_) => {
                print_newline(tabs);
            },
            Token::Punctuation(symbol) => {
                match symbol {
                    Symbol::CloseCurlyBrace => {
                        tabs-= 1;
                        print_newline(tabs);
                    },
                    Symbol::OpenCurlyBrace => {
                        tabs+= 1;
                    }
                    _ => (),
                }
            },
            _ => (),
        }

        if index == error_index {
            print!(" ERROR<< {:?} >> ", token);
        } else {
            print!(" {} ", token);
        }

        match token {
            Token::Punctuation(symbol) => {
                match symbol {
                    Symbol::CloseCurlyBrace => println!(),
                    _ => (),
                }
            },
            _ => (),
        }
    }
}
