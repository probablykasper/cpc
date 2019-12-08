use std::str::FromStr;
use decimal::d128;
use crate::{Token, TokenVector, Operator::*};

pub fn lex(input: &str) -> Result<TokenVector, String> {

  let mut chars = input.chars().enumerate().peekable();
  let mut tokens: TokenVector = vec![];
  
  while let Some((index, current_char)) = chars.next() {
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
      ',' => continue,
      value if value.is_whitespace() => continue,
      value if value.is_alphabetic() => {

      },
      '.' | '0'..='9' => {

        let start_index = index;
        let mut end_index = index+1;
        while let Some((_idk, current_char)) = chars.peek() {
          if current_char == &'.' || current_char.is_digit(10) {
            chars.next();
            end_index += 1;
          } else {
            break;
          }
        }
        
        match d128::from_str(&input[start_index..end_index]) {
          Ok(number) => {
            if d128::get_status().is_empty() {
              tokens.push(Token::Number(number));
            } else {
              return Err(format!("Error parsing d128 number: {}", &input[start_index..end_index]));
            }
          },
          Err(_e) => {
            return Err("Error parsing d128 number (This should not happen because d128 does not throw errors)".to_owned());
          }
        };

      },
      _ => {
        return Err(format!("Unknown character: {}", current_char));
      },
    }
  };
  return Ok(tokens)
}
