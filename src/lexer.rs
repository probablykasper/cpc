use std::str::FromStr;
use decimal::d128;
use crate::{Token, TokenVector};
use crate::Operator::{Caret, Divide, Factorial, LeftParen, Minus, Modulo, Multiply, PercentOrModulo, Plus, RightParen};
use crate::Identifier::{Acos, Acosh, Asin, Asinh, Atan, Atanh, Cbrt, Ceil, Cos, Cosh, Exp, Fabs, Floor, Ln, Log, Pi, Round, Sin, Sinh, Sqrt, Tan, Tanh, E};

pub fn lex(input: &str) -> Result<TokenVector, String> {

  let mut chars = input.chars().enumerate().peekable();
  let mut tokens: TokenVector = vec![];
  let max_word_length = 5;
  
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
      'π' => tokens.push(Token::Identifier(Pi)),
      ',' => continue,
      value if value.is_whitespace() => continue,
      value if value.is_alphabetic() => {

        let start_index = byte_index;
        let mut end_index = byte_index;
        while let Some((_index, current_char)) = chars.peek() {
          // don't loop more than max_word_length:
          if end_index >= start_index + max_word_length - 1 { break; }

          if current_char.is_alphabetic() {
            println!("{}", current_char);
            byte_index += current_char.len_utf8();
            chars.next();
            end_index += 1;
          } else {
            break;
          }
        }

        let string = &input[start_index..=end_index];
        println!("STR {}", string);
        match string {
          
          // MAKE SURE max_word_length IS EQUAL TO THE
          // LENGTH OF THE LONGEST STRING IN THIS MATCH STATEMENT.

          "pi" => tokens.push(Token::Identifier(Pi)),
          "e" => tokens.push(Token::Identifier(E)),
          
          "mod" => tokens.push(Token::Operator(Modulo)),

          "sqrt" => tokens.push(Token::Identifier(Sqrt)),
          "cbrt" => tokens.push(Token::Identifier(Cbrt)),
          
          "log" => tokens.push(Token::Identifier(Log)),
          "ln" => tokens.push(Token::Identifier(Ln)),
          "exp" => tokens.push(Token::Identifier(Exp)),

          "ceil" => tokens.push(Token::Identifier(Ceil)),
          "floor" => tokens.push(Token::Identifier(Floor)),
          "round" | "rint" => tokens.push(Token::Identifier(Round)),
          "fabs" => tokens.push(Token::Identifier(Fabs)),

          "sin" => tokens.push(Token::Identifier(Sin)),
          "cos" => tokens.push(Token::Identifier(Cos)),
          "tan" => tokens.push(Token::Identifier(Tan)),
          "asin" => tokens.push(Token::Identifier(Asin)),
          "acos" => tokens.push(Token::Identifier(Acos)),
          "atan" => tokens.push(Token::Identifier(Atan)),
          "sinh" => tokens.push(Token::Identifier(Sinh)),
          "cosh" => tokens.push(Token::Identifier(Cosh)),
          "tanh" => tokens.push(Token::Identifier(Tanh)),
          "asinh" => tokens.push(Token::Identifier(Asinh)),
          "acosh" => tokens.push(Token::Identifier(Acosh)),
          "atanh" => tokens.push(Token::Identifier(Atanh)),
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
            byte_index += current_char.len_utf8();
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
