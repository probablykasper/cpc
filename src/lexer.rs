use std::str::FromStr;
use decimal::d128;
use crate::{Token, TokenVector};
use crate::Operator::{Caret, Divide, LeftParen, Minus, Modulo, Multiply, Plus, RightParen};
use crate::UnaryOperator::{Percent, Factorial};
use crate::TextOperator::{Of, To};
use crate::Constant::{E, Pi};
use crate::LexerKeyword::{In, PercentChar, Per, Mercury, Hg, PoundForce, PoundWord, Force, DoubleQuotes};
use crate::FunctionIdentifier::{Cbrt, Ceil, Cos, Exp, Abs, Floor, Ln, Log, Round, Sin, Sqrt, Tan};
use crate::units::Unit::*;

pub fn lex(input: &str) -> Result<TokenVector, String> {

  let input = input.replace(",", "");
  let mut chars = input.chars().peekable();
  let mut tokens: TokenVector = vec![];
  let max_word_length = 30;
  
  let mut left_paren_count = 0;
  let mut right_paren_count = 0;

  let mut byte_index = 0;
  while let Some(current_char) = chars.next() {
    match current_char {
      '+' => tokens.push(Token::Operator(Plus)),
      '-' => tokens.push(Token::Operator(Minus)),
      '*' => tokens.push(Token::Operator(Multiply)),
      '/' => tokens.push(Token::Operator(Divide)),
      '%' => tokens.push(Token::LexerKeyword(PercentChar)),
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
      '\'' => tokens.push(Token::Unit(Foot)),
      '"' | '“' | '”' | '″' => tokens.push(Token::LexerKeyword(DoubleQuotes)),
      value if value.is_whitespace() => {},
      value if value.is_alphabetic() => {

        let start_index = byte_index;
        let mut end_index = byte_index;
        while let Some(current_char) = chars.peek() {
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
              "nautical" | "light" | "sq" | "square" | "cubic" | "metric" | "newton" => {
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

          "per" => tokens.push(Token::LexerKeyword(Per)),
          "hg" => tokens.push(Token::LexerKeyword(Hg)), // can be hectogram or mercury

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
          "in" => tokens.push(Token::LexerKeyword(In)),
          "inch" | "inches" => tokens.push(Token::Unit(Inch)),
          "ft" | "foot" | "feet" => tokens.push(Token::Unit(Foot)),
          "yd" | "yard" | "yards" => tokens.push(Token::Unit(Yard)),
          "mi" | "mile" | "miles" => tokens.push(Token::Unit(Mile)),
          "nmi" | "nautical mile" | "nautical miles" => tokens.push(Token::Unit(NauticalMile)),
          "ly" | "lightyear" | "lightyears" | "light yr" | "light yrs" | "light year" | "light years" => tokens.push(Token::Unit(LightYear)),
          "lightsec" | "lightsecs" | "lightsecond" | "lightseconds" | "light sec" | "light secs" | "light second" | "light seconds" => tokens.push(Token::Unit(LightYear)),
          
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
          "hectogram" | "hectograms" => tokens.push(Token::Unit(Hectogram)),
          "kg" | "kilo" | "kilos" | "kilogram" | "kilograms" => tokens.push(Token::Unit(Kilogram)),
          "t" | "tonne" | "tonnes" | "metric ton" | "metric tons" | "metric tonne" | "metric tonnes" => tokens.push(Token::Unit(MetricTon)),
          "oz" | "ounces" => tokens.push(Token::Unit(Ounce)),
          "lb" | "lbs" | "pounds" => tokens.push(Token::Unit(Pound)),
          "pound" => tokens.push(Token::LexerKeyword(PoundWord)),
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

          "millijoule" | "millijoules" => tokens.push(Token::Unit(Millijoule)),
          "j"| "joule" | "joules" => tokens.push(Token::Unit(Joule)),
          "nm" | "newton meter" | "newton meters" | "newton-meter" | "newton-meters" => tokens.push(Token::Unit(NewtonMeter)),
          "kj" | "kilojoule" | "kilojoules" => tokens.push(Token::Unit(Kilojoule)),
          "mj" | "megajoule" | "megajoules" => tokens.push(Token::Unit(Megajoule)),
          "gj" | "gigajoule" | "gigajoules" => tokens.push(Token::Unit(Gigajoule)),
          "tj" | "terajoule" | "terajoules" => tokens.push(Token::Unit(Terajoule)),
          "cal" | "calorie" | "calories" => tokens.push(Token::Unit(Calorie)),
          "kcal" | "kilocalorie" | "kilocalories" => tokens.push(Token::Unit(KiloCalorie)),
          "btu" | "british thermal unit" | "british thermal units" => tokens.push(Token::Unit(BritishThermalUnit)),
          "wh" | "watt hour" | "watt hours" => tokens.push(Token::Unit(WattHour)),
          "kwh" | "kilowatt hour" | "kilowatt hours" => tokens.push(Token::Unit(KilowattHour)),
          "mwh" | "megawatt hour" | "megawatt hours" => tokens.push(Token::Unit(MegawattHour)),
          "gwh" | "gigawatt hour" | "gigawatt hours" => tokens.push(Token::Unit(GigawattHour)),
          "twh" | "terawatt hour" | "terawatt hours" => tokens.push(Token::Unit(TerawattHour)),
          "pwh" | "petawatt hour" | "petawatt hours" => tokens.push(Token::Unit(PetawattHour)),

          "milliwatt" | "milliwatts" => tokens.push(Token::Unit(Milliwatt)),
          "w" | "watt" | "watts" => tokens.push(Token::Unit(Watt)),
          "kw" | "kilowatt" | "kilowatts" => tokens.push(Token::Unit(Kilowatt)),
          "mw" | "megawatt" | "megawatts" => tokens.push(Token::Unit(Megawatt)),
          "gw" | "gigawatt" | "gigawatts" => tokens.push(Token::Unit(Gigawatt)),
          "tw" | "terawatt" | "terawatts" => tokens.push(Token::Unit(Terawatt)),
          "pw" | "petawatt" | "petawatts" => tokens.push(Token::Unit(Petawatt)),
          "hp" | "hps" | "horsepower" | "horsepowers" => tokens.push(Token::Unit(Horsepower)),
          "mhp" | "hpm" | "metric hp" | "metric hps" | "metric horsepower" | "metric horsepowers" => tokens.push(Token::Unit(MetricHorsepower)),

          // for pound-force per square inch
          "lbf" => tokens.push(Token::LexerKeyword(PoundForce)),
          "force" => tokens.push(Token::LexerKeyword(Force)),
          
          "pa" | "pascal" | "pascals" => tokens.push(Token::Unit(Pascal)),
          "kpa" | "kilopascal" | "kilopascals" => tokens.push(Token::Unit(Kilopascal)),
          "atm" | "atms" | "atmosphere" | "atmospheres" => tokens.push(Token::Unit(Atmosphere)),
          "mbar" | "mbars" | "millibar" | "millibars" => tokens.push(Token::Unit(Millibar)),
          "bar" | "bars" => tokens.push(Token::Unit(Bar)),
          "inhg" => tokens.push(Token::Unit(InchOfMercury)),
          "mercury" => tokens.push(Token::LexerKeyword(Mercury)),
          "psi" => tokens.push(Token::Unit(PoundsPerSquareInch)),
          "torr" | "torrs" => tokens.push(Token::Unit(Torr)),

          "kph" | "kmh" => tokens.push(Token::Unit(KilometersPerHour)),
          "mps" => tokens.push(Token::Unit(MetersPerSecond)),
          "mph" => tokens.push(Token::Unit(MilesPerHour)),
          "fps" => tokens.push(Token::Unit(FeetPerSecond)),
          "kn" | "kt" | "knot" | "knots" => tokens.push(Token::Unit(Knot)),

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
        while let Some(current_char) = chars.peek() {
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

  let mut token_index = 0;
  loop {
    match tokens[token_index] {
      // decide if % is percent or modulo
      Token::LexerKeyword(PercentChar) => {
        match tokens.get(token_index + 1) {
          Some(Token::TextOperator(Of)) => {
            // "10% of 1km" should be percentage
            tokens[token_index] = Token::UnaryOperator(Percent);
          },
          Some(Token::Operator(operator)) => {
            match operator {
              LeftParen => {
                // "10%(2)" should be modulo
                tokens[token_index] = Token::Operator(Modulo);
              },
              _ => {
                // "10%*2" should be a percentage
                tokens[token_index] = Token::UnaryOperator(Percent);
              }
            }
          },
          Some(Token::UnaryOperator(_operator)) => {
            // "10%!" should be a percentage
            tokens[token_index] = Token::UnaryOperator(Percent);
          },
          None => {
            // percent if there's no element afterwards
            tokens[token_index] = Token::UnaryOperator(Percent);
          },
          _ => {
            // everything else should be modulo, for example if the % is
            // before a number, function or constants
            tokens[token_index] = Token::Operator(Modulo);
          },
        }
      },
      // decide if " is inch of inch of mercury
      Token::LexerKeyword(DoubleQuotes) => {
        match tokens.get(token_index + 1) {
          Some(Token::LexerKeyword(Hg)) => {
            // "hg should be inch of mercury
            tokens[token_index] = Token::Unit(InchOfMercury);
            tokens.remove(token_index + 1);
          },
          _ => {
            // otherwise, Inch
            tokens[token_index] = Token::Unit(Inch);
          },
        }
      },
      // if hg wasn't already turned into inch of mercury, it's hectogram
      Token::LexerKeyword(Hg) => {
        tokens[token_index] = Token::Unit(Hectogram);
      },
      // decide if "in" is Inch or To
      Token::LexerKeyword(In) => {
        match tokens.get(token_index + 1) {
          Some(Token::Unit(_)) => {
            // "in" should be To
            tokens[token_index] = Token::TextOperator(To);
          },
          _ => {
            // otherwise, Inch
            tokens[token_index] = Token::Unit(Inch);
          },
        }
      },
      _ => {},
    }
    // parse units like km/h, lbf per square inch
    if token_index >= 2 {
      let token1 = &tokens[token_index-2];
      let token2 = match &tokens[token_index-1] {
        // treat km/h the same as km per h
        Token::Operator(Divide) => &Token::LexerKeyword(Per),
        _ => &tokens[token_index-1],
      };
      let token3 = &tokens[token_index];
      let mut replaced = true;
      match (token1, token2, token3) {
        // km/h
        (Token::Unit(Kilometer), Token::LexerKeyword(Per), Token::Unit(Hour)) => {
          tokens[token_index-2] = Token::Unit(KilometersPerHour);
        },
        // mi/h
        (Token::Unit(Mile), Token::LexerKeyword(Per), Token::Unit(Hour)) => {
          tokens[token_index-2] = Token::Unit(MilesPerHour);
        },
        // m/s
        (Token::Unit(Meter), Token::LexerKeyword(Per), Token::Unit(Second)) => {
          tokens[token_index-2] = Token::Unit(MetersPerSecond);
        },
        // ft/s
        (Token::Unit(Foot), Token::LexerKeyword(Per), Token::Unit(Second)) => {
          tokens[token_index-2] = Token::Unit(FeetPerSecond);
        },
        // btu/min
        (Token::Unit(BritishThermalUnit), Token::LexerKeyword(Per), Token::Unit(Minute)) => {
          tokens[token_index-2] = Token::Unit(BritishThermalUnitsPerMinute);
        },
        // btu/h
        (Token::Unit(BritishThermalUnit), Token::LexerKeyword(Per), Token::Unit(Hour)) => {
          tokens[token_index-2] = Token::Unit(BritishThermalUnitsPerHour);
        },
        // pound-force
        (Token::LexerKeyword(PoundWord), Token::Operator(Minus), Token::LexerKeyword(Force)) => {
          tokens[token_index-2] = Token::LexerKeyword(PoundForce);
        },
        // lbs/sqin
        (Token::LexerKeyword(PoundForce), Token::LexerKeyword(Per), Token::Unit(SquareInch)) => {
          tokens[token_index-2] = Token::Unit(PoundsPerSquareInch);
        },
        // inch of mercury
        (Token::Unit(Inch), Token::TextOperator(Of), Token::LexerKeyword(Mercury)) => {
          tokens[token_index-2] = Token::Unit(InchOfMercury);
        },
        _ => {
          replaced = false;
        },
      }
      if replaced {
        tokens.remove(token_index);
        tokens.remove(token_index-1);
        token_index -= 2;
      }
    }
    if token_index == tokens.len()-1 {
      break;
    } else {
      token_index += 1;
    }
  }

  Ok(tokens)
}
