use crate::Constant::*;
use crate::FunctionIdentifier::*;
use crate::LexerKeyword::*;
use crate::NamedNumber::*;
use crate::Operator::*;
use crate::TextOperator::*;
use crate::Token;
use crate::UnaryOperator::{Factorial, Percent};
use crate::units::Unit::*;
use fastnum::D128;
use fastnum::decimal::Context;
use fxhash::FxHashMap;
use std::fmt;
use std::iter::Peekable;
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

fn is_word_char_str(input: &str) -> bool {
	match input {
		"A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O"
		| "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" => true,
		"a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o"
		| "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" => true,
		"Ω" | "Ω" | "µ" | "μ" => true,
		_ => false,
	}
}

fn is_numeric_str(input: &str) -> bool {
	matches!(
		input,
		"." | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
	)
}

// todo!();
/// Read next characters as a word, otherwise return empty string.
/// Returns an empty string if there's leading whitespace.
fn _read_word_plain(chars: &mut Peekable<Graphemes>) -> String {
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
			}
			"3" | "³" => {
				word += "3";
				chars.next();
			}
			_ => {}
		}
	}
	word
}

#[derive(Clone, Debug, PartialEq)]
/// Math operators like [`Multiply`](Operator::Multiply), parentheses, etc.
enum LexOperator {
	Plus,
	Minus,
	Multiply,
	Divide,
	Caret,
	PercentSign,
	ExclamationMark,
}

#[derive(Clone, PartialEq)]
/// A basic token lexed from characters
enum Atom {
	Number(D128),
	Ident(String),
	Operator(LexOperator),
	ParenLeft,
	ParenRight,
}
impl fmt::Debug for Atom {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Atom::Number(n) => write!(f, "Number({})", n),
			Atom::Ident(s) => write!(f, "Ident({})", s),
			Atom::Operator(o) => write!(f, "Operator({:?})", o),
			Atom::ParenLeft => write!(f, "ParenLeft"),
			Atom::ParenRight => write!(f, "ParenRight"),
		}
	}
}

fn lex_next_chars(c: &str, lexer: &mut Lexer) -> Result<(), String> {
	let token = match c {
		value if value.trim().is_empty() => return Ok(()),
		value if is_word_char_str(value) => {
			let word = read_word(c, lexer);
			Atom::Ident(word)
		}
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
			match D128::from_str(&number_string, Context::default()) {
				Ok(number) => Atom::Number(number),
				Err(_e) => {
					return Err(format!("Error lexing d128 number: {}", number_string));
				}
			}
		}
		"+" => Atom::Operator(LexOperator::Plus),
		"-" => Atom::Operator(LexOperator::Minus),
		"*" => Atom::Operator(LexOperator::Multiply),
		"/" | "÷" => Atom::Operator(LexOperator::Divide),
		"^" => Atom::Operator(LexOperator::Caret),
		"%" => Atom::Operator(LexOperator::PercentSign),
		"!" => Atom::Operator(LexOperator::ExclamationMark),
		"(" => {
			lexer.left_paren_count += 1;
			Atom::ParenLeft
		}
		")" => {
			lexer.right_paren_count += 1;
			Atom::ParenRight
		}
		"\"" | "“" | "”" | "″" => Atom::Ident("\"".to_string()),
		symbol => Atom::Ident(symbol.to_string()),
	};
	lexer.atoms.push(token);
	Ok(())
}

fn _parse_word_if_non_empty(_word: &str, _lexer: &mut Lexer) -> Result<(), String> {
	todo!();
	// match word {
	// 	"" => Ok(()),
	// 	_ => parse_word(word, lexer),
	// }
}

fn default_token_map() -> Result<TokenMap, String> {
	let mut trie = TokenMap::new();

	trie.try_add("to", Token::TextOperator(To))?;
	trie.try_add("of", Token::TextOperator(Of))?;

	trie.try_add("hundred", Token::NamedNumber(Hundred))?;
	trie.try_add("thousand", Token::NamedNumber(Thousand))?;
	trie.try_add_multi(&["mil", "mill", "million"], Token::NamedNumber(Million))?;
	trie.try_add_multi(&["bil", "bill", "billion"], Token::NamedNumber(Billion))?;
	trie.try_add_multi(&["tri", "tril", "trillion"], Token::NamedNumber(Trillion))?;
	trie.try_add("quadrillion", Token::NamedNumber(Quadrillion))?;
	trie.try_add("quintillion", Token::NamedNumber(Quintillion))?;
	trie.try_add("sextillion", Token::NamedNumber(Sextillion))?;
	trie.try_add("septillion", Token::NamedNumber(Septillion))?;
	trie.try_add("octillion", Token::NamedNumber(Octillion))?;
	trie.try_add("nonillion", Token::NamedNumber(Nonillion))?;
	trie.try_add("decillion", Token::NamedNumber(Decillion))?;
	trie.try_add("undecillion", Token::NamedNumber(Undecillion))?;
	trie.try_add("duodecillion", Token::NamedNumber(Duodecillion))?;
	trie.try_add("tredecillion", Token::NamedNumber(Tredecillion))?;
	trie.try_add("quattuordecillion", Token::NamedNumber(Quattuordecillion))?;
	trie.try_add("quindecillion", Token::NamedNumber(Quindecillion))?;
	trie.try_add("sexdecillion", Token::NamedNumber(Sexdecillion))?;
	trie.try_add("septendecillion", Token::NamedNumber(Septendecillion))?;
	trie.try_add("octodecillion", Token::NamedNumber(Octodecillion))?;
	trie.try_add("novemdecillion", Token::NamedNumber(Novemdecillion))?;
	trie.try_add("vigintillion", Token::NamedNumber(Vigintillion))?;
	trie.try_add("centillion", Token::NamedNumber(Centillion))?;
	trie.try_add("googol", Token::NamedNumber(Googol))?;

	trie.try_add("pi", Token::Constant(Pi))?;
	trie.try_add("e", Token::Constant(E))?;

	trie.try_add("plus", Token::Operator(Plus))?;
	trie.try_add("minus", Token::Operator(Minus))?;
	trie.try_add("times", Token::Operator(Multiply))?;
	trie.try_add("multiplied by", Token::Operator(Multiply))?;
	trie.try_add("divide by", Token::Operator(Divide))?;
	trie.try_add("mod", Token::Operator(Modulo))?;

	trie.try_add("sqrt", Token::FunctionIdentifier(Sqrt))?;
	trie.try_add("cbrt", Token::FunctionIdentifier(Cbrt))?;

	trie.try_add("log", Token::FunctionIdentifier(Log))?;
	trie.try_add("ln", Token::FunctionIdentifier(Ln))?;
	trie.try_add("exp", Token::FunctionIdentifier(Exp))?;

	trie.try_add_multi(&["round", "rint"], Token::FunctionIdentifier(Round))?;
	trie.try_add("ceil", Token::FunctionIdentifier(Ceil))?;
	trie.try_add("floor", Token::FunctionIdentifier(Floor))?;
	trie.try_add_multi(&["abs", "fabs"], Token::FunctionIdentifier(Abs))?;

	trie.try_add("sin", Token::FunctionIdentifier(Sin))?;
	trie.try_add("cos", Token::FunctionIdentifier(Cos))?;
	trie.try_add("tan", Token::FunctionIdentifier(Tan))?;

	trie.try_add("per", Token::LexerKeyword(Per))?;
	trie.try_add("hg", Token::LexerKeyword(Hg))?; // can be hectogram or mercury

	trie.try_add_multi(
		&["ns", "nanosec", "nanosecs", "nanosecond", "nanoseconds"],
		Token::Unit(Nanosecond),
	)?;
	// // µ and μ are two different characters
	trie.try_add_multi(
		&[
			"µs",
			"μs",
			"microsec",
			"microsecs",
			"microsecond",
			"microseconds",
		],
		Token::Unit(Microsecond),
	)?;
	trie.try_add_multi(
		&["ms", "millisec", "millisecs", "millisecond", "milliseconds"],
		Token::Unit(Millisecond),
	)?;
	trie.try_add_multi(
		&["s", "sec", "secs", "second", "seconds"],
		Token::Unit(Second),
	)?;
	trie.try_add_multi(&["min", "mins", "minute", "minutes"], Token::Unit(Minute))?;
	trie.try_add_multi(&["h", "hr", "hrs", "hour", "hours"], Token::Unit(Hour))?;
	trie.try_add_multi(&["day", "days"], Token::Unit(Day))?;
	trie.try_add_multi(&["wk", "wks", "week", "weeks"], Token::Unit(Week))?;
	trie.try_add_multi(&["mo", "mos", "month", "months"], Token::Unit(Month))?;
	trie.try_add_multi(&["q", "quarter", "quarters"], Token::Unit(Quarter))?;
	trie.try_add_multi(&["yr", "yrs", "year", "years"], Token::Unit(Year))?;
	trie.try_add_multi(&["decade", "decades"], Token::Unit(Decade))?;
	trie.try_add_multi(&["century", "centuries"], Token::Unit(Century))?;
	trie.try_add_multi(
		&["millenium", "millenia", "milleniums"],
		Token::Unit(Millennium),
	)?;

	trie.try_add_multi(
		&[
			"mm",
			"millimeter",
			"millimeters",
			"millimetre",
			"millimetres",
		],
		Token::Unit(Millimeter),
	)?;
	trie.try_add_multi(
		&[
			"cm",
			"centimeter",
			"centimeters",
			"centimetre",
			"centimetres",
		],
		Token::Unit(Centimeter),
	)?;
	trie.try_add_multi(
		&["dm", "decimeter", "decimeters", "decimetre", "decimetres"],
		Token::Unit(Decimeter),
	)?;
	trie.try_add_multi(
		&["m", "meter", "meters", "metre", "metres"],
		Token::Unit(Meter),
	)?;
	trie.try_add_multi(
		&["km", "kilometer", "kilometers", "kilometre", "kilometres"],
		Token::Unit(Kilometer),
	)?;
	trie.try_add("in", Token::LexerKeyword(In))?;
	trie.try_add_multi(&["inch", "inches"], Token::Unit(Inch))?;
	trie.try_add_multi(&["ft", "foot", "feet"], Token::Unit(Foot))?;
	trie.try_add_multi(&["yd", "yard", "yards"], Token::Unit(Yard))?;
	trie.try_add_multi(&["mi", "mile", "miles"], Token::Unit(Mile))?;
	trie.try_add_multi(&["marathon", "marathons"], Token::Unit(Marathon))?;
	trie.try_add("nmi", Token::Unit(NauticalMile))?;
	trie.try_add_multi(
		&["nautical mile", "nautical miles"],
		Token::Unit(NauticalMile),
	)?;
	trie.try_add_multi(&["ly", "lightyear", "lightyears"], Token::Unit(LightYear))?;
	trie.try_add_multi(
		&["lightsec", "lightsecs", "lightsecond", "lightseconds"],
		Token::Unit(LightSecond),
	)?;
	trie.try_add_multi(
		&["light yr", "light yrs", "light year", "light years"],
		Token::Unit(LightYear),
	)?;
	trie.try_add_multi(
		&["light sec", "light secs", "light second", "light seconds"],
		Token::Unit(LightSecond),
	)?;

	trie.try_add_multi(
		&[
			"sqmm",
			"mm2",
			"millimeter2",
			"millimeters2",
			"millimetre2",
			"millimetres2",
		],
		Token::Unit(SquareMillimeter),
	)?;
	trie.try_add_multi(
		&[
			"sqcm",
			"cm2",
			"centimeter2",
			"centimeters2",
			"centimetre2",
			"centimetres2",
		],
		Token::Unit(SquareCentimeter),
	)?;
	trie.try_add_multi(
		&[
			"sqdm",
			"dm2",
			"decimeter2",
			"decimeters2",
			"decimetre2",
			"decimetres2",
		],
		Token::Unit(SquareDecimeter),
	)?;
	trie.try_add_multi(
		&["sqm", "m2", "meter2", "meters2", "metre2", "metres2"],
		Token::Unit(SquareMeter),
	)?;
	trie.try_add_multi(
		&[
			"sqkm",
			"km2",
			"kilometer2",
			"kilometers2",
			"kilometre2",
			"kilometres2",
		],
		Token::Unit(SquareKilometer),
	)?;
	trie.try_add_multi(
		&["sqin", "in2", "inch2", "inches2"],
		Token::Unit(SquareInch),
	)?;
	trie.try_add_multi(&["sqft", "ft2", "foot2", "feet2"], Token::Unit(SquareFoot))?;
	trie.try_add_multi(&["sqyd", "yd2", "yard2", "yards2"], Token::Unit(SquareYard))?;
	trie.try_add_multi(&["sqmi", "mi2", "mile2", "miles2"], Token::Unit(SquareMile))?;
	let square_entries = [
		(
			&[
				"mm",
				"millimeter",
				"millimeters",
				"millimetre",
				"millimetres",
			][..],
			Token::Unit(SquareMillimeter),
		),
		(
			&[
				"cm",
				"centimeter",
				"centimeters",
				"centimetre",
				"centimetres",
			][..],
			Token::Unit(SquareCentimeter),
		),
		(
			&["dm", "decimeter", "decimeters", "decimetre", "decimetres"][..],
			Token::Unit(SquareDecimeter),
		),
		(
			&["m", "meter", "meters", "metre", "metres"][..],
			Token::Unit(SquareMeter),
		),
		(
			&["km", "kilometer", "kilometers", "kilometre", "kilometres"][..],
			Token::Unit(SquareKilometer),
		),
		(&["in", "inch", "inches"][..], Token::Unit(SquareInch)),
		(&["ft", "foot", "feet"][..], Token::Unit(SquareFoot)),
		(&["yd", "yard", "yards"][..], Token::Unit(SquareYard)),
		(&["mi", "mile", "miles"][..], Token::Unit(SquareMile)),
	];
	for entry in square_entries {
		for key in entry.0 {
			trie.try_add(&format!("sq {key}"), entry.1.clone())?;
			trie.try_add(&format!("square {key}"), entry.1.clone())?;
		}
	}
	trie.try_add_multi(&["are", "ares"], Token::Unit(Are))?;
	trie.try_add_multi(&["decare", "decares"], Token::Unit(Decare))?;
	trie.try_add_multi(&["ha", "hectare", "hectares"], Token::Unit(Hectare))?;
	trie.try_add_multi(&["acre", "acres"], Token::Unit(Acre))?;

	trie.try_add_multi(
		&[
			"mm3",
			"millimeter3",
			"millimeters3",
			"millimetre3",
			"millimetres3",
		],
		Token::Unit(CubicMillimeter),
	)?;
	trie.try_add_multi(
		&[
			"cm3",
			"centimeter3",
			"centimeters3",
			"centimetre3",
			"centimetres3",
		],
		Token::Unit(CubicCentimeter),
	)?;
	trie.try_add_multi(
		&[
			"dm3",
			"decimeter3",
			"decimeters3",
			"decimetre3",
			"decimetres3",
		],
		Token::Unit(CubicDecimeter),
	)?;
	trie.try_add_multi(
		&["m3", "meter3", "meters3", "metre3", "metres3"],
		Token::Unit(CubicMeter),
	)?;
	trie.try_add_multi(
		&[
			"km3",
			"kilometer3",
			"kilometers3",
			"kilometre3",
			"kilometres3",
		],
		Token::Unit(CubicKilometer),
	)?;
	trie.try_add_multi(&["inc3", "inch3", "inches3"], Token::Unit(CubicInch))?;
	trie.try_add_multi(&["ft3", "foot3", "feet3"], Token::Unit(CubicFoot))?;
	trie.try_add_multi(&["yd3", "yard3", "yards3"], Token::Unit(CubicYard))?;
	trie.try_add_multi(&["mi3", "mile3", "miles3"], Token::Unit(CubicMile))?;
	let cubic_entries = &[
		(
			&[
				"mm",
				"millimeter",
				"millimeters",
				"millimetre",
				"millimetres",
			][..],
			Token::Unit(CubicMillimeter),
		),
		(
			&[
				"cm",
				"centimeter",
				"centimeters",
				"centimetre",
				"centimetres",
			][..],
			Token::Unit(CubicCentimeter),
		),
		(
			&["dm", "decimeter", "decimeters", "decimetre", "decimetres"][..],
			Token::Unit(CubicDecimeter),
		),
		(
			&["m", "meter", "meters", "metre", "metres"][..],
			Token::Unit(CubicMeter),
		),
		(
			&["km", "kilometer", "kilometers", "kilometre", "kilometres"][..],
			Token::Unit(CubicKilometer),
		),
		(&["in", "inch", "inches"][..], Token::Unit(CubicInch)),
		(&["ft", "foot", "feet"][..], Token::Unit(CubicFoot)),
		(&["yd", "yard", "yards"][..], Token::Unit(CubicYard)),
		(&["mi", "mile", "miles"][..], Token::Unit(CubicMile)),
	];
	for entry in cubic_entries {
		for key in entry.0 {
			trie.try_add(format!("cubic {key}"), entry.1.clone())?;
		}
	}

	trie.try_add_multi(
		&[
			"ml",
			"milliliter",
			"milliliters",
			"millilitre",
			"millilitres",
		],
		Token::Unit(Milliliter),
	)?;
	trie.try_add_multi(
		&[
			"cl",
			"centiliter",
			"centiliters",
			"centilitre",
			"centilitres",
		],
		Token::Unit(Centiliter),
	)?;
	trie.try_add_multi(
		&["dl", "deciliter", "deciliters", "decilitre", "decilitres"],
		Token::Unit(Deciliter),
	)?;
	trie.try_add_multi(
		&["l", "liter", "liters", "litre", "litres"],
		Token::Unit(Liter),
	)?;
	trie.try_add_multi(
		&["ts", "tsp", "tspn", "tspns", "teaspoon", "teaspoons"],
		Token::Unit(Teaspoon),
	)?;
	trie.try_add_multi(
		&["tbs", "tbsp", "tablespoon", "tablespoons"],
		Token::Unit(Tablespoon),
	)?;
	trie.try_add_multi(
		&[
			"floz",
			"fl oz",
			"fl ounce",
			"fl ounces",
			"fluid oz",
			"fluid ounce",
			"fluid ounces",
		],
		Token::Unit(FluidOunce),
	)?;
	trie.try_add_multi(&["cup", "cups"], Token::Unit(Cup))?;
	trie.try_add_multi(&["pt", "pint", "pints"], Token::Unit(Pint))?;
	trie.try_add_multi(&["qt", "quart", "quarts"], Token::Unit(Quart))?;
	trie.try_add_multi(&["gal", "gallon", "gallons"], Token::Unit(Gallon))?;
	trie.try_add_multi(
		&["bbl", "oil barrel", "oil barrels"],
		Token::Unit(OilBarrel),
	)?;

	trie.try_add_multi(
		&["metric ton", "metric tons", "metric tonne", "metric tonnes"],
		Token::Unit(MetricTon),
	)?;
	trie.try_add_multi(
		&[
			"metric hp",
			"metric hps",
			"metric horsepower",
			"metric horsepowers",
		],
		Token::Unit(MetricHorsepower),
	)?;

	trie.try_add_multi(&["mg", "milligram", "milligrams"], Token::Unit(Milligram))?;
	trie.try_add_multi(&["g", "gram", "grams"], Token::Unit(Gram))?;
	trie.try_add_multi(&["hectogram", "hectograms"], Token::Unit(Hectogram))?;
	trie.try_add_multi(
		&["kg", "kilo", "kilos", "kilogram", "kilograms"],
		Token::Unit(Kilogram),
	)?;
	trie.try_add_multi(&["t", "tonne", "tonnes"], Token::Unit(MetricTon))?;
	trie.try_add_multi(&["oz", "ounces"], Token::Unit(Ounce))?;
	trie.try_add_multi(&["lb", "lbs"], Token::Unit(Pound))?;
	// TODO: add ["pound-force", "pounds-force", "pound force", "pounds force"]
	trie.try_add_multi(&["stone", "stones"], Token::Unit(Stone))?;
	trie.try_add_multi(
		&[
			"st",
			"ton",
			"tons",
			"short ton",
			"short tons",
			"short tonne",
			"short tonnes",
		],
		Token::Unit(ShortTon),
	)?;
	trie.try_add_multi(
		&["lt", "long ton", "long tons", "long tonne", "long tonnes"],
		Token::Unit(LongTon),
	)?;

	trie.try_add_multi(&["bit", "bits"], Token::Unit(Bit))?;
	trie.try_add_multi(&["kbit", "kilobit", "kilobits"], Token::Unit(Kilobit))?;
	trie.try_add_multi(&["mbit", "megabit", "megabits"], Token::Unit(Megabit))?;
	trie.try_add_multi(&["gbit", "gigabit", "gigabits"], Token::Unit(Gigabit))?;
	trie.try_add_multi(&["tbit", "terabit", "terabits"], Token::Unit(Terabit))?;
	trie.try_add_multi(&["pbit", "petabit", "petabits"], Token::Unit(Petabit))?;
	trie.try_add_multi(&["ebit", "exabit", "exabits"], Token::Unit(Exabit))?;
	trie.try_add_multi(&["zbit", "zettabit", "zettabits"], Token::Unit(Zettabit))?;
	trie.try_add_multi(&["ybit", "yottabit", "yottabits"], Token::Unit(Yottabit))?;
	trie.try_add_multi(&["kibit", "kibibit", "kibibits"], Token::Unit(Kibibit))?;
	trie.try_add_multi(&["mibit", "mebibit", "mebibits"], Token::Unit(Mebibit))?;
	trie.try_add_multi(&["gibit", "gibibit", "gibibits"], Token::Unit(Gibibit))?;
	trie.try_add_multi(&["tibit", "tebibit", "tebibits"], Token::Unit(Tebibit))?;
	trie.try_add_multi(&["pibit", "pebibit", "pebibits"], Token::Unit(Pebibit))?;
	trie.try_add_multi(&["eibit", "exbibit", "exbibits"], Token::Unit(Exbibit))?;
	trie.try_add_multi(&["zibit", "zebibit", "zebibits"], Token::Unit(Zebibit))?;
	trie.try_add_multi(&["yibit", "yobibit", "yobibits"], Token::Unit(Yobibit))?;
	trie.try_add_multi(&["byte", "bytes"], Token::Unit(Byte))?;
	trie.try_add_multi(&["kb", "kilobyte", "kilobytes"], Token::Unit(Kilobyte))?;
	trie.try_add_multi(&["mb", "megabyte", "megabytes"], Token::Unit(Megabyte))?;
	trie.try_add_multi(&["gb", "gigabyte", "gigabytes"], Token::Unit(Gigabyte))?;
	trie.try_add_multi(&["tb", "terabyte", "terabytes"], Token::Unit(Terabyte))?;
	trie.try_add_multi(&["pb", "petabyte", "petabytes"], Token::Unit(Petabyte))?;
	trie.try_add_multi(&["eb", "exabyte", "exabytes"], Token::Unit(Exabyte))?;
	trie.try_add_multi(&["zb", "zettabyte", "zettabytes"], Token::Unit(Zettabyte))?;
	trie.try_add_multi(&["yb", "yottabyte", "yottabytes"], Token::Unit(Yottabyte))?;
	trie.try_add_multi(&["kib", "kibibyte", "kibibytes"], Token::Unit(Kibibyte))?;
	trie.try_add_multi(&["mib", "mebibyte", "mebibytes"], Token::Unit(Mebibyte))?;
	trie.try_add_multi(&["gib", "gibibyte", "gibibytes"], Token::Unit(Gibibyte))?;
	trie.try_add_multi(&["tib", "tebibyte", "tebibytes"], Token::Unit(Tebibyte))?;
	trie.try_add_multi(&["pib", "pebibyte", "pebibytes"], Token::Unit(Pebibyte))?;
	trie.try_add_multi(&["eib", "exbibyte", "exbibytes"], Token::Unit(Exbibyte))?;
	trie.try_add_multi(&["zib", "zebibyte", "zebibytes"], Token::Unit(Zebibyte))?;
	trie.try_add_multi(&["yib", "yobibyte", "yobibytes"], Token::Unit(Yobibyte))?;

	trie.try_add("bps", Token::Unit(BitsPerSecond))?;
	trie.try_add("kbps", Token::Unit(KilobitsPerSecond))?;
	trie.try_add("mbps", Token::Unit(MegabitsPerSecond))?;
	trie.try_add("gbps", Token::Unit(GigabitsPerSecond))?;
	trie.try_add("tbps", Token::Unit(TerabitsPerSecond))?;
	trie.try_add("pbps", Token::Unit(PetabitsPerSecond))?;
	trie.try_add("ebps", Token::Unit(ExabitsPerSecond))?;
	trie.try_add("zbps", Token::Unit(ZettabitsPerSecond))?;
	trie.try_add("ybps", Token::Unit(YottabitsPerSecond))?;

	trie.try_add("flop", Token::Unit(Flop))?;
	trie.try_add_multi(&["kflop", "kiloflop"], Token::Unit(KiloFlop))?;
	trie.try_add_multi(&["mflop", "megaflop"], Token::Unit(MegaFlop))?;
	trie.try_add_multi(&["gflop", "gigaflop"], Token::Unit(GigaFlop))?;
	trie.try_add_multi(&["tflop", "teraflop"], Token::Unit(TeraFlop))?;
	trie.try_add_multi(&["pflop", "petaflop"], Token::Unit(PetaFlop))?;
	trie.try_add_multi(&["eflop", "exaflop"], Token::Unit(ExaFlop))?;
	trie.try_add_multi(&["zflop", "zettaflop"], Token::Unit(ZettaFlop))?;
	trie.try_add_multi(&["yflop", "yottaflop"], Token::Unit(YottaFlop))?;
	trie.try_add_multi(&["rflop", "ronnaflop"], Token::Unit(RonnaFlop))?;
	trie.try_add_multi(&["qflop", "quettaflop"], Token::Unit(QuettaFlop))?;

	trie.try_add("flops", Token::Unit(FlopPerSecond))?;
	trie.try_add_multi(&["kflops", "kiloflops"], Token::Unit(KiloFlopPerSecond))?;
	trie.try_add_multi(&["mflops", "megaflops"], Token::Unit(MegaFlopPerSecond))?;
	trie.try_add_multi(&["gflops", "gigaflops"], Token::Unit(GigaFlopPerSecond))?;
	trie.try_add_multi(&["tflops", "teraflops"], Token::Unit(TeraFlopPerSecond))?;
	trie.try_add_multi(&["pflops", "petaflops"], Token::Unit(PetaFlopPerSecond))?;
	trie.try_add_multi(&["eflops", "exaflops"], Token::Unit(ExaFlopPerSecond))?;
	trie.try_add_multi(&["zflops", "zettaflops"], Token::Unit(ZettaFlopPerSecond))?;
	trie.try_add_multi(&["yflops", "yottaflops"], Token::Unit(YottaFlopPerSecond))?;
	trie.try_add_multi(&["rflops", "ronnaflops"], Token::Unit(RonnaFlopPerSecond))?;
	trie.try_add_multi(&["qflops", "quettaflops"], Token::Unit(QuettaFlopPerSecond))?;

	trie.try_add_multi(&["millijoule", "millijoules"], Token::Unit(Millijoule))?;
	trie.try_add_multi(&["j", "joule", "joules"], Token::Unit(Joule))?;
	trie.try_add("nm", Token::Unit(NewtonMeter))?;

	trie.try_add_multi(
		&[
			"newton meter",
			"newton meters",
			"newton metre",
			"newton metres",
		],
		Token::Unit(NewtonMeter),
	)?;
	trie.try_add_multi(&["kj", "kilojoule", "kilojoules"], Token::Unit(Kilojoule))?;
	trie.try_add_multi(&["mj", "megajoule", "megajoules"], Token::Unit(Megajoule))?;
	trie.try_add_multi(&["gj", "gigajoule", "gigajoules"], Token::Unit(Gigajoule))?;
	trie.try_add_multi(&["tj", "terajoule", "terajoules"], Token::Unit(Terajoule))?;
	trie.try_add_multi(&["cal", "calorie", "calories"], Token::Unit(Calorie))?;
	trie.try_add_multi(
		&["kcal", "kilocalorie", "kilocalories"],
		Token::Unit(KiloCalorie),
	)?;
	trie.try_add_multi(
		&["btu", "british thermal unit", "british thermal units"],
		Token::Unit(BritishThermalUnit),
	)?;
	trie.try_add_multi(
		&["wh", "watt hr", "watt hrs", "watt hour", "watt hours"],
		Token::Unit(WattHour),
	)?;
	trie.try_add_multi(
		&[
			"kwh",
			"kilowatt hr",
			"kilowatt hrs",
			"kilowatt hour",
			"kilowatt hours",
		],
		Token::Unit(KilowattHour),
	)?;
	trie.try_add_multi(
		&[
			"mwh",
			"megawatt hr",
			"megawatt hrs",
			"megawatt hour",
			"megawatt hours",
		],
		Token::Unit(MegawattHour),
	)?;
	trie.try_add_multi(
		&[
			"gwh",
			"gigawatt hr",
			"gigawatt hrs",
			"gigawatt hour",
			"gigawatt hours",
		],
		Token::Unit(GigawattHour),
	)?;
	trie.try_add_multi(
		&[
			"twh",
			"terawatt hr",
			"terawatt hrs",
			"terawatt hour",
			"terawatt hours",
		],
		Token::Unit(TerawattHour),
	)?;
	trie.try_add_multi(
		&[
			"pwh",
			"petawatt hr",
			"petawatt hrs",
			"petawatt hour",
			"petawatt hours",
		],
		Token::Unit(PetawattHour),
	)?;

	trie.try_add_multi(&["milliwatt", "milliwatts"], Token::Unit(Milliwatt))?;
	trie.try_add_multi(&["w", "watts"], Token::Unit(Watt))?;
	trie.try_add_multi(&["kw", "kilowatts"], Token::Unit(Kilowatt))?;
	trie.try_add_multi(&["mw", "megawatts"], Token::Unit(Megawatt))?;
	trie.try_add_multi(&["gw", "gigawatts"], Token::Unit(Gigawatt))?;
	trie.try_add_multi(&["tw", "terawatts"], Token::Unit(Terawatt))?;
	trie.try_add_multi(&["pw", "petawatts"], Token::Unit(Petawatt))?;
	trie.try_add_multi(
		&["hp", "hps", "horsepower", "horsepowers"],
		Token::Unit(Horsepower),
	)?;
	trie.try_add_multi(&["mhp", "hpm"], Token::Unit(MetricHorsepower))?;

	trie.try_add_multi(
		&["ma", "milliamp", "milliamps", "milliampere", "milliamperes"],
		Token::Unit(Milliampere),
	)?;
	trie.try_add_multi(
		&["a", "amp", "amps", "ampere", "amperes"],
		Token::Unit(Ampere),
	)?;
	trie.try_add_multi(
		&["ka", "kiloamp", "kiloamps", "kiloampere", "kiloamperes"],
		Token::Unit(Kiloampere),
	)?;
	trie.try_add_multi(
		&["bi", "biot", "biots", "aba", "abampere", "abamperes"],
		Token::Unit(Abampere),
	)?;

	trie.try_add_multi(
		&["mΩ", "mΩ", "milliohm", "milliohms"],
		Token::Unit(Milliohm),
	)?;
	trie.try_add_multi(&["Ω", "Ω", "ohm", "ohms"], Token::Unit(Ohm))?;
	trie.try_add_multi(&["kΩ", "kΩ", "kiloohm", "kiloohms"], Token::Unit(Kiloohm))?;

	trie.try_add_multi(&["mv", "millivolt", "millivolts"], Token::Unit(Millivolt))?;
	trie.try_add_multi(&["v", "volt", "volts"], Token::Unit(Volt))?;
	trie.try_add_multi(&["kv", "kilovolt", "kilovolts"], Token::Unit(Kilovolt))?;

	// // for pound-force per square inch
	trie.try_add("lbf", Token::LexerKeyword(PoundForce))?;
	trie.try_add("force", Token::LexerKeyword(Force))?;

	trie.try_add_multi(&["pa", "pascal", "pascals"], Token::Unit(Pascal))?;
	trie.try_add_multi(
		&["kpa", "kilopascal", "kilopascals"],
		Token::Unit(Kilopascal),
	)?;
	trie.try_add_multi(
		&["atm", "atms", "atmosphere", "atmospheres"],
		Token::Unit(Atmosphere),
	)?;
	trie.try_add_multi(
		&["mbar", "mbars", "millibar", "millibars"],
		Token::Unit(Millibar),
	)?;
	trie.try_add_multi(&["bar", "bars"], Token::Unit(Bar))?;
	trie.try_add("inhg", Token::Unit(InchOfMercury))?;
	trie.try_add("mercury", Token::LexerKeyword(Mercury))?;
	trie.try_add("psi", Token::Unit(PoundsPerSquareInch))?;
	trie.try_add_multi(&["torr", "torrs"], Token::Unit(Torr))?;

	trie.try_add_multi(&["hz", "hertz"], Token::Unit(Hertz))?;
	trie.try_add_multi(&["khz", "kilohertz"], Token::Unit(Kilohertz))?;
	trie.try_add_multi(&["mhz", "megahertz"], Token::Unit(Megahertz))?;
	trie.try_add_multi(&["ghz", "gigahertz"], Token::Unit(Gigahertz))?;
	trie.try_add_multi(&["thz", "terahertz"], Token::Unit(Terahertz))?;
	trie.try_add_multi(&["phz", "petahertz"], Token::Unit(Petahertz))?;
	trie.try_add("rpm", Token::Unit(RevolutionsPerMinute))?;
	trie.try_add_multi(
		&["r", "rev", "revolution", "revolutions"],
		Token::LexerKeyword(Revolution),
	)?;

	trie.try_add_multi(&["kph", "kmh"], Token::Unit(KilometersPerHour))?;
	trie.try_add("mps", Token::Unit(MetersPerSecond))?;
	trie.try_add("mph", Token::Unit(MilesPerHour))?;
	trie.try_add("fps", Token::Unit(FeetPerSecond))?;
	trie.try_add_multi(&["kn", "kt", "knot", "knots"], Token::Unit(Knot))?;

	trie.try_add_multi(&["k", "kelvin", "kelvins"], Token::Unit(Kelvin))?;
	trie.try_add_multi(&["c", "celsius"], Token::Unit(Celsius))?;
	trie.try_add_multi(&["f", "fahrenheit", "fahrenheits"], Token::Unit(Fahrenheit))?;
	Ok(trie)
}

struct Lexer<'a> {
	left_paren_count: u16,
	right_paren_count: u16,
	chars: Peekable<Graphemes<'a>>,
	atoms: Vec<Atom>,
}

fn auto_insert_parens<'a>(lexer: &mut Lexer<'a>) {
	let tokens = &mut lexer.atoms;
	// auto insert missing parentheses in first and last position
	if lexer.left_paren_count > lexer.right_paren_count {
		let missing_right_parens = lexer.left_paren_count - lexer.right_paren_count;
		for _ in 0..missing_right_parens {
			tokens.push(Atom::ParenRight);
		}
	} else if lexer.left_paren_count < lexer.right_paren_count {
		let missing_left_parens = lexer.right_paren_count - lexer.left_paren_count;
		for _ in 0..missing_left_parens {
			tokens.insert(0, Atom::ParenLeft);
		}
	}
}

/// Lex an input string and returns [`Token`]s
pub fn lex(input: &str, remove_trailing_operator: bool) -> Result<Vec<Token>, String> {
	// TODO: keep commas
	let mut input = input.replace(',', "").to_ascii_lowercase();

	if remove_trailing_operator {
		match &input.chars().last().unwrap_or('x') {
			'+' | '-' | '*' | '/' | '^' | '(' => {
				input.pop();
			}
			_ => {}
		}
	}

	let mut lexer = Lexer {
		left_paren_count: 0,
		right_paren_count: 0,
		chars: UnicodeSegmentation::graphemes(input.as_str(), true).peekable(),
		atoms: Vec::new(),
	};

	while let Some(c) = lexer.chars.next() {
		lex_next_chars(c, &mut lexer)?;
	}

	auto_insert_parens(&mut lexer);

	if lexer.atoms.is_empty() {
		return Err("Input was empty".to_string());
	}

	println!("atoms {:?}", lexer.atoms);
	let tokens = resolve_semantic_tokens(lexer.atoms)?;
	println!("tokens {tokens:?}");
	Ok(tokens)
}

struct Tokeniser {
	i: usize,
	atoms: Vec<Atom>,
	tokens: Vec<Token>,
	token_map: TokenMap,
}

fn resolve_semantic_tokens(atoms: Vec<Atom>) -> Result<Vec<Token>, String> {
	let start = std::time::Instant::now();

	let mut tokeniser = Tokeniser {
		i: 0,
		atoms,
		tokens: Vec::new(),
		token_map: default_token_map()?,
	};
	let time = std::time::Instant::now().duration_since(start).as_nanos() as f32;

	println!("\u{23f1}  {:.3}ms dtm", time / 1000.0 / 1000.0);

	while tokeniser.i < tokeniser.atoms.len() {
		tokeniser = resolve_semantic_token(tokeniser)?;
	}

	Ok(tokeniser.tokens)
}

// enum TrieNode {
// 	Value(Token),
// 	Map(HashMap<String, TrieNode>),
// }
// struct TokenTrie {
// 	map: HashMap<String, TrieNode>,
// }
// impl TokenTrie {
// 	fn new() -> Self {
// 		Self {
// 			map: HashMap::new(),
// 		}
// 	}
// 	fn insert(&mut self, key: Vec<String>, value: Token) {
// 		let mut map = &mut self.map;
// 		let mut subkeys = key.into_iter().peekable();
// 		while let Some(subkey) = subkeys.next() {
// 			let is_last = subkeys.peek().is_none();
// 			match map.get_mut(&subkey.clone()) {
// 				Some(TrieNode::Value(old_value)) => {
// 					if is_last {
// 						panic!("Duplicate trie value");
// 					} else {
// 						// Turn single value into multiple
// 						let mut new_map = HashMap::new();
// 						new_map.insert("".to_string(), TrieNode::Value(old_value.clone()));
// 						map.insert(subkey.clone(), TrieNode::Map(new_map));
// 						match map.get_mut(&subkey.clone()).unwrap() {
// 							TrieNode::Value(_) => panic!(),
// 							TrieNode::Map(hash_map) => map = hash_map,
// 						};
// 					}
// 				}
// 				Some(TrieNode::Map(map)) => {
// 					if is_last {
// 						let deleted = map.insert("".to_string(), TrieNode::Value(value));
// 						if deleted.is_some() {
// 							panic!("Duplicate trie value");
// 						}
// 						return;
// 					} else {
// 						continue;
// 					}
// 				}
// 				None => {
// 					if is_last {
// 						map.insert(subkey.clone(), TrieNode::Value(value.clone()));
// 					} else {
// 						new_map.insert("".to_string(), TrieNode::Value(old_value.clone()));
// 						map.insert(subkey.clone(), TrieNode::Map(new_map));
// 						match map.get_mut(&subkey.clone()).unwrap() {
// 							TrieNode::Value(_) => panic!(),
// 							TrieNode::Map(hash_map) => map = hash_map,
// 						};
// 					}
// 				}
// 			}
// 		}
// 	}
// }

#[derive(Debug, Default)]
struct TokenMap {
	token: Option<Token>,
	children: FxHashMap<String, TokenMap>,
}

impl TokenMap {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn try_add(&mut self, key: impl Into<String>, token: Token) -> Result<(), String> {
		let key = key.into();
		let mut node = self;
		for word in key.split(' ') {
			// Only allocate if the key doesn't already exist
			node = if node.children.contains_key(word) {
				node.children.get_mut(word).unwrap()
			} else {
				node.children.entry(word.to_string()).or_default()
			};
		}
		match node.token {
			Some(_) => Err(format!("Token already exists for key: {key:?}")),
			None => {
				node.token = Some(token);
				Ok(())
			}
		}
	}

	pub fn try_add_multi(
		&mut self,
		keys: &[impl Into<String> + Clone],
		token: Token,
	) -> Result<(), String> {
		for key in keys {
			self.try_add(key.clone(), token.clone())?;
		}
		Ok(())
	}

	pub fn get_descendant(&self, key: impl Into<String>) -> Option<&TokenMap> {
		let mut node = self;
		for word in key.into().split(' ') {
			node = node.children.get(word)?;
		}
		Some(node)
	}
}

fn resolve_semantic_token(mut tokeniser: Tokeniser) -> Result<Tokeniser, String> {
	let mut i = tokeniser.i;
	let atoms = &mut tokeniser.atoms;
	let tokens = &mut tokeniser.tokens;

	macro_rules! push_and_increment {
		($token:expr) => {{
			tokens.push($token);
			i += 1;
		}};
	}

	match &atoms[i] {
		Atom::Number(decimal) => push_and_increment!(Token::Number(*decimal)),
		Atom::Ident(s) if s == "of" => push_and_increment!(Token::TextOperator(Of)),
		Atom::Ident(s) if s == "to" => push_and_increment!(Token::TextOperator(To)),
		Atom::Operator(operator) => match operator {
			LexOperator::Plus => push_and_increment!(Token::Operator(Plus)),
			LexOperator::Minus => push_and_increment!(Token::Operator(Minus)),
			LexOperator::Multiply => push_and_increment!(Token::Operator(Multiply)),
			LexOperator::Divide => push_and_increment!(Token::Operator(Divide)),
			LexOperator::Caret => push_and_increment!(Token::Operator(Caret)),
			// decide if % is percent or modulo
			LexOperator::PercentSign => {
				match atoms.get(i + 1) {
					// "10% of" should be percentage
					Some(Atom::Ident(s)) if s == "of" => {
						push_and_increment!(Token::UnaryOperator(Percent));
					}
					// "10%(2)" should be modulo
					Some(Atom::ParenLeft) => push_and_increment!(Token::Operator(Modulo)),
					Some(Atom::ParenRight) => {
						push_and_increment!(Token::UnaryOperator(Percent))
					}
					// "10%+2" should be a percentage
					// "10%%" should be a percentage (only the first % though)
					// "10%!" should be a percentage
					Some(Atom::Operator(_)) => {
						push_and_increment!(Token::UnaryOperator(Percent))
					}
					// everything else should be modulo, for example if the % is
					// before a number, function or constants
					Some(Atom::Ident(_)) | Some(Atom::Number(_)) => {
						push_and_increment!(Token::Operator(Modulo))
					}
					// if there's nothing afterwards, percentage
					None => push_and_increment!(Token::UnaryOperator(Percent)),
				}
			}
			LexOperator::ExclamationMark => push_and_increment!(Token::UnaryOperator(Factorial)),
		},
		Atom::ParenLeft => push_and_increment!(Token::Operator(LeftParen)),
		Atom::ParenRight => push_and_increment!(Token::Operator(RightParen)),
		Atom::Ident(ident) => {
			let mut key = ident.clone();
			let mut word_count = 1;
			let mut matched_token = None;
			let mut matched_word_count = 1;
			while let Some(node) = tokeniser.token_map.get_descendant(&key) {
				if let Some(value) = &node.token {
					matched_token = Some(value);
					matched_word_count = word_count;
				}
				if node.children.is_empty() {
					break;
				}
				let next_ident = match atoms.get(i + word_count) {
					Some(Atom::Ident(ident)) => ident,
					_ => break,
				};
				key = format!("{key} {next_ident}");
				word_count += 1;
			}
			let matched_token = match matched_token {
				Some(token) => token,
				None => return Err(format!("Invalid identifier: {ident}")),
			};
			tokens.push(matched_token.clone());
			i += matched_word_count;
		}
	}
	tokeniser.i = i;
	Ok(tokeniser)
}

fn _old_semantic() {
	// let tokens = Vec::new();
	// let token_index = 0;
	// loop {
	// 	match tokens[token_index] {
	// 		// decide if " is 'inch' or 'inch of mercury'
	// 		Token::LexerKeyword(DoubleQuotes) => {
	// 			todo!();
	// 			// match tokens.get(token_index + 1) {
	// 			// 	Some(Token::LexerKeyword(Hg)) => {
	// 			// 		// "hg should be inch of mercury
	// 			// 		tokens[token_index] = Token::Unit(InchOfMercury);
	// 			// 		tokens.remove(token_index + 1);
	// 			// 	}
	// 			// 	_ => {
	// 			// 		// otherwise, Inch
	// 			// 		tokens[token_index] = Token::Unit(Inch);
	// 			// 	}
	// 			// }
	// 		}
	// 		// if hg wasn't already turned into inch of mercury, it's hectogram
	// 		Token::LexerKeyword(Hg) => {
	// 			tokens[token_index] = Token::Unit(Hectogram);
	// 		}
	// 		// decide if "in" is Inch or To
	// 		Token::LexerKeyword(In) => {
	// 			match tokens.get(token_index + 1) {
	// 				Some(Token::Unit(_)) => {
	// 					// "in" should be To
	// 					tokens[token_index] = Token::TextOperator(To);
	// 				}
	// 				_ => {
	// 					// otherwise, Inch
	// 					tokens[token_index] = Token::Unit(Inch);
	// 				}
	// 			}
	// 		}
	// 		_ => {}
	// 	}
	// 	// parse units like km/h, lbf per square inch
	// 	if token_index >= 2 {
	// 		// 	let token1 = &tokens[token_index - 2];
	// 		// 	let token2 = match &tokens[token_index - 1] {
	// 		// 		// treat km/h the same as km per h
	// 		// 		Token::Operator(Divide) => &Token::LexerKeyword(Per),
	// 		// 		_ => &tokens[token_index - 1],
	// 		// 	};
	// 		// 	let token3 = &tokens[token_index];
	// 		// 	let mut replaced = true;
	// 		// 	match (token1, token2, token3) {
	// 		// 		// km/h
	// 		// 		(Token::Unit(Kilometer), Token::LexerKeyword(Per), Token::Unit(Hour)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(KilometersPerHour);
	// 		// 		}
	// 		// 		// mi/h
	// 		// 		(Token::Unit(Mile), Token::LexerKeyword(Per), Token::Unit(Hour)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(MilesPerHour);
	// 		// 		}
	// 		// 		// m/s
	// 		// 		(Token::Unit(Meter), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(MetersPerSecond);
	// 		// 		}
	// 		// 		// ft/s
	// 		// 		(Token::Unit(Foot), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(FeetPerSecond);
	// 		// 		}
	// 		// 		// bits per second
	// 		// 		(Token::Unit(Bit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(BitsPerSecond);
	// 		// 		}
	// 		// 		// kilobits per second
	// 		// 		(Token::Unit(Kilobit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(KilobitsPerSecond);
	// 		// 		}
	// 		// 		// megabits per second
	// 		// 		(Token::Unit(Megabit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(MegabitsPerSecond);
	// 		// 		}
	// 		// 		// gigabits per second
	// 		// 		(Token::Unit(Gigabit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(GigabitsPerSecond);
	// 		// 		}
	// 		// 		// terabits per second
	// 		// 		(Token::Unit(Terabit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(TerabitsPerSecond);
	// 		// 		}
	// 		// 		// petabits per second
	// 		// 		(Token::Unit(Petabit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(PetabitsPerSecond);
	// 		// 		}
	// 		// 		// exabits per second
	// 		// 		(Token::Unit(Exabit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(ExabitsPerSecond);
	// 		// 		}
	// 		// 		// zettabits per second
	// 		// 		(Token::Unit(Zettabit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(ZettabitsPerSecond);
	// 		// 		}
	// 		// 		// yottabits per second
	// 		// 		(Token::Unit(Yottabit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(YottabitsPerSecond);
	// 		// 		}
	// 		// 		// kibibits per second
	// 		// 		(Token::Unit(Kibibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(KibibitsPerSecond);
	// 		// 		}
	// 		// 		// mebibits per second
	// 		// 		(Token::Unit(Mebibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(MebibitsPerSecond);
	// 		// 		}
	// 		// 		// gibibits per second
	// 		// 		(Token::Unit(Gibibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(GibibitsPerSecond);
	// 		// 		}
	// 		// 		// tebibits per second
	// 		// 		(Token::Unit(Tebibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(TebibitsPerSecond);
	// 		// 		}
	// 		// 		// pebibits per second
	// 		// 		(Token::Unit(Pebibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(PebibitsPerSecond);
	// 		// 		}
	// 		// 		// exbibits per second
	// 		// 		(Token::Unit(Exbibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(ExbibitsPerSecond);
	// 		// 		}
	// 		// 		// zebibits per second
	// 		// 		(Token::Unit(Zebibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(ZebibitsPerSecond);
	// 		// 		}
	// 		// 		// yobibits per second
	// 		// 		(Token::Unit(Yobibit), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(YobibitsPerSecond);
	// 		// 		}
	// 		// 		// bytes per second
	// 		// 		(Token::Unit(Byte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(BytesPerSecond);
	// 		// 		}
	// 		// 		// kilobytes per second
	// 		// 		(Token::Unit(Kilobyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(KilobytesPerSecond);
	// 		// 		}
	// 		// 		// megabytes per second
	// 		// 		(Token::Unit(Megabyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(MegabytesPerSecond);
	// 		// 		}
	// 		// 		// gigabytes per second
	// 		// 		(Token::Unit(Gigabyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(GigabytesPerSecond);
	// 		// 		}
	// 		// 		// terabytes per second
	// 		// 		(Token::Unit(Terabyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(TerabytesPerSecond);
	// 		// 		}
	// 		// 		// petabytes per second
	// 		// 		(Token::Unit(Petabyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(PetabytesPerSecond);
	// 		// 		}
	// 		// 		// exabytes per second
	// 		// 		(Token::Unit(Exabyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(ExabytesPerSecond);
	// 		// 		}
	// 		// 		// zettabytes per second
	// 		// 		(Token::Unit(Zettabyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(ZettabytesPerSecond);
	// 		// 		}
	// 		// 		// yottabytes per second
	// 		// 		(Token::Unit(Yottabyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(YottabytesPerSecond);
	// 		// 		}
	// 		// 		// kibibytes per second
	// 		// 		(Token::Unit(Kibibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(KibibytesPerSecond);
	// 		// 		}
	// 		// 		// mebibytes per second
	// 		// 		(Token::Unit(Mebibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(MebibytesPerSecond);
	// 		// 		}
	// 		// 		// gibibytes per second
	// 		// 		(Token::Unit(Gibibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(GibibytesPerSecond);
	// 		// 		}
	// 		// 		// tebibytes per second
	// 		// 		(Token::Unit(Tebibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(TebibytesPerSecond);
	// 		// 		}
	// 		// 		// pebibytes per second
	// 		// 		(Token::Unit(Pebibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(PebibytesPerSecond);
	// 		// 		}
	// 		// 		// exbibytes per second
	// 		// 		(Token::Unit(Exbibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(ExbibytesPerSecond);
	// 		// 		}
	// 		// 		// zebibytes per second
	// 		// 		(Token::Unit(Zebibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(ZebibytesPerSecond);
	// 		// 		}
	// 		// 		// yobibytes per second
	// 		// 		(Token::Unit(Yobibyte), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(YobibytesPerSecond);
	// 		// 		}
	// 		// 		// FLOP per second
	// 		// 		(Token::Unit(Flop), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(FlopPerSecond);
	// 		// 		}
	// 		// 		// kiloFLOP per second
	// 		// 		(Token::Unit(KiloFlop), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(KiloFlopPerSecond);
	// 		// 		}
	// 		// 		// megaFLOP per second
	// 		// 		(Token::Unit(MegaFlop), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(MegaFlopPerSecond);
	// 		// 		}
	// 		// 		// gigaFLOP per second
	// 		// 		(Token::Unit(GigaFlop), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(GigaFlopPerSecond);
	// 		// 		}
	// 		// 		// teraFLOP per second
	// 		// 		(Token::Unit(TeraFlop), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(TeraFlopPerSecond);
	// 		// 		}
	// 		// 		// petaFLOP per second
	// 		// 		(Token::Unit(PetaFlop), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(PetaFlopPerSecond);
	// 		// 		}
	// 		// 		// exaFLOP per second
	// 		// 		(Token::Unit(ExaFlop), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(ExaFlopPerSecond);
	// 		// 		}
	// 		// 		// zettaFLOP per second
	// 		// 		(Token::Unit(ZettaFlop), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(ZettaFlopPerSecond);
	// 		// 		}
	// 		// 		// yottaFLOP per second
	// 		// 		(Token::Unit(YottaFlop), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(YottaFlopPerSecond);
	// 		// 		}
	// 		// 		// ronnaFLOP per second
	// 		// 		(Token::Unit(RonnaFlop), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(RonnaFlopPerSecond);
	// 		// 		}
	// 		// 		// quettaFLOP per second
	// 		// 		(Token::Unit(QuettaFlop), Token::LexerKeyword(Per), Token::Unit(Second)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(QuettaFlopPerSecond);
	// 		// 		}
	// 		// 		// btu/min
	// 		// 		(
	// 		// 			Token::Unit(BritishThermalUnit),
	// 		// 			Token::LexerKeyword(Per),
	// 		// 			Token::Unit(Minute),
	// 		// 		) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(BritishThermalUnitsPerMinute);
	// 		// 		}
	// 		// 		// btu/h
	// 		// 		(Token::Unit(BritishThermalUnit), Token::LexerKeyword(Per), Token::Unit(Hour)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(BritishThermalUnitsPerHour);
	// 		// 		}
	// 		// 		// lbs/sqin
	// 		// 		(
	// 		// 			Token::LexerKeyword(PoundForce),
	// 		// 			Token::LexerKeyword(Per),
	// 		// 			Token::Unit(SquareInch),
	// 		// 		) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(PoundsPerSquareInch);
	// 		// 		}
	// 		// 		// inch of mercury
	// 		// 		(Token::Unit(Inch), Token::TextOperator(Of), Token::LexerKeyword(Mercury)) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(InchOfMercury);
	// 		// 		}
	// 		// 		// revolutions per minute
	// 		// 		(
	// 		// 			Token::LexerKeyword(Revolution),
	// 		// 			Token::LexerKeyword(Per),
	// 		// 			Token::Unit(Minute),
	// 		// 		) => {
	// 		// 			tokens[token_index - 2] = Token::Unit(RevolutionsPerMinute);
	// 		// 		}
	// 		// 		_ => {
	// 		// 			replaced = false;
	// 		// 		}
	// 		// 	}
	// 		// 	if replaced {
	// 		// 		tokens.remove(token_index);
	// 		// 		tokens.remove(token_index - 1);
	// 		// 		token_index -= 2;
	// 		// 	}
	// 	}
	// 	// if token_index == tokens.len() - 1 {
	// 	// 	break;
	// 	// } else {
	// 	// 	token_index += 1;
	// 	// }
	// }
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
			let info_msg = format!(
				"run_lex input: {}\nexpected: {:?}\nreceived: {:?}",
				input, expected_tokens, tokens
			);
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
			let input_afterdigit_stripped_spaces =
				strip_afterdigit_spacing.replace_all(input, "$1");
			let tokens_afterdigit_stripped_spaces =
				lex(&input_afterdigit_stripped_spaces, false).unwrap();
			assert!(
				tokens_afterdigit_stripped_spaces == expected_tokens,
				"{info_msg}"
			);
		};

		let run_datarate_lex = |input: &str, expected_tokens: Vec<Token>| {
			run_lex(input, (*expected_tokens).to_vec());

			// Prove plural and non-plural data units behave identically
			let input_nonplural_units = nonplural_data_units.replace_all(input, "$1");
			let tokens_nonplural_units = lex(&input_nonplural_units, false).unwrap();
			let info_msg = format!(
				"run_datarate_lex input: {}\nexpected: {:?}\nreceived: {:?}",
				input, expected_tokens, tokens_nonplural_units
			);
			assert!(tokens_nonplural_units == expected_tokens, "{info_msg}");
		};

		run_lex(
			"88 kilometres * 2",
			vec![
				numtok!(88),
				Token::Unit(Kilometer),
				Token::Operator(Multiply),
				numtok!(2),
			],
		);
		run_lex("0.5 marathon", vec![numtok!(0.5), Token::Unit(Marathon)]);
		run_lex("100 nmi", vec![numtok!(100), Token::Unit(NauticalMile)]);
		run_lex(
			"101 nautical miles",
			vec![numtok!(101), Token::Unit(NauticalMile)],
		);
		run_lex("2 lightyears", vec![numtok!(2), Token::Unit(LightYear)]);
		run_lex("1 light year", vec![numtok!(1), Token::Unit(LightYear)]);
		run_lex("10 lightsec", vec![numtok!(10), Token::Unit(LightSecond)]);
		run_lex("12 light secs", vec![numtok!(12), Token::Unit(LightSecond)]);
		run_lex(
			"33.3 square meters",
			vec![numtok!(33.3), Token::Unit(SquareMeter)],
		);
		run_lex("54 m2", vec![numtok!(54), Token::Unit(SquareMeter)]);
		run_lex("87 sq miles", vec![numtok!(87), Token::Unit(SquareMile)]);
		run_lex("500 feet2", vec![numtok!(500), Token::Unit(SquareFoot)]);
		run_lex("500 feet²", vec![numtok!(500), Token::Unit(SquareFoot)]);
		run_lex("4 cubic metres", vec![numtok!(4), Token::Unit(CubicMeter)]);
		run_lex(
			"34 cubic feet + 23 cubic yards",
			vec![
				numtok!(34),
				Token::Unit(CubicFoot),
				Token::Operator(Plus),
				numtok!(23),
				Token::Unit(CubicYard),
			],
		);
		run_lex(
			"66 inches3 + 65 millimetre³",
			vec![
				numtok!(66),
				Token::Unit(CubicInch),
				Token::Operator(Plus),
				numtok!(65),
				Token::Unit(CubicMillimeter),
			],
		);
		run_lex(
			"66 inches³ + 65 millimetre3",
			vec![
				numtok!(66),
				Token::Unit(CubicInch),
				Token::Operator(Plus),
				numtok!(65),
				Token::Unit(CubicMillimeter),
			],
		);
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
		run_lex(
			"210 pounds-force",
			vec![numtok!(210), Token::LexerKeyword(PoundForce)],
		);
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
		run_datarate_lex(
			"0.49 zettabytes",
			vec![numtok!(0.49), Token::Unit(Zettabyte)],
		);
		run_datarate_lex(
			"0.23 yottabytes",
			vec![numtok!(0.23), Token::Unit(Yottabyte)],
		);
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
		run_lex(
			"4.2 gbps",
			vec![numtok!(4.2), Token::Unit(GigabitsPerSecond)],
		);
		run_lex(
			"2.2 tbps",
			vec![numtok!(2.2), Token::Unit(TerabitsPerSecond)],
		);
		run_lex(
			"1.7 pbps",
			vec![numtok!(1.7), Token::Unit(PetabitsPerSecond)],
		);
		run_lex(
			"0.99 ebps",
			vec![numtok!(0.99), Token::Unit(ExabitsPerSecond)],
		);
		run_lex(
			"0.64 zbps",
			vec![numtok!(0.64), Token::Unit(ZettabitsPerSecond)],
		);
		run_lex(
			"0.278 ybps",
			vec![numtok!(0.278), Token::Unit(YottabitsPerSecond)],
		);
		run_datarate_lex(
			"4 bits per second",
			vec![numtok!(4), Token::Unit(BitsPerSecond)],
		);
		run_datarate_lex(
			"5 kilobits per second",
			vec![numtok!(5), Token::Unit(KilobitsPerSecond)],
		);
		run_datarate_lex(
			"6 megabits per second",
			vec![numtok!(6), Token::Unit(MegabitsPerSecond)],
		);
		run_datarate_lex(
			"7 gigabits per second",
			vec![numtok!(7), Token::Unit(GigabitsPerSecond)],
		);
		run_datarate_lex(
			"8 terabits per second",
			vec![numtok!(8), Token::Unit(TerabitsPerSecond)],
		);
		run_datarate_lex(
			"9 petabits per second",
			vec![numtok!(9), Token::Unit(PetabitsPerSecond)],
		);
		run_datarate_lex(
			"10 exabits per second",
			vec![numtok!(10), Token::Unit(ExabitsPerSecond)],
		);
		run_datarate_lex(
			"11 zettabits per second",
			vec![numtok!(11), Token::Unit(ZettabitsPerSecond)],
		);
		run_datarate_lex(
			"12 yottabits per second",
			vec![numtok!(12), Token::Unit(YottabitsPerSecond)],
		);
		run_datarate_lex(
			"13 kibibits per second",
			vec![numtok!(13), Token::Unit(KibibitsPerSecond)],
		);
		run_datarate_lex(
			"14 mebibits per second",
			vec![numtok!(14), Token::Unit(MebibitsPerSecond)],
		);
		run_datarate_lex(
			"15 gibibits per second",
			vec![numtok!(15), Token::Unit(GibibitsPerSecond)],
		);
		run_datarate_lex(
			"16 tebibits per second",
			vec![numtok!(16), Token::Unit(TebibitsPerSecond)],
		);
		run_datarate_lex(
			"17 pebibits per second",
			vec![numtok!(17), Token::Unit(PebibitsPerSecond)],
		);
		run_datarate_lex(
			"18 exbibits per second",
			vec![numtok!(18), Token::Unit(ExbibitsPerSecond)],
		);
		run_datarate_lex(
			"19 zebibits per second",
			vec![numtok!(19), Token::Unit(ZebibitsPerSecond)],
		);
		run_datarate_lex(
			"20 yobibits per second",
			vec![numtok!(20), Token::Unit(YobibitsPerSecond)],
		);
		run_datarate_lex(
			"4 bytes per second",
			vec![numtok!(4), Token::Unit(BytesPerSecond)],
		);
		run_datarate_lex(
			"5 kilobytes per second",
			vec![numtok!(5), Token::Unit(KilobytesPerSecond)],
		);
		run_datarate_lex(
			"6 megabytes per second",
			vec![numtok!(6), Token::Unit(MegabytesPerSecond)],
		);
		run_datarate_lex(
			"7 gigabytes per second",
			vec![numtok!(7), Token::Unit(GigabytesPerSecond)],
		);
		run_datarate_lex(
			"8 terabytes per second",
			vec![numtok!(8), Token::Unit(TerabytesPerSecond)],
		);
		run_datarate_lex(
			"9 petabytes per second",
			vec![numtok!(9), Token::Unit(PetabytesPerSecond)],
		);
		run_datarate_lex(
			"10 exabytes per second",
			vec![numtok!(10), Token::Unit(ExabytesPerSecond)],
		);
		run_datarate_lex(
			"11 zettabytes per second",
			vec![numtok!(11), Token::Unit(ZettabytesPerSecond)],
		);
		run_datarate_lex(
			"12 yottabytes per second",
			vec![numtok!(12), Token::Unit(YottabytesPerSecond)],
		);
		run_datarate_lex(
			"13 kibibytes per second",
			vec![numtok!(13), Token::Unit(KibibytesPerSecond)],
		);
		run_datarate_lex(
			"14 mebibytes per second",
			vec![numtok!(14), Token::Unit(MebibytesPerSecond)],
		);
		run_datarate_lex(
			"15 gibibytes per second",
			vec![numtok!(15), Token::Unit(GibibytesPerSecond)],
		);
		run_datarate_lex(
			"16 tebibytes per second",
			vec![numtok!(16), Token::Unit(TebibytesPerSecond)],
		);
		run_datarate_lex(
			"17 pebibytes per second",
			vec![numtok!(17), Token::Unit(PebibytesPerSecond)],
		);
		run_datarate_lex(
			"18 exbibytes per second",
			vec![numtok!(18), Token::Unit(ExbibytesPerSecond)],
		);
		run_datarate_lex(
			"19 zebibytes per second",
			vec![numtok!(19), Token::Unit(ZebibytesPerSecond)],
		);
		run_datarate_lex(
			"20 yobibytes per second",
			vec![numtok!(20), Token::Unit(YobibytesPerSecond)],
		);
		run_lex("1 flop", vec![numtok!(1), Token::Unit(Flop)]);
		run_lex("2 kflop", vec![numtok!(2), Token::Unit(KiloFlop)]);
		run_lex("3 mflop", vec![numtok!(3), Token::Unit(MegaFlop)]);
		run_lex("4 gflop", vec![numtok!(4), Token::Unit(GigaFlop)]);
		run_lex("5 tflop", vec![numtok!(5), Token::Unit(TeraFlop)]);
		run_lex("6 pflop", vec![numtok!(6), Token::Unit(PetaFlop)]);
		run_lex("7 eflop", vec![numtok!(7), Token::Unit(ExaFlop)]);
		run_lex("8 zflop", vec![numtok!(8), Token::Unit(ZettaFlop)]);
		run_lex("9 yflop", vec![numtok!(9), Token::Unit(YottaFlop)]);
		run_lex("10 rflop", vec![numtok!(10), Token::Unit(RonnaFlop)]);
		run_lex("11 qflop", vec![numtok!(11), Token::Unit(QuettaFlop)]);
		run_lex("1 flop/s", vec![numtok!(1), Token::Unit(FlopPerSecond)]);
		run_lex(
			"2 kflop/s",
			vec![numtok!(2), Token::Unit(KiloFlopPerSecond)],
		);
		run_lex(
			"3 mflop/s",
			vec![numtok!(3), Token::Unit(MegaFlopPerSecond)],
		);
		run_lex(
			"4 gflop/s",
			vec![numtok!(4), Token::Unit(GigaFlopPerSecond)],
		);
		run_lex(
			"5 tflop/s",
			vec![numtok!(5), Token::Unit(TeraFlopPerSecond)],
		);
		run_lex(
			"6 pflop/s",
			vec![numtok!(6), Token::Unit(PetaFlopPerSecond)],
		);
		run_lex("7 eflop/s", vec![numtok!(7), Token::Unit(ExaFlopPerSecond)]);
		run_lex(
			"8 zflop/s",
			vec![numtok!(8), Token::Unit(ZettaFlopPerSecond)],
		);
		run_lex(
			"9 yflop/s",
			vec![numtok!(9), Token::Unit(YottaFlopPerSecond)],
		);
		run_lex(
			"10 rflop/s",
			vec![numtok!(10), Token::Unit(RonnaFlopPerSecond)],
		);
		run_lex(
			"11 qflop/s",
			vec![numtok!(11), Token::Unit(QuettaFlopPerSecond)],
		);
		run_lex(
			"1 flop per second",
			vec![numtok!(1), Token::Unit(FlopPerSecond)],
		);
		run_lex(
			"2 kflop per second",
			vec![numtok!(2), Token::Unit(KiloFlopPerSecond)],
		);
		run_lex(
			"3 mflop per second",
			vec![numtok!(3), Token::Unit(MegaFlopPerSecond)],
		);
		run_lex(
			"4 gflop per second",
			vec![numtok!(4), Token::Unit(GigaFlopPerSecond)],
		);
		run_lex(
			"5 tflop per second",
			vec![numtok!(5), Token::Unit(TeraFlopPerSecond)],
		);
		run_lex(
			"6 pflop per second",
			vec![numtok!(6), Token::Unit(PetaFlopPerSecond)],
		);
		run_lex(
			"7 eflop per second",
			vec![numtok!(7), Token::Unit(ExaFlopPerSecond)],
		);
		run_lex(
			"8 zflop per second",
			vec![numtok!(8), Token::Unit(ZettaFlopPerSecond)],
		);
		run_lex(
			"9 yflop per second",
			vec![numtok!(9), Token::Unit(YottaFlopPerSecond)],
		);
		run_lex(
			"10 rflop per second",
			vec![numtok!(10), Token::Unit(RonnaFlopPerSecond)],
		);
		run_lex(
			"11 qflop per second",
			vec![numtok!(11), Token::Unit(QuettaFlopPerSecond)],
		);
		run_lex("1 flops", vec![numtok!(1), Token::Unit(FlopPerSecond)]);
		run_lex("2 kflops", vec![numtok!(2), Token::Unit(KiloFlopPerSecond)]);
		run_lex("3 mflops", vec![numtok!(3), Token::Unit(MegaFlopPerSecond)]);
		run_lex("4 gflops", vec![numtok!(4), Token::Unit(GigaFlopPerSecond)]);
		run_lex("5 tflops", vec![numtok!(5), Token::Unit(TeraFlopPerSecond)]);
		run_lex("6 pflops", vec![numtok!(6), Token::Unit(PetaFlopPerSecond)]);
		run_lex("7 eflops", vec![numtok!(7), Token::Unit(ExaFlopPerSecond)]);
		run_lex(
			"8 zflops",
			vec![numtok!(8), Token::Unit(ZettaFlopPerSecond)],
		);
		run_lex(
			"9 yflops",
			vec![numtok!(9), Token::Unit(YottaFlopPerSecond)],
		);
		run_lex(
			"10 rflops",
			vec![numtok!(10), Token::Unit(RonnaFlopPerSecond)],
		);
		run_lex(
			"11 qflops",
			vec![numtok!(11), Token::Unit(QuettaFlopPerSecond)],
		);
		run_lex("234 wh", vec![numtok!(234), Token::Unit(WattHour)]);
		run_lex("1 w", vec![numtok!(1), Token::Unit(Watt)]);
		run_lex("1 watt", vec![numtok!(1), Token::Unit(Watt)]);
		run_lex("1 watts", vec![numtok!(1), Token::Unit(Watt)]);
		run_lex("1 watt hour", vec![numtok!(1), Token::Unit(WattHour)]);
		run_lex(
			"0 watt + 1 watts",
			vec![
				numtok!(0),
				Token::Unit(Watt),
				Token::Operator(Plus),
				numtok!(1),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"0 watt * 1",
			vec![
				numtok!(0),
				Token::Unit(Watt),
				Token::Operator(Multiply),
				numtok!(1),
			],
		);
		run_lex(
			"2 watts + 3 watts",
			vec![
				numtok!(2),
				Token::Unit(Watt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"2 watts * 3",
			vec![
				numtok!(2),
				Token::Unit(Watt),
				Token::Operator(Multiply),
				numtok!(3),
			],
		);
		run_lex(
			"4 watt plus 5 watts",
			vec![
				numtok!(4),
				Token::Unit(Watt),
				Token::Operator(Plus),
				numtok!(5),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"4 watt times 5",
			vec![
				numtok!(4),
				Token::Unit(Watt),
				Token::Operator(Multiply),
				numtok!(5),
			],
		);
		run_lex(
			"6 watts plus 7 watts",
			vec![
				numtok!(6),
				Token::Unit(Watt),
				Token::Operator(Plus),
				numtok!(7),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"6 watts times 7",
			vec![
				numtok!(6),
				Token::Unit(Watt),
				Token::Operator(Multiply),
				numtok!(7),
			],
		);
		run_lex("2.3 kwh", vec![numtok!(2.3), Token::Unit(KilowattHour)]);
		run_lex("1 kw", vec![numtok!(1), Token::Unit(Kilowatt)]);
		run_lex("1 kilowatt", vec![numtok!(1), Token::Unit(Kilowatt)]);
		run_lex("1 kilowatts", vec![numtok!(1), Token::Unit(Kilowatt)]);
		run_lex(
			"1 kilowatt hour",
			vec![numtok!(1), Token::Unit(KilowattHour)],
		);
		run_lex(
			"2 kilowatt + 3 watt",
			vec![
				numtok!(2),
				Token::Unit(Kilowatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"2 kilowatt * 4",
			vec![
				numtok!(2),
				Token::Unit(Kilowatt),
				Token::Operator(Multiply),
				numtok!(4),
			],
		);
		run_lex(
			"2 kilowatt times 4",
			vec![
				numtok!(2),
				Token::Unit(Kilowatt),
				Token::Operator(Multiply),
				numtok!(4),
			],
		);
		run_lex(
			"2 kilowatt + 3 watts",
			vec![
				numtok!(2),
				Token::Unit(Kilowatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"2 kilowatts + 3 watt",
			vec![
				numtok!(2),
				Token::Unit(Kilowatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"2 kilowatts + 3 watts",
			vec![
				numtok!(2),
				Token::Unit(Kilowatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"2 kilowatt plus 3 watt",
			vec![
				numtok!(2),
				Token::Unit(Kilowatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"2 kilowatt plus 3 watts",
			vec![
				numtok!(2),
				Token::Unit(Kilowatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"2 kilowatts plus 3 watt",
			vec![
				numtok!(2),
				Token::Unit(Kilowatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"2 kilowatts plus 3 watts",
			vec![
				numtok!(2),
				Token::Unit(Kilowatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"6.6 watts + 4 kilowatts",
			vec![
				numtok!(6.6),
				Token::Unit(Watt),
				Token::Operator(Plus),
				numtok!(4),
				Token::Unit(Kilowatt),
			],
		);
		run_lex(
			"6.6 watts plus 4 kilowatts",
			vec![
				numtok!(6.6),
				Token::Unit(Watt),
				Token::Operator(Plus),
				numtok!(4),
				Token::Unit(Kilowatt),
			],
		);
		run_lex("2.3 mwh", vec![numtok!(2.3), Token::Unit(MegawattHour)]);
		run_lex("1 mw", vec![numtok!(1), Token::Unit(Megawatt)]);
		run_lex("1 megawatt", vec![numtok!(1), Token::Unit(Megawatt)]);
		run_lex(
			"1 megawatt hour",
			vec![numtok!(1), Token::Unit(MegawattHour)],
		);
		run_lex(
			"2 megawatt + 3 watt",
			vec![
				numtok!(2),
				Token::Unit(Megawatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"2 megawatt * 6",
			vec![
				numtok!(2),
				Token::Unit(Megawatt),
				Token::Operator(Multiply),
				numtok!(6),
			],
		);
		run_lex(
			"2 megawatt times 6",
			vec![
				numtok!(2),
				Token::Unit(Megawatt),
				Token::Operator(Multiply),
				numtok!(6),
			],
		);
		run_lex(
			"2 megawatt + 3 watts",
			vec![
				numtok!(2),
				Token::Unit(Megawatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"2 megawatts + 3 watt",
			vec![
				numtok!(2),
				Token::Unit(Megawatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"2 megawatts + 3 watts",
			vec![
				numtok!(2),
				Token::Unit(Megawatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"2 megawatt plus 3 watt",
			vec![
				numtok!(2),
				Token::Unit(Megawatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"2 megawatt plus 3 watts",
			vec![
				numtok!(2),
				Token::Unit(Megawatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"2 megawatts plus 3 watt",
			vec![
				numtok!(2),
				Token::Unit(Megawatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"2 megawatts plus 3 watts",
			vec![
				numtok!(2),
				Token::Unit(Megawatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"6.6 watts + 4 megawatts",
			vec![
				numtok!(6.6),
				Token::Unit(Watt),
				Token::Operator(Plus),
				numtok!(4),
				Token::Unit(Megawatt),
			],
		);
		run_lex(
			"6.6 watts plus 4 megawatts",
			vec![
				numtok!(6.6),
				Token::Unit(Watt),
				Token::Operator(Plus),
				numtok!(4),
				Token::Unit(Megawatt),
			],
		);
		run_lex("234 gwh", vec![numtok!(234), Token::Unit(GigawattHour)]);
		run_lex("1 gw", vec![numtok!(1), Token::Unit(Gigawatt)]);
		run_lex("1 gigawatt", vec![numtok!(1), Token::Unit(Gigawatt)]);
		run_lex("1 gigawatts", vec![numtok!(1), Token::Unit(Gigawatt)]);
		run_lex(
			"1 gigawatt hour",
			vec![numtok!(1), Token::Unit(GigawattHour)],
		);
		run_lex(
			"0 gigawatt + 1 gigawatts",
			vec![
				numtok!(0),
				Token::Unit(Gigawatt),
				Token::Operator(Plus),
				numtok!(1),
				Token::Unit(Gigawatt),
			],
		);
		run_lex(
			"0 gigawatt * 1",
			vec![
				numtok!(0),
				Token::Unit(Gigawatt),
				Token::Operator(Multiply),
				numtok!(1),
			],
		);
		run_lex(
			"2 gigawatts + 3 gigawatts",
			vec![
				numtok!(2),
				Token::Unit(Gigawatt),
				Token::Operator(Plus),
				numtok!(3),
				Token::Unit(Gigawatt),
			],
		);
		run_lex(
			"2 gigawatts * 3",
			vec![
				numtok!(2),
				Token::Unit(Gigawatt),
				Token::Operator(Multiply),
				numtok!(3),
			],
		);
		run_lex(
			"4 gigawatt plus 5 watt",
			vec![
				numtok!(4),
				Token::Unit(Gigawatt),
				Token::Operator(Plus),
				numtok!(5),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"4 gigawatt plus 5 megawatt",
			vec![
				numtok!(4),
				Token::Unit(Gigawatt),
				Token::Operator(Plus),
				numtok!(5),
				Token::Unit(Megawatt),
			],
		);
		run_lex(
			"4 gigawatt plus 5 gigawatt",
			vec![
				numtok!(4),
				Token::Unit(Gigawatt),
				Token::Operator(Plus),
				numtok!(5),
				Token::Unit(Gigawatt),
			],
		);
		run_lex(
			"4 gigawatt plus 5 watts",
			vec![
				numtok!(4),
				Token::Unit(Gigawatt),
				Token::Operator(Plus),
				numtok!(5),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"4 gigawatt plus 5 megawatts",
			vec![
				numtok!(4),
				Token::Unit(Gigawatt),
				Token::Operator(Plus),
				numtok!(5),
				Token::Unit(Megawatt),
			],
		);
		run_lex(
			"4 gigawatt plus 5 gigawatts",
			vec![
				numtok!(4),
				Token::Unit(Gigawatt),
				Token::Operator(Plus),
				numtok!(5),
				Token::Unit(Gigawatt),
			],
		);
		run_lex(
			"4 gigawatt times 5",
			vec![
				numtok!(4),
				Token::Unit(Gigawatt),
				Token::Operator(Multiply),
				numtok!(5),
			],
		);
		run_lex(
			"6 gigawatts plus 7 watt",
			vec![
				numtok!(6),
				Token::Unit(Gigawatt),
				Token::Operator(Plus),
				numtok!(7),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"6 gigawatts plus 7 megawatt",
			vec![
				numtok!(6),
				Token::Unit(Gigawatt),
				Token::Operator(Plus),
				numtok!(7),
				Token::Unit(Megawatt),
			],
		);
		run_lex(
			"6 gigawatts plus 7 gigawatt",
			vec![
				numtok!(6),
				Token::Unit(Gigawatt),
				Token::Operator(Plus),
				numtok!(7),
				Token::Unit(Gigawatt),
			],
		);
		run_lex(
			"6 gigawatts plus 7 watts",
			vec![
				numtok!(6),
				Token::Unit(Gigawatt),
				Token::Operator(Plus),
				numtok!(7),
				Token::Unit(Watt),
			],
		);
		run_lex(
			"6 gigawatts plus 7 megawatts",
			vec![
				numtok!(6),
				Token::Unit(Gigawatt),
				Token::Operator(Plus),
				numtok!(7),
				Token::Unit(Megawatt),
			],
		);
		run_lex(
			"6 gigawatts plus 7 gigawatts",
			vec![
				numtok!(6),
				Token::Unit(Gigawatt),
				Token::Operator(Plus),
				numtok!(7),
				Token::Unit(Gigawatt),
			],
		);
		run_lex(
			"6 gigawatts times 7",
			vec![
				numtok!(6),
				Token::Unit(Gigawatt),
				Token::Operator(Multiply),
				numtok!(7),
			],
		);
		run_lex(
			"88 mw * 3",
			vec![
				numtok!(88),
				Token::Unit(Megawatt),
				Token::Operator(Multiply),
				numtok!(3),
			],
		);
		run_lex(
			"88 mw times 3",
			vec![
				numtok!(88),
				Token::Unit(Megawatt),
				Token::Operator(Multiply),
				numtok!(3),
			],
		);
		run_lex("999 kb", vec![numtok!(999), Token::Unit(Kilobyte)]);
		run_lex(
			"200 gb - 100 mb",
			vec![
				numtok!(200),
				Token::Unit(Gigabyte),
				Token::Operator(Minus),
				numtok!(100),
				Token::Unit(Megabyte),
			],
		);
		run_lex("999 kib", vec![numtok!(999), Token::Unit(Kibibyte)]);
		run_lex(
			"200 gib - 100 mib",
			vec![
				numtok!(200),
				Token::Unit(Gibibyte),
				Token::Operator(Minus),
				numtok!(100),
				Token::Unit(Mebibyte),
			],
		);
		run_lex("45 btu", vec![numtok!(45), Token::Unit(BritishThermalUnit)]);
		run_lex(
			"45.5 british thermal unit",
			vec![numtok!(45.5), Token::Unit(BritishThermalUnit)],
		);
		run_lex(
			"46 british thermal units",
			vec![numtok!(46), Token::Unit(BritishThermalUnit)],
		);
		run_lex(
			"5432 newton metres",
			vec![numtok!(5432), Token::Unit(NewtonMeter)],
		);
		run_lex(
			"2345 newton-meters",
			vec![numtok!(2345), Token::Unit(NewtonMeter)],
		);
		run_lex("20 lbf", vec![numtok!(20), Token::LexerKeyword(PoundForce)]);
		run_lex("60 hz", vec![numtok!(60), Token::Unit(Hertz)]);
		run_lex(
			"1100 rpm",
			vec![numtok!(1100), Token::Unit(RevolutionsPerMinute)],
		);
		run_lex(
			"1150 revolutions per minute",
			vec![numtok!(1150), Token::Unit(RevolutionsPerMinute)],
		);
		run_lex(
			"1 revolution per min",
			vec![numtok!(1), Token::Unit(RevolutionsPerMinute)],
		);
		run_lex(
			"4 revolution / mins",
			vec![numtok!(4), Token::Unit(RevolutionsPerMinute)],
		);
		run_lex(
			"1250 r / min",
			vec![numtok!(1250), Token::Unit(RevolutionsPerMinute)],
		);
		run_lex(
			"1300 rev / min",
			vec![numtok!(1300), Token::Unit(RevolutionsPerMinute)],
		);
		run_lex(
			"1350 rev / minute",
			vec![numtok!(1350), Token::Unit(RevolutionsPerMinute)],
		);
		run_lex(
			"1250 r per min",
			vec![numtok!(1250), Token::Unit(RevolutionsPerMinute)],
		);
		run_lex(
			"1300 rev per min",
			vec![numtok!(1300), Token::Unit(RevolutionsPerMinute)],
		);
		run_lex(
			"1350 rev per minute",
			vec![numtok!(1350), Token::Unit(RevolutionsPerMinute)],
		);
		run_lex(
			"100 kph",
			vec![numtok!(100), Token::Unit(KilometersPerHour)],
		);
		run_lex(
			"100 kmh",
			vec![numtok!(100), Token::Unit(KilometersPerHour)],
		);
		run_lex(
			"100 kilometers per hour",
			vec![numtok!(100), Token::Unit(KilometersPerHour)],
		);
		run_lex(
			"100 kilometre / hrs",
			vec![numtok!(100), Token::Unit(KilometersPerHour)],
		);
		run_lex("3.6 mps", vec![numtok!(3.6), Token::Unit(MetersPerSecond)]);
		run_lex(
			"3.6 meters per second",
			vec![numtok!(3.6), Token::Unit(MetersPerSecond)],
		);
		run_lex(
			"3.6 metre / secs",
			vec![numtok!(3.6), Token::Unit(MetersPerSecond)],
		);
		run_lex("60 mph", vec![numtok!(60), Token::Unit(MilesPerHour)]);
		run_lex(
			"60 miles per hour",
			vec![numtok!(60), Token::Unit(MilesPerHour)],
		);
		run_lex("60 mile / hr", vec![numtok!(60), Token::Unit(MilesPerHour)]);
		run_lex("35 fps", vec![numtok!(35), Token::Unit(FeetPerSecond)]);
		run_lex("35 ft / sec", vec![numtok!(35), Token::Unit(FeetPerSecond)]);
		run_lex(
			"35 ft per seconds",
			vec![numtok!(35), Token::Unit(FeetPerSecond)],
		);
		run_lex(
			"35 foot / secs",
			vec![numtok!(35), Token::Unit(FeetPerSecond)],
		);
		run_lex(
			"35 foot per seconds",
			vec![numtok!(35), Token::Unit(FeetPerSecond)],
		);
		run_lex(
			"35 feet / sec",
			vec![numtok!(35), Token::Unit(FeetPerSecond)],
		);
		run_lex(
			"35 feet per second",
			vec![numtok!(35), Token::Unit(FeetPerSecond)],
		);
		run_lex("30 pa", vec![numtok!(30), Token::Unit(Pascal)]);
		run_lex(
			"23 celsius + 4 celsius",
			vec![
				numtok!(23),
				Token::Unit(Celsius),
				Token::Operator(Plus),
				numtok!(4),
				Token::Unit(Celsius),
			],
		);
		run_lex(
			"54 f - 1.5 fahrenheit",
			vec![
				numtok!(54),
				Token::Unit(Fahrenheit),
				Token::Operator(Minus),
				numtok!(1.5),
				Token::Unit(Fahrenheit),
			],
		);
		run_lex(
			"50 metric tonnes",
			vec![numtok!(50), Token::Unit(MetricTon)],
		);
		run_lex(
			"77 metric hps",
			vec![numtok!(77), Token::Unit(MetricHorsepower)],
		);

		run_lex(
			"100 + 99",
			vec![numtok!(100), Token::Operator(Plus), numtok!(99)],
		);
		run_lex(
			"100 plus 99",
			vec![numtok!(100), Token::Operator(Plus), numtok!(99)],
		);
		run_lex(
			"12 - 4",
			vec![numtok!(12), Token::Operator(Minus), numtok!(4)],
		);
		run_lex(
			"12 minus 4",
			vec![numtok!(12), Token::Operator(Minus), numtok!(4)],
		);
		run_lex(
			"50.5 * 2",
			vec![numtok!(50.5), Token::Operator(Multiply), numtok!(2)],
		);
		run_lex(
			"50.5 times 2",
			vec![numtok!(50.5), Token::Operator(Multiply), numtok!(2)],
		);
		run_lex(
			"50.5 multiplied by 2",
			vec![numtok!(50.5), Token::Operator(Multiply), numtok!(2)],
		);
		run_lex(
			"6 / 3",
			vec![numtok!(6), Token::Operator(Divide), numtok!(3)],
		);
		run_lex(
			"50 / 10",
			vec![numtok!(50), Token::Operator(Divide), numtok!(10)],
		);
		run_lex(
			"52 ÷ 12",
			vec![numtok!(52), Token::Operator(Divide), numtok!(12)],
		);
		run_lex(
			"6 divided by 3",
			vec![numtok!(6), Token::Operator(Divide), numtok!(3)],
		);
		run_lex(
			"7 mod 5",
			vec![numtok!(7), Token::Operator(Modulo), numtok!(5)],
		);

		run_lex(
			"(2 + 3) * 4",
			vec![
				Token::Operator(LeftParen),
				numtok!(2),
				Token::Operator(Plus),
				numtok!(3),
				Token::Operator(RightParen),
				Token::Operator(Multiply),
				numtok!(4),
			],
		);
		run_lex(
			"52 weeks * (12 hrs + 12 hours)",
			vec![
				numtok!(52),
				Token::Unit(Week),
				Token::Operator(Multiply),
				Token::Operator(LeftParen),
				numtok!(12),
				Token::Unit(Hour),
				Token::Operator(Plus),
				numtok!(12),
				Token::Unit(Hour),
				Token::Operator(RightParen),
			],
		);
		run_lex(
			"12 pound+",
			vec![numtok!(12), Token::Unit(Pound), Token::Operator(Plus)],
		);

		run_lex(
			"5 π m",
			vec![numtok!(5), Token::Constant(Pi), Token::Unit(Meter)],
		);
		run_lex(
			"5 Ω + 2 mΩ",
			vec![
				numtok!(5),
				Token::Unit(Ohm),
				Token::Operator(Plus),
				numtok!(2),
				Token::Unit(Milliohm),
			],
		);
	}
}
