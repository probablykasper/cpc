use std::str::FromStr;
use decimal::d128;
use crate::{Token, TokenVector};
use crate::Operator::{Caret, Divide, LeftParen, Minus, Modulo, Multiply, Plus, RightParen};
use crate::UnaryOperator::{Percent, Factorial};
use crate::TextOperator::{Of, To};
use crate::Constant::{E, Pi};
use crate::FunctionIdentifier::{Cbrt, Ceil, Cos, Exp, Abs, Floor, Ln, Log, Round, Sin, Sqrt, Tan};
use crate::units::Unit::*;

pub fn lex(input: &str) -> Result<TokenVector, String> {

  let mut chars = input.chars().enumerate().peekable();
  let mut tokens: TokenVector = vec![];
  let max_word_length = 30;
  
  let mut left_paren_count = 0;
  let mut right_paren_count = 0;

  let mut byte_index = 0;
  while let Some((_index, current_char)) = chars.next() {
    match current_char {
      '+' => tokens.push(Token::Operator(Plus)),
      '-' => tokens.push(Token::Operator(Minus)),
      '*' => tokens.push(Token::Operator(Multiply)),
      '/' => tokens.push(Token::Operator(Divide)),
      '%' => tokens.push(Token::Operator(Modulo)),
      '^' => tokens.push(Token::Operator(Caret)),
      '!' => tokens.push(Token::UnaryOperator(Factorial)),
      '(' => {
        left_paren_count += 1;
        tokens.push(Token::Operator(LeftParen));
      },
      ')' => {
        right_paren_count += 1;
        tokens.push(Token::Operator(RightParen));
      },
      'π' => tokens.push(Token::Constant(Pi)),
      ',' => {},
      value if value.is_whitespace() => {},
      value if value.is_alphabetic() => {

        let start_index = byte_index;
        let mut end_index = byte_index;
        while let Some((_index, current_char)) = chars.peek() {
          // don't loop more than max_word_length:
          if end_index >= start_index + max_word_length - 1 {
            let string = &input[start_index..=end_index];
            return Err(format!("Invalid string starting with: {}", string));
          }

          if current_char.is_alphabetic() {
            byte_index += current_char.len_utf8();
            chars.next();
            end_index += 1;
          } else {
            let string = &input[start_index..=end_index];
            match string.trim_end() {
              // allow for two-word units
              "nautical" | "light" | "sq" | "square" | "cubic" => {
                byte_index += current_char.len_utf8();
                chars.next();
                end_index += 1;
              },
              _ => {
                break;
              },
            }
          }
        }

        let string = &input[start_index..=end_index];
        let string: &str = &string.replacen("square", "sq", 1);
        match string {
          
          // MAKE SURE max_word_length IS EQUAL TO THE
          // LENGTH OF THE LONGEST STRING IN THIS MATCH STATEMENT.

          "to" => tokens.push(Token::TextOperator(To)),
          "of" => tokens.push(Token::TextOperator(Of)),

          "pi" => tokens.push(Token::Constant(Pi)),
          "e" => tokens.push(Token::Constant(E)),
          
          "mod" => tokens.push(Token::Operator(Modulo)),

          "sqrt" => tokens.push(Token::FunctionIdentifier(Sqrt)),
          "cbrt" => tokens.push(Token::FunctionIdentifier(Cbrt)),
          
          "log" => tokens.push(Token::FunctionIdentifier(Log)),
          "ln" => tokens.push(Token::FunctionIdentifier(Ln)),
          "exp" => tokens.push(Token::FunctionIdentifier(Exp)),

          "round" | "rint" => tokens.push(Token::FunctionIdentifier(Round)),
          "ceil" => tokens.push(Token::FunctionIdentifier(Ceil)),
          "floor" => tokens.push(Token::FunctionIdentifier(Floor)),
          "abs" | "fabs" => tokens.push(Token::FunctionIdentifier(Abs)),

          "sin" => tokens.push(Token::FunctionIdentifier(Sin)),
          "cos" => tokens.push(Token::FunctionIdentifier(Cos)),
          "tan" => tokens.push(Token::FunctionIdentifier(Tan)),

          "ns" | "nanosec" | "nanosecs" | "nanosecond" | "nanoseconds" => tokens.push(Token::Unit(Nanosecond)),
          "μs" | "microsec" | "microsecs" | "microsecond" | "microseconds" => tokens.push(Token::Unit(Microsecond)),
          "ms" | "millisec" | "millisecs" | "millisecond" | "milliseconds" => tokens.push(Token::Unit(Millisecond)),
          "s" | "sec" | "secs" | "second" | "seconds" => tokens.push(Token::Unit(Second)),
          "min" | "mins" | "minute" | "minutes" => tokens.push(Token::Unit(Minute)),
          "h" | "hr" | "hrs" | "hour" | "hours" => tokens.push(Token::Unit(Hour)),
          "day" | "days" => tokens.push(Token::Unit(Day)),
          "wk" | "wks" | "week" | "weeks" => tokens.push(Token::Unit(Week)),
          "mo" | "mos" | "month" | "months" => tokens.push(Token::Unit(Month)),

          "q" | "quater" | "quaters" => tokens.push(Token::Unit(Month)),
          "yr" | "yrs" | "year" | "years" => tokens.push(Token::Unit(Year)),
          "decade" | "decades" => tokens.push(Token::Unit(Decade)),
          "century" | "centuries" => tokens.push(Token::Unit(Century)),
          "millenium" | "millenia" | "milleniums" => tokens.push(Token::Unit(Millenium)),

          "mm" | "millimeter" | "millimeters" => tokens.push(Token::Unit(Millimeter)),
          "cm" | "centimeter" | "centimeters" => tokens.push(Token::Unit(Centimeter)),
          "dm" | "decimeter" | "decimeters" => tokens.push(Token::Unit(Centimeter)),
          "m" | "meter" | "meters" => tokens.push(Token::Unit(Meter)),
          "km" | "kilometer" | "kilometers" => tokens.push(Token::Unit(Kilometer)),
          "in" | "inch" | "inches" => tokens.push(Token::Unit(Inch)),
          "ft" | "foot" | "feet" => tokens.push(Token::Unit(Foot)),
          "yd" | "yard" | "yards" => tokens.push(Token::Unit(Yard)),
          "mi" | "mile" | "miles" => tokens.push(Token::Unit(Mile)),
          "nmi" | "nautical mile" | "nautical miles" => tokens.push(Token::Unit(NauticalMile)),
          "lightyear" | "lightyears" | "light year" | "light years" => tokens.push(Token::Unit(LightYear)),
          
          "sqmm" | "sq mm" | "sq millimeter" | "sq millimeters" => tokens.push(Token::Unit(SquareMillimeter)),
          "sqcm" | "sq cm" | "sq centimeter" | "sq centimeters" => tokens.push(Token::Unit(SquareCentimeter)),
          "sqdm" | "sq dm" | "sq decimeter" | "sq decimeters" => tokens.push(Token::Unit(SquareDecimeter)),
          "sqm" | "sq m" | "sq meter" | "sq meters" => tokens.push(Token::Unit(SquareMeter)),
          "sqkm" | "sq km" | "sq kilometer" | "sq kilometers" => tokens.push(Token::Unit(SquareKilometer)),
          "sqin" | "sq in" | "sq inch" | "sq inches" => tokens.push(Token::Unit(SquareInch)),
          "sqft" | "sq ft" | "sq foot" | "sq feet" => tokens.push(Token::Unit(SquareFoot)),
          "sqyd" | "sq yd" | "sq yard" | "sq yards" => tokens.push(Token::Unit(SquareYard)),
          "sqmi" | "sq mi" | "sq mile" | "sq miles" => tokens.push(Token::Unit(SquareMile)),
          "are" | "ares" => tokens.push(Token::Unit(Are)),
          "decare" | "decares" => tokens.push(Token::Unit(Decare)),
          "ha" | "hectare" | "hectares" => tokens.push(Token::Unit(Hectare)),
          "acre" | "acres" => tokens.push(Token::Unit(Acre)),
          
          "cubic millimeter" | "cubic millimeters" => tokens.push(Token::Unit(CubicMillimeter)),
          "cubic centimeter" | "cubic centimeters" => tokens.push(Token::Unit(CubicCentimeter)),
          "cubic decimeter" | "cubic decimeters" => tokens.push(Token::Unit(CubicDecimeter)),
          "cubic meter" | "cubic meters" => tokens.push(Token::Unit(CubicMeter)),
          "cubic kilometer" | "cubic kilometers" => tokens.push(Token::Unit(CubicKilometer)),
          "cubic inch" | "cubic inches" => tokens.push(Token::Unit(CubicInch)),
          "cubic foot" | "cubic feet" => tokens.push(Token::Unit(CubicFoot)),
          "cubic yard" | "cubic yards" => tokens.push(Token::Unit(CubicYard)),
          "cubic mile" | "cubic miles" => tokens.push(Token::Unit(CubicMile)),
          "ml" | "milliliter" | "milliliters" => tokens.push(Token::Unit(Milliliter)),
          "cl" | "centiliter" | "centiliters" => tokens.push(Token::Unit(Centiliter)),
          "dl" | "deciliter" | "deciliters" => tokens.push(Token::Unit(Deciliter)),
          "l" | "liter" | "liters" => tokens.push(Token::Unit(Liter)),
          "ts" | "tsp" | "tspn" | "tspns" | "teaspoon" | "teaspoons" => tokens.push(Token::Unit(Teaspoon)),
          "tbs" | "tbsp" | "tablespoon" | "tablespoons" => tokens.push(Token::Unit(Tablespoon)),
          "floz" | "fl oz" | "fl ounce" | "fl ounces" | "fluid oz" | "fluid ounce" | "fluid ounces" => tokens.push(Token::Unit(FluidOunce)),
          "cup" | "cups" => tokens.push(Token::Unit(Cup)),
          "pt" | "pint" | "pints" => tokens.push(Token::Unit(Pint)),
          "qt" | "quart" | "quarts" => tokens.push(Token::Unit(Quart)),
          "gal" | "gallon" | "gallons" => tokens.push(Token::Unit(Gallon)),
          "bbl" | "oil barrel" | "oil barrels" => tokens.push(Token::Unit(OilBarrel)),
          
          "mg" | "milligram" | "milligrams" => tokens.push(Token::Unit(Milligram)),
          "g" | "gram" | "grams" => tokens.push(Token::Unit(Gram)),
          "hg" | "hectogram" | "hectograms" => tokens.push(Token::Unit(Hectogram)),
          "kg" | "kilo" | "kilos" | "kilogram" | "kilograms" => tokens.push(Token::Unit(Kilogram)),
          "t" | "tonne" | "tonnes" | "metric ton" | "metric tons" | "metric tonne" | "metric tonnes" => tokens.push(Token::Unit(MetricTon)),
          "oz" | "ounces" => tokens.push(Token::Unit(Ounce)),
          "lb" | "lbs" | "pound" | "pounds" => tokens.push(Token::Unit(Pound)),
          "st" | "ton" | "tons" | "short ton" | "short tons" | "short tonne" | "short tonnes" => tokens.push(Token::Unit(ShortTon)),
          "lt" | "long ton" | "long tons" | "long tonne" | "long tonnes" => tokens.push(Token::Unit(LongTon)),

          "bit" | "bits" => tokens.push(Token::Unit(Bit)),
          "kbit" | "kilobit" | "kilobits" => tokens.push(Token::Unit(Kilobit)),
          "mbit" | "megabit" | "megabits" => tokens.push(Token::Unit(Megabit)),
          "gbit" | "gigabit" | "gigabits" => tokens.push(Token::Unit(Gigabit)),
          "tbit" | "terabit" | "terabits" => tokens.push(Token::Unit(Terabit)),
          "pbit" | "petabit" | "petabits" => tokens.push(Token::Unit(Petabit)),
          "ebit" | "exabit" | "exabits" => tokens.push(Token::Unit(Exabit)),
          "zbit" | "zettabit" | "zettabits" => tokens.push(Token::Unit(Zettabit)),
          "ybit" | "yottabit" | "yottabits" => tokens.push(Token::Unit(Yottabit)),
          "kibit" | "kibibit" | "kibibits" => tokens.push(Token::Unit(Kibibit)),
          "mibit" | "mebibit" | "mebibits" => tokens.push(Token::Unit(Mebibit)),
          "gibit" | "gibibit" | "gibibits" => tokens.push(Token::Unit(Gibibit)),
          "tibit" | "tebibit" | "tebibits" => tokens.push(Token::Unit(Tebibit)),
          "pibit" | "pebibit" | "pebibits" => tokens.push(Token::Unit(Pebibit)),
          "eibit" | "exbibit" | "exbibits" => tokens.push(Token::Unit(Exbibit)),
          "zibit" | "zebibit" | "zebibits" => tokens.push(Token::Unit(Zebibit)),
          "yibit" | "yobibit" | "yobibits" => tokens.push(Token::Unit(Yobibit)),
          "byte" | "bytes" => tokens.push(Token::Unit(Byte)),
          "kb" | "kilobyte" | "kilobytes" => tokens.push(Token::Unit(Kilobyte)),
          "mb" | "megabyte" | "megabytes" => tokens.push(Token::Unit(Megabyte)),
          "gb" | "gigabyte" | "gigabytes" => tokens.push(Token::Unit(Gigabyte)),
          "tb" | "terabyte" | "terabytes" => tokens.push(Token::Unit(Terabyte)),
          "pb" | "petabyte" | "petabytes" => tokens.push(Token::Unit(Petabyte)),
          "eb" | "exabyte" | "exabytes" => tokens.push(Token::Unit(Exabyte)),
          "zb" | "zettabyte" | "zettabytes" => tokens.push(Token::Unit(Zettabyte)),
          "yb" | "yottabyte" | "yottabytes" => tokens.push(Token::Unit(Yottabyte)),
          "kib" | "kibibyte" | "kibibytes" => tokens.push(Token::Unit(Kibibyte)),
          "mib" | "mebibyte" | "mebibytes" => tokens.push(Token::Unit(Mebibyte)),
          "gib" | "gibibyte" | "gibibytes" => tokens.push(Token::Unit(Gibibyte)),
          "tib" | "tebibyte" | "tebibytes" => tokens.push(Token::Unit(Tebibyte)),
          "pib" | "pebibyte" | "pebibytes" => tokens.push(Token::Unit(Pebibyte)),
          "eib" | "exbibyte" | "exbibytes" => tokens.push(Token::Unit(Exbibyte)),
          "zib" | "zebibyte" | "zebibytes" => tokens.push(Token::Unit(Zebibyte)),
          "yib" | "yobibyte" | "yobibytes" => tokens.push(Token::Unit(Yobibyte)),

          "k" | "kelvin" | "kelvins" => tokens.push(Token::Unit(Kelvin)),
          "c" | "celcius" => tokens.push(Token::Unit(Celcius)),
          "f" | "fahrenheit" | "fahrenheits" => tokens.push(Token::Unit(Fahrenheit)),

          _ => {
            return Err(format!("Invalid string: {}", string));
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

  // auto insert missing parentheses in first and last position
  if left_paren_count > right_paren_count {
    let missing_right_parens = left_paren_count - right_paren_count;
    for _ in 0..missing_right_parens {
      tokens.push(Token::Operator(RightParen));
    }
  } else if left_paren_count < right_paren_count {
    let missing_left_parens = right_paren_count - left_paren_count;
    for _ in 0..missing_left_parens {
      tokens.insert(0, Token::Operator(LeftParen));
    }
  }

  // the lexer parses percentages as modulo, so here modulos become percentages
  let mut token_index = 0;
  for _i in 1..tokens.len() {
    match tokens[token_index] {
      Token::Operator(Modulo) => {
        match &tokens[token_index + 1] {
          Token::TextOperator(Of) => {
            // for example "10% of 1km" should be a percentage, not modulo
            tokens[token_index] = Token::UnaryOperator(Percent);
          },
          Token::Operator(operator) => {
            match operator {
              LeftParen => {},
              _ => {
                // for example "10%*2" should be a percentage, but "10%(2)" should be modulo
                tokens[token_index] = Token::UnaryOperator(Percent);
              }
            }
          },
          Token::UnaryOperator(_operator) => {
            // for example "10%!" should be a percentage, but "10%(2)" should be modulo
            tokens[token_index] = Token::UnaryOperator(Percent);
          },
          _ => {},
        }
      }
      _ => {},
    }
    token_index += 1;
  }

  Ok(tokens)
}
