use std::iter::Peekable;
use std::str::FromStr;
use decimal::d128;
use crate::Token;
use crate::Operator::{Caret, Divide, LeftParen, Minus, Modulo, Multiply, Plus, RightParen};
use crate::UnaryOperator::{Percent, Factorial};
use crate::TextOperator::{Of, To};
use crate::NamedNumber::*;
use crate::Constant::{E, Pi};
use crate::LexerKeyword::{In, PercentChar, Per, Mercury, Hg, PoundForce, Force, DoubleQuotes, Revolution};
use crate::FunctionIdentifier::{Cbrt, Ceil, Cos, Exp, Abs, Floor, Ln, Log, Round, Sin, Sqrt, Tan};
use crate::units::Unit::*;
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

fn is_word_char_str(input: &str) -> bool {
	match input {
		"A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L"
		| "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X"
		| "Y" | "Z" => true,
		"a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l"
		| "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x"
		| "y" | "z" => true,
		"Ω" | "Ω" | "µ" | "μ" => true,
		_ => false,
	}
}

fn is_numeric_str(input: &str) -> bool {
	matches!(input, "." | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9")
}

/// Read next characters as a word, otherwise return empty string.
/// Returns an empty string if there's leading whitespace.
fn read_word_plain(chars: &mut Peekable<Graphemes>) -> String {
	let mut word = String::new();
	while let Some(next_char) = chars.peek() {
		if is_word_char_str(next_char) {
			word += chars.next().unwrap();
		} else {
			break;
		}
	}
	word
}

/// Read next as a word, otherwise return empty string.
/// Leading whitespace is ignored. A trailing digit may be included.
fn read_word(first_c: &str, lexer: &mut Lexer) -> String {
	let chars = &mut lexer.chars;
	let mut word = first_c.trim().to_owned();
	if word.is_empty() {
		// skip whitespace
		while let Some(current_char) = chars.peek() {
			if current_char.trim().is_empty() {
				chars.next();
			} else {
				break;
			}
		}
	}
	while let Some(next_char) = chars.peek() {
		if is_word_char_str(next_char) {
			word += chars.next().unwrap();
		} else {
			break;
		}
	}
	if !word.is_empty() {
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
	}
	word
}

fn parse_token(c: &str, lexer: &mut Lexer) -> Result<(), String> {
	let tokens = &mut lexer.tokens;
	match c {
		value if value.trim().is_empty() => {},
		value if is_word_char_str(value) => {
			parse_word(read_word(c, lexer).as_str(), lexer)?;
		},
		value if is_numeric_str(value) => {
			let mut number_string = value.to_owned();
			while let Some(number_char) = lexer.chars.peek() {
				if is_numeric_str(number_char) {
					number_string += number_char;
					lexer.chars.next();
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
				}
			};
		},
		"+" => tokens.push(Token::Operator(Plus)),
		"-" => tokens.push(Token::Operator(Minus)),
		"*" => tokens.push(Token::Operator(Multiply)),
		"/" | "÷" => tokens.push(Token::Operator(Divide)),
		"%" => tokens.push(Token::LexerKeyword(PercentChar)),
		"^" => tokens.push(Token::Operator(Caret)),
		"!" => tokens.push(Token::UnaryOperator(Factorial)),
		"(" => {
			lexer.left_paren_count += 1;
			tokens.push(Token::Operator(LeftParen));
		},
		")" => {
			lexer.right_paren_count += 1;
			tokens.push(Token::Operator(RightParen));
		},
		"π" => tokens.push(Token::Constant(Pi)),
		"'" => tokens.push(Token::Unit(Foot)),
		"\"" | "“" | "”" | "″" => tokens.push(Token::LexerKeyword(DoubleQuotes)),
		_ => {
			return Err(format!("Invalid character: {}", c));
		},
	}
	Ok(())
}

fn parse_word_if_non_empty(word: &str, lexer: &mut Lexer) -> Result<(), String> {
	match word {
		"" => Ok(()),
		_ => parse_word(word, lexer)
	}
}

fn parse_word(word: &str, lexer: &mut Lexer) -> Result<(), String> {
	let token = match word {
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

		"plus" => Token::Operator(Plus),
		"minus" => Token::Operator(Minus),
		"times" => Token::Operator(Multiply),
		"multiplied" => {
			match read_word("", lexer).as_str() {
				"by" => Token::Operator(Multiply),
				string => return Err(format!("Invalid string: {}", string)),
			}
		},
		"divided" => {
			match read_word("", lexer).as_str() {
				"by" => Token::Operator(Divide),
				string => return Err(format!("Invalid string: {}", string)),
			}
		},
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
		"marathon" | "marathons" => Token::Unit(Marathon),
		"nmi" => Token::Unit(NauticalMile),
		"nautical" => {
			match read_word("", lexer).as_str() {
				"mile" | "miles" => Token::Unit(NauticalMile),
				string => return Err(format!("Invalid string: {}", string)),
			}
		},
		"ly" | "lightyear" | "lightyears" => Token::Unit(LightYear),
		"lightsec" | "lightsecs" | "lightsecond" | "lightseconds" => Token::Unit(LightSecond),
		"light" => {
			match read_word("", lexer).as_str() {
				"yr" | "yrs" | "year" | "years" => Token::Unit(LightYear),
				"sec" | "secs" | "second" | "seconds" => Token::Unit(LightSecond),
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
			match read_word("", lexer).as_str() {
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
			match read_word("", lexer).as_str() {
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
			match read_word("", lexer).as_str() {
				"oz" | "ounce" | "ounces" => Token::Unit(FluidOunce),
				string => return Err(format!("Invalid string: {}", string)),
			}
		},
		"cup" | "cups" => Token::Unit(Cup),
		"pt" | "pint" | "pints" => Token::Unit(Pint),
		"qt" | "quart" | "quarts" => Token::Unit(Quart),
		"gal" | "gallon" | "gallons" => Token::Unit(Gallon),
		"bbl" => Token::Unit(OilBarrel),
		"oil" => {
			match read_word("", lexer).as_str() {
				"barrel" | "barrels" => Token::Unit(OilBarrel),
				string => return Err(format!("Invalid string: {}", string)),
			}
		},

		"metric" => {
			match read_word("", lexer).as_str() {
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
			match lexer.chars.next() {
				Some("-") => {
					match read_word_plain(&mut lexer.chars).as_str() {
						"force" => Token::LexerKeyword(PoundForce),
						other => {
							lexer.tokens.push(Token::Unit(Pound));
							lexer.tokens.push(Token::Operator(Minus));
							parse_word_if_non_empty(other, lexer)?;
							return Ok(());
						}
					}
				},
				Some(c) => {
					lexer.tokens.push(Token::Unit(Pound));
					parse_token(c, lexer)?;
					return Ok(());
				},
				None => {
					lexer.tokens.push(Token::Unit(Pound));
					return Ok(());
				},
			}
		},
		"stone" | "stones" => Token::Unit(Stone),
		"st" | "ton" | "tons" => Token::Unit(ShortTon),
		"short" => {
			match read_word("", lexer).as_str() {
				"ton" | "tons" | "tonne" | "tonnes" => Token::Unit(ShortTon),
				string => return Err(format!("Invalid string: {}", string)),
			}
		},
		"lt" => Token::Unit(LongTon),
		"long" => {
			match read_word("", lexer).as_str() {
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

		"bps" => Token::Unit(BitsPerSecond),
		"kbps" => Token::Unit(KilobitsPerSecond),
		"mbps" => Token::Unit(MegabitsPerSecond),
		"gbps" => Token::Unit(GigabitsPerSecond),
		"tbps" => Token::Unit(TerabitsPerSecond),
		"pbps" => Token::Unit(PetabitsPerSecond),
		"ebps" => Token::Unit(ExabitsPerSecond),
		"zbps" => Token::Unit(ZettabitsPerSecond),
		"ybps" => Token::Unit(YottabitsPerSecond),

		"millijoule" | "millijoules" => Token::Unit(Millijoule),
		"j"| "joule" | "joules" => Token::Unit(Joule),
		"nm" => Token::Unit(NewtonMeter),
		"newton" => {
			match lexer.chars.next() {
				Some("-") => {
					match read_word_plain(&mut lexer.chars).as_str() {
						"meter" | "meters" | "metre" | "metres" => Token::Unit(NewtonMeter),
						string => return Err(format!("Invalid string: {}", string)),
					}
				},
				Some(c) => {
					match read_word(c, lexer).as_str() {
						"meter" | "meters" | "metre" | "metres" => Token::Unit(NewtonMeter),
						string => return Err(format!("Invalid string: {}", string)),
					}
				},
				None => return Err(format!("Invalid string: {}", word)),
			}
		},
		"kj" | "kilojoule" | "kilojoules" => Token::Unit(Kilojoule),
		"mj" | "megajoule" | "megajoules" => Token::Unit(Megajoule),
		"gj" | "gigajoule" | "gigajoules" => Token::Unit(Gigajoule),
		"tj" | "terajoule" | "terajoules" => Token::Unit(Terajoule),
		"cal" | "calorie" | "calories" => Token::Unit(Calorie),
		"kcal" | "kilocalorie" | "kilocalories" => Token::Unit(KiloCalorie),
		"btu" => Token::Unit(BritishThermalUnit),
		"british" => {
			match read_word("", lexer).as_str() {
				"thermal" => {
					match read_word("", lexer).as_str() {
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
			match read_word("", lexer).as_str() {
				"hr" | "hrs" | "hour" | "hours" => Token::Unit(WattHour),
				other => {
					lexer.tokens.push(Token::Unit(Watt));
					parse_word_if_non_empty(other, lexer)?;
					return Ok(());
				},
			}
		}
		"kilowatt" => {
			match read_word("", lexer).as_str() {
				"hr" | "hrs" | "hour" | "hours" => Token::Unit(KilowattHour),
				other => {
					lexer.tokens.push(Token::Unit(Kilowatt));
					parse_word_if_non_empty(other, lexer)?;
					return Ok(());
				},
			}
		}
		"megawatt" => {
			match read_word("", lexer).as_str() {
				"hr" | "hrs" | "hour" | "hours" => Token::Unit(MegawattHour),
				other => {
					lexer.tokens.push(Token::Unit(Megawatt));
					parse_word_if_non_empty(other, lexer)?;
					return Ok(());
				},
			}
		}
		"gigawatt" => {
			match read_word("", lexer).as_str() {
				"hr" | "hrs" | "hour" | "hours" => Token::Unit(GigawattHour),
				other => {
					lexer.tokens.push(Token::Unit(Gigawatt));
					parse_word_if_non_empty(other, lexer)?;
					return Ok(());
				},
			}
		}
		"terawatt" => {
			match read_word("", lexer).as_str() {
				"hr" | "hrs" | "hour" | "hours" => Token::Unit(TerawattHour),
				other => {
					lexer.tokens.push(Token::Unit(Terawatt));
					parse_word_if_non_empty(other, lexer)?;
					return Ok(());
				},
			}
		}
		"petawatt" => {
			match read_word("", lexer).as_str() {
				"hr" | "hrs" | "hour" | "hours" => Token::Unit(PetawattHour),
				other => {
					lexer.tokens.push(Token::Unit(Petawatt));
					parse_word_if_non_empty(other, lexer)?;
					return Ok(());
				},
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
		"rpm" => Token::Unit(RevolutionsPerMinute),
		"r" | "rev" | "revolution" | "revolutions" => Token::LexerKeyword(Revolution),

		"kph" | "kmh" => Token::Unit(KilometersPerHour),
		"mps" => Token::Unit(MetersPerSecond),
		"mph" => Token::Unit(MilesPerHour),
		"fps" => Token::Unit(FeetPerSecond),
		"kn" | "kt" | "knot" | "knots" => Token::Unit(Knot),

		"k" | "kelvin" | "kelvins" => Token::Unit(Kelvin),
		"c" | "celsius" => Token::Unit(Celsius),
		"f" | "fahrenheit" | "fahrenheits" => Token::Unit(Fahrenheit),

		string => {
			return Err(format!("Invalid string: {}", string));
		}
	};
	lexer.tokens.push(token);
	Ok(())
}

struct Lexer<'a> {
	left_paren_count: u16,
	right_paren_count: u16,
	chars: Peekable<Graphemes<'a>>,
	tokens: Vec<Token>,
}

/// Lex an input string and returns [`Token`]s
pub fn lex(input: &str, remove_trailing_operator: bool) -> Result<Vec<Token>, String> {
	let mut input = input.replace(',', "").to_ascii_lowercase();

	if remove_trailing_operator {
		match &input.chars().last().unwrap_or('x') {
			'+' | '-' | '*' | '/' | '^' | '(' => {
				input.pop();
			},
			_ => {},
		}
	}

	let mut lexer = Lexer {
		left_paren_count: 0,
		right_paren_count: 0,
		chars: UnicodeSegmentation::graphemes(input.as_str(), true).peekable(),
		tokens: Vec::new(),
	};

	while let Some(c) = lexer.chars.next() {
		parse_token(c, &mut lexer)?;
	}
	let tokens = &mut lexer.tokens;
	// auto insert missing parentheses in first and last position
	if lexer.left_paren_count > lexer.right_paren_count {
		let missing_right_parens = lexer.left_paren_count - lexer.right_paren_count;
		for _ in 0..missing_right_parens {
			tokens.push(Token::Operator(RightParen));
		}
	} else if lexer.left_paren_count < lexer.right_paren_count {
		let missing_left_parens = lexer.right_paren_count - lexer.left_paren_count;
		for _ in 0..missing_left_parens {
			tokens.insert(0, Token::Operator(LeftParen));
		}
	}

	if tokens.is_empty() {
		return Err("Input was empty".to_string());
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
			// decide if " is 'inch' or 'inch of mercury'
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
				// bits per second
				(Token::Unit(Bit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(BitsPerSecond);
				},
				// kilobits per second
				(Token::Unit(Kilobit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(KilobitsPerSecond);
				},
				// megabits per second
				(Token::Unit(Megabit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(MegabitsPerSecond);
				},
				// gigabits per second
				(Token::Unit(Gigabit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(GigabitsPerSecond);
				},
				// terabits per second
				(Token::Unit(Terabit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(TerabitsPerSecond);
				},
				// petabits per second
				(Token::Unit(Petabit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(PetabitsPerSecond);
				},
				// exabits per second
				(Token::Unit(Exabit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(ExabitsPerSecond);
				},
				// zettabits per second
				(Token::Unit(Zettabit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(ZettabitsPerSecond);
				},
				// yottabits per second
				(Token::Unit(Yottabit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(YottabitsPerSecond);
				},
				// kibibits per second
				(Token::Unit(Kibibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(KibibitsPerSecond);
				},
				// mebibits per second
				(Token::Unit(Mebibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(MebibitsPerSecond);
				},
				// gibibits per second
				(Token::Unit(Gibibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(GibibitsPerSecond);
				},
				// tebibits per second
				(Token::Unit(Tebibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(TebibitsPerSecond);
				},
				// pebibits per second
				(Token::Unit(Pebibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(PebibitsPerSecond);
				},
				// exbibits per second
				(Token::Unit(Exbibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(ExbibitsPerSecond);
				},
				// zebibits per second
				(Token::Unit(Zebibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(ZebibitsPerSecond);
				},
				// yobibits per second
				(Token::Unit(Yobibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(YobibitsPerSecond);
				},
				// bytes per second
				(Token::Unit(Byte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(BytesPerSecond);
				},
				// kilobytes per second
				(Token::Unit(Kilobyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(KilobytesPerSecond);
				},
				// megabytes per second
				(Token::Unit(Megabyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(MegabytesPerSecond);
				},
				// gigabytes per second
				(Token::Unit(Gigabyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(GigabytesPerSecond);
				},
				// terabytes per second
				(Token::Unit(Terabyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(TerabytesPerSecond);
				},
				// petabytes per second
				(Token::Unit(Petabyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(PetabytesPerSecond);
				},
				// exabytes per second
				(Token::Unit(Exabyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(ExabytesPerSecond);
				},
				// zettabytes per second
				(Token::Unit(Zettabyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(ZettabytesPerSecond);
				},
				// yottabytes per second
				(Token::Unit(Yottabyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(YottabytesPerSecond);
				},
				// kibibytes per second
				(Token::Unit(Kibibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(KibibytesPerSecond);
				},
				// mebibytes per second
				(Token::Unit(Mebibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(MebibytesPerSecond);
				},
				// gibibytes per second
				(Token::Unit(Gibibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(GibibytesPerSecond);
				},
				// tebibytes per second
				(Token::Unit(Tebibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(TebibytesPerSecond);
				},
				// pebibytes per second
				(Token::Unit(Pebibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(PebibytesPerSecond);
				},
				// exbibytes per second
				(Token::Unit(Exbibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(ExbibytesPerSecond);
				},
				// zebibytes per second
				(Token::Unit(Zebibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(ZebibytesPerSecond);
				},
				// yobibytes per second
				(Token::Unit(Yobibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
					tokens[token_index-2] = Token::Unit(YobibytesPerSecond);
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

	Ok(lexer.tokens)
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
		let nonplural_data_units = Regex::new(r"(bit|byte)s").unwrap();

		let run_lex = |input: &str, expected_tokens: Vec<Token>| {
			let tokens = match lex(input, false) {
				Ok(tokens) => tokens,
				Err(e) => {
					panic!("lex error: {}\nrun_lex input: {}", e, input);
				}
			};
			let info_msg = format!("run_lex input: {}\nexpected: {:?}\nreceived: {:?}", input, expected_tokens, tokens);
			assert!(tokens == expected_tokens, "{info_msg}");

			// Prove we can handle multiple spaces wherever we handle a single space
			let input_extra_spaces = input.replace(" ", "   ");
			let tokens_extra_spaces = lex(&input_extra_spaces, false).unwrap();
			assert!(tokens_extra_spaces == expected_tokens, "{info_msg}");

			// Prove we don't need spaces around operators
			let input_stripped_spaces = strip_operator_spacing.replace_all(input, "$1");
			let tokens_stripped_spaces = lex(&input_stripped_spaces, false).unwrap();
			assert!(tokens_stripped_spaces == expected_tokens, "{info_msg}");

			// Prove we don't need a space after a digit
			let input_afterdigit_stripped_spaces = strip_afterdigit_spacing.replace_all(input, "$1");
			let tokens_afterdigit_stripped_spaces = lex(&input_afterdigit_stripped_spaces, false).unwrap();
			assert!(tokens_afterdigit_stripped_spaces == expected_tokens, "{info_msg}");
		};

		let run_datarate_lex = |input: &str, expected_tokens: Vec<Token>| {
			run_lex(input, (*expected_tokens).to_vec());

			// Prove plural and non-plural data units behave identically
			let input_nonplural_units = nonplural_data_units.replace_all(input, "$1");
			let tokens_nonplural_units = lex(&input_nonplural_units, false).unwrap();
			let info_msg = format!("run_datarate_lex input: {}\nexpected: {:?}\nreceived: {:?}", input, expected_tokens, tokens_nonplural_units);
			assert!(tokens_nonplural_units == expected_tokens, "{info_msg}");
		};

		run_lex("88 kilometres * 2", vec![numtok!(88), Token::Unit(Kilometer), Token::Operator(Multiply), numtok!(2)]);
		run_lex("0.5 marathon", vec![numtok!(0.5), Token::Unit(Marathon)]);
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
		run_datarate_lex("1 bit", vec![numtok!(1), Token::Unit(Bit)]);
		run_datarate_lex("8 bits", vec![numtok!(8), Token::Unit(Bit)]);
		run_datarate_lex("63 kilobits", vec![numtok!(63), Token::Unit(Kilobit)]);
		run_datarate_lex("32 megabits", vec![numtok!(32), Token::Unit(Megabit)]);
		run_datarate_lex("3.5 gigabits", vec![numtok!(3.5), Token::Unit(Gigabit)]);
		run_datarate_lex("2.1 terabits", vec![numtok!(2.1), Token::Unit(Terabit)]);
		run_datarate_lex("1.08 petabits", vec![numtok!(1.08), Token::Unit(Petabit)]);
		run_datarate_lex("0.73 exabits", vec![numtok!(0.73), Token::Unit(Exabit)]);
		run_datarate_lex("0.49 zettabits", vec![numtok!(0.49), Token::Unit(Zettabit)]);
		run_datarate_lex("0.23 yottabits", vec![numtok!(0.23), Token::Unit(Yottabit)]);
		run_datarate_lex("63 kibibits", vec![numtok!(63), Token::Unit(Kibibit)]);
		run_datarate_lex("32 mebibits", vec![numtok!(32), Token::Unit(Mebibit)]);
		run_datarate_lex("3.5 gibibits", vec![numtok!(3.5), Token::Unit(Gibibit)]);
		run_datarate_lex("2.1 tebibits", vec![numtok!(2.1), Token::Unit(Tebibit)]);
		run_datarate_lex("1.08 pebibits", vec![numtok!(1.08), Token::Unit(Pebibit)]);
		run_datarate_lex("0.73 exbibits", vec![numtok!(0.73), Token::Unit(Exbibit)]);
		run_datarate_lex("0.49 zebibits", vec![numtok!(0.49), Token::Unit(Zebibit)]);
		run_datarate_lex("0.23 yobibits", vec![numtok!(0.23), Token::Unit(Yobibit)]);
		run_datarate_lex("1 byte", vec![numtok!(1), Token::Unit(Byte)]);
		run_datarate_lex("3 bytes", vec![numtok!(3), Token::Unit(Byte)]);
		run_datarate_lex("63 kilobytes", vec![numtok!(63), Token::Unit(Kilobyte)]);
		run_datarate_lex("32 megabytes", vec![numtok!(32), Token::Unit(Megabyte)]);
		run_datarate_lex("3.5 gigabytes", vec![numtok!(3.5), Token::Unit(Gigabyte)]);
		run_datarate_lex("2.1 terabytes", vec![numtok!(2.1), Token::Unit(Terabyte)]);
		run_datarate_lex("1.08 petabytes", vec![numtok!(1.08), Token::Unit(Petabyte)]);
		run_datarate_lex("0.73 exabytes", vec![numtok!(0.73), Token::Unit(Exabyte)]);
		run_datarate_lex("0.49 zettabytes", vec![numtok!(0.49), Token::Unit(Zettabyte)]);
		run_datarate_lex("0.23 yottabytes", vec![numtok!(0.23), Token::Unit(Yottabyte)]);
		run_datarate_lex("63 kibibytes", vec![numtok!(63), Token::Unit(Kibibyte)]);
		run_datarate_lex("32 mebibytes", vec![numtok!(32), Token::Unit(Mebibyte)]);
		run_datarate_lex("3.5 gibibytes", vec![numtok!(3.5), Token::Unit(Gibibyte)]);
		run_datarate_lex("2.1 tebibytes", vec![numtok!(2.1), Token::Unit(Tebibyte)]);
		run_datarate_lex("1.08 pebibytes", vec![numtok!(1.08), Token::Unit(Pebibyte)]);
		run_datarate_lex("0.73 exbibytes", vec![numtok!(0.73), Token::Unit(Exbibyte)]);
		run_datarate_lex("0.49 zebibytes", vec![numtok!(0.49), Token::Unit(Zebibyte)]);
		run_datarate_lex("0.23 yobibytes", vec![numtok!(0.23), Token::Unit(Yobibyte)]);
		run_lex("432 bps", vec![numtok!(432), Token::Unit(BitsPerSecond)]);
		run_lex("56 kbps", vec![numtok!(56), Token::Unit(KilobitsPerSecond)]);
		run_lex("12 mbps", vec![numtok!(12), Token::Unit(MegabitsPerSecond)]);
		run_lex("4.2 gbps", vec![numtok!(4.2), Token::Unit(GigabitsPerSecond)]);
		run_lex("2.2 tbps", vec![numtok!(2.2), Token::Unit(TerabitsPerSecond)]);
		run_lex("1.7 pbps", vec![numtok!(1.7), Token::Unit(PetabitsPerSecond)]);
		run_lex("0.99 ebps", vec![numtok!(0.99), Token::Unit(ExabitsPerSecond)]);
		run_lex("0.64 zbps", vec![numtok!(0.64), Token::Unit(ZettabitsPerSecond)]);
		run_lex("0.278 ybps", vec![numtok!(0.278), Token::Unit(YottabitsPerSecond)]);
		run_datarate_lex("4 bits per second", vec![numtok!(4), Token::Unit(BitsPerSecond)]);
		run_datarate_lex("5 kilobits per second", vec![numtok!(5), Token::Unit(KilobitsPerSecond)]);
		run_datarate_lex("6 megabits per second", vec![numtok!(6), Token::Unit(MegabitsPerSecond)]);
		run_datarate_lex("7 gigabits per second", vec![numtok!(7), Token::Unit(GigabitsPerSecond)]);
		run_datarate_lex("8 terabits per second", vec![numtok!(8), Token::Unit(TerabitsPerSecond)]);
		run_datarate_lex("9 petabits per second", vec![numtok!(9), Token::Unit(PetabitsPerSecond)]);
		run_datarate_lex("10 exabits per second", vec![numtok!(10), Token::Unit(ExabitsPerSecond)]);
		run_datarate_lex("11 zettabits per second", vec![numtok!(11), Token::Unit(ZettabitsPerSecond)]);
		run_datarate_lex("12 yottabits per second", vec![numtok!(12), Token::Unit(YottabitsPerSecond)]);
		run_datarate_lex("13 kibibits per second", vec![numtok!(13), Token::Unit(KibibitsPerSecond)]);
		run_datarate_lex("14 mebibits per second", vec![numtok!(14), Token::Unit(MebibitsPerSecond)]);
		run_datarate_lex("15 gibibits per second", vec![numtok!(15), Token::Unit(GibibitsPerSecond)]);
		run_datarate_lex("16 tebibits per second", vec![numtok!(16), Token::Unit(TebibitsPerSecond)]);
		run_datarate_lex("17 pebibits per second", vec![numtok!(17), Token::Unit(PebibitsPerSecond)]);
		run_datarate_lex("18 exbibits per second", vec![numtok!(18), Token::Unit(ExbibitsPerSecond)]);
		run_datarate_lex("19 zebibits per second", vec![numtok!(19), Token::Unit(ZebibitsPerSecond)]);
		run_datarate_lex("20 yobibits per second", vec![numtok!(20), Token::Unit(YobibitsPerSecond)]);
		run_datarate_lex("4 bytes per second", vec![numtok!(4), Token::Unit(BytesPerSecond)]);
		run_datarate_lex("5 kilobytes per second", vec![numtok!(5), Token::Unit(KilobytesPerSecond)]);
		run_datarate_lex("6 megabytes per second", vec![numtok!(6), Token::Unit(MegabytesPerSecond)]);
		run_datarate_lex("7 gigabytes per second", vec![numtok!(7), Token::Unit(GigabytesPerSecond)]);
		run_datarate_lex("8 terabytes per second", vec![numtok!(8), Token::Unit(TerabytesPerSecond)]);
		run_datarate_lex("9 petabytes per second", vec![numtok!(9), Token::Unit(PetabytesPerSecond)]);
		run_datarate_lex("10 exabytes per second", vec![numtok!(10), Token::Unit(ExabytesPerSecond)]);
		run_datarate_lex("11 zettabytes per second", vec![numtok!(11), Token::Unit(ZettabytesPerSecond)]);
		run_datarate_lex("12 yottabytes per second", vec![numtok!(12), Token::Unit(YottabytesPerSecond)]);
		run_datarate_lex("13 kibibytes per second", vec![numtok!(13), Token::Unit(KibibytesPerSecond)]);
		run_datarate_lex("14 mebibytes per second", vec![numtok!(14), Token::Unit(MebibytesPerSecond)]);
		run_datarate_lex("15 gibibytes per second", vec![numtok!(15), Token::Unit(GibibytesPerSecond)]);
		run_datarate_lex("16 tebibytes per second", vec![numtok!(16), Token::Unit(TebibytesPerSecond)]);
		run_datarate_lex("17 pebibytes per second", vec![numtok!(17), Token::Unit(PebibytesPerSecond)]);
		run_datarate_lex("18 exbibytes per second", vec![numtok!(18), Token::Unit(ExbibytesPerSecond)]);
		run_datarate_lex("19 zebibytes per second", vec![numtok!(19), Token::Unit(ZebibytesPerSecond)]);
		run_datarate_lex("20 yobibytes per second", vec![numtok!(20), Token::Unit(YobibytesPerSecond)]);
		run_lex("234 wh", vec![numtok!(234), Token::Unit(WattHour)]);
		run_lex("1 w", vec![numtok!(1), Token::Unit(Watt)]);
		run_lex("1 watt", vec![numtok!(1), Token::Unit(Watt)]);
		run_lex("1 watts", vec![numtok!(1), Token::Unit(Watt)]);
		run_lex("1 watt hour", vec![numtok!(1), Token::Unit(WattHour)]);
		run_lex("0 watt + 1 watts", vec![numtok!(0), Token::Unit(Watt), Token::Operator(Plus), numtok!(1), Token::Unit(Watt)]);
		run_lex("0 watt * 1", vec![numtok!(0), Token::Unit(Watt), Token::Operator(Multiply), numtok!(1)]);
		run_lex("2 watts + 3 watts", vec![numtok!(2), Token::Unit(Watt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("2 watts * 3", vec![numtok!(2), Token::Unit(Watt), Token::Operator(Multiply), numtok!(3)]);
		run_lex("4 watt plus 5 watts", vec![numtok!(4), Token::Unit(Watt), Token::Operator(Plus), numtok!(5), Token::Unit(Watt)]);
		run_lex("4 watt times 5", vec![numtok!(4), Token::Unit(Watt), Token::Operator(Multiply), numtok!(5)]);
		run_lex("6 watts plus 7 watts", vec![numtok!(6), Token::Unit(Watt), Token::Operator(Plus), numtok!(7), Token::Unit(Watt)]);
		run_lex("6 watts times 7", vec![numtok!(6), Token::Unit(Watt), Token::Operator(Multiply), numtok!(7)]);
		run_lex("2.3 kwh", vec![numtok!(2.3), Token::Unit(KilowattHour)]);
		run_lex("1 kw", vec![numtok!(1), Token::Unit(Kilowatt)]);
		run_lex("1 kilowatt", vec![numtok!(1), Token::Unit(Kilowatt)]);
		run_lex("1 kilowatts", vec![numtok!(1), Token::Unit(Kilowatt)]);
		run_lex("1 kilowatt hour", vec![numtok!(1), Token::Unit(KilowattHour)]);
		run_lex("2 kilowatt + 3 watt", vec![numtok!(2), Token::Unit(Kilowatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("2 kilowatt * 4", vec![numtok!(2), Token::Unit(Kilowatt), Token::Operator(Multiply), numtok!(4)]);
		run_lex("2 kilowatt times 4", vec![numtok!(2), Token::Unit(Kilowatt), Token::Operator(Multiply), numtok!(4)]);
		run_lex("2 kilowatt + 3 watts", vec![numtok!(2), Token::Unit(Kilowatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("2 kilowatts + 3 watt", vec![numtok!(2), Token::Unit(Kilowatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("2 kilowatts + 3 watts", vec![numtok!(2), Token::Unit(Kilowatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("2 kilowatt plus 3 watt", vec![numtok!(2), Token::Unit(Kilowatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("2 kilowatt plus 3 watts", vec![numtok!(2), Token::Unit(Kilowatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("2 kilowatts plus 3 watt", vec![numtok!(2), Token::Unit(Kilowatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("2 kilowatts plus 3 watts", vec![numtok!(2), Token::Unit(Kilowatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("6.6 watts + 4 kilowatts", vec![numtok!(6.6), Token::Unit(Watt), Token::Operator(Plus), numtok!(4), Token::Unit(Kilowatt)]);
		run_lex("6.6 watts plus 4 kilowatts", vec![numtok!(6.6), Token::Unit(Watt), Token::Operator(Plus), numtok!(4), Token::Unit(Kilowatt)]);
		run_lex("2.3 mwh", vec![numtok!(2.3), Token::Unit(MegawattHour)]);
		run_lex("1 mw", vec![numtok!(1), Token::Unit(Megawatt)]);
		run_lex("1 megawatt", vec![numtok!(1), Token::Unit(Megawatt)]);
		run_lex("1 megawatt hour", vec![numtok!(1), Token::Unit(MegawattHour)]);
		run_lex("2 megawatt + 3 watt", vec![numtok!(2), Token::Unit(Megawatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("2 megawatt * 6", vec![numtok!(2), Token::Unit(Megawatt), Token::Operator(Multiply), numtok!(6)]);
		run_lex("2 megawatt times 6", vec![numtok!(2), Token::Unit(Megawatt), Token::Operator(Multiply), numtok!(6)]);
		run_lex("2 megawatt + 3 watts", vec![numtok!(2), Token::Unit(Megawatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("2 megawatts + 3 watt", vec![numtok!(2), Token::Unit(Megawatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("2 megawatts + 3 watts", vec![numtok!(2), Token::Unit(Megawatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("2 megawatt plus 3 watt", vec![numtok!(2), Token::Unit(Megawatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("2 megawatt plus 3 watts", vec![numtok!(2), Token::Unit(Megawatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("2 megawatts plus 3 watt", vec![numtok!(2), Token::Unit(Megawatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("2 megawatts plus 3 watts", vec![numtok!(2), Token::Unit(Megawatt), Token::Operator(Plus), numtok!(3), Token::Unit(Watt)]);
		run_lex("6.6 watts + 4 megawatts", vec![numtok!(6.6), Token::Unit(Watt), Token::Operator(Plus), numtok!(4), Token::Unit(Megawatt)]);
		run_lex("6.6 watts plus 4 megawatts", vec![numtok!(6.6), Token::Unit(Watt), Token::Operator(Plus), numtok!(4), Token::Unit(Megawatt)]);
		run_lex("234 gwh", vec![numtok!(234), Token::Unit(GigawattHour)]);
		run_lex("1 gw", vec![numtok!(1), Token::Unit(Gigawatt)]);
		run_lex("1 gigawatt", vec![numtok!(1), Token::Unit(Gigawatt)]);
		run_lex("1 gigawatts", vec![numtok!(1), Token::Unit(Gigawatt)]);
		run_lex("1 gigawatt hour", vec![numtok!(1), Token::Unit(GigawattHour)]);
		run_lex("0 gigawatt + 1 gigawatts", vec![numtok!(0), Token::Unit(Gigawatt), Token::Operator(Plus), numtok!(1), Token::Unit(Gigawatt)]);
		run_lex("0 gigawatt * 1", vec![numtok!(0), Token::Unit(Gigawatt), Token::Operator(Multiply), numtok!(1)]);
		run_lex("2 gigawatts + 3 gigawatts", vec![numtok!(2), Token::Unit(Gigawatt), Token::Operator(Plus), numtok!(3), Token::Unit(Gigawatt)]);
		run_lex("2 gigawatts * 3", vec![numtok!(2), Token::Unit(Gigawatt), Token::Operator(Multiply), numtok!(3)]);
		run_lex("4 gigawatt plus 5 watt", vec![numtok!(4), Token::Unit(Gigawatt), Token::Operator(Plus), numtok!(5), Token::Unit(Watt)]);
		run_lex("4 gigawatt plus 5 megawatt", vec![numtok!(4), Token::Unit(Gigawatt), Token::Operator(Plus), numtok!(5), Token::Unit(Megawatt)]);
		run_lex("4 gigawatt plus 5 gigawatt", vec![numtok!(4), Token::Unit(Gigawatt), Token::Operator(Plus), numtok!(5), Token::Unit(Gigawatt)]);
		run_lex("4 gigawatt plus 5 watts", vec![numtok!(4), Token::Unit(Gigawatt), Token::Operator(Plus), numtok!(5), Token::Unit(Watt)]);
		run_lex("4 gigawatt plus 5 megawatts", vec![numtok!(4), Token::Unit(Gigawatt), Token::Operator(Plus), numtok!(5), Token::Unit(Megawatt)]);
		run_lex("4 gigawatt plus 5 gigawatts", vec![numtok!(4), Token::Unit(Gigawatt), Token::Operator(Plus), numtok!(5), Token::Unit(Gigawatt)]);
		run_lex("4 gigawatt times 5", vec![numtok!(4), Token::Unit(Gigawatt), Token::Operator(Multiply), numtok!(5)]);
		run_lex("6 gigawatts plus 7 watt", vec![numtok!(6), Token::Unit(Gigawatt), Token::Operator(Plus), numtok!(7), Token::Unit(Watt)]);
		run_lex("6 gigawatts plus 7 megawatt", vec![numtok!(6), Token::Unit(Gigawatt), Token::Operator(Plus), numtok!(7), Token::Unit(Megawatt)]);
		run_lex("6 gigawatts plus 7 gigawatt", vec![numtok!(6), Token::Unit(Gigawatt), Token::Operator(Plus), numtok!(7), Token::Unit(Gigawatt)]);
		run_lex("6 gigawatts plus 7 watts", vec![numtok!(6), Token::Unit(Gigawatt), Token::Operator(Plus), numtok!(7), Token::Unit(Watt)]);
		run_lex("6 gigawatts plus 7 megawatts", vec![numtok!(6), Token::Unit(Gigawatt), Token::Operator(Plus), numtok!(7), Token::Unit(Megawatt)]);
		run_lex("6 gigawatts plus 7 gigawatts", vec![numtok!(6), Token::Unit(Gigawatt), Token::Operator(Plus), numtok!(7), Token::Unit(Gigawatt)]);
		run_lex("6 gigawatts times 7", vec![numtok!(6), Token::Unit(Gigawatt), Token::Operator(Multiply), numtok!(7)]);
		run_lex("88 mw * 3", vec![numtok!(88), Token::Unit(Megawatt), Token::Operator(Multiply), numtok!(3)]);
		run_lex("88 mw times 3", vec![numtok!(88), Token::Unit(Megawatt), Token::Operator(Multiply), numtok!(3)]);
		run_lex("999 kb", vec![numtok!(999), Token::Unit(Kilobyte)]);
		run_lex("200 gb - 100 mb", vec![numtok!(200), Token::Unit(Gigabyte), Token::Operator(Minus), numtok!(100), Token::Unit(Megabyte)]);
		run_lex("999 kib", vec![numtok!(999), Token::Unit(Kibibyte)]);
		run_lex("200 gib - 100 mib", vec![numtok!(200), Token::Unit(Gibibyte), Token::Operator(Minus), numtok!(100), Token::Unit(Mebibyte)]);
		run_lex("45 btu", vec![numtok!(45), Token::Unit(BritishThermalUnit)]);
		run_lex("45.5 british thermal unit", vec![numtok!(45.5), Token::Unit(BritishThermalUnit)]);
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
		run_lex("52 ÷ 12", vec![numtok!(52), Token::Operator(Divide), numtok!(12)]);
		run_lex("6 divided by 3", vec![numtok!(6), Token::Operator(Divide), numtok!(3)]);
		run_lex("7 mod 5", vec![numtok!(7), Token::Operator(Modulo), numtok!(5)]);

		run_lex("(2 + 3) * 4", vec![Token::Operator(LeftParen), numtok!(2), Token::Operator(Plus), numtok!(3), Token::Operator(RightParen), Token::Operator(Multiply), numtok!(4)]);
		run_lex("52 weeks * (12 hrs + 12 hours)", vec![numtok!(52), Token::Unit(Week), Token::Operator(Multiply), Token::Operator(LeftParen), numtok!(12), Token::Unit(Hour), Token::Operator(Plus), numtok!(12), Token::Unit(Hour), Token::Operator(RightParen)]);
		run_lex("12 pound+", vec![numtok!(12), Token::Unit(Pound), Token::Operator(Plus)]);

		run_lex("5 π m", vec![numtok!(5), Token::Constant(Pi), Token::Unit(Meter)]);
		run_lex("5 Ω + 2 mΩ", vec![numtok!(5), Token::Unit(Ohm), Token::Operator(Plus), numtok!(2), Token::Unit(Milliohm)]);
	}
}
