use crate::SchemaValue;
use crate::parser::tokens::{Token, Symbol};
use super::ParseResult;
use crate::parser::debug;

use std::collections;

macro_rules! consume_next_token {
    ($array: ident, $index: ident,  $expected_token: expr) => {
        if *$index >= $array.len() {
            return Err("Reached EOF while parsing object, expected another token.");
        }

        let token= &$array[*$index];
        if *token != $expected_token {
            println!("Consuming next token, expected: '{:?}', found: '{:?}'", $expected_token, token);
            return Err("Invalid token encountered.")
        }
        *$index+= 1;
    }
}

fn parse_value<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> ParseResult<SchemaValue<'a>> {
    if *index < tokens.len() {
        let token= &tokens[*index];
        *index+= 1;

        match token {
            Token::Integer(num) => Ok(SchemaValue::Integer(*num)),
            Token::Decimal(num) => Ok(SchemaValue::Decimal(*num)),
            Token::String(str) => Ok(SchemaValue::String(str)),
            Token::Identifier(ident) => {
                if ident == "true" {
                    Ok(SchemaValue::Bool(true))
                } else if ident == "false" {
                    Ok(SchemaValue::Bool(false))
                } else {
                    // Assume this is an enum variant... we could do better here.
                    // Maybe checking explicitly if this is a valid enum (requires type info)
                    parse_enum(&ident, tokens, index)
                }
            }
            Token::Punctuation(Symbol::OpenBrace) => {
                parse_array(tokens, index)
            }
            Token::Punctuation(Symbol::OpenCurlyBrace) => {
                parse_object(tokens, index)
            }
            _ => {
                debug::print_tokens(tokens, *index-1);
                println!("Found invalid token '{:?}' while parsing value.", token);
                Err("Invalid token found while parsing value.")
            }
        }
    } else {
        Err("Reached end of token stream while trying to parse value.")
    }
}

fn parse_array<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> ParseResult<SchemaValue<'a>> {
    let mut vector= Vec::new();

    // Special case: check for an empty array `[]`
    if *index < tokens.len() {
        // peek the next character
        match &tokens[*index] {
            Token::Punctuation(symbol) => match symbol {
                Symbol::CloseBrace => {
                    // consume the close brace, and return an empty vector
                    *index+= 1;
                    return Ok(SchemaValue::Array(vector));
                },
                _ => ()
            },
            _ => ()
        }
    }

    while *index < tokens.len() {
        // Read the value
        let schema_value= parse_value(tokens, index)?;
        vector.push(schema_value);

        let token= &tokens[*index];
        *index+= 1;
        match token {
            Token::Punctuation(Symbol::Comma) => (), // Read the next value...
            Token::Punctuation(Symbol::CloseBrace) => return Ok(SchemaValue::Array(vector)),
            _ => {
                debug::print_tokens(tokens, *index-1);
                println!("Found invalid token '{:?}' while parsing array.", token);
                return Err("Invalid token found while parsing array.")
            }
        }
    }

    Err("Reached EOF while parsing array.")
}

fn parse_object<'a>(tokens: &'a Vec<Token>, index: &mut usize) -> ParseResult<SchemaValue<'a>> {
    let mut fields_map= collections::HashMap::<&str, SchemaValue>::new();

    // Parse an object of format: { field_name: <value>, ...,  }
    // NOTE: due to current implementation, commas are completely optional
    while *index < tokens.len() {
        let token= &tokens[*index];
        *index+= 1;
        match token {
            Token::Identifier(ident) => {
                // Parsing a field of this struct
                consume_next_token!(tokens, index, Token::Punctuation(Symbol::Colon));
                let field_value= parse_value(tokens, index)?;
                fields_map.insert(&ident, field_value);
            }
            Token::Punctuation(Symbol::Comma) => (),
            Token::Punctuation(Symbol::CloseCurlyBrace) => {
                return Ok(SchemaValue::Object(fields_map))
            }
            _ => {
                debug::print_tokens(tokens, *index-1);
                println!("Found invalid token '{:?}' while parsing object.", token);
                return Err("Invalid token found while parsing object.")
            }
        }
    }

    Err("Reached EOF while parsing object.")
}

fn parse_enum<'a>(ident: &'a String, tokens: &'a Vec<Token>, index: &mut usize) -> ParseResult<SchemaValue<'a>> {
    let variant_field= if *index < tokens.len() {
        // peek the next character
        match &tokens[*index] {
            Token::Punctuation(symbol) => match symbol {
                Symbol::OpenCurlyBrace => {
                    // consume the open brace
                    *index+= 1;

                    let field_value= parse_value(tokens, index)?;

                    consume_next_token!(tokens, index, Token::Punctuation(Symbol::CloseCurlyBrace));

                    field_value
                },
                _ => SchemaValue::Null
            },
            _ => SchemaValue::Null
        }
    } else {
        SchemaValue::Null
    };

    // TODO: Shouldn't allocate this on the heap.
    // TODO: Don't allocate any more if Null.
    Ok(SchemaValue::EnumVariant(ident, Box::new(variant_field)))
}

pub fn tokens_to_schema_value(tokens: &Vec<Token>) -> ParseResult<SchemaValue> {
    if tokens.len() == 0 {
        return Err("Cannot parse empty token stream.");
    }

    let mut index= 0;
    match tokens[index] {
        Token::Punctuation(Symbol::OpenCurlyBrace) => {
            index+= 1;
            parse_object(tokens, &mut index)
        }
        _ => {
            return Err("Token stream does not represent valid object, should start with '{'.")
        }
    }
}
