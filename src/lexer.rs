use std::str::FromStr;
use std::ops::RangeInclusive;
use decimal::d128;
use crate::Token;
use crate::Operator::{Caret, Divide, LeftParen, Minus, Modulo, Multiply, Plus, RightParen};
use crate::UnaryOperator::{Percent, Factorial};
use crate::TextOperator::{Of, To};
use crate::NamedNumber::*;
use crate::Constant::{E, Pi};
use crate::LexerKeyword::{In, PercentChar, Per, Mercury, Hg, PoundForce, Force, DoubleQuotes, Revolution};
use crate::FunctionIdentifier::{Cbrt, Ceil, Cos, Exp, Abs, Floor, Ln, Log, Round, Sin, Sqrt, Tan};
use crate::units::Unit;
use crate::units::Unit::*;

use unicode_segmentation::UnicodeSegmentation;

const LOWERCASE_LETTERS: RangeInclusive<char> = 'a'..='z';
const UPPERCASE_LETTERS: RangeInclusive<char> = 'A'..='Z';

pub fn is_str_alphabetic_extended(input: &str) -> bool {
  match input {
    "Ω" | "Ω" | "µ" | "μ" | "π" => true,
    value if value.chars().all(|c| LOWERCASE_LETTERS.contains(&c)) => true,
    value if value.chars().all(|c| UPPERCASE_LETTERS.contains(&c)) => true,
    _ => false,
  }
}

pub fn is_str_numeric(input: &str) -> bool {
  match input {
    "." => true,
    "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => true,
    _ => false,
  }
}

/// Lex an input string and returns [`Token`]s
pub fn lex(input: &str, allow_trailing_operators: bool, default_degree: Unit) -> Result<Vec<Token>, String> {
  // ignore commas, and neutralise case-sensitivity
  let mut input = input.replace(",", "").to_lowercase();

  if allow_trailing_operators {
    match &input.chars().last().unwrap_or('x') {
      '+' | '-' | '*' | '/' | '^' | '(' => {
        input.pop();
      },
      _ => {},
    }
  }

  let graphemes = UnicodeSegmentation::graphemes(input.as_str(), true).collect::<Vec<&str>>();

  let mut tokens: Vec<Token> = vec![];

  let mut left_paren_count = 0;
  let mut right_paren_count = 0;

  let mut grapheme_counter = 0;
  while grapheme_counter < graphemes.len() {
    let grapheme = graphemes[grapheme_counter];
    match grapheme {
      "+" => tokens.push(Token::Operator(Plus)),
      "-" => tokens.push(Token::Operator(Minus)),
      "*" => tokens.push(Token::Operator(Multiply)),
      "/" => tokens.push(Token::Operator(Divide)),
      "%" => tokens.push(Token::LexerKeyword(PercentChar)),
      "^" => tokens.push(Token::Operator(Caret)),
      "!" => tokens.push(Token::UnaryOperator(Factorial)),
      "(" => {
        left_paren_count += 1;
        tokens.push(Token::Operator(LeftParen));
      },
      ")" => {
        right_paren_count += 1;
        tokens.push(Token::Operator(RightParen));
      },
      "π" => tokens.push(Token::Constant(Pi)),
      "'" => tokens.push(Token::Unit(Foot)),
      "\"" | "“" | "”" | "″" => tokens.push(Token::LexerKeyword(DoubleQuotes)),
      value if value.trim().is_empty() => {},
      "Ω" | "Ω" => tokens.push(Token::Unit(Ohm)),
      value if is_str_alphabetic_extended(&value) => {
        let next_word = |cur_grapheme_counter: usize| {
          let mut _grapheme_counter = cur_grapheme_counter;
          let mut word_string = graphemes[_grapheme_counter].to_string();
          while (_grapheme_counter + 1) < graphemes.len() {
            let next_grapheme = &graphemes[_grapheme_counter + 1];
            if is_str_alphabetic_extended(next_grapheme) {
              word_string.push_str(next_grapheme);
              _grapheme_counter += 1;
            } else {
              break;
            }
          }

          if _grapheme_counter < (graphemes.len() - 1) {
            let next_grapheme = graphemes[_grapheme_counter + 1];
            match next_grapheme {
              "2" | "3" => {
                word_string.push_str(next_grapheme);
                _grapheme_counter += 1;
              },
              "²" => {
                word_string.push_str("2");
                _grapheme_counter += 1;
              },
              "³" => {
                word_string.push_str("3");
                _grapheme_counter += 1;
              },
              _ => {},
            }
          }

          return (_grapheme_counter, word_string);
        };

        let skip_whitespace = |cur_grapheme_counter: usize| {
          let mut _grapheme_counter = cur_grapheme_counter;
          if _grapheme_counter < (graphemes.len() - 1) {
            let next_grapheme = graphemes[_grapheme_counter + 1];
            if ! next_grapheme.trim().is_empty() {
              return Err(format!("Unexpected non-whitespace character: {}", next_grapheme));
            }
          } else {
            return Err(format!("Unexpected end of input."));
          }

          while (_grapheme_counter + 1) < graphemes.len() {
            _grapheme_counter += 1;
            let next_grapheme = &graphemes[_grapheme_counter];
            if ! next_grapheme.trim().is_empty() {
              break;
            }
          }

          return Ok(_grapheme_counter);
        };

        let (new_grapheme_counter, word) = next_word(grapheme_counter.to_owned());
        grapheme_counter = new_grapheme_counter;
        let word_str = word.as_str();
        match word_str {
          "in" => tokens.push(Token::LexerKeyword(In)),
          "to" => tokens.push(Token::TextOperator(To)),
          "of" => tokens.push(Token::TextOperator(Of)),

          "hundred" => tokens.push(Token::NamedNumber(Hundred)),
          "thousand" => tokens.push(Token::NamedNumber(Thousand)),
          "mil" | "mill" | "million" => tokens.push(Token::NamedNumber(Million)),
          "bil" | "bill" | "billion" => tokens.push(Token::NamedNumber(Billion)),
          "tri" | "tril" | "trillion" => tokens.push(Token::NamedNumber(Trillion)),
          "quadrillion" => tokens.push(Token::NamedNumber(Quadrillion)),
          "quintillion" => tokens.push(Token::NamedNumber(Quintillion)),
          "sextillion" => tokens.push(Token::NamedNumber(Sextillion)),
          "septillion" => tokens.push(Token::NamedNumber(Septillion)),
          "octillion" => tokens.push(Token::NamedNumber(Octillion)),
          "nonillion" => tokens.push(Token::NamedNumber(Nonillion)),
          "decillion" => tokens.push(Token::NamedNumber(Decillion)),
          "undecillion" => tokens.push(Token::NamedNumber(Undecillion)),
          "duodecillion" => tokens.push(Token::NamedNumber(Duodecillion)),
          "tredecillion" => tokens.push(Token::NamedNumber(Tredecillion)),
          "quattuordecillion" => tokens.push(Token::NamedNumber(Quattuordecillion)),
          "quindecillion" => tokens.push(Token::NamedNumber(Quindecillion)),
          "sexdecillion" => tokens.push(Token::NamedNumber(Sexdecillion)),
          "septendecillion" => tokens.push(Token::NamedNumber(Septendecillion)),
          "octodecillion" => tokens.push(Token::NamedNumber(Octodecillion)),
          "novemdecillion" => tokens.push(Token::NamedNumber(Novemdecillion)),
          "vigintillion" => tokens.push(Token::NamedNumber(Vigintillion)),
          "centillion" => tokens.push(Token::NamedNumber(Centillion)),
          "googol" => tokens.push(Token::NamedNumber(Googol)),

          "pi" => tokens.push(Token::Constant(Pi)),
          "e" => tokens.push(Token::Constant(E)),

          "plus" => tokens.push(Token::Operator(Plus)),
          "minus" => tokens.push(Token::Operator(Minus)),
          "times" => tokens.push(Token::Operator(Multiply)),
          "multiplied" => {
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => return Err(format!("Invalid string: {}", word_str)),
            }

            let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
            grapheme_counter = new_grapheme_counter;
            let word2_str = word2.as_str();
            match word2_str {
              "by" => tokens.push(Token::Operator(Multiply)),
              _ => return Err(format!("Invalid string: {} {}", word_str, word2_str)),
            }
          },
          "divided" => {
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => return Err(format!("Invalid string: {}", word_str)),
            }

            let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
            grapheme_counter = new_grapheme_counter;
            let word2_str = word2.as_str();
            match word2_str {
              "by" => tokens.push(Token::Operator(Divide)),
              _ => return Err(format!("Invalid string: {} {}", word_str, word2_str)),
            }
          },
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
          // µ and μ are two different characters
          "µs" | "μs" | "microsec" | "microsecs" | "microsecond" | "microseconds" => tokens.push(Token::Unit(Microsecond)),
          "ms" | "millisec" | "millisecs" | "millisecond" | "milliseconds" => tokens.push(Token::Unit(Millisecond)),
          "s" | "sec" | "secs" | "second" | "seconds" => tokens.push(Token::Unit(Second)),
          "min" | "mins" | "minute" | "minutes" => tokens.push(Token::Unit(Minute)),
          "h" | "hr" | "hrs" | "hour" | "hours" => tokens.push(Token::Unit(Hour)),
          "day" | "days" => tokens.push(Token::Unit(Day)),
          "wk" | "wks" | "week" | "weeks" => tokens.push(Token::Unit(Week)),
          "mo" | "mos" | "month" | "months" => tokens.push(Token::Unit(Month)),
          "q" | "quarter" | "quarters" => tokens.push(Token::Unit(Quarter)),
          "yr" | "yrs" | "year" | "years" => tokens.push(Token::Unit(Year)),
          "decade" | "decades" => tokens.push(Token::Unit(Decade)),
          "century" | "centuries" => tokens.push(Token::Unit(Century)),
          "millenium" | "millenia" | "milleniums" => tokens.push(Token::Unit(Millenium)),

          "mm" | "millimeter" | "millimeters" | "millimetre" | "millimetres" => tokens.push(Token::Unit(Millimeter)),
          "cm" | "centimeter" | "centimeters" | "centimetre" | "centimetres" => tokens.push(Token::Unit(Centimeter)),
          "dm" | "decimeter" | "decimeters" | "decimetre" | "decimetres" => tokens.push(Token::Unit(Decimeter)),
          "m" | "meter" | "meters" | "metre" | "metres" => tokens.push(Token::Unit(Meter)),
          "km" | "kilometer" | "kilometers" | "kilometre" | "kilometres" => tokens.push(Token::Unit(Kilometer)),
          "inch" | "inches" => tokens.push(Token::Unit(Inch)),
          "ft" | "foot" | "feet" => tokens.push(Token::Unit(Foot)),
          "yd" | "yard" | "yards" => tokens.push(Token::Unit(Yard)),
          "mi" | "mile" | "miles" => tokens.push(Token::Unit(Mile)),
          "nmi" => tokens.push(Token::Unit(NauticalMile)),
          "nautical" => {
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => return Err(format!("Invalid string: {}", word_str)),
            }

            let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
            grapheme_counter = new_grapheme_counter;
            let word2_str = word2.as_str();
            match word2_str {
              "mile" | "miles" => tokens.push(Token::Unit(NauticalMile)),
              _ => return Err(format!("Invalid string: {} {}", word_str, word2_str)),
            }
          },
          "ly" | "lightyear" | "lightyears" => tokens.push(Token::Unit(LightYear)),
          "lightsec" | "lightsecs" | "lightsecond" | "lightseconds" => tokens.push(Token::Unit(LightSecond)),
          "light" => {
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => return Err(format!("Invalid string: {}", word_str)),
            }

            let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
            grapheme_counter = new_grapheme_counter;
            let word2_str = word2.as_str();
            match word2_str {
              "yr" | "yrs" | "year" | "years" => tokens.push(Token::Unit(LightYear)),
              "sec" | "secs" | "second" | "seconds" => tokens.push(Token::Unit(LightSecond)),
              _ => return Err(format!("Invalid string: {} {}", word_str, word2_str)),
            }
          },

          "sqmm" | "mm2" | "millimeter2" | "millimeters2" | "millimetre2" | "millimetres2" => tokens.push(Token::Unit(SquareMillimeter)),
          "sqcm" | "cm2" | "centimeter2" | "centimeters2" | "centimetre2" | "centimetres2" => tokens.push(Token::Unit(SquareCentimeter)),
          "sqdm" | "dm2" | "decimeter2" | "decimeters2" | "decimetre2" | "decimetres2" => tokens.push(Token::Unit(SquareDecimeter)),
          "sqm" | "m2" | "meter2" | "meters2" | "metre2" | "metres2" => tokens.push(Token::Unit(SquareMeter)),
          "sqkm" | "km2" | "kilometer2" | "kilometers2" | "kilometre2" | "kilometres2" => tokens.push(Token::Unit(SquareKilometer)),
          "sqin" | "in2" | "inch2" | "inches2" => tokens.push(Token::Unit(SquareInch)),
          "sqft" | "ft2" | "foot2" | "feet2" => tokens.push(Token::Unit(SquareFoot)),
          "sqyd" | "yd2" | "yard2" | "yards2" => tokens.push(Token::Unit(SquareYard)),
          "sqmi" | "mi2" | "mile2" | "miles2" => tokens.push(Token::Unit(SquareMile)),
          "sq" | "square" => {
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => return Err(format!("Invalid string: {}", word_str)),
            }

            let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
            grapheme_counter = new_grapheme_counter;
            let word2_str = word2.as_str();
            match word2_str {
              "mm" | "millimeter" | "millimeters" | "millimetre" | "millimetres" => tokens.push(Token::Unit(SquareMillimeter)),
              "cm" | "centimeter" | "centimeters" | "centimetre" | "centimetres" => tokens.push(Token::Unit(SquareCentimeter)),
              "dm" | "decimeter" | "decimeters" | "decimetre" | "decimetres" => tokens.push(Token::Unit(SquareDecimeter)),
              "m" | "meter" | "meters" | "metre" | "metres" => tokens.push(Token::Unit(SquareMeter)),
              "km" | "kilometer" | "kilometers" | "kilometre" | "kilometres" => tokens.push(Token::Unit(SquareKilometer)),
              "in" | "inch" | "inches" => tokens.push(Token::Unit(SquareInch)),
              "ft" | "foot" | "feet" => tokens.push(Token::Unit(SquareFoot)),
              "yd" | "yard" | "yards" => tokens.push(Token::Unit(SquareYard)),
              "mi" | "mile" | "miles" => tokens.push(Token::Unit(SquareMile)),
              _ => return Err(format!("Invalid string: {} {}", word_str, word2_str)),
            }
          },
          "are" | "ares" => tokens.push(Token::Unit(Are)),
          "decare" | "decares" => tokens.push(Token::Unit(Decare)),
          "ha" | "hectare" | "hectares" => tokens.push(Token::Unit(Hectare)),
          "acre" | "acres" => tokens.push(Token::Unit(Acre)),

          "mm3" | "millimeter3" | "millimeters3" | "millimetre3" | "millimetres3" => tokens.push(Token::Unit(CubicMillimeter)),
          "cm3" | "centimeter3" | "centimeters3" | "centimetre3" | "centimetres3" => tokens.push(Token::Unit(CubicCentimeter)),
          "dm3" | "decimeter3" | "decimeters3" | "decimetre3" | "decimetres3" => tokens.push(Token::Unit(CubicDecimeter)),
          "m3" | "meter3" | "meters3" | "metre3" | "metres3" => tokens.push(Token::Unit(CubicMeter)),
          "km3" | "kilometer3" | "kilometers3" | "kilometre3" | "kilometres3" => tokens.push(Token::Unit(CubicKilometer)),
          "inc3" | "inch3" | "inches3" => tokens.push(Token::Unit(CubicInch)),
          "ft3" | "foot3" | "feet3" => tokens.push(Token::Unit(CubicFoot)),
          "yd3" | "yard3" | "yards3" => tokens.push(Token::Unit(CubicYard)),
          "mi3" | "mile3" | "miles3" => tokens.push(Token::Unit(CubicMile)),
          "cubic" => {
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => return Err(format!("Invalid string: {}", word_str)),
            }

            let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
            grapheme_counter = new_grapheme_counter;
            let word2_str = word2.as_str();
            match word2_str {
              "millimeter" | "millimeters" | "millimetre" | "millimetres" => tokens.push(Token::Unit(CubicMillimeter)),
              "centimeter" | "centimeters" | "centimetre" | "centimetres" => tokens.push(Token::Unit(CubicCentimeter)),
              "decimeter" | "decimeters" | "decimetre" | "decimetres" => tokens.push(Token::Unit(CubicDecimeter)),
              "meter" | "meters" | "metre" | "metres" => tokens.push(Token::Unit(CubicMeter)),
              "kilometer" | "kilometers" | "kilometre" | "kilometres" => tokens.push(Token::Unit(CubicKilometer)),
              "inch" | "inches" => tokens.push(Token::Unit(CubicInch)),
              "foot" | "feet" => tokens.push(Token::Unit(CubicFoot)),
              "yard" | "yards" => tokens.push(Token::Unit(CubicYard)),
              "mile" | "miles" => tokens.push(Token::Unit(CubicMile)),
              _ => return Err(format!("Invalid string: {} {}", word_str, word2_str)),
            }
          },
          "ml" | "milliliter" | "milliliters" | "millilitre" | "millilitres" => tokens.push(Token::Unit(Milliliter)),
          "cl" | "centiliter" | "centiliters" | "centilitre" | "centilitres" => tokens.push(Token::Unit(Centiliter)),
          "dl" | "deciliter" | "deciliters" | "decilitre" | "decilitres" => tokens.push(Token::Unit(Deciliter)),
          "l" | "liter" | "liters" | "litre" | "litres" => tokens.push(Token::Unit(Liter)),
          "ts" | "tsp" | "tspn" | "tspns" | "teaspoon" | "teaspoons" => tokens.push(Token::Unit(Teaspoon)),
          "tbs" | "tbsp" | "tablespoon" | "tablespoons" => tokens.push(Token::Unit(Tablespoon)),
          "floz" => tokens.push(Token::Unit(FluidOunce)),
          "fl" | "fluid" => {
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => return Err(format!("Invalid string: {}", word_str)),
            }

            let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
            grapheme_counter = new_grapheme_counter;
            let word2_str = word2.as_str();
            match word2_str {
              "oz" | "ounce" | "ounces" => tokens.push(Token::Unit(FluidOunce)),
              _ => return Err(format!("Invalid string: {} {}", word_str, word2_str)),
            }
          },
          "cup" | "cups" => tokens.push(Token::Unit(Cup)),
          "pt" | "pint" | "pints" => tokens.push(Token::Unit(Pint)),
          "qt" | "quart" | "quarts" => tokens.push(Token::Unit(Quart)),
          "gal" | "gallon" | "gallons" => tokens.push(Token::Unit(Gallon)),
          "bbl" => tokens.push(Token::Unit(OilBarrel)),
          "oil" => {
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => return Err(format!("Invalid string: {}", word_str)),
            }

            let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
            grapheme_counter = new_grapheme_counter;
            let word2_str = word2.as_str();
            match word2_str {
              "barrel" | "barrels" => tokens.push(Token::Unit(OilBarrel)),
              _ => return Err(format!("Invalid string: {} {}", word_str, word2_str)),
            }
          },

          "mg" | "milligram" | "milligrams" => tokens.push(Token::Unit(Milligram)),
          "g" | "gram" | "grams" => tokens.push(Token::Unit(Gram)),
          "hectogram" | "hectograms" => tokens.push(Token::Unit(Hectogram)),
          "kg" | "kilo" | "kilos" | "kilogram" | "kilograms" => tokens.push(Token::Unit(Kilogram)),
          "t" | "tonne" | "tonnes" => tokens.push(Token::Unit(MetricTon)),
          "oz" | "ounces" => tokens.push(Token::Unit(Ounce)),
          "lb" | "lbs" => tokens.push(Token::Unit(Pound)),
          "pound" | "pounds" => {
            let mut check_for_next_word = true;
            if grapheme_counter < (graphemes.len() - 1) {
              let next_grapheme = graphemes[grapheme_counter + 1];
              if next_grapheme == "-" {
                grapheme_counter += 2;
              } else {
                tokens.push(Token::Unit(Pound));
                check_for_next_word = false;
              }
            } else {
              tokens.push(Token::Unit(Pound));
              check_for_next_word = false;
            }

            if check_for_next_word {
              let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
              let word2_str = word2.as_str();
              match word2_str {
                "force" => {
                  grapheme_counter = new_grapheme_counter;
                  tokens.push(Token::LexerKeyword(PoundForce));
                },
                _ => tokens.push(Token::Unit(Pound)),
              }
            }
          },
          "stone" | "stones" => tokens.push(Token::Unit(Stone)),
          "st" | "ton" | "tons" => tokens.push(Token::Unit(ShortTon)),
          "short" => {
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => return Err(format!("Invalid string: {}", word_str)),
            }

            let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
            grapheme_counter = new_grapheme_counter;
            let word2_str = word2.as_str();
            match word2_str {
              "ton" | "tons" | "tonne" | "tonnes" => tokens.push(Token::Unit(ShortTon)),
              _ => return Err(format!("Invalid string: {} {}", word_str, word2_str)),
            }
          },
          "lt" => tokens.push(Token::Unit(LongTon)),
          "long" => {
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => return Err(format!("Invalid string: {}", word_str)),
            }

            let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
            grapheme_counter = new_grapheme_counter;
            let word2_str = word2.as_str();
            match word2_str {
              "ton" | "tons" | "tonne" | "tonnes" => tokens.push(Token::Unit(LongTon)),
              _ => return Err(format!("Invalid string: {} {}", word_str, word2_str)),
            }
          },

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
          "kj" | "kilojoule" | "kilojoules" => tokens.push(Token::Unit(Kilojoule)),
          "mj" | "megajoule" | "megajoules" => tokens.push(Token::Unit(Megajoule)),
          "gj" | "gigajoule" | "gigajoules" => tokens.push(Token::Unit(Gigajoule)),
          "tj" | "terajoule" | "terajoules" => tokens.push(Token::Unit(Terajoule)),
          "cal" | "calorie" | "calories" => tokens.push(Token::Unit(Calorie)),
          "kcal" | "kilocalorie" | "kilocalories" => tokens.push(Token::Unit(KiloCalorie)),
          "btu" => tokens.push(Token::Unit(BritishThermalUnit)),
          "british" => {
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => return Err(format!("Invalid string: {}", word_str)),
            }

            let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
            grapheme_counter = new_grapheme_counter;
            let word2_str = word2.as_str();
            match word2_str {
              "thermal" => {},
              _ => return Err(format!("Invalid string: {} {}", word_str, word2_str)),
            }

            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => return Err(format!("Invalid string: {} {}", word_str, word2_str)),
            }

            let (new_grapheme_counter, word3) = next_word(grapheme_counter.to_owned());
            grapheme_counter = new_grapheme_counter;
            let word3_str = word3.as_str();
            match word3_str {
              "unit" | "units" => tokens.push(Token::Unit(BritishThermalUnit)),
              _ => return Err(format!("Invalid string: {} {} {}", word_str, word2_str, word3_str)),
            }
          },

          "nm" => tokens.push(Token::Unit(NewtonMeter)),
          "newton" => {
            let mut separated_by_dash = false;
            if grapheme_counter < (graphemes.len() - 1) {
              let next_grapheme = graphemes[grapheme_counter + 1];
              if next_grapheme == "-" {
                grapheme_counter += 2;
                separated_by_dash = true;
              } else if ! next_grapheme.trim().is_empty() {
                return Err(format!("Invalid string: {}{}", word_str, next_grapheme));
              } else {
                while (grapheme_counter + 1) < graphemes.len() {
                  grapheme_counter += 1;
                  let next_grapheme = graphemes[grapheme_counter];
                  if ! next_grapheme.trim().is_empty() {
                    break;
                  }
                }
              }
            } else {
              return Err(format!("Invalid string: {}", word_str));
            }

            let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
            grapheme_counter = new_grapheme_counter;
            let word2_str = word2.as_str();
            match word2_str {
              "meter" | "meters" | "metre" | "metres" => tokens.push(Token::Unit(NewtonMeter)),
              _ => {
                if separated_by_dash {
                  return Err(format!("Invalid string: {}-{}", word_str, word2_str));
                } else {
                  return Err(format!("Invalid string: {} {}", word_str, word2_str));
                }
              },
            }
          },

          // TODO: milliwatt hours?
          "wh" => tokens.push(Token::Unit(WattHour)),
          "watt" => {
            let mut check_for_next_word = true;
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => {
                tokens.push(Token::Unit(Watt));
                check_for_next_word = false;
              },
            }

            if check_for_next_word {
              let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
              let word2_str = word2.as_str();
              match word2_str {
                "hr" | "hrs" | "hour" | "hours" => {
                  grapheme_counter = new_grapheme_counter;
                  tokens.push(Token::Unit(WattHour));
                },
                _ => tokens.push(Token::Unit(Watt)),
              }
            }
          },
          "kwh" => tokens.push(Token::Unit(KilowattHour)),
          "kilowatt" => {
            let mut check_for_next_word = true;
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => {
                tokens.push(Token::Unit(Kilowatt));
                check_for_next_word = false;
              },
            }

            if check_for_next_word {
              let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
              let word2_str = word2.as_str();
              match word2_str {
                "hr" | "hrs" | "hour" | "hours" => {
                  grapheme_counter = new_grapheme_counter;
                  tokens.push(Token::Unit(KilowattHour));
                },
                _ => tokens.push(Token::Unit(Kilowatt)),
              }
            }
          },
          "mwh" => tokens.push(Token::Unit(MegawattHour)),
          "megawatt" => {
            let mut check_for_next_word = true;
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => {
                tokens.push(Token::Unit(Megawatt));
                check_for_next_word = false;
              },
            }

            if check_for_next_word {
              let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
              let word2_str = word2.as_str();
              match word2_str {
                "hr" | "hrs" | "hour" | "hours" => {
                  grapheme_counter = new_grapheme_counter;
                  tokens.push(Token::Unit(MegawattHour));
                },
                _ => tokens.push(Token::Unit(Megawatt)),
              }
            }
          },
          "gwh" => tokens.push(Token::Unit(GigawattHour)),
          "gigawatt" => {
            let mut check_for_next_word = true;
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => {
                tokens.push(Token::Unit(Gigawatt));
                check_for_next_word = false;
              },
            }

            if check_for_next_word {
              let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
              let word2_str = word2.as_str();
              match word2_str {
                "hr" | "hrs" | "hour" | "hours" => {
                  grapheme_counter = new_grapheme_counter;
                  tokens.push(Token::Unit(GigawattHour));
                },
                _ => tokens.push(Token::Unit(Gigawatt)),
              }
            }
          },
          "twh" => tokens.push(Token::Unit(TerawattHour)),
          "terawatt" => {
            let mut check_for_next_word = true;
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => {
                tokens.push(Token::Unit(Terawatt));
                check_for_next_word = false;
              },
            }

            if check_for_next_word {
              let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
              let word2_str = word2.as_str();
              match word2_str {
                "hr" | "hrs" | "hour" | "hours" => {
                  grapheme_counter = new_grapheme_counter;
                  tokens.push(Token::Unit(TerawattHour));
                },
                _ => tokens.push(Token::Unit(Terawatt)),
              }
            }
          },
          "pwh" => tokens.push(Token::Unit(PetawattHour)),
          "petawatt" => {
            let mut check_for_next_word = true;
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => {
                tokens.push(Token::Unit(Petawatt));
                check_for_next_word = false;
              },
            }

            if check_for_next_word {
              let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
              let word2_str = word2.as_str();
              match word2_str {
                "hr" | "hrs" | "hour" | "hours" => {
                  grapheme_counter = new_grapheme_counter;
                  tokens.push(Token::Unit(PetawattHour));
                },
                _ => tokens.push(Token::Unit(Petawatt)),
              }
            }
          },

          "milliwatt" | "milliwatts" => tokens.push(Token::Unit(Milliwatt)),
          "w" | "watts" => tokens.push(Token::Unit(Watt)),
          "kw" | "kilowatts" => tokens.push(Token::Unit(Kilowatt)),
          "mw" | "megawatts" => tokens.push(Token::Unit(Megawatt)),
          "gw" | "gigawatts" => tokens.push(Token::Unit(Gigawatt)),
          "tw" | "terawatts" => tokens.push(Token::Unit(Terawatt)),
          "pw" | "petawatts" => tokens.push(Token::Unit(Petawatt)),
          "hp" | "hps" | "horsepower" | "horsepowers" => tokens.push(Token::Unit(Horsepower)),
          "mhp" | "hpm" => tokens.push(Token::Unit(MetricHorsepower)),

          "ma" | "milliamp" | "milliamps" | "milliampere" | "milliamperes" => tokens.push(Token::Unit(Milliampere)),
          "a" | "amp" | "amps" | "ampere" | "amperes" => tokens.push(Token::Unit(Ampere)),
          "ka" | "kiloamp" | "kiloamps" | "kiloampere" | "kiloamperes" => tokens.push(Token::Unit(Kiloampere)),
          "bi" | "biot" | "biots" | "aba" | "abampere" | "abamperes" => tokens.push(Token::Unit(Abampere)),

          "mΩ" | "mΩ" | "milliohm" | "milliohms" => tokens.push(Token::Unit(Milliohm)),
          "Ω" | "Ω" | "ohm" | "ohms" => tokens.push(Token::Unit(Ohm)),
          "kΩ" | "kΩ" | "kiloohm" | "kiloohms" => tokens.push(Token::Unit(Kiloohm)),

          "mv" | "millivolt" | "millivolts" => tokens.push(Token::Unit(Millivolt)),
          "v" | "volt" | "volts" => tokens.push(Token::Unit(Volt)),
          "kv" | "kilovolt" | "kilovolts" => tokens.push(Token::Unit(Kilovolt)),

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

          "hz" | "hertz" => tokens.push(Token::Unit(Hertz)),
          "khz" | "kilohertz" => tokens.push(Token::Unit(Kilohertz)),
          "mhz" | "megahertz" => tokens.push(Token::Unit(Megahertz)),
          "ghz" | "gigahertz" => tokens.push(Token::Unit(Gigahertz)),
          "thz" | "terahertz" => tokens.push(Token::Unit(Terahertz)),
          "phz" | "petahertz" => tokens.push(Token::Unit(Petahertz)),
          "rpm" => tokens.push(Token::Unit(RevolutionsPerMinute)),
          "r" | "rev" | "revolution" | "revolutions" => tokens.push(Token::LexerKeyword(Revolution)),

          "kph" | "kmh" => tokens.push(Token::Unit(KilometersPerHour)),
          "mps" => tokens.push(Token::Unit(MetersPerSecond)),
          "mph" => tokens.push(Token::Unit(MilesPerHour)),
          "fps" => tokens.push(Token::Unit(FeetPerSecond)),
          "kn" | "kt" | "knot" | "knots" => tokens.push(Token::Unit(Knot)),

          "k" | "kelvin" | "kelvins" => tokens.push(Token::Unit(Kelvin)),
          "c" | "celsius" => tokens.push(Token::Unit(Celsius)),
          "f" | "fahrenheit" | "fahrenheits" => tokens.push(Token::Unit(Fahrenheit)),
          "deg" | "degree" | "degrees" => tokens.push(Token::Unit(default_degree)),

          "metric" => {
            match skip_whitespace(grapheme_counter.to_owned()) {
              Ok(new_grapheme_counter) => grapheme_counter = new_grapheme_counter,
              Err(_e) => return Err(format!("Invalid string: {}", word_str)),
            }

            let (new_grapheme_counter, word2) = next_word(grapheme_counter.to_owned());
            grapheme_counter = new_grapheme_counter;
            let word2_str = word2.as_str();
            match word2_str {
              "ton" | "tons" | "tonne" | "tonnes" => tokens.push(Token::Unit(MetricTon)),
              "hp" | "hps" | "horsepower" | "horsepowers" => tokens.push(Token::Unit(MetricHorsepower)),
              _ => return Err(format!("Invalid string: {} {}", word_str, word2_str)),
            }
          },

          _ => {
            return Err(format!("Invalid string: {}", word_str));
          },
        }
      },
      value if is_str_numeric(&value) => {
        let mut number_string = value.to_string();
        while (grapheme_counter + 1) < graphemes.len() {
          let next_grapheme = &graphemes[grapheme_counter + 1];
          if is_str_numeric(next_grapheme) {
            number_string.push_str(next_grapheme);
            grapheme_counter += 1;
          } else {
            break;
          }
        }

        d128::set_status(decimal::Status::empty());
        match d128::from_str(&number_string) {
          Ok(number) => {
            if d128::get_status().is_empty() {
              tokens.push(Token::Number(number));
            } else {
              return Err(format!("Error lexing d128 number: {}", number_string));
            }
          },
          Err(_e) => {
            return Err(format!("Error lexing d128 number: {}", number_string));
          },
        }
      },
      _ => {
        return Err(format!("Invalid character: {}", grapheme));
      },
    }
    grapheme_counter += 1;
  }

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

  if tokens.len() == 0 {
    return Err(format!("Input was empty"))
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
          Some(Token::LexerKeyword(PercentChar)) => {
            // "10%%" should be a percentage
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
        // lbs/sqin
        (Token::LexerKeyword(PoundForce), Token::LexerKeyword(Per), Token::Unit(SquareInch)) => {
          tokens[token_index-2] = Token::Unit(PoundsPerSquareInch);
        },
        // inch of mercury
        (Token::Unit(Inch), Token::TextOperator(Of), Token::LexerKeyword(Mercury)) => {
          tokens[token_index-2] = Token::Unit(InchOfMercury);
        },
        // revolutions per minute
        (Token::LexerKeyword(Revolution), Token::LexerKeyword(Per), Token::Unit(Minute)) => {
          tokens[token_index-2] = Token::Unit(RevolutionsPerMinute);
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::numtok;
  use regex::Regex;

  #[test]
  fn test_lex() {
    let strip_operator_spacing = Regex::new(r" ([+\-*/]) ").unwrap();
    let strip_afterdigit_spacing = Regex::new(r"(\d) ").unwrap();

    let run_lex = |input: &str, expected_tokens: Vec<Token>| {
      let tokens = lex(input, false, Unit::Celsius).unwrap();
      let matching_tokens = tokens.iter().zip(&expected_tokens).filter(|&(a, b)| a == b);
      assert_eq!(matching_tokens.count(), expected_tokens.len());

      // Prove we can handle multiple spaces wherever we handle a single space
      let input_extra_spaces = input.replace(" ", "   ");
      let tokens_extra_spaces = lex(&input_extra_spaces, false, Unit::Celsius).unwrap();
      let matching_tokens_extra_spaces = tokens_extra_spaces.iter().zip(&expected_tokens).filter(|&(a, b)| a == b);
      assert_eq!(matching_tokens_extra_spaces.count(), expected_tokens.len());

      // Prove we don't need spaces around operators
      let input_stripped_spaces = strip_operator_spacing.replace_all(input, "$1");
      let tokens_stripped_spaces = lex(&input_stripped_spaces, false, Unit::Celsius).unwrap();
      let matching_tokens_stripped_spaces = tokens_stripped_spaces.iter().zip(&expected_tokens).filter(|&(a, b)| a == b);
      assert_eq!(matching_tokens_stripped_spaces.count(), expected_tokens.len());

      // Prove we don't need a space after a digit
      let input_afterdigit_stripped_spaces = strip_afterdigit_spacing.replace_all(input, "$1");
      let tokens_afterdigit_stripped_spaces = lex(&input_afterdigit_stripped_spaces, false, Unit::Celsius).unwrap();
      let matching_tokens_afterdigit_stripped_spaces = tokens_afterdigit_stripped_spaces.iter().zip(&expected_tokens).filter(|&(a, b)| a == b);
      assert_eq!(matching_tokens_afterdigit_stripped_spaces.count(), expected_tokens.len());
    };

    run_lex("88 kilometres * 2", vec![numtok!(88), Token::Unit(Kilometer), Token::Operator(Multiply), numtok!(2)]);
    run_lex("100 nmi", vec![numtok!(100), Token::Unit(NauticalMile)]);
    run_lex("101 nautical miles", vec![numtok!(101), Token::Unit(NauticalMile)]);
    run_lex("2 lightyears", vec![numtok!(2), Token::Unit(LightYear)]);
    run_lex("1 light year", vec![numtok!(1), Token::Unit(LightYear)]);
    run_lex("10 lightsec", vec![numtok!(10), Token::Unit(LightSecond)]);
    run_lex("12 light secs", vec![numtok!(12), Token::Unit(LightSecond)]);
    run_lex("33.3 square meters", vec![numtok!(33.3), Token::Unit(SquareMeter)]);
    run_lex("54 m2", vec![numtok!(54), Token::Unit(SquareMeter)]);
    run_lex("87 sq miles", vec![numtok!(87), Token::Unit(SquareMile)]);
    run_lex("500 feet2", vec![numtok!(500), Token::Unit(SquareFoot)]);
    run_lex("500 feet²", vec![numtok!(500), Token::Unit(SquareFoot)]);
    run_lex("4 cubic metres", vec![numtok!(4), Token::Unit(CubicMeter)]);
    run_lex("34 cubic feet + 23 cubic yards", vec![numtok!(34), Token::Unit(CubicFoot), Token::Operator(Plus), numtok!(23), Token::Unit(CubicYard)]);
    run_lex("66 inches3 + 65 millimetre³", vec![numtok!(66), Token::Unit(CubicInch), Token::Operator(Plus), numtok!(65), Token::Unit(CubicMillimeter)]);
    run_lex("66 inches³ + 65 millimetre3", vec![numtok!(66), Token::Unit(CubicInch), Token::Operator(Plus), numtok!(65), Token::Unit(CubicMillimeter)]);
    run_lex("42 millilitres", vec![numtok!(42), Token::Unit(Milliliter)]);
    run_lex("3 tbs", vec![numtok!(3), Token::Unit(Tablespoon)]);
    run_lex("6 floz", vec![numtok!(6), Token::Unit(FluidOunce)]);
    run_lex("6 fl oz", vec![numtok!(6), Token::Unit(FluidOunce)]);
    run_lex("6 fluid ounces", vec![numtok!(6), Token::Unit(FluidOunce)]);
    run_lex("3 oil barrels", vec![numtok!(3), Token::Unit(OilBarrel)]);
    run_lex("67 kg", vec![numtok!(67), Token::Unit(Kilogram)]);
    run_lex("34 oz", vec![numtok!(34), Token::Unit(Ounce)]);
    run_lex("34 ounces", vec![numtok!(34), Token::Unit(Ounce)]);
    run_lex("210 lb", vec![numtok!(210), Token::Unit(Pound)]);
    run_lex("210 lbs", vec![numtok!(210), Token::Unit(Pound)]);
    run_lex("210 pound", vec![numtok!(210), Token::Unit(Pound)]);
    run_lex("210 pounds", vec![numtok!(210), Token::Unit(Pound)]);
    run_lex("210 pounds-force", vec![numtok!(210), Token::LexerKeyword(PoundForce)]);
    run_lex("3 ton", vec![numtok!(3), Token::Unit(ShortTon)]);
    run_lex("3 short tons", vec![numtok!(3), Token::Unit(ShortTon)]);
    run_lex("4 lt", vec![numtok!(4), Token::Unit(LongTon)]);
    run_lex("4 long tonnes", vec![numtok!(4), Token::Unit(LongTon)]);
    run_lex("234 wh", vec![numtok!(234), Token::Unit(WattHour)]);
    run_lex("1 watt", vec![numtok!(1), Token::Unit(Watt)]);
    run_lex("1 watt hour", vec![numtok!(1), Token::Unit(WattHour)]);
    run_lex("2 watts + 3 watts", vec![numtok!(2), Token::Unit(Watt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
    run_lex("2.3 kwh", vec![numtok!(2.3), Token::Unit(KilowattHour)]);
    run_lex("1 kilowatt", vec![numtok!(1), Token::Unit(Kilowatt)]);
    run_lex("1 kilowatt hour", vec![numtok!(1), Token::Unit(KilowattHour)]);
    run_lex("2 kilowatts + 3 watts", vec![numtok!(2), Token::Unit(Kilowatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
    run_lex("6.6 watts + 4 kilowatts", vec![numtok!(6.6), Token::Unit(Watt), Token::Operator(Plus), numtok!(4), Token::Unit(Kilowatt)]);
    run_lex("2.3 mwh", vec![numtok!(2.3), Token::Unit(MegawattHour)]);
    run_lex("1 megawatt", vec![numtok!(1), Token::Unit(Megawatt)]);
    run_lex("1 megawatt hour", vec![numtok!(1), Token::Unit(MegawattHour)]);
    run_lex("2 megawatts + 3 watts", vec![numtok!(2), Token::Unit(Megawatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
    run_lex("6.6 watts + 4 megawatts", vec![numtok!(6.6), Token::Unit(Watt), Token::Operator(Plus), numtok!(4), Token::Unit(Megawatt)]);
    run_lex("88 mw * 3", vec![numtok!(88), Token::Unit(Megawatt), Token::Operator(Multiply), numtok!(3)]);
    run_lex("999 kb", vec![numtok!(999), Token::Unit(Kilobyte)]);
    run_lex("200 gb - 100 mb", vec![numtok!(200), Token::Unit(Gigabyte), Token::Operator(Minus), numtok!(100), Token::Unit(Megabyte)]);
    run_lex("999 kib", vec![numtok!(999), Token::Unit(Kibibyte)]);
    run_lex("200 gib - 100 mib", vec![numtok!(200), Token::Unit(Gibibyte), Token::Operator(Minus), numtok!(100), Token::Unit(Mebibyte)]);
    run_lex("45 btu", vec![numtok!(45), Token::Unit(BritishThermalUnit)]);
    run_lex("46 british thermal units", vec![numtok!(46), Token::Unit(BritishThermalUnit)]);
    run_lex("5432 newton metres", vec![numtok!(5432), Token::Unit(NewtonMeter)]);
    run_lex("2345 newton-meters", vec![numtok!(2345), Token::Unit(NewtonMeter)]);
    run_lex("20 lbf", vec![numtok!(20), Token::LexerKeyword(PoundForce)]);
    run_lex("60 hz", vec![numtok!(60), Token::Unit(Hertz)]);
    run_lex("1100 rpm", vec![numtok!(1100), Token::Unit(RevolutionsPerMinute)]);
    run_lex("1150 revolutions per minute", vec![numtok!(1150), Token::Unit(RevolutionsPerMinute)]);
    run_lex("1 revolution per min", vec![numtok!(1), Token::Unit(RevolutionsPerMinute)]);
    run_lex("4 revolution / mins", vec![numtok!(4), Token::Unit(RevolutionsPerMinute)]);
    run_lex("1250 r / min", vec![numtok!(1250), Token::Unit(RevolutionsPerMinute)]);
    run_lex("1300 rev / min", vec![numtok!(1300), Token::Unit(RevolutionsPerMinute)]);
    run_lex("1350 rev / minute", vec![numtok!(1350), Token::Unit(RevolutionsPerMinute)]);
    run_lex("1250 r per min", vec![numtok!(1250), Token::Unit(RevolutionsPerMinute)]);
    run_lex("1300 rev per min", vec![numtok!(1300), Token::Unit(RevolutionsPerMinute)]);
    run_lex("1350 rev per minute", vec![numtok!(1350), Token::Unit(RevolutionsPerMinute)]);
    run_lex("100 kph", vec![numtok!(100), Token::Unit(KilometersPerHour)]);
    run_lex("100 kmh", vec![numtok!(100), Token::Unit(KilometersPerHour)]);
    run_lex("100 kilometers per hour", vec![numtok!(100), Token::Unit(KilometersPerHour)]);
    run_lex("100 kilometre / hrs", vec![numtok!(100), Token::Unit(KilometersPerHour)]);
    run_lex("3.6 mps", vec![numtok!(3.6), Token::Unit(MetersPerSecond)]);
    run_lex("3.6 meters per second", vec![numtok!(3.6), Token::Unit(MetersPerSecond)]);
    run_lex("3.6 metre / secs", vec![numtok!(3.6), Token::Unit(MetersPerSecond)]);
    run_lex("60 mph", vec![numtok!(60), Token::Unit(MilesPerHour)]);
    run_lex("60 miles per hour", vec![numtok!(60), Token::Unit(MilesPerHour)]);
    run_lex("60 mile / hr", vec![numtok!(60), Token::Unit(MilesPerHour)]);
    run_lex("35 fps", vec![numtok!(35), Token::Unit(FeetPerSecond)]);
    run_lex("35 ft / sec", vec![numtok!(35), Token::Unit(FeetPerSecond)]);
    run_lex("35 ft per seconds", vec![numtok!(35), Token::Unit(FeetPerSecond)]);
    run_lex("35 foot / secs", vec![numtok!(35), Token::Unit(FeetPerSecond)]);
    run_lex("35 foot per seconds", vec![numtok!(35), Token::Unit(FeetPerSecond)]);
    run_lex("35 feet / sec", vec![numtok!(35), Token::Unit(FeetPerSecond)]);
    run_lex("35 feet per second", vec![numtok!(35), Token::Unit(FeetPerSecond)]);
    run_lex("30 pa", vec![numtok!(30), Token::Unit(Pascal)]);
    run_lex("23 celsius + 4 celsius", vec![numtok!(23), Token::Unit(Celsius), Token::Operator(Plus), numtok!(4), Token::Unit(Celsius)]);
    run_lex("54 f - 1.5 fahrenheit", vec![numtok!(54), Token::Unit(Fahrenheit), Token::Operator(Minus), numtok!(1.5), Token::Unit(Fahrenheit)]);
    run_lex("50 metric tonnes", vec![numtok!(50), Token::Unit(MetricTon)]);
    run_lex("77 metric hps", vec![numtok!(77), Token::Unit(MetricHorsepower)]);

    run_lex("100 + 99", vec![numtok!(100), Token::Operator(Plus), numtok!(99)]);
    run_lex("100 plus 99", vec![numtok!(100), Token::Operator(Plus), numtok!(99)]);
    run_lex("12 - 4", vec![numtok!(12), Token::Operator(Minus), numtok!(4)]);
    run_lex("12 minus 4", vec![numtok!(12), Token::Operator(Minus), numtok!(4)]);
    run_lex("50.5 * 2", vec![numtok!(50.5), Token::Operator(Multiply), numtok!(2)]);
    run_lex("50.5 times 2", vec![numtok!(50.5), Token::Operator(Multiply), numtok!(2)]);
    run_lex("50.5 multiplied by 2", vec![numtok!(50.5), Token::Operator(Multiply), numtok!(2)]);
    run_lex("6 / 3", vec![numtok!(6), Token::Operator(Divide), numtok!(3)]);
    run_lex("50 / 10", vec![numtok!(50), Token::Operator(Divide), numtok!(10)]);
    run_lex("6 divided by 3", vec![numtok!(6), Token::Operator(Divide), numtok!(3)]);
    run_lex("7 mod 5", vec![numtok!(7), Token::Operator(Modulo), numtok!(5)]);

    run_lex("(2 + 3) * 4", vec![Token::Operator(LeftParen), numtok!(2), Token::Operator(Plus), numtok!(3), Token::Operator(RightParen), Token::Operator(Multiply), numtok!(4)]);
    run_lex("52 weeks * (12 hrs + 12 hours)", vec![numtok!(52), Token::Unit(Week), Token::Operator(Multiply), Token::Operator(LeftParen), numtok!(12), Token::Unit(Hour), Token::Operator(Plus), numtok!(12), Token::Unit(Hour), Token::Operator(RightParen)]);
  }
}
