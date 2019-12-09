use std::str::FromStr;
use decimal::d128;
use crate::{Token, TokenVector};
use crate::Operator::{Caret, Divide, Factorial, LeftParen, Minus, Multiply, PercentOrModulo, Plus, RightParen};
use crate::Constant::{Pi, EulersNumber};

pub fn lex(input: &str) -> Result<TokenVector, String> {

  let mut chars = input.chars().enumerate().peekable();
  let mut tokens: TokenVector = vec![];
  
  let mut byte_index = 0;
  while let Some((_index, current_char)) = chars.next() {
    match current_char {
      '+' => tokens.push(Token::Operator(Plus)),
      '-' => tokens.push(Token::Operator(Minus)),
      '*' => tokens.push(Token::Operator(Multiply)),
      '/' => tokens.push(Token::Operator(Divide)),
      '%' => tokens.push(Token::Operator(PercentOrModulo)),
      '^' => tokens.push(Token::Operator(Caret)),
      '!' => tokens.push(Token::Operator(Factorial)),
      '(' => tokens.push(Token::Operator(LeftParen)),
      ')' => tokens.push(Token::Operator(RightParen)),
      'π' => tokens.push(Token::Constant(Pi)),
      ',' => continue,
      value if value.is_whitespace() => continue,
      value if value.is_alphabetic() => {

        let start_index = byte_index;
        let mut end_index = byte_index;
        while let Some((_index, current_char)) = chars.peek() {
          if current_char.is_alphabetic() {
            chars.next();
            end_index += 1;
          } else {
            break;
          }
        }

        let string = &input[start_index..=end_index];
        match string {
          "pi" => tokens.push(Token::Constant(Pi)),
          "e" => tokens.push(Token::Constant(EulersNumber)),
          _ => {
            return Err(format!("Invalid string: {}", string))
          }
        }
        
      },
      '.' | '0'..='9' => {

        let start_index = byte_index;
        let mut end_index = byte_index;
        while let Some((_index, current_char)) = chars.peek() {
          if current_char == &'.' || current_char.is_digit(10) {
            chars.next();
            end_index += 1;
          } else {
            break;
          }
        }
        
        let number_string = &input[start_index..=end_index];
        match d128::from_str(number_string) {
          Ok(number) => {
            if d128::get_status().is_empty() {
              tokens.push(Token::Number(number));
            } else {
              return Err(format!("Error parsing d128 number: {}", number_string));
            }
          },
          Err(_e) => {
            return Err(format!("Error parsing d128 number: {}", number_string));
          }
        };

      },
      _ => {
        return Err(format!("Invalid character: {}", current_char));
      },
    }
    // The π character, for example, is more than one byte, so in that case
    // byte_index needs to be incremented by 2. This is because we're slicing
    // strings to get digits/words, and Rust slices bytes, not utf8 graphemes
    // (aka "user-perceived characters").
    byte_index += current_char.len_utf8();
  };
  return Ok(tokens)
}
