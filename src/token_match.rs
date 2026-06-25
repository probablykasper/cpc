use crate::Constant::*;
use crate::FunctionIdentifier::*;
use crate::LexerKeyword::*;
use crate::NamedNumber::*;
use crate::Operator::*;
use crate::TextOperator::*;
use crate::Token;
use crate::units::Unit::*;

macro_rules! token_match {
    // Handle OR patterns: ("kph", "kilometers per hour") => ...
    (($($alt:literal),+ ) => $token:expr, $($rest:tt)*) => {
        token_match!(
            $($alt => $token),*,
            $($rest)*
        );
    };

    // Multiple patterns
    ($pattern:tt => $token:expr, $($rest:tt)*) => {
        __token_match_internal! {
            patterns: [ $pattern => $token, $($rest)* ]
        }
    };

    // Single pattern
    ($pattern:tt => $token:expr) => {
        __token_match_internal! {
            patterns: [ $pattern => $token ]
        }
    };
}

macro_rules! __token_match_internal {
    (patterns: [ $($pattern:tt => $token:expr),+ ]) => {
        match words[0] {
            $(
                __first_word!($pattern) => __token_match_next! {
                    words: words,
                    depth: 1,
                    patterns: [ __rest_words!($pattern) => $token ]
                },
            )+
            _ => unreachable!(),
        }
    };
}

macro_rules! __first_word {
	($word:literal) => {
		$word
	};
	($word:literal $($rest:tt)*) => {
		$word
	};
}

macro_rules! __rest_words {
    ($word:literal) => { };
    ($word:literal $($rest:tt)*) => { $($rest)* };
}

macro_rules! __token_match_next {
    (words: $w:ident, depth: $d:literal, patterns: [ => $token:expr ]) => { $token };
    (words: $w:ident, depth: $d:literal, patterns: [ $first:literal $($rest:tt)* => $token:expr ]) => {
        match $w[$d] {
            $first => __token_match_next! {
                words: $w,
                depth: $d + 1,
                patterns: [ $($rest)* => $token ]
            },
            _ => unreachable!(),
        }
    };
}

fn token_match() -> Result<Token, String> {
	token_match!(
		"meter" => Token::Unit(Meter),
		"sq meter" => Token::Unit(SquareMeter),
		("kph", "kilometers per hour") => Token::Unit(KilometersPerHour),
	);
	// trie.try_add("to", Token::TextOperator(To))?;
	// trie.try_add("of", Token::TextOperator(Of))?;

	// trie.try_add("hundred", Token::NamedNumber(Hundred))?;
	// trie.try_add("thousand", Token::NamedNumber(Thousand))?;
	// trie.try_add_multi(&["mil", "mill", "million"], Token::NamedNumber(Million))?;
	// trie.try_add_multi(&["bil", "bill", "billion"], Token::NamedNumber(Billion))?;
	// trie.try_add_multi(&["tri", "tril", "trillion"], Token::NamedNumber(Trillion))?;
	// trie.try_add("quadrillion", Token::NamedNumber(Quadrillion))?;
	// trie.try_add("quintillion", Token::NamedNumber(Quintillion))?;
	// trie.try_add("sextillion", Token::NamedNumber(Sextillion))?;
	// trie.try_add("septillion", Token::NamedNumber(Septillion))?;
	// trie.try_add("octillion", Token::NamedNumber(Octillion))?;
	// trie.try_add("nonillion", Token::NamedNumber(Nonillion))?;
	// trie.try_add("decillion", Token::NamedNumber(Decillion))?;
	// trie.try_add("undecillion", Token::NamedNumber(Undecillion))?;
	// trie.try_add("duodecillion", Token::NamedNumber(Duodecillion))?;
	// trie.try_add("tredecillion", Token::NamedNumber(Tredecillion))?;
	// trie.try_add("quattuordecillion", Token::NamedNumber(Quattuordecillion))?;
	// trie.try_add("quindecillion", Token::NamedNumber(Quindecillion))?;
	// trie.try_add("sexdecillion", Token::NamedNumber(Sexdecillion))?;
	// trie.try_add("septendecillion", Token::NamedNumber(Septendecillion))?;
	// trie.try_add("octodecillion", Token::NamedNumber(Octodecillion))?;
	// trie.try_add("novemdecillion", Token::NamedNumber(Novemdecillion))?;
	// trie.try_add("vigintillion", Token::NamedNumber(Vigintillion))?;
	// trie.try_add("centillion", Token::NamedNumber(Centillion))?;
	// trie.try_add("googol", Token::NamedNumber(Googol))?;

	// trie.try_add("pi", Token::Constant(Pi))?;
	// trie.try_add("e", Token::Constant(E))?;

	// trie.try_add("plus", Token::Operator(Plus))?;
	// trie.try_add("minus", Token::Operator(Minus))?;
	// trie.try_add("times", Token::Operator(Multiply))?;
	// trie.try_add("multiplied by", Token::Operator(Multiply))?;
	// trie.try_add("divide by", Token::Operator(Divide))?;
	// trie.try_add("mod", Token::Operator(Modulo))?;

	// trie.try_add("sqrt", Token::FunctionIdentifier(Sqrt))?;
	// trie.try_add("cbrt", Token::FunctionIdentifier(Cbrt))?;

	// trie.try_add("log", Token::FunctionIdentifier(Log))?;
	// trie.try_add("ln", Token::FunctionIdentifier(Ln))?;
	// trie.try_add("exp", Token::FunctionIdentifier(Exp))?;

	// trie.try_add_multi(&["round", "rint"], Token::FunctionIdentifier(Round))?;
	// trie.try_add("ceil", Token::FunctionIdentifier(Ceil))?;
	// trie.try_add("floor", Token::FunctionIdentifier(Floor))?;
	// trie.try_add_multi(&["abs", "fabs"], Token::FunctionIdentifier(Abs))?;

	// trie.try_add("sin", Token::FunctionIdentifier(Sin))?;
	// trie.try_add("cos", Token::FunctionIdentifier(Cos))?;
	// trie.try_add("tan", Token::FunctionIdentifier(Tan))?;

	// trie.try_add("per", Token::LexerKeyword(Per))?;
	// trie.try_add("hg", Token::LexerKeyword(Hg))?; // can be hectogram or mercury

	// trie.try_add_multi(
	// 	&["ns", "nanosec", "nanosecs", "nanosecond", "nanoseconds"],
	// 	Token::Unit(Nanosecond),
	// )?;
	// // // µ and μ are two different characters
	// trie.try_add_multi(
	// 	&[
	// 		"µs",
	// 		"μs",
	// 		"microsec",
	// 		"microsecs",
	// 		"microsecond",
	// 		"microseconds",
	// 	],
	// 	Token::Unit(Microsecond),
	// )?;
	// trie.try_add_multi(
	// 	&["ms", "millisec", "millisecs", "millisecond", "milliseconds"],
	// 	Token::Unit(Millisecond),
	// )?;
	// trie.try_add_multi(
	// 	&["s", "sec", "secs", "second", "seconds"],
	// 	Token::Unit(Second),
	// )?;
	// trie.try_add_multi(&["min", "mins", "minute", "minutes"], Token::Unit(Minute))?;
	// trie.try_add_multi(&["h", "hr", "hrs", "hour", "hours"], Token::Unit(Hour))?;
	// trie.try_add_multi(&["day", "days"], Token::Unit(Day))?;
	// trie.try_add_multi(&["wk", "wks", "week", "weeks"], Token::Unit(Week))?;
	// trie.try_add_multi(&["mo", "mos", "month", "months"], Token::Unit(Month))?;
	// trie.try_add_multi(&["q", "quarter", "quarters"], Token::Unit(Quarter))?;
	// trie.try_add_multi(&["yr", "yrs", "year", "years"], Token::Unit(Year))?;
	// trie.try_add_multi(&["decade", "decades"], Token::Unit(Decade))?;
	// trie.try_add_multi(&["century", "centuries"], Token::Unit(Century))?;
	// trie.try_add_multi(
	// 	&["millenium", "millenia", "milleniums"],
	// 	Token::Unit(Millennium),
	// )?;

	// trie.try_add_multi(
	// 	&[
	// 		"mm",
	// 		"millimeter",
	// 		"millimeters",
	// 		"millimetre",
	// 		"millimetres",
	// 	],
	// 	Token::Unit(Millimeter),
	// )?;
	// trie.try_add_multi(
	// 	&[
	// 		"cm",
	// 		"centimeter",
	// 		"centimeters",
	// 		"centimetre",
	// 		"centimetres",
	// 	],
	// 	Token::Unit(Centimeter),
	// )?;
	// trie.try_add_multi(
	// 	&["dm", "decimeter", "decimeters", "decimetre", "decimetres"],
	// 	Token::Unit(Decimeter),
	// )?;
	// trie.try_add_multi(
	// 	&["m", "meter", "meters", "metre", "metres"],
	// 	Token::Unit(Meter),
	// )?;
	// trie.try_add_multi(
	// 	&["km", "kilometer", "kilometers", "kilometre", "kilometres"],
	// 	Token::Unit(Kilometer),
	// )?;
	// trie.try_add("in", Token::LexerKeyword(In))?;
	// trie.try_add_multi(&["inch", "inches"], Token::Unit(Inch))?;
	// trie.try_add_multi(&["ft", "foot", "feet"], Token::Unit(Foot))?;
	// trie.try_add_multi(&["yd", "yard", "yards"], Token::Unit(Yard))?;
	// trie.try_add_multi(&["mi", "mile", "miles"], Token::Unit(Mile))?;
	// trie.try_add_multi(&["marathon", "marathons"], Token::Unit(Marathon))?;
	// trie.try_add("nmi", Token::Unit(NauticalMile))?;
	// trie.try_add_multi(
	// 	&["nautical mile", "nautical miles"],
	// 	Token::Unit(NauticalMile),
	// )?;
	// trie.try_add_multi(&["ly", "lightyear", "lightyears"], Token::Unit(LightYear))?;
	// trie.try_add_multi(
	// 	&["lightsec", "lightsecs", "lightsecond", "lightseconds"],
	// 	Token::Unit(LightSecond),
	// )?;
	// trie.try_add_multi(
	// 	&["light yr", "light yrs", "light year", "light years"],
	// 	Token::Unit(LightYear),
	// )?;
	// trie.try_add_multi(
	// 	&["light sec", "light secs", "light second", "light seconds"],
	// 	Token::Unit(LightSecond),
	// )?;

	// trie.try_add_multi(
	// 	&[
	// 		"sqmm",
	// 		"mm2",
	// 		"millimeter2",
	// 		"millimeters2",
	// 		"millimetre2",
	// 		"millimetres2",
	// 	],
	// 	Token::Unit(SquareMillimeter),
	// )?;
	// trie.try_add_multi(
	// 	&[
	// 		"sqcm",
	// 		"cm2",
	// 		"centimeter2",
	// 		"centimeters2",
	// 		"centimetre2",
	// 		"centimetres2",
	// 	],
	// 	Token::Unit(SquareCentimeter),
	// )?;
	// trie.try_add_multi(
	// 	&[
	// 		"sqdm",
	// 		"dm2",
	// 		"decimeter2",
	// 		"decimeters2",
	// 		"decimetre2",
	// 		"decimetres2",
	// 	],
	// 	Token::Unit(SquareDecimeter),
	// )?;
	// trie.try_add_multi(
	// 	&["sqm", "m2", "meter2", "meters2", "metre2", "metres2"],
	// 	Token::Unit(SquareMeter),
	// )?;
	// trie.try_add_multi(
	// 	&[
	// 		"sqkm",
	// 		"km2",
	// 		"kilometer2",
	// 		"kilometers2",
	// 		"kilometre2",
	// 		"kilometres2",
	// 	],
	// 	Token::Unit(SquareKilometer),
	// )?;
	// trie.try_add_multi(
	// 	&["sqin", "in2", "inch2", "inches2"],
	// 	Token::Unit(SquareInch),
	// )?;
	// trie.try_add_multi(&["sqft", "ft2", "foot2", "feet2"], Token::Unit(SquareFoot))?;
	// trie.try_add_multi(&["sqyd", "yd2", "yard2", "yards2"], Token::Unit(SquareYard))?;
	// trie.try_add_multi(&["sqmi", "mi2", "mile2", "miles2"], Token::Unit(SquareMile))?;
	// let square_entries = [
	// 	(
	// 		&[
	// 			"mm",
	// 			"millimeter",
	// 			"millimeters",
	// 			"millimetre",
	// 			"millimetres",
	// 		][..],
	// 		Token::Unit(SquareMillimeter),
	// 	),
	// 	(
	// 		&[
	// 			"cm",
	// 			"centimeter",
	// 			"centimeters",
	// 			"centimetre",
	// 			"centimetres",
	// 		][..],
	// 		Token::Unit(SquareCentimeter),
	// 	),
	// 	(
	// 		&["dm", "decimeter", "decimeters", "decimetre", "decimetres"][..],
	// 		Token::Unit(SquareDecimeter),
	// 	),
	// 	(
	// 		&["m", "meter", "meters", "metre", "metres"][..],
	// 		Token::Unit(SquareMeter),
	// 	),
	// 	(
	// 		&["km", "kilometer", "kilometers", "kilometre", "kilometres"][..],
	// 		Token::Unit(SquareKilometer),
	// 	),
	// 	(&["in", "inch", "inches"][..], Token::Unit(SquareInch)),
	// 	(&["ft", "foot", "feet"][..], Token::Unit(SquareFoot)),
	// 	(&["yd", "yard", "yards"][..], Token::Unit(SquareYard)),
	// 	(&["mi", "mile", "miles"][..], Token::Unit(SquareMile)),
	// ];
	// for entry in square_entries {
	// 	for key in entry.0 {
	// 		trie.try_add(&format!("sq {key}"), entry.1.clone())?;
	// 		trie.try_add(&format!("square {key}"), entry.1.clone())?;
	// 	}
	// }
	// trie.try_add_multi(&["are", "ares"], Token::Unit(Are))?;
	// trie.try_add_multi(&["decare", "decares"], Token::Unit(Decare))?;
	// trie.try_add_multi(&["ha", "hectare", "hectares"], Token::Unit(Hectare))?;
	// trie.try_add_multi(&["acre", "acres"], Token::Unit(Acre))?;

	// trie.try_add_multi(
	// 	&[
	// 		"mm3",
	// 		"millimeter3",
	// 		"millimeters3",
	// 		"millimetre3",
	// 		"millimetres3",
	// 	],
	// 	Token::Unit(CubicMillimeter),
	// )?;
	// trie.try_add_multi(
	// 	&[
	// 		"cm3",
	// 		"centimeter3",
	// 		"centimeters3",
	// 		"centimetre3",
	// 		"centimetres3",
	// 	],
	// 	Token::Unit(CubicCentimeter),
	// )?;
	// trie.try_add_multi(
	// 	&[
	// 		"dm3",
	// 		"decimeter3",
	// 		"decimeters3",
	// 		"decimetre3",
	// 		"decimetres3",
	// 	],
	// 	Token::Unit(CubicDecimeter),
	// )?;
	// trie.try_add_multi(
	// 	&["m3", "meter3", "meters3", "metre3", "metres3"],
	// 	Token::Unit(CubicMeter),
	// )?;
	// trie.try_add_multi(
	// 	&[
	// 		"km3",
	// 		"kilometer3",
	// 		"kilometers3",
	// 		"kilometre3",
	// 		"kilometres3",
	// 	],
	// 	Token::Unit(CubicKilometer),
	// )?;
	// trie.try_add_multi(&["inc3", "inch3", "inches3"], Token::Unit(CubicInch))?;
	// trie.try_add_multi(&["ft3", "foot3", "feet3"], Token::Unit(CubicFoot))?;
	// trie.try_add_multi(&["yd3", "yard3", "yards3"], Token::Unit(CubicYard))?;
	// trie.try_add_multi(&["mi3", "mile3", "miles3"], Token::Unit(CubicMile))?;
	// let cubic_entries = &[
	// 	(
	// 		&[
	// 			"mm",
	// 			"millimeter",
	// 			"millimeters",
	// 			"millimetre",
	// 			"millimetres",
	// 		][..],
	// 		Token::Unit(CubicMillimeter),
	// 	),
	// 	(
	// 		&[
	// 			"cm",
	// 			"centimeter",
	// 			"centimeters",
	// 			"centimetre",
	// 			"centimetres",
	// 		][..],
	// 		Token::Unit(CubicCentimeter),
	// 	),
	// 	(
	// 		&["dm", "decimeter", "decimeters", "decimetre", "decimetres"][..],
	// 		Token::Unit(CubicDecimeter),
	// 	),
	// 	(
	// 		&["m", "meter", "meters", "metre", "metres"][..],
	// 		Token::Unit(CubicMeter),
	// 	),
	// 	(
	// 		&["km", "kilometer", "kilometers", "kilometre", "kilometres"][..],
	// 		Token::Unit(CubicKilometer),
	// 	),
	// 	(&["in", "inch", "inches"][..], Token::Unit(CubicInch)),
	// 	(&["ft", "foot", "feet"][..], Token::Unit(CubicFoot)),
	// 	(&["yd", "yard", "yards"][..], Token::Unit(CubicYard)),
	// 	(&["mi", "mile", "miles"][..], Token::Unit(CubicMile)),
	// ];
	// for entry in cubic_entries {
	// 	for key in entry.0 {
	// 		trie.try_add(format!("cubic {key}"), entry.1.clone())?;
	// 	}
	// }

	// trie.try_add_multi(
	// 	&[
	// 		"ml",
	// 		"milliliter",
	// 		"milliliters",
	// 		"millilitre",
	// 		"millilitres",
	// 	],
	// 	Token::Unit(Milliliter),
	// )?;
	// trie.try_add_multi(
	// 	&[
	// 		"cl",
	// 		"centiliter",
	// 		"centiliters",
	// 		"centilitre",
	// 		"centilitres",
	// 	],
	// 	Token::Unit(Centiliter),
	// )?;
	// trie.try_add_multi(
	// 	&["dl", "deciliter", "deciliters", "decilitre", "decilitres"],
	// 	Token::Unit(Deciliter),
	// )?;
	// trie.try_add_multi(
	// 	&["l", "liter", "liters", "litre", "litres"],
	// 	Token::Unit(Liter),
	// )?;
	// trie.try_add_multi(
	// 	&["ts", "tsp", "tspn", "tspns", "teaspoon", "teaspoons"],
	// 	Token::Unit(Teaspoon),
	// )?;
	// trie.try_add_multi(
	// 	&["tbs", "tbsp", "tablespoon", "tablespoons"],
	// 	Token::Unit(Tablespoon),
	// )?;
	// trie.try_add_multi(
	// 	&[
	// 		"floz",
	// 		"fl oz",
	// 		"fl ounce",
	// 		"fl ounces",
	// 		"fluid oz",
	// 		"fluid ounce",
	// 		"fluid ounces",
	// 	],
	// 	Token::Unit(FluidOunce),
	// )?;
	// trie.try_add_multi(&["cup", "cups"], Token::Unit(Cup))?;
	// trie.try_add_multi(&["pt", "pint", "pints"], Token::Unit(Pint))?;
	// trie.try_add_multi(&["qt", "quart", "quarts"], Token::Unit(Quart))?;
	// trie.try_add_multi(&["gal", "gallon", "gallons"], Token::Unit(Gallon))?;
	// trie.try_add_multi(
	// 	&["bbl", "oil barrel", "oil barrels"],
	// 	Token::Unit(OilBarrel),
	// )?;

	// trie.try_add_multi(
	// 	&["metric ton", "metric tons", "metric tonne", "metric tonnes"],
	// 	Token::Unit(MetricTon),
	// )?;
	// trie.try_add_multi(
	// 	&[
	// 		"metric hp",
	// 		"metric hps",
	// 		"metric horsepower",
	// 		"metric horsepowers",
	// 	],
	// 	Token::Unit(MetricHorsepower),
	// )?;

	// trie.try_add_multi(&["mg", "milligram", "milligrams"], Token::Unit(Milligram))?;
	// trie.try_add_multi(&["g", "gram", "grams"], Token::Unit(Gram))?;
	// trie.try_add_multi(&["hectogram", "hectograms"], Token::Unit(Hectogram))?;
	// trie.try_add_multi(
	// 	&["kg", "kilo", "kilos", "kilogram", "kilograms"],
	// 	Token::Unit(Kilogram),
	// )?;
	// trie.try_add_multi(&["t", "tonne", "tonnes"], Token::Unit(MetricTon))?;
	// trie.try_add_multi(&["oz", "ounces"], Token::Unit(Ounce))?;
	// trie.try_add_multi(&["lb", "lbs"], Token::Unit(Pound))?;
	// // TODO: add ["pound-force", "pounds-force", "pound force", "pounds force"]
	// trie.try_add_multi(&["stone", "stones"], Token::Unit(Stone))?;
	// trie.try_add_multi(
	// 	&[
	// 		"st",
	// 		"ton",
	// 		"tons",
	// 		"short ton",
	// 		"short tons",
	// 		"short tonne",
	// 		"short tonnes",
	// 	],
	// 	Token::Unit(ShortTon),
	// )?;
	// trie.try_add_multi(
	// 	&["lt", "long ton", "long tons", "long tonne", "long tonnes"],
	// 	Token::Unit(LongTon),
	// )?;

	// trie.try_add_multi(&["bit", "bits"], Token::Unit(Bit))?;
	// trie.try_add_multi(&["kbit", "kilobit", "kilobits"], Token::Unit(Kilobit))?;
	// trie.try_add_multi(&["mbit", "megabit", "megabits"], Token::Unit(Megabit))?;
	// trie.try_add_multi(&["gbit", "gigabit", "gigabits"], Token::Unit(Gigabit))?;
	// trie.try_add_multi(&["tbit", "terabit", "terabits"], Token::Unit(Terabit))?;
	// trie.try_add_multi(&["pbit", "petabit", "petabits"], Token::Unit(Petabit))?;
	// trie.try_add_multi(&["ebit", "exabit", "exabits"], Token::Unit(Exabit))?;
	// trie.try_add_multi(&["zbit", "zettabit", "zettabits"], Token::Unit(Zettabit))?;
	// trie.try_add_multi(&["ybit", "yottabit", "yottabits"], Token::Unit(Yottabit))?;
	// trie.try_add_multi(&["kibit", "kibibit", "kibibits"], Token::Unit(Kibibit))?;
	// trie.try_add_multi(&["mibit", "mebibit", "mebibits"], Token::Unit(Mebibit))?;
	// trie.try_add_multi(&["gibit", "gibibit", "gibibits"], Token::Unit(Gibibit))?;
	// trie.try_add_multi(&["tibit", "tebibit", "tebibits"], Token::Unit(Tebibit))?;
	// trie.try_add_multi(&["pibit", "pebibit", "pebibits"], Token::Unit(Pebibit))?;
	// trie.try_add_multi(&["eibit", "exbibit", "exbibits"], Token::Unit(Exbibit))?;
	// trie.try_add_multi(&["zibit", "zebibit", "zebibits"], Token::Unit(Zebibit))?;
	// trie.try_add_multi(&["yibit", "yobibit", "yobibits"], Token::Unit(Yobibit))?;
	// trie.try_add_multi(&["byte", "bytes"], Token::Unit(Byte))?;
	// trie.try_add_multi(&["kb", "kilobyte", "kilobytes"], Token::Unit(Kilobyte))?;
	// trie.try_add_multi(&["mb", "megabyte", "megabytes"], Token::Unit(Megabyte))?;
	// trie.try_add_multi(&["gb", "gigabyte", "gigabytes"], Token::Unit(Gigabyte))?;
	// trie.try_add_multi(&["tb", "terabyte", "terabytes"], Token::Unit(Terabyte))?;
	// trie.try_add_multi(&["pb", "petabyte", "petabytes"], Token::Unit(Petabyte))?;
	// trie.try_add_multi(&["eb", "exabyte", "exabytes"], Token::Unit(Exabyte))?;
	// trie.try_add_multi(&["zb", "zettabyte", "zettabytes"], Token::Unit(Zettabyte))?;
	// trie.try_add_multi(&["yb", "yottabyte", "yottabytes"], Token::Unit(Yottabyte))?;
	// trie.try_add_multi(&["kib", "kibibyte", "kibibytes"], Token::Unit(Kibibyte))?;
	// trie.try_add_multi(&["mib", "mebibyte", "mebibytes"], Token::Unit(Mebibyte))?;
	// trie.try_add_multi(&["gib", "gibibyte", "gibibytes"], Token::Unit(Gibibyte))?;
	// trie.try_add_multi(&["tib", "tebibyte", "tebibytes"], Token::Unit(Tebibyte))?;
	// trie.try_add_multi(&["pib", "pebibyte", "pebibytes"], Token::Unit(Pebibyte))?;
	// trie.try_add_multi(&["eib", "exbibyte", "exbibytes"], Token::Unit(Exbibyte))?;
	// trie.try_add_multi(&["zib", "zebibyte", "zebibytes"], Token::Unit(Zebibyte))?;
	// trie.try_add_multi(&["yib", "yobibyte", "yobibytes"], Token::Unit(Yobibyte))?;

	// trie.try_add("bps", Token::Unit(BitsPerSecond))?;
	// trie.try_add("kbps", Token::Unit(KilobitsPerSecond))?;
	// trie.try_add("mbps", Token::Unit(MegabitsPerSecond))?;
	// trie.try_add("gbps", Token::Unit(GigabitsPerSecond))?;
	// trie.try_add("tbps", Token::Unit(TerabitsPerSecond))?;
	// trie.try_add("pbps", Token::Unit(PetabitsPerSecond))?;
	// trie.try_add("ebps", Token::Unit(ExabitsPerSecond))?;
	// trie.try_add("zbps", Token::Unit(ZettabitsPerSecond))?;
	// trie.try_add("ybps", Token::Unit(YottabitsPerSecond))?;

	// trie.try_add("flop", Token::Unit(Flop))?;
	// trie.try_add_multi(&["kflop", "kiloflop"], Token::Unit(KiloFlop))?;
	// trie.try_add_multi(&["mflop", "megaflop"], Token::Unit(MegaFlop))?;
	// trie.try_add_multi(&["gflop", "gigaflop"], Token::Unit(GigaFlop))?;
	// trie.try_add_multi(&["tflop", "teraflop"], Token::Unit(TeraFlop))?;
	// trie.try_add_multi(&["pflop", "petaflop"], Token::Unit(PetaFlop))?;
	// trie.try_add_multi(&["eflop", "exaflop"], Token::Unit(ExaFlop))?;
	// trie.try_add_multi(&["zflop", "zettaflop"], Token::Unit(ZettaFlop))?;
	// trie.try_add_multi(&["yflop", "yottaflop"], Token::Unit(YottaFlop))?;
	// trie.try_add_multi(&["rflop", "ronnaflop"], Token::Unit(RonnaFlop))?;
	// trie.try_add_multi(&["qflop", "quettaflop"], Token::Unit(QuettaFlop))?;

	// trie.try_add("flops", Token::Unit(FlopPerSecond))?;
	// trie.try_add_multi(&["kflops", "kiloflops"], Token::Unit(KiloFlopPerSecond))?;
	// trie.try_add_multi(&["mflops", "megaflops"], Token::Unit(MegaFlopPerSecond))?;
	// trie.try_add_multi(&["gflops", "gigaflops"], Token::Unit(GigaFlopPerSecond))?;
	// trie.try_add_multi(&["tflops", "teraflops"], Token::Unit(TeraFlopPerSecond))?;
	// trie.try_add_multi(&["pflops", "petaflops"], Token::Unit(PetaFlopPerSecond))?;
	// trie.try_add_multi(&["eflops", "exaflops"], Token::Unit(ExaFlopPerSecond))?;
	// trie.try_add_multi(&["zflops", "zettaflops"], Token::Unit(ZettaFlopPerSecond))?;
	// trie.try_add_multi(&["yflops", "yottaflops"], Token::Unit(YottaFlopPerSecond))?;
	// trie.try_add_multi(&["rflops", "ronnaflops"], Token::Unit(RonnaFlopPerSecond))?;
	// trie.try_add_multi(&["qflops", "quettaflops"], Token::Unit(QuettaFlopPerSecond))?;

	// trie.try_add_multi(&["millijoule", "millijoules"], Token::Unit(Millijoule))?;
	// trie.try_add_multi(&["j", "joule", "joules"], Token::Unit(Joule))?;
	// trie.try_add("nm", Token::Unit(NewtonMeter))?;

	// trie.try_add_multi(
	// 	&[
	// 		"newton meter",
	// 		"newton meters",
	// 		"newton metre",
	// 		"newton metres",
	// 	],
	// 	Token::Unit(NewtonMeter),
	// )?;
	// trie.try_add_multi(&["kj", "kilojoule", "kilojoules"], Token::Unit(Kilojoule))?;
	// trie.try_add_multi(&["mj", "megajoule", "megajoules"], Token::Unit(Megajoule))?;
	// trie.try_add_multi(&["gj", "gigajoule", "gigajoules"], Token::Unit(Gigajoule))?;
	// trie.try_add_multi(&["tj", "terajoule", "terajoules"], Token::Unit(Terajoule))?;
	// trie.try_add_multi(&["cal", "calorie", "calories"], Token::Unit(Calorie))?;
	// trie.try_add_multi(
	// 	&["kcal", "kilocalorie", "kilocalories"],
	// 	Token::Unit(KiloCalorie),
	// )?;
	// trie.try_add_multi(
	// 	&["btu", "british thermal unit", "british thermal units"],
	// 	Token::Unit(BritishThermalUnit),
	// )?;
	// trie.try_add_multi(
	// 	&["wh", "watt hr", "watt hrs", "watt hour", "watt hours"],
	// 	Token::Unit(WattHour),
	// )?;
	// trie.try_add_multi(
	// 	&[
	// 		"kwh",
	// 		"kilowatt hr",
	// 		"kilowatt hrs",
	// 		"kilowatt hour",
	// 		"kilowatt hours",
	// 	],
	// 	Token::Unit(KilowattHour),
	// )?;
	// trie.try_add_multi(
	// 	&[
	// 		"mwh",
	// 		"megawatt hr",
	// 		"megawatt hrs",
	// 		"megawatt hour",
	// 		"megawatt hours",
	// 	],
	// 	Token::Unit(MegawattHour),
	// )?;
	// trie.try_add_multi(
	// 	&[
	// 		"gwh",
	// 		"gigawatt hr",
	// 		"gigawatt hrs",
	// 		"gigawatt hour",
	// 		"gigawatt hours",
	// 	],
	// 	Token::Unit(GigawattHour),
	// )?;
	// trie.try_add_multi(
	// 	&[
	// 		"twh",
	// 		"terawatt hr",
	// 		"terawatt hrs",
	// 		"terawatt hour",
	// 		"terawatt hours",
	// 	],
	// 	Token::Unit(TerawattHour),
	// )?;
	// trie.try_add_multi(
	// 	&[
	// 		"pwh",
	// 		"petawatt hr",
	// 		"petawatt hrs",
	// 		"petawatt hour",
	// 		"petawatt hours",
	// 	],
	// 	Token::Unit(PetawattHour),
	// )?;

	// trie.try_add_multi(&["milliwatt", "milliwatts"], Token::Unit(Milliwatt))?;
	// trie.try_add_multi(&["w", "watts"], Token::Unit(Watt))?;
	// trie.try_add_multi(&["kw", "kilowatts"], Token::Unit(Kilowatt))?;
	// trie.try_add_multi(&["mw", "megawatts"], Token::Unit(Megawatt))?;
	// trie.try_add_multi(&["gw", "gigawatts"], Token::Unit(Gigawatt))?;
	// trie.try_add_multi(&["tw", "terawatts"], Token::Unit(Terawatt))?;
	// trie.try_add_multi(&["pw", "petawatts"], Token::Unit(Petawatt))?;
	// trie.try_add_multi(
	// 	&["hp", "hps", "horsepower", "horsepowers"],
	// 	Token::Unit(Horsepower),
	// )?;
	// trie.try_add_multi(&["mhp", "hpm"], Token::Unit(MetricHorsepower))?;

	// trie.try_add_multi(
	// 	&["ma", "milliamp", "milliamps", "milliampere", "milliamperes"],
	// 	Token::Unit(Milliampere),
	// )?;
	// trie.try_add_multi(
	// 	&["a", "amp", "amps", "ampere", "amperes"],
	// 	Token::Unit(Ampere),
	// )?;
	// trie.try_add_multi(
	// 	&["ka", "kiloamp", "kiloamps", "kiloampere", "kiloamperes"],
	// 	Token::Unit(Kiloampere),
	// )?;
	// trie.try_add_multi(
	// 	&["bi", "biot", "biots", "aba", "abampere", "abamperes"],
	// 	Token::Unit(Abampere),
	// )?;

	// trie.try_add_multi(
	// 	&["mΩ", "mΩ", "milliohm", "milliohms"],
	// 	Token::Unit(Milliohm),
	// )?;
	// trie.try_add_multi(&["Ω", "Ω", "ohm", "ohms"], Token::Unit(Ohm))?;
	// trie.try_add_multi(&["kΩ", "kΩ", "kiloohm", "kiloohms"], Token::Unit(Kiloohm))?;

	// trie.try_add_multi(&["mv", "millivolt", "millivolts"], Token::Unit(Millivolt))?;
	// trie.try_add_multi(&["v", "volt", "volts"], Token::Unit(Volt))?;
	// trie.try_add_multi(&["kv", "kilovolt", "kilovolts"], Token::Unit(Kilovolt))?;

	// // // for pound-force per square inch
	// trie.try_add("lbf", Token::LexerKeyword(PoundForce))?;
	// trie.try_add("force", Token::LexerKeyword(Force))?;

	// trie.try_add_multi(&["pa", "pascal", "pascals"], Token::Unit(Pascal))?;
	// trie.try_add_multi(
	// 	&["kpa", "kilopascal", "kilopascals"],
	// 	Token::Unit(Kilopascal),
	// )?;
	// trie.try_add_multi(
	// 	&["atm", "atms", "atmosphere", "atmospheres"],
	// 	Token::Unit(Atmosphere),
	// )?;
	// trie.try_add_multi(
	// 	&["mbar", "mbars", "millibar", "millibars"],
	// 	Token::Unit(Millibar),
	// )?;
	// trie.try_add_multi(&["bar", "bars"], Token::Unit(Bar))?;
	// trie.try_add("inhg", Token::Unit(InchOfMercury))?;
	// trie.try_add("mercury", Token::LexerKeyword(Mercury))?;
	// trie.try_add("psi", Token::Unit(PoundsPerSquareInch))?;
	// trie.try_add_multi(&["torr", "torrs"], Token::Unit(Torr))?;

	// trie.try_add_multi(&["hz", "hertz"], Token::Unit(Hertz))?;
	// trie.try_add_multi(&["khz", "kilohertz"], Token::Unit(Kilohertz))?;
	// trie.try_add_multi(&["mhz", "megahertz"], Token::Unit(Megahertz))?;
	// trie.try_add_multi(&["ghz", "gigahertz"], Token::Unit(Gigahertz))?;
	// trie.try_add_multi(&["thz", "terahertz"], Token::Unit(Terahertz))?;
	// trie.try_add_multi(&["phz", "petahertz"], Token::Unit(Petahertz))?;
	// trie.try_add("rpm", Token::Unit(RevolutionsPerMinute))?;
	// trie.try_add_multi(
	// 	&["r", "rev", "revolution", "revolutions"],
	// 	Token::LexerKeyword(Revolution),
	// )?;

	// trie.try_add_multi(&["kph", "kmh"], Token::Unit(KilometersPerHour))?;
	// trie.try_add("mps", Token::Unit(MetersPerSecond))?;
	// trie.try_add("mph", Token::Unit(MilesPerHour))?;
	// trie.try_add("fps", Token::Unit(FeetPerSecond))?;
	// trie.try_add_multi(&["kn", "kt", "knot", "knots"], Token::Unit(Knot))?;

	// trie.try_add_multi(&["k", "kelvin", "kelvins"], Token::Unit(Kelvin))?;
	// trie.try_add_multi(&["c", "celsius"], Token::Unit(Celsius))?;
	// trie.try_add_multi(&["f", "fahrenheit", "fahrenheits"], Token::Unit(Fahrenheit))?;
	Ok(token)
}
