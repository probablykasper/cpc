use std::iter::Peekable;
use std::str::FromStr;
use decimal::d128;
use crate::Token;
use crate::Operator::{Caret, Divide, LeftParen, Minus, Modulo, Multiply, Plus, RightParen};
use crate::UnaryOperator::{Percent, Factorial};
use crate::TextOperator::{Of, To};
use crate::NamedNumber::*;
use crate::Constant::{E, Pi};
use crate::LexerKeyword::{In, PercentChar, Per, Mercury, Hg, PoundForce, Force, DoubleQuotes};
use crate::FunctionIdentifier::{Cbrt, Ceil, Cos, Exp, Abs, Floor, Ln, Log, Round, Sin, Sqrt, Tan};
use crate::units::Unit;
use crate::units::Unit::*;
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

pub const fn is_alphabetic_extended(input: &char) -> bool {
  match input {
    'A'..='Z' | 'a'..='z' | 'Ω' | 'Ω' | 'µ' | 'μ' | 'π' => true,
    _ => false,
  }
}

pub fn is_alphabetic_extended_str(input: &str) -> bool {
  let x = match input {
    value if value.chars().all(|c| ('a'..='z').contains(&c)) => true,
    value if value.chars().all(|c| ('A'..='Z').contains(&c)) => true,
    "Ω" | "Ω" | "µ" | "μ" | "π" => true,
    _ => false,
  };
  return x;
}

pub fn is_numeric_str(input: &str) -> bool {
  match input {
    "." => true,
    "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => true,
    _ => false,
  }
}

/// Read next characters as a word, otherwise return empty string.
/// Returns an empty string if there's leading whitespace.
pub fn read_word_plain(chars: &mut Peekable<Graphemes>) -> String {
  let mut word = "".to_string();
  while let Some(next_char) = chars.peek() {
    if is_alphabetic_extended_str(&next_char) {
      word += chars.next().unwrap();
    } else {
      break;
    }
  }
  return word;
}

/// Read next as a word, otherwise return empty string.
/// Leading whitespace is ignored. A trailing digit may be included.
pub fn read_word(chars: &mut Peekable<Graphemes>) -> String {
  // skip whitespace
  while let Some(current_char) = chars.peek() {
    if current_char.trim().is_empty() {
    chars.next();
    } else {
      break;
    }
  }
  let mut word = "".to_string();
  while let Some(next_char) = chars.peek() {
    if is_alphabetic_extended_str(&next_char) {
      word += chars.next().unwrap();
    } else {
      break;
    }
  }
  match *chars.peek().unwrap_or(&"") {
    "2" | "²" => {
      word += "2";
      chars.next();
    },
    "3" | "³" => {
      word += "3";
      chars.next();
    },
    _ => {},
  }
  return word;
}

pub fn parse_word(tokens: &mut Vec<Token>, chars: &mut Peekable<Graphemes>, default_degree: Unit) -> Result<(), String> {
  let token = match read_word(chars).as_str() {
    "to" => Token::TextOperator(To),
    "of" => Token::TextOperator(Of),

    "hundred" => Token::NamedNumber(Hundred),
    "thousand" => Token::NamedNumber(Thousand),
    "mil" | "mill" | "million" => Token::NamedNumber(Million),
    "bil" | "bill" | "billion" => Token::NamedNumber(Billion),
    "tri" | "tril" | "trillion" => Token::NamedNumber(Trillion),
    "quadrillion" => Token::NamedNumber(Quadrillion),
    "quintillion" => Token::NamedNumber(Quintillion),
    "sextillion" => Token::NamedNumber(Sextillion),
    "septillion" => Token::NamedNumber(Septillion),
    "octillion" => Token::NamedNumber(Octillion),
    "nonillion" => Token::NamedNumber(Nonillion),
    "decillion" => Token::NamedNumber(Decillion),
    "undecillion" => Token::NamedNumber(Undecillion),
    "duodecillion" => Token::NamedNumber(Duodecillion),
    "tredecillion" => Token::NamedNumber(Tredecillion),
    "quattuordecillion" => Token::NamedNumber(Quattuordecillion),
    "quindecillion" => Token::NamedNumber(Quindecillion),
    "sexdecillion" => Token::NamedNumber(Sexdecillion),
    "septendecillion" => Token::NamedNumber(Septendecillion),
    "octodecillion" => Token::NamedNumber(Octodecillion),
    "novemdecillion" => Token::NamedNumber(Novemdecillion),
    "vigintillion" => Token::NamedNumber(Vigintillion),
    "centillion" => Token::NamedNumber(Centillion),
    "googol" => Token::NamedNumber(Googol),

    "pi" => Token::Constant(Pi),
    "e" => Token::Constant(E),
  
    "mod" => Token::Operator(Modulo),

    "sqrt" => Token::FunctionIdentifier(Sqrt),
    "cbrt" => Token::FunctionIdentifier(Cbrt),

    "log" => Token::FunctionIdentifier(Log),
    "ln" => Token::FunctionIdentifier(Ln),
    "exp" => Token::FunctionIdentifier(Exp),

    "round" | "rint" => Token::FunctionIdentifier(Round),
    "ceil" => Token::FunctionIdentifier(Ceil),
    "floor" => Token::FunctionIdentifier(Floor),
    "abs" | "fabs" => Token::FunctionIdentifier(Abs),

    "sin" => Token::FunctionIdentifier(Sin),
    "cos" => Token::FunctionIdentifier(Cos),
    "tan" => Token::FunctionIdentifier(Tan),

    "per" => Token::LexerKeyword(Per),
    "hg" => Token::LexerKeyword(Hg), // can be hectogram or mercury

    "ns" | "nanosec" | "nanosecs" | "nanosecond" | "nanoseconds" => Token::Unit(Nanosecond),
    // µ and μ are two different characters
    "µs" | "μs" | "microsec" | "microsecs" | "microsecond" | "microseconds" => Token::Unit(Microsecond),
    "ms" | "millisec" | "millisecs" | "millisecond" | "milliseconds" => Token::Unit(Millisecond),
    "s" | "sec" | "secs" | "second" | "seconds" => Token::Unit(Second),
    "min" | "mins" | "minute" | "minutes" => Token::Unit(Minute),
    "h" | "hr" | "hrs" | "hour" | "hours" => Token::Unit(Hour),
    "day" | "days" => Token::Unit(Day),
    "wk" | "wks" | "week" | "weeks" => Token::Unit(Week),
    "mo" | "mos" | "month" | "months" => Token::Unit(Month),
    "q" | "quarter" | "quarters" => Token::Unit(Quarter),
    "yr" | "yrs" | "year" | "years" => Token::Unit(Year),
    "decade" | "decades" => Token::Unit(Decade),
    "century" | "centuries" => Token::Unit(Century),
    "millenium" | "millenia" | "milleniums" => Token::Unit(Millenium),

    "mm" | "millimeter" | "millimeters" | "millimetre" | "millimetres" => Token::Unit(Millimeter),
    "cm" | "centimeter" | "centimeters" | "centimetre" | "centimetres" => Token::Unit(Centimeter),
    "dm" | "decimeter" | "decimeters" | "decimetre" | "decimetres" => Token::Unit(Decimeter),
    "m" | "meter" | "meters" | "metre" | "metres" => Token::Unit(Meter),
    "km" | "kilometer" | "kilometers" | "kilometre" | "kilometres" => Token::Unit(Kilometer),
    "in" => Token::LexerKeyword(In),
    "inch" | "inches" => Token::Unit(Inch),
    "ft" | "foot" | "feet" => Token::Unit(Foot),
    "yd" | "yard" | "yards" => Token::Unit(Yard),
    "mi" | "mile" | "miles" => Token::Unit(Mile),
    "nmi" => Token::Unit(NauticalMile),
    "nautical" => {
      match read_word(chars).as_str() {
        "mile" | "miles" => Token::Unit(NauticalMile),
        string => return Err(format!("Invalid string: {}", string)),
      }
    },
    "ly" | "lightyear" | "lightyears" => Token::Unit(LightYear),
    "lightsec" | "lightsecs" | "lightsecond" | "lightseconds" => Token::Unit(LightSecond),
    "light" => {
      match read_word(chars).as_str() {
        "yr" | "yrs" | "year" | "years" => Token::Unit(LightYear),
        "sec" | "secs" | "second" | "seconds" => Token::Unit(LightYear),
        string => return Err(format!("Invalid string: {}", string)),
      }
    }

    "sqmm" | "mm2" | "millimeter2" | "millimeters2" | "millimetre2" | "millimetres2" => Token::Unit(SquareMillimeter),
    "sqcm" | "cm2" | "centimeter2" | "centimeters2" | "centimetre2" | "centimetres2" => Token::Unit(SquareCentimeter),
    "sqdm" | "dm2" | "decimeter2" | "decimeters2" | "decimetre2" | "decimetres2" => Token::Unit(SquareDecimeter),
    "sqm" | "m2" | "meter2" | "meters2" | "metre2" | "metres2" => Token::Unit(SquareMeter),
    "sqkm" | "km2" | "kilometer2" | "kilometers2" | "kilometre2" | "kilometres2" => Token::Unit(SquareKilometer),
    "sqin" | "in2" | "inch2" | "inches2" => Token::Unit(SquareInch),
    "sqft" | "ft2" | "foot2" | "feet2" => Token::Unit(SquareFoot),
    "sqyd" | "yd2" | "yard2" | "yards2" => Token::Unit(SquareYard),
    "sqmi" | "mi2" | "mile2" | "miles2" => Token::Unit(SquareMile),
    "sq" | "square" => {
      match read_word(chars).as_str() {
        "mm" | "millimeter" | "millimeters" | "millimetre" | "millimetres" => Token::Unit(SquareMillimeter),
        "cm" | "centimeter" | "centimeters" | "centimetre" | "centimetres" => Token::Unit(SquareCentimeter),
        "dm" | "decimeter" | "decimeters" | "decimetre" | "decimetres" => Token::Unit(SquareDecimeter),
        "m" | "meter" | "meters" | "metre" | "metres" => Token::Unit(SquareMeter),
        "km" | "kilometer" | "kilometers" | "kilometre" | "kilometres" => Token::Unit(SquareKilometer),
        "in" | "inch" | "inches" => Token::Unit(SquareInch),
        "ft" | "foot" | "feet" => Token::Unit(SquareFoot),
        "yd" | "yard" | "yards" => Token::Unit(SquareYard),
        "mi" | "mile" | "miles" => Token::Unit(SquareMile),
        string => return Err(format!("Invalid string: {}", string)),
      }
    }
    "are" | "ares" => Token::Unit(Are),
    "decare" | "decares" => Token::Unit(Decare),
    "ha" | "hectare" | "hectares" => Token::Unit(Hectare),
    "acre" | "acres" => Token::Unit(Acre),

    "mm3" | "millimeter3" | "millimeters3" | "millimetre3" | "millimetres3" => Token::Unit(CubicMillimeter),
    "cm3" | "centimeter3" | "centimeters3" | "centimetre3" | "centimetres3" => Token::Unit(CubicCentimeter),
    "dm3" | "decimeter3" | "decimeters3" | "decimetre3" | "decimetres3" => Token::Unit(CubicDecimeter),
    "m3" | "meter3" | "meters3" | "metre3" | "metres3" => Token::Unit(CubicMeter),
    "km3" | "kilometer3" | "kilometers3" | "kilometre3" | "kilometres3" => Token::Unit(CubicKilometer),
    "inc3" | "inch3" | "inches3" => Token::Unit(CubicInch),
    "ft3" | "foot3" | "feet3" => Token::Unit(CubicFoot),
    "yd3" | "yard3" | "yards3" => Token::Unit(CubicYard),
    "mi3" | "mile3" | "miles3" => Token::Unit(CubicMile),
    "cubic" => {
      match read_word(chars).as_str() {
        "mm" | "millimeter" | "millimeters" | "millimetre" | "millimetres" => Token::Unit(CubicMillimeter),
        "cm" | "centimeter" | "centimeters" | "centimetre" | "centimetres" => Token::Unit(CubicCentimeter),
        "dm" | "decimeter" | "decimeters" | "decimetre" | "decimetres" => Token::Unit(CubicDecimeter),
        "m" | "meter" | "meters" | "metre" | "metres" => Token::Unit(CubicMeter),
        "km" | "kilometer" | "kilometers" | "kilometre" | "kilometres" => Token::Unit(CubicKilometer),
        "in" | "inch" | "inches" => Token::Unit(CubicInch),
        "ft" | "foot" | "feet" => Token::Unit(CubicFoot),
        "yd" | "yard" | "yards" => Token::Unit(CubicYard),
        "mi" | "mile" | "miles" => Token::Unit(CubicMile),
        string => return Err(format!("Invalid string: {}", string)),
      }
    },
    "ml" | "milliliter" | "milliliters" | "millilitre" | "millilitres" => Token::Unit(Milliliter),
    "cl" | "centiliter" | "centiliters" | "centilitre" | "centilitres" => Token::Unit(Centiliter),
    "dl" | "deciliter" | "deciliters" | "decilitre" | "decilitres" => Token::Unit(Deciliter),
    "l" | "liter" | "liters" | "litre" | "litres" => Token::Unit(Liter),
    "ts" | "tsp" | "tspn" | "tspns" | "teaspoon" | "teaspoons" => Token::Unit(Teaspoon),
    "tbs" | "tbsp" | "tablespoon" | "tablespoons" => Token::Unit(Tablespoon),
    "floz" => Token::Unit(FluidOunce),
    "fl" | "fluid" => {
      match read_word(chars).as_str() {
        "oz" | "ounce" | "ounces" => Token::Unit(FluidOunce),
        string => return Err(format!("Invalid string: {}", string)),
      }
    },
    "cup" | "cups" => Token::Unit(Cup),
    "pt" | "pint" | "pints" => Token::Unit(Pint),
    "qt" | "quart" | "quarts" => Token::Unit(Quart),
    "gal" | "gallon" | "gallons" => Token::Unit(Gallon),
    "bbl" | "oil barrel" | "oil barrels" => Token::Unit(OilBarrel),
    "oil" => {
      match read_word(chars).as_str() {
        "barrel" | "barrels" => Token::Unit(OilBarrel),
        string => return Err(format!("Invalid string: {}", string)),
      }
    },

    "metric" => {
      match read_word(chars).as_str() {
        "ton" | "tons" | "tonne" | "tonnes" => Token::Unit(MetricTon),
        "hp" | "hps" | "horsepower" | "horsepowers" => Token::Unit(MetricHorsepower),
        string => return Err(format!("Invalid string: {}", string)),
      }
    },

    "mg" | "milligram" | "milligrams" => Token::Unit(Milligram),
    "g" | "gram" | "grams" => Token::Unit(Gram),
    "hectogram" | "hectograms" => Token::Unit(Hectogram),
    "kg" | "kilo" | "kilos" | "kilogram" | "kilograms" => Token::Unit(Kilogram),
    "t" | "tonne" | "tonnes" => Token::Unit(MetricTon),
    "oz" | "ounces" => Token::Unit(Ounce),
    "lb" | "lbs" => Token::Unit(Pound),
    "pound" | "pounds" => {
      todo!();
      // if chars.peek() == Some(&"-") {
      //   let dash_chars_iter = chars.clone();
      //   dash_chars_iter.next();
      //   match read_word_plain(dash_chars_iter).as_str() {
      //     "force" => {
            
      //     }
      //   }
      //   chars.next();
      //   match read_word_plain(chars).as_str() {
      //     "force" => Token::LexerKeyword(PoundForce),
      //     string => return Err(format!("Invalid string: {}", string)),
      //   }
      //   match read_word(chars).as_str() {
      //     "force" => Token::LexerKeyword(PoundForce),
      //     string => return Err(format!("Invalid string: {}", string)),
      //   }
      // } else {
      //   Token::Unit(Pound)
      // }
    },
    "stone" | "stones" => Token::Unit(Stone),
    "st" | "ton" | "tons" => Token::Unit(ShortTon),
    "short" => {
      match read_word(chars).as_str() {
        "ton" | "tons" | "tonne" | "tonnes" => Token::Unit(ShortTon),
        string => return Err(format!("Invalid string: {}", string)),
      }
    },
    "lt" => Token::Unit(LongTon),
    "long" => {
      match read_word(chars).as_str() {
        "ton" | "tons" | "tonne" | "tonnes" => Token::Unit(LongTon),
        string => return Err(format!("Invalid string: {}", string)),
      }
    },

    "bit" | "bits" => Token::Unit(Bit),
    "kbit" | "kilobit" | "kilobits" => Token::Unit(Kilobit),
    "mbit" | "megabit" | "megabits" => Token::Unit(Megabit),
    "gbit" | "gigabit" | "gigabits" => Token::Unit(Gigabit),
    "tbit" | "terabit" | "terabits" => Token::Unit(Terabit),
    "pbit" | "petabit" | "petabits" => Token::Unit(Petabit),
    "ebit" | "exabit" | "exabits" => Token::Unit(Exabit),
    "zbit" | "zettabit" | "zettabits" => Token::Unit(Zettabit),
    "ybit" | "yottabit" | "yottabits" => Token::Unit(Yottabit),
    "kibit" | "kibibit" | "kibibits" => Token::Unit(Kibibit),
    "mibit" | "mebibit" | "mebibits" => Token::Unit(Mebibit),
    "gibit" | "gibibit" | "gibibits" => Token::Unit(Gibibit),
    "tibit" | "tebibit" | "tebibits" => Token::Unit(Tebibit),
    "pibit" | "pebibit" | "pebibits" => Token::Unit(Pebibit),
    "eibit" | "exbibit" | "exbibits" => Token::Unit(Exbibit),
    "zibit" | "zebibit" | "zebibits" => Token::Unit(Zebibit),
    "yibit" | "yobibit" | "yobibits" => Token::Unit(Yobibit),
    "byte" | "bytes" => Token::Unit(Byte),
    "kb" | "kilobyte" | "kilobytes" => Token::Unit(Kilobyte),
    "mb" | "megabyte" | "megabytes" => Token::Unit(Megabyte),
    "gb" | "gigabyte" | "gigabytes" => Token::Unit(Gigabyte),
    "tb" | "terabyte" | "terabytes" => Token::Unit(Terabyte),
    "pb" | "petabyte" | "petabytes" => Token::Unit(Petabyte),
    "eb" | "exabyte" | "exabytes" => Token::Unit(Exabyte),
    "zb" | "zettabyte" | "zettabytes" => Token::Unit(Zettabyte),
    "yb" | "yottabyte" | "yottabytes" => Token::Unit(Yottabyte),
    "kib" | "kibibyte" | "kibibytes" => Token::Unit(Kibibyte),
    "mib" | "mebibyte" | "mebibytes" => Token::Unit(Mebibyte),
    "gib" | "gibibyte" | "gibibytes" => Token::Unit(Gibibyte),
    "tib" | "tebibyte" | "tebibytes" => Token::Unit(Tebibyte),
    "pib" | "pebibyte" | "pebibytes" => Token::Unit(Pebibyte),
    "eib" | "exbibyte" | "exbibytes" => Token::Unit(Exbibyte),
    "zib" | "zebibyte" | "zebibytes" => Token::Unit(Zebibyte),
    "yib" | "yobibyte" | "yobibytes" => Token::Unit(Yobibyte),

    "millijoule" | "millijoules" => Token::Unit(Millijoule),
    "j"| "joule" | "joules" => Token::Unit(Joule),
    "nm" => Token::Unit(NewtonMeter),
    "newton" => {
      todo!();
      // "-meter" | "-meters" | "metre" | "metres" => Token::Unit(NewtonMeter),
      // "meter" | "meters" | "metre" | "metres" => Token::Unit(NewtonMeter),
    },
    "kj" | "kilojoule" | "kilojoules" => Token::Unit(Kilojoule),
    "mj" | "megajoule" | "megajoules" => Token::Unit(Megajoule),
    "gj" | "gigajoule" | "gigajoules" => Token::Unit(Gigajoule),
    "tj" | "terajoule" | "terajoules" => Token::Unit(Terajoule),
    "cal" | "calorie" | "calories" => Token::Unit(Calorie),
    "kcal" | "kilocalorie" | "kilocalories" => Token::Unit(KiloCalorie),
    "btu" => Token::Unit(BritishThermalUnit),
    "british" => {
      match read_word(chars).as_str() {
        "thermal" => {
          match read_word(chars).as_str() {
            "unit" | "units" => Token::Unit(BritishThermalUnit),
            string => return Err(format!("Invalid string: {}", string)),
          }
        },
        string => return Err(format!("Invalid string: {}", string)),
      }
    },
    "wh" => Token::Unit(WattHour),
    "kwh" => Token::Unit(KilowattHour),
    "mwh" => Token::Unit(MegawattHour),
    "gwh" => Token::Unit(GigawattHour),
    "twh" => Token::Unit(TerawattHour),
    "pwh" => Token::Unit(PetawattHour),

    "milliwatt" | "milliwatts" => Token::Unit(Milliwatt),
    "w" | "watts" => Token::Unit(Watt),
    "kw" | "kilowatts" => Token::Unit(Kilowatt),
    "mw" | "megawatts" => Token::Unit(Megawatt),
    "gw" | "gigawatts" => Token::Unit(Gigawatt),
    "tw" | "terawatts" => Token::Unit(Terawatt),
    "pw" | "petawatts" => Token::Unit(Petawatt),
    "hp" | "hps" | "horsepower" | "horsepowers" => Token::Unit(Horsepower),
    "mhp" | "hpm" => Token::Unit(MetricHorsepower),

    "watt" => {
      match read_word(chars).as_str() {
        "hr" | "hrs" | "hour" | "hours" => Token::Unit(WattHour),
        _ => Token::Unit(Watt)
      }
    }
    "kilowatt" => {
      match read_word(chars).as_str() {
        "hr" | "hrs" | "hour" | "hours" => Token::Unit(KilowattHour),
        _ => Token::Unit(Kilowatt),
      }
    }
    "megawatt" => {
      match read_word(chars).as_str() {
        "hr" | "hrs" | "hour" | "hours" => Token::Unit(MegawattHour),
        _ => Token::Unit(Megawatt),
      }
    }
    "gigawatt" => {
      match read_word(chars).as_str() {
        "hr" | "hrs" | "hour" | "hours" => Token::Unit(GigawattHour),
        _ => Token::Unit(Gigawatt),
      }
    }
    "terawatt" => {
      match read_word(chars).as_str() {
        "hr" | "hrs" | "hour" | "hours" => Token::Unit(TerawattHour),
        _ => Token::Unit(Terawatt),
      }
    }
    "petawatt" => {
      match read_word(chars).as_str() {
        "hr" | "hrs" | "hour" | "hours" => Token::Unit(PetawattHour),
        _ => Token::Unit(Petawatt),
      }
    }

    "ma" | "milliamp" | "milliamps" | "milliampere" | "milliamperes" => Token::Unit(Milliampere),
    "a" | "amp" | "amps" | "ampere" | "amperes" => Token::Unit(Ampere),
    "ka" | "kiloamp" | "kiloamps" | "kiloampere" | "kiloamperes" => Token::Unit(Kiloampere),
    "bi" | "biot" | "biots" | "aba" | "abampere" | "abamperes" => Token::Unit(Abampere),

    "mΩ" | "mΩ" | "milliohm" | "milliohms" => Token::Unit(Milliohm),
    "Ω" | "Ω" | "ohm" | "ohms" => Token::Unit(Ohm),
    "kΩ" | "kΩ" | "kiloohm" | "kiloohms" => Token::Unit(Kiloohm),

    "mv" | "millivolt" | "millivolts" => Token::Unit(Millivolt),
    "v" | "volt" | "volts" => Token::Unit(Volt),
    "kv" | "kilovolt" | "kilovolts" => Token::Unit(Kilovolt),

    // for pound-force per square inch
    "lbf" => Token::LexerKeyword(PoundForce),
    "force" => Token::LexerKeyword(Force),

    "pa" | "pascal" | "pascals" => Token::Unit(Pascal),
    "kpa" | "kilopascal" | "kilopascals" => Token::Unit(Kilopascal),
    "atm" | "atms" | "atmosphere" | "atmospheres" => Token::Unit(Atmosphere),
    "mbar" | "mbars" | "millibar" | "millibars" => Token::Unit(Millibar),
    "bar" | "bars" => Token::Unit(Bar),
    "inhg" => Token::Unit(InchOfMercury),
    "mercury" => Token::LexerKeyword(Mercury),
    "psi" => Token::Unit(PoundsPerSquareInch),
    "torr" | "torrs" => Token::Unit(Torr),

    "hz" | "hertz" => Token::Unit(Hertz),
    "khz" | "kilohertz" => Token::Unit(Kilohertz),
    "mhz" | "megahertz" => Token::Unit(Megahertz),
    "ghz" | "gigahertz" => Token::Unit(Gigahertz),
    "thz" | "terahertz" => Token::Unit(Terahertz),
    "phz" | "petahertz" => Token::Unit(Petahertz),
    "rpm" | "r/min" | "rev/min" => Token::Unit(RevolutionsPerMinute),

    "kph" | "kmh" => Token::Unit(KilometersPerHour),
    "mps" => Token::Unit(MetersPerSecond),
    "mph" => Token::Unit(MilesPerHour),
    "fps" => Token::Unit(FeetPerSecond),
    "kn" | "kt" | "knot" | "knots" => Token::Unit(Knot),

    "k" | "kelvin" | "kelvins" => Token::Unit(Kelvin),
    "c" | "celsius" => Token::Unit(Celsius),
    "f" | "fahrenheit" | "fahrenheits" => Token::Unit(Fahrenheit),
    "deg" | "degree" | "degrees" => Token::Unit(default_degree),

    string => {
      return Err(format!("Invalid string: {}", string));
    }
  };
  tokens.push(token);
  return Ok(());
}

/// Lex an input string and returns [`Token`]s
pub fn lex(input: &str, allow_trailing_operators: bool, default_degree: Unit) -> Result<Vec<Token>, String> {

  let mut input = input.replace(",", ""); // ignore commas

  input = input.to_lowercase();

  if allow_trailing_operators {
    match &input.chars().last().unwrap_or('x') {
      '+' | '-' | '*' | '/' | '^' | '(' => {
        input.pop();
      },
      _ => {},
    }
  }

  let mut left_paren_count = 0;
  let mut right_paren_count = 0;

  let mut chars = UnicodeSegmentation::graphemes(input.as_str(), true).peekable();
  let mut tokens: Vec<Token> = vec![];

  while let Some(_) = chars.peek() {
    let current_char = chars.peek().unwrap();
    println!("1: {}", current_char);
    let token = match *current_char {
      value if value.trim().is_empty() => {
        chars.next();
        continue;
      },
      value if is_alphabetic_extended_str(&value) => {
        parse_word(&mut tokens, &mut chars, default_degree)?;
        continue;
      },
      value if is_numeric_str(value) => {
        let mut number_string = "".to_string();
        while let Some(number_char) = chars.peek() {
          if is_numeric_str(number_char) {
            number_string += number_char;
            chars.next();
          } else {
            break;
          }
        }
        d128::set_status(decimal::Status::empty());
        let token;
        match d128::from_str(&number_string) {
          Ok(number) => {
            if d128::get_status().is_empty() {
              token = Token::Number(number);
            } else {
              return Err(format!("Error lexing d128 number: {}", number_string));
            }
          },
          Err(_e) => {
            return Err(format!("Error lexing d128 number: {}", number_string));
          }
        };
        token
      },
      "+" => Token::Operator(Plus),
      "-" => Token::Operator(Minus),
      "*" => Token::Operator(Multiply),
      "/" => Token::Operator(Divide),
      "%" => Token::LexerKeyword(PercentChar),
      "^" => Token::Operator(Caret),
      "!" => Token::UnaryOperator(Factorial),
      "(" => {
        left_paren_count += 1;
        Token::Operator(LeftParen)
      },
      ")" => {
        right_paren_count += 1;
        Token::Operator(RightParen)
      },
      "π" => Token::Constant(Pi),
      "'" => Token::Unit(Foot),
      "\"" | "“" | "”" | "″" => Token::LexerKeyword(DoubleQuotes),
      "Ω" | "Ω" => Token::Unit(Ohm),
      _ => {
        return Err(format!("Invalid character: {}", current_char));
      },
    };
    chars.next();
    tokens.push(token);
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
          Some(Token::UnaryOperator(_)) => {
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
      // decide if " is inch or inch of mercury
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

  #[test]
  fn test_lex() {
    pub fn run_lex(input: &str, expected_tokens: Vec<Token>) {
      let tokens = lex(input, false, Unit::Celsius).unwrap();
      let matching_tokens = tokens.iter().zip(&expected_tokens).filter(|&(a, b)| a == b);
      assert_eq!(matching_tokens.count(), expected_tokens.len());
    }

    run_lex("42 millilitres", vec![numtok!(42), Token::Unit(Milliliter)]);
    run_lex("50 / 10", vec![numtok!(50), Token::Operator(Divide), numtok!(10)]);
    run_lex("33.3 square meters", vec![numtok!(33.3), Token::Unit(SquareMeter)]);
    run_lex("101 nautical miles", vec![numtok!(101), Token::Unit(NauticalMile)]);
    run_lex("87 sq miles", vec![numtok!(87), Token::Unit(SquareMile)]);
    run_lex("1 light year", vec![numtok!(1), Token::Unit(LightYear)]);
    run_lex("34 cubic feet + 23 cubic yards", vec![numtok!(34), Token::Unit(CubicFoot), Token::Operator(Plus), numtok!(23), Token::Unit(CubicYard)]);
    run_lex("50 metric tonnes", vec![numtok!(50), Token::Unit(MetricTon)]);
    run_lex("5432 newton metres", vec![numtok!(5432), Token::Unit(NewtonMeter)]);
    run_lex("2345 newton-meters", vec![numtok!(2345), Token::Unit(NewtonMeter)]);
  }
}
