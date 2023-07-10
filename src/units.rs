use decimal::d128;
use crate::Number;

#[derive(Clone, Copy, PartialEq, Debug)]
/// An enum of all possible unit types, like [`Length`], [`DigitalStorage`] etc.
/// There is also a [`NoType`] unit type for normal numbers.
pub enum UnitType {
	/// A normal number, for example `5`
	NoType,
	/// A unit of time, for example [`Hour`]
	Time,
	/// A unit of length, for example [`Mile`]
	Length,
	/// A unit of area, for example [`SquareKilometer`]
	Area,
	/// A unit of volume, for example [`Liter`] or [`Tablespoon`]
	Volume,
	/// A unit of mass, for example [`Gram`]
	Mass,
	/// A unit of digital storage, for example [`Kilobyte`]
	DigitalStorage,
	/// A unit of data rate transfer, for example [`KilobytesPerSecond`]
	DataTransferRate,
	/// A unit of energy, for example [`Joule`] or [`KilowattHour`]
	Energy,
	/// A unit of power, for example [`Watt`]
	Power,
	/// A unit of electrical current, for example [`Ampere`]
	ElectricCurrent,
	/// A unit of electric resistance, for example [`Ohm`]
	Resistance,
	/// A unit of voltage, for example [`Volt`]
	Voltage,
	/// A unit of pressure, for example [`Bar`]
	Pressure,
	/// A unit of frequency, for example [`Hertz`]
	Frequency,
	/// A unit of x, for example [`KilometersPerHour`]
	Speed,
	/// A unit of temperature, for example [`Kelvin`]
	Temperature,
}
use UnitType::*;

// Macro for creating units. Not possible to extend/change the default units
// with this because the default units are imported into the lexer, parser
// and evaluator
macro_rules! create_units {
	( $( $variant:ident : $properties:expr ),*, ) => {
		#[derive(Clone, Copy, PartialEq, Debug)]
		/// A Unit enum. Note that it can also be [`NoUnit`].
		pub enum Unit {
			$($variant),*
		}
		use Unit::*;

		impl Unit {
			pub fn category(&self) -> UnitType {
				match self {
					$(
						Unit::$variant => $properties.0
					),*
				}
			}
			pub fn weight(&self) -> d128 {
				match self {
					$(
						Unit::$variant => $properties.1
					),*
				}
			}
			pub(crate) fn singular(&self) -> &str {
				match self {
					$(
						Unit::$variant => $properties.2
					),*
				}
			}
			pub(crate) fn plural(&self) -> &str {
				match self {
					$(
						Unit::$variant => $properties.3
					),*
				}
			}
		}
	}
}

create_units!(
	NoUnit:             (NoType, d128!(1), "", ""),

	Nanosecond:         (Time, d128!(1), "nanosecond", "nanoseconds"),
	Microsecond:        (Time, d128!(1000), "microsecond", "microseconds"),
	Millisecond:        (Time, d128!(1000000), "millisecond", "milliseconds"),
	Second:             (Time, d128!(1000000000), "second", "seconds"),
	Minute:             (Time, d128!(60000000000), "minute", "minutes"),
	Hour:               (Time, d128!(3600000000000), "hour", "hours"),
	Day:                (Time, d128!(86400000000000), "day", "days"),
	Week:               (Time, d128!(604800000000000), "week", "weeks"),
	Month:              (Time, d128!(2629746000000000), "month", "months"),
	Quarter:            (Time, d128!(7889238000000000), "quarter", "quarters"),
	Year:               (Time, d128!(31556952000000000), "year", "years"),
	Decade:             (Time, d128!(315569520000000000), "decade", "decades"),
	Century:            (Time, d128!(3155695200000000000), "century", "centuries"),
	Millenium:          (Time, d128!(31556952000000000000), "millenium", "millenia"),

	Millimeter:         (Length, d128!(1), "millimeter", "millimeters"),
	Centimeter:         (Length, d128!(10), "centimeter", "centimeters"),
	Decimeter:          (Length, d128!(100), "decimeter", "decimeters"),
	Meter:              (Length, d128!(1000), "meter", "meters"),
	Kilometer:          (Length, d128!(1000000), "kilometer", "kilometers"),
	Inch:               (Length, d128!(25.4), "inch", "inches"),
	Foot:               (Length, d128!(304.8), "foot", "feet"),
	Yard:               (Length, d128!(914.4), "yard", "yards"),
	Mile:               (Length, d128!(1609344), "mile", "miles"),
	// 1-dimensional only:
	Marathon:           (Length, d128!(42195000), "marathon", "marathons"),
	NauticalMile:       (Length, d128!(1852000), "nautical mile", "nautical miles"),
	LightYear:          (Length, d128!(9460730472580800000), "light year", "light years"),
	LightSecond:        (Length, d128!(299792458000), "light second", "light seconds"),

	SquareMillimeter:   (Area, d128!(1), "square millimeter", "square millimeters"),
	SquareCentimeter:   (Area, d128!(100), "square centimeter", "square centimeters"),
	SquareDecimeter:    (Area, d128!(10000), "square decimeter", "square decimeters"),
	SquareMeter:        (Area, d128!(1000000), "square meter", "square meters"),
	SquareKilometer:    (Area, d128!(1000000000000), "square kilometer", "square kilometers"),
	SquareInch:         (Area, d128!(645.16), "square inch", "square inches"),
	SquareFoot:         (Area, d128!(92903.04), "square foot", "square feet"),
	SquareYard:         (Area, d128!(836127.36), "square yard", "square yards"),
	SquareMile:         (Area, d128!(2589988110336.00), "square mile", "square miles"),
	// 2-dimensional only
	Are:                (Area, d128!(100000000), "are", "ares"),
	Decare:             (Area, d128!(1000000000), "decare", "decare"),
	Hectare:            (Area, d128!(10000000000), "hectare", "hectares"),
	Acre:               (Area, d128!(4046856422.40), "acre", "acres"),

	CubicMillimeter:    (Volume, d128!(1), "cubic millimeter", "cubic millimeters"),
	CubicCentimeter:    (Volume, d128!(1000), "cubic centimeter", "cubic centimeters"),
	CubicDecimeter:     (Volume, d128!(1000000), "cubic decimeter", "cubic decimeters"),
	CubicMeter:         (Volume, d128!(1000000000), "cubic meter", "cubic meters"),
	CubicKilometer:     (Volume, d128!(1000000000000000000), "cubic kilometer", "cubic kilometers"),
	CubicInch:          (Volume, d128!(16387.064), "cubic inch", "cubic inches"),
	CubicFoot:          (Volume, d128!(28316846.592), "cubic foot", "cubic feet"),
	CubicYard:          (Volume, d128!(764554857.984), "cubic yard", "cubic yards"),
	CubicMile:          (Volume, d128!(4168181825440579584), "cubic mile", "cubic miles"),
	// 3-dimensional only
	Milliliter:         (Volume, d128!(1000), "milliliter", "milliliters"),
	Centiliter:         (Volume, d128!(10000), "centiliter", "centiliters"),
	Deciliter:          (Volume, d128!(100000), "deciliter", "deciliters"),
	Liter:              (Volume, d128!(1000000), "liter", "liters"),
	Teaspoon:           (Volume, d128!(4928.92159375), "teaspoon", "teaspoons"),
	Tablespoon:         (Volume, d128!(14786.76478125), "tablespoon", "tablespoons"),
	FluidOunce:         (Volume, d128!(29573.5295625), "fluid ounce", "fluid ounces"),
	Cup:                (Volume, d128!(236588.2365), "cup", "cups"),
	Pint:               (Volume, d128!(473176.473), "pint", "pints"),
	Quart:              (Volume, d128!(946352.946), "quart", "quarts"),
	Gallon:             (Volume, d128!(3785411.784), "gallon", "gallons"),
	OilBarrel:          (Volume, d128!(158987294.928), "oil barrel", "oil barrels"),

	Milligram:          (Mass, d128!(0.001), "milligram", "milligrams"),
	Gram:               (Mass, d128!(1), "gram", "grams"),
	Hectogram:          (Mass, d128!(100), "hectogram", "hectograms"),
	Kilogram:           (Mass, d128!(1000), "kilogram", "kilograms"),
	MetricTon:          (Mass, d128!(1000000), "metric ton", "metric tons"),
	Ounce:              (Mass, d128!(28.349523125), "ounce", "ounces"),
	Pound:              (Mass, d128!(453.59237), "pound", "pounds"),
	Stone:              (Mass, d128!(6350.29318), "stone", "stones"),
	ShortTon:           (Mass, d128!(907184.74), "short ton", "short tons"),
	LongTon:            (Mass, d128!(1016046.9088), "long ton", "long tons"),

	Bit:                (DigitalStorage, d128!(1), "bit", "bits"),
	Kilobit:            (DigitalStorage, d128!(1000), "kilobit", "kilobits"),
	Megabit:            (DigitalStorage, d128!(1000000), "megabit", "megabits"),
	Gigabit:            (DigitalStorage, d128!(1000000000), "gigabit", "gigabits"),
	Terabit:            (DigitalStorage, d128!(1000000000000), "terabit", "terabits"),
	Petabit:            (DigitalStorage, d128!(1000000000000000), "petabit", "petabits"),
	Exabit:             (DigitalStorage, d128!(1000000000000000000), "exabit", "exabits"),
	Zettabit:           (DigitalStorage, d128!(1000000000000000000000), "zettabit", "zettabits"),
	Yottabit:           (DigitalStorage, d128!(1000000000000000000000000), "yottabit", "yottabits"),
	Kibibit:            (DigitalStorage, d128!(1024), "kibibit", "kibibits"),
	Mebibit:            (DigitalStorage, d128!(1048576), "mebibit", "mebibits"),
	Gibibit:            (DigitalStorage, d128!(1073741824), "gibibit", "gibibits"),
	Tebibit:            (DigitalStorage, d128!(1099511627776), "tebibit", "tebibits"),
	Pebibit:            (DigitalStorage, d128!(1125899906842624), "pebibit", "pebibits"),
	Exbibit:            (DigitalStorage, d128!(1152921504606846976), "exbibit", "exbibits"),
	Zebibit:            (DigitalStorage, d128!(1180591620717411303424), "zebibit", "zebibits"),
	Yobibit:            (DigitalStorage, d128!(1208925819614629174706176), "yobibit", "yobibits"),
	Byte:               (DigitalStorage, d128!(8), "byte", "bytes"),
	Kilobyte:           (DigitalStorage, d128!(8000), "kilobyte", "kilobytes"),
	Megabyte:           (DigitalStorage, d128!(8000000), "megabyte", "megabytes"),
	Gigabyte:           (DigitalStorage, d128!(8000000000), "gigabyte", "gigabytes"),
	Terabyte:           (DigitalStorage, d128!(8000000000000), "terabyte", "terabytes"),
	Petabyte:           (DigitalStorage, d128!(8000000000000000), "petabyte", "petabytes"),
	Exabyte:            (DigitalStorage, d128!(8000000000000000000), "exabyte", "exabytes"),
	Zettabyte:          (DigitalStorage, d128!(8000000000000000000000), "zettabyte", "zettabytes"),
	Yottabyte:          (DigitalStorage, d128!(8000000000000000000000000), "yottabyte", "yottabytes"),
	Kibibyte:           (DigitalStorage, d128!(8192), "kibibyte", "kibibytes"),
	Mebibyte:           (DigitalStorage, d128!(8388608), "mebibyte", "mebibytes"),
	Gibibyte:           (DigitalStorage, d128!(8589934592), "gibibyte", "gibibytes"),
	Tebibyte:           (DigitalStorage, d128!(8796093022208), "tebibyte", "tebibytes"),
	Pebibyte:           (DigitalStorage, d128!(9007199254740992), "pebibyte", "pebibytes"),
	Exbibyte:           (DigitalStorage, d128!(9223372036854775808), "exbibyte", "exbibytes"),
	Zebibyte:           (DigitalStorage, d128!(9444732965739290427392), "zebibyte", "zebibytes"),
	Yobibyte:           (DigitalStorage, d128!(9671406556917033397649408), "yobibyte", "yobibytes"),

	BitsPerSecond:        (DataTransferRate, d128!(1), "bit per second", "bits per second"),
	KilobitsPerSecond:    (DataTransferRate, d128!(1000), "kilobit per second", "kilobits per second"),
	MegabitsPerSecond:    (DataTransferRate, d128!(1000000), "megabit per second", "megabits per second"),
	GigabitsPerSecond:    (DataTransferRate, d128!(1000000000), "gigabit per second", "gigabits per second"),
	TerabitsPerSecond:    (DataTransferRate, d128!(1000000000000), "terabit per second", "terabits per second"),
	PetabitsPerSecond:    (DataTransferRate, d128!(1000000000000000), "petabit per second", "petabits per second"),
	ExabitsPerSecond:     (DataTransferRate, d128!(1000000000000000000), "exabit per second", "exabits per second"),
	ZettabitsPerSecond:   (DataTransferRate, d128!(1000000000000000000000), "zettabit per second", "zettabits per second"),
	YottabitsPerSecond:   (DataTransferRate, d128!(1000000000000000000000000), "yottabit per second", "yottabits per second"),
	KibibitsPerSecond:    (DataTransferRate, d128!(1024), "kibibit per second", "kibibits per second"),
	MebibitsPerSecond:    (DataTransferRate, d128!(1048576), "mebibit per second", "mebibits per second"),
	GibibitsPerSecond:    (DataTransferRate, d128!(1073741824), "gibibit per second", "gibibits per second"),
	TebibitsPerSecond:    (DataTransferRate, d128!(1099511627776), "tebibit per second", "tebibits per second"),
	PebibitsPerSecond:    (DataTransferRate, d128!(1125899906842624), "pebibit per second", "pebibits per second"),
	ExbibitsPerSecond:    (DataTransferRate, d128!(1152921504606846976), "exbibit per second", "exbibits per second"),
	ZebibitsPerSecond:    (DataTransferRate, d128!(1180591620717411303424), "zebibit per second", "zebibits per second"),
	YobibitsPerSecond:    (DataTransferRate, d128!(1208925819614629174706176), "yobibit per second", "yobibits per second"),
	BytesPerSecond:       (DataTransferRate, d128!(8), "byte per second", "bytes per second"),
	KilobytesPerSecond:   (DataTransferRate, d128!(8000), "kilobyte per second", "kilobytes per second"),
	MegabytesPerSecond:   (DataTransferRate, d128!(8000000), "megabyte per second", "megabytes per second"),
	GigabytesPerSecond:   (DataTransferRate, d128!(8000000000), "gigabyte per second", "gigabytes per second"),
	TerabytesPerSecond:   (DataTransferRate, d128!(8000000000000), "terabyte per second", "terabytes per second"),
	PetabytesPerSecond:   (DataTransferRate, d128!(8000000000000000), "petabyte per second", "petabytes per second"),
	ExabytesPerSecond:    (DataTransferRate, d128!(8000000000000000000), "exabyte per second", "exabytes per second"),
	ZettabytesPerSecond:  (DataTransferRate, d128!(8000000000000000000000), "zettabyte per second", "zettabytes per second"),
	YottabytesPerSecond:  (DataTransferRate, d128!(8000000000000000000000000), "yottabyte per second", "yottabytes per second"),
	KibibytesPerSecond:   (DataTransferRate, d128!(8192), "kibibyte per second", "kibibytes per second"),
	MebibytesPerSecond:   (DataTransferRate, d128!(8388608), "mebibyte per second", "mebibytes per second"),
	GibibytesPerSecond:   (DataTransferRate, d128!(8589934592), "gibibyte per second", "gibibytes per second"),
	TebibytesPerSecond:   (DataTransferRate, d128!(8796093022208), "tebibyte per second", "tebibytes per second"),
	PebibytesPerSecond:   (DataTransferRate, d128!(9007199254740992), "pebibyte per second", "pebibytes per second"),
	ExbibytesPerSecond:   (DataTransferRate, d128!(9223372036854775808), "exbibyte per second", "exbibytes per second"),
	ZebibytesPerSecond:   (DataTransferRate, d128!(9444732965739290427392), "zebibyte per second", "zebibytes per second"),
	YobibytesPerSecond:   (DataTransferRate, d128!(9671406556917033397649408), "yobibyte per second", "yobibytes per second"),

	// ! If updating Millijoule, also update get_inverted_millijoule_weight()
	Millijoule:         (Energy, d128!(0.001), "millijoule", "millijoules"),
	Joule:              (Energy, d128!(1), "joule", "joules"),
	NewtonMeter:        (Energy, d128!(1), "newton meter", "newton meters"),
	Kilojoule:          (Energy, d128!(1000), "kilojoule", "kilojoules"),
	Megajoule:          (Energy, d128!(1000000), "megajoule", "megajoules"),
	Gigajoule:          (Energy, d128!(1000000000), "gigajoule", "gigajoules"),
	Terajoule:          (Energy, d128!(1000000000000), "terajoule", "terajoules"),
	Calorie:            (Energy, d128!(4.1868), "calorie", "calories"),
	KiloCalorie:        (Energy, d128!(4186.8), "kilocalorie", "kilocalories"),
	BritishThermalUnit: (Energy, d128!(1055.05585262), "British thermal unit", "British thermal units"),
	WattHour:           (Energy, d128!(3600), "watt-hour", "watt-hours"),
	KilowattHour:       (Energy, d128!(3600000), "kilowatt-hour", "kilowatt-hours"),
	MegawattHour:       (Energy, d128!(3600000000),	"megawatt-hour", "megawatt-hours"),
	GigawattHour:       (Energy, d128!(3600000000000), "gigawatt-hour", "gigawatt-hours"),
	TerawattHour:       (Energy, d128!(3600000000000000), "terawatt-hour", "terawatt-hours"),
	PetawattHour:       (Energy, d128!(3600000000000000000), "petawatt-hour", "petawatt-hours"),

	// ! If updating Milliwatt, also update get_inverted_milliwatt_weight()
	Milliwatt:                    (Power, d128!(0.001), "milliwatt", "milliwatts"),
	Watt:                         (Power, d128!(1), "watt", "watts"),
	Kilowatt:                     (Power, d128!(1000), "kilowatt", "kilowatts"),
	Megawatt:                     (Power, d128!(1000000), "megawatt", "megawatts"),
	Gigawatt:                     (Power, d128!(1000000000), "gigawatt", "gigawatts"),
	Terawatt:                     (Power, d128!(1000000000000), "terawatt", "terawatts"),
	Petawatt:                     (Power, d128!(1000000000000000), "petawatt", "petawatts"),
	// probably inexact:
	BritishThermalUnitsPerMinute: (Power, d128!(0.0568690272188), "british thermal unit per minute", "british thermal units per minute"),
	// probably inexact:
	BritishThermalUnitsPerHour:   (Power, d128!(3.412141633128), "british thermal unit per hour", "british thermal units per hour"),
	// exact according to wikipedia:
	Horsepower:                   (Power, d128!(745.69987158227022), "horsepower", "horsepower"),
	MetricHorsepower:             (Power, d128!(735.49875), "metric horsepower", "metric horsepower"),

	// ! If updating Milliampere, also update get_inverted_milliampere_weight()
	Milliampere:                  (ElectricCurrent, d128!(0.001), "milliampere", "milliamperes"),
	Ampere:                       (ElectricCurrent, d128!(1), "ampere", "amperes"),
	Kiloampere:                   (ElectricCurrent, d128!(1000), "kiloampere", "kiloamperes"),
	Abampere:                     (ElectricCurrent, d128!(10), "abampere", "abamperes"),

	// ! If updating Milliohm, also update get_inverted_milliohm_weight()
	Milliohm:                     (Resistance, d128!(0.001), "milliohm", "milliohms"),
	Ohm:                          (Resistance, d128!(1), "ohm", "ohms"),
	Kiloohm:                      (Resistance, d128!(1000), "kiloohm", "kiloohms"),

	// ! If updating Millivolt, also update get_inverted_millivolt_weight()
	Millivolt:                    (Voltage, d128!(0.001), "millivolt", "millivolts"),
	Volt:                         (Voltage, d128!(1), "volt", "volts"),
	Kilovolt:                     (Voltage, d128!(1000), "kilovolt", "kilovolts"),

	Pascal:                       (Pressure, d128!(1), "pascal", "pascals"),
	Kilopascal:                   (Pressure, d128!(1000), "kilopascal", "kilopascals"),
	Atmosphere:                   (Pressure, d128!(101325), "atmosphere", "atmospheres"),
	Millibar:                     (Pressure, d128!(100), "millibar", "millibars"),
	Bar:                          (Pressure, d128!(100000), "bar", "bars"),
	InchOfMercury:                (Pressure, d128!(3386.389), "inch of mercury", "inches of mercury"),
	// inexact:
	PoundsPerSquareInch:          (Pressure, d128!(6894.757293168361), "pound per square inch", "pounds per square inch"),
	Torr:                         (Pressure, d128!(162.12), "torr", "torr"),

	Hertz:                        (Frequency, d128!(1), "hertz", "hertz"),
	Kilohertz:                    (Frequency, d128!(1000), "kilohertz", "kilohertz"),
	Megahertz:                    (Frequency, d128!(1000000), "megahertz", "megahertz"),
	Gigahertz:                    (Frequency, d128!(1000000000), "gigahertz", "gigahertz"),
	Terahertz:                    (Frequency, d128!(1000000000000), "terahertz", "terahertz"),
	Petahertz:                    (Frequency, d128!(1000000000000000), "petahertz", "petahertz"),
	RevolutionsPerMinute:         (Frequency, d128!(60), "revolution per minute", "revolutions per minute"),

	KilometersPerHour:  (Speed, d128!(1), "kilometer per hour", "kilometers per hour"),
	MetersPerSecond:    (Speed, d128!(3.6), "meter per second", "meters per second"),
	MilesPerHour:       (Speed, d128!(1.609344), "mile per hour", "miles per hour"),
	FeetPerSecond:      (Speed, d128!(1.09728), "foot per second", "feet per second"),
	Knot:               (Speed, d128!(1.852), "knot", "knots"),

	Kelvin:             (Temperature, d128!(0), "kelvin", "kelvin"),
	Celsius:            (Temperature, d128!(0), "celsius", "celsius"),
	Fahrenheit:         (Temperature, d128!(0), "fahrenheit", "fahrenheit"),
);

// These functions are here to avoid dividing by small numbers like 0.01,
// because d128 gives numbers in E notation in that case.
fn get_inverted_millijoule_weight() -> d128 {
	d128!(1000)
}
fn get_inverted_milliwatt_weight() -> d128 {
	d128!(1000)
}
fn get_inverted_milliohm_weight() -> d128 {
	d128!(1000)
}
fn get_inverted_milliampere_weight() -> d128 {
	d128!(1000)
}
fn get_inverted_millivolt_weight() -> d128 {
	d128!(1000)
}

/// Returns the conversion factor between two units.
/// 
/// The conversion factor is what you need to multiply `unit` with to get
/// `to_unit`. For example, the conversion factor from 1 minute to 1 second
/// is 60.
/// 
/// This is not sufficient for [`Temperature`] units.
pub fn get_conversion_factor(unit: Unit, to_unit: Unit) -> d128 {
	unit.weight() / to_unit.weight()
}

/// Convert a [`Number`] to a specified [`Unit`].
pub fn convert(number: Number, to_unit: Unit) -> Result<Number, String> {
	if number.unit.category() != to_unit.category() {
		return Err(format!("Cannot convert from {:?} to {:?}", number.unit, to_unit));
	}
	let value = number.value;
	let ok = |new_value| {
		Ok(Number::new(new_value, to_unit))
	};
	if number.unit.category() == UnitType::Temperature {
		match (number.unit, to_unit) {
			(Kelvin, Kelvin)         => ok(value),
			(Kelvin, Celsius)        => ok(value-d128!(273.15)),
			(Kelvin, Fahrenheit)     => ok(value*d128!(1.8)-d128!(459.67)),
			(Celsius, Celsius)       => ok(value),
			(Celsius, Kelvin)        => ok(value+d128!(273.15)),
			(Celsius, Fahrenheit)    => ok(value*d128!(1.8)+d128!(32)),
			(Fahrenheit, Fahrenheit) => ok(value),
			(Fahrenheit, Kelvin)     => ok((value+d128!(459.67))*d128!(5)/d128!(9)),
			(Fahrenheit, Celsius)    => ok((value-d128!(32))/d128!(1.8)),
			_ => Err(format!("Error converting temperature {:?} to {:?}", number.unit, to_unit)),
		}
	} else {
		let conversion_factor = get_conversion_factor(number.unit, to_unit);
		ok(number.value * conversion_factor)
	}
}

/// If one of two provided [`Number`]s has a larger [`Unit`] than the other, convert
/// the large one to the unit of the small one.
pub fn convert_to_lowest(left: Number, right: Number) -> Result<(Number, Number), String> {
	if left.unit.weight() == right.unit.weight() {
		Ok((left, right))
	} else if left.unit.weight() > right.unit.weight() {
		let left_converted = convert(left, right.unit)?;
		Ok((left_converted, right))
	} else {
		let right_converted = convert(right, left.unit)?;
		Ok((left, right_converted))
	}
}

/// Return the sum of two [`Number`]s
pub fn add(left: Number, right: Number) -> Result<Number, String> {
	if left.unit == right.unit {
		Ok(Number::new(left.value + right.value, left.unit))
	} else if left.unit.category() == right.unit.category() && left.unit.category() != Temperature {
		let (left, right) = convert_to_lowest(left, right)?;
		Ok(Number::new(left.value + right.value, left.unit))
	} else {
		Err(format!("Cannot add {:?} and {:?}", left.unit, right.unit))
	}
}

/// Subtract a [`Number`] from another [`Number`]
pub fn subtract(left: Number, right: Number) -> Result<Number, String> {
	if left.unit == right.unit {
		Ok(Number::new(left.value - right.value, left.unit))
	} else if left.unit.category() == right.unit.category() && left.unit.category() != Temperature {
		let (left, right) = convert_to_lowest(left, right)?;
		Ok(Number::new(left.value - right.value, left.unit))
	} else {
		Err(format!("Cannot subtract {:?} by {:?}", left.unit, right.unit))
	}
}

/// Convert a [`Number`] to an ideal unit.
/// 
/// If you have 1,000,000 millimeters, this will return 1 kilometer.
/// 
/// This only affects units of `Length`, `Time`, `Area`, `Volume`,
/// `Energy`, `Power`, `ElectricCurrent`, `Resistance`, and `Voltage`.
/// Other units are passed through.
pub fn to_ideal_unit(number: Number) -> Number {
	let value = number.value * number.unit.weight();
	if number.unit.category() == Length {
		if value >= d128!(1000000000000000000) { // ≈ 0.1 light years
			return Number::new(value/LightYear.weight(), LightYear)
		} else if value >= d128!(1000000) { // 1 km
			return Number::new(value/Kilometer.weight(), Kilometer)
		} else if value >= d128!(1000) { // 1 m
			return Number::new(value/Meter.weight(), Meter)
		} else if value >= d128!(10) { // 1 cm
			return Number::new(value/Centimeter.weight(), Centimeter)
		} else {
			return Number::new(value, Millimeter)
		}
	} else if number.unit.category() == Time {
		if value >= d128!(31556952000000000) {
			return Number::new(value/Year.weight(), Year);
		} else if value >= d128!(86400000000000) {
			return Number::new(value/Day.weight(), Day);
		} else if value >= d128!(3600000000000) {
			return Number::new(value/Hour.weight(), Hour);
		} else if value >= d128!(60000000000) {
			return Number::new(value/Minute.weight(), Minute);
		} else if value >= d128!(1000000000) {
			return Number::new(value/Second.weight(), Second);
		} else if value >= d128!(1000000) {
			return Number::new(value/Millisecond.weight(), Millisecond);
		} else if value >= d128!(1000) {
			return Number::new(value/Microsecond.weight(), Microsecond);
		} else {
			return Number::new(value, Nanosecond);
		}
	} else if number.unit.category() == Area {
		if value >= d128!(1000000000000) { // 1 km2
			return Number::new(value/SquareKilometer.weight(), SquareKilometer)
		} else if value >= d128!(10000000000) { // 1 hectare
			return Number::new(value/Hectare.weight(), Hectare)
		} else if value >= d128!(1000000) { // 1 m2
			return Number::new(value/SquareMeter.weight(), SquareMeter)
		} else if value >= d128!(100) { // 1 cm2
			return Number::new(value/SquareCentimeter.weight(), SquareCentimeter)
		} else {
			return Number::new(value, SquareMillimeter)
		}
	} else if number.unit.category() == Volume {
		if value >= d128!(1000000000000000000) { // 1 km3
			return Number::new(value/CubicKilometer.weight(), CubicKilometer)
		} else if value >= d128!(1000000000) { // 1 m3
			return Number::new(value/CubicMeter.weight(), CubicMeter)
		} else if value >= d128!(1000000) { // 1 l
			return Number::new(value/Liter.weight(), Liter)
		} else if value >= d128!(1000) { // 1 ml
			return Number::new(value/Milliliter.weight(), Milliliter)
		} else {
			return Number::new(value, CubicMillimeter)
		}
	} else if number.unit.category() == Energy {
		if value >= d128!(3600000000000000000) { // 1 petawatthour
			return Number::new(value/PetawattHour.weight(), PetawattHour)
		} else if value >= d128!(3600000000000000) { // 1 terawatthour
			return Number::new(value/TerawattHour.weight(), TerawattHour)
		} else if value >= d128!(3600000000000) { // 1 gigawatthour
			return Number::new(value/GigawattHour.weight(), GigawattHour)
		} else if value >= d128!(3600000000) { // 1 megawatthour
			return Number::new(value/MegawattHour.weight(), MegawattHour)
		} else if value >= d128!(3600000) { // 1 kilowatthour
			return Number::new(value/KilowattHour.weight(), KilowattHour)
		} else if value >= d128!(3600) { // 1 watthour
			return Number::new(value/WattHour.weight(), WattHour)
		} else if value >= d128!(1) { // 1 joule
			return Number::new(value, Joule)
		} else {
			return Number::new(value * get_inverted_millijoule_weight(), Millijoule)
		}
	} else if number.unit.category() == Power {
		if value >= d128!(1000000000000000) { // 1 petawatt
			return Number::new(value/Petawatt.weight(), Petawatt)
		} else if value >= d128!(1000000000000) { // 1 terawatt
			return Number::new(value/Terawatt.weight(), Terawatt)
		} else if value >= d128!(1000000000) { // 1 gigawatt
			return Number::new(value/Gigawatt.weight(), Gigawatt)
		} else if value >= d128!(1000000) { // megawatt
			return Number::new(value/Megawatt.weight(), Megawatt)
		} else if value >= d128!(1000) { // 1 kilowatt
			return Number::new(value/Kilowatt.weight(), Kilowatt)
		} else if value >= d128!(1) { // 1 watt
			return Number::new(value, Watt)
		} else {
			return Number::new(value * get_inverted_milliwatt_weight(), Milliwatt)
		}
	} else if number.unit.category() == ElectricCurrent {
		if value >= d128!(1000) { // 1 kiloampere
			return Number::new(value/Kiloampere.weight(), Kiloampere)
		} else if value >= d128!(1) { // 1 ampere
			return Number::new(value, Ampere)
		} else {
			return Number::new(value * get_inverted_milliampere_weight(), Milliampere)
		}
	} else if number.unit.category() == Resistance {
		if value >= d128!(1000) { // 1 kiloohm
			return Number::new(value/Kiloohm.weight(), Kiloohm)
		} else if value >= d128!(1) { // 1 ohm
			return Number::new(value, Ohm)
		} else {
			return Number::new(value * get_inverted_milliohm_weight(), Milliohm)
		}
	} else if number.unit.category() == Voltage {
		if value >= d128!(1000) { // 1 kilovolt
			return Number::new(value/Kilovolt.weight(), Kilovolt)
		} else if value >= d128!(1) { // 1 volt
			return Number::new(value, Volt)
		} else {
			return Number::new(value * get_inverted_millivolt_weight(), Millivolt)
		}
	}
	number
}

/// Convert a [`Number`] to an ideal [`Joule`] unit, if the number is a unit of [`Energy`].
pub fn to_ideal_joule_unit(number: Number) -> Number {
	let value = number.value * number.unit.weight();
	if number.unit.category() == Energy {
		if value >= d128!(1000000000000) { // 1 terajoule
			return Number::new(value/Terajoule.weight(), Terajoule)
		} else if value >= d128!(1000000000) { // 1 gigajoule
			return Number::new(value/Gigajoule.weight(), Gigajoule)
		} else if value >= d128!(1000000) { // 1 megajoule
			return Number::new(value/Megajoule.weight(), Megajoule)
		} else if value >= d128!(1000) { // 1 kilojoule
			return Number::new(value/Kilojoule.weight(), Kilojoule)
		} else if value >= d128!(1) { // 1 joule
			return Number::new(value/Joule.weight(), Joule)
		} else {
			return Number::new(value * get_inverted_millijoule_weight(), Millijoule)
		}
	}
	number
}

/// Multiply two [`Number`]s
/// 
/// - Temperatures don't work
/// - If you multiply [`NoType`] with any other unit, the result gets that other unit
/// - If you multiply [`Length`] with [`Length`], the result has a unit of [`Area`], etc.
/// - If you multiply [`Speed`] with [`Time`], the result has a unit of [`Length`]
/// - If you multiply [`Voltage`] with [`ElectricCurrent`], the result has a unit of [`Power`]
/// - If you multiply [`ElectricCurrent`] with [`Resistance`], the result has a unit of [`Voltage`]
/// - If you multiply [`Power`] with [`Time`], the result has a unit of [`Energy`]
pub fn multiply(left: Number, right: Number) -> Result<Number, String> {
	actual_multiply(left, right, false)
}

fn actual_multiply(left: Number, right: Number, swapped: bool) -> Result<Number, String> {
	let lcat = left.unit.category();
	let rcat = right.unit.category();
	if left.unit == NoUnit && right.unit == NoUnit {
		// 3 * 2
		Ok(Number::new(left.value * right.value, left.unit))
	} else if left.unit.category() == Temperature || right.unit.category() == Temperature {
		// if temperature
		Err(format!("Cannot multiply {:?} and {:?}", left.unit, right.unit))
	} else if left.unit == NoUnit && right.unit != NoUnit {
		// 3 * 2 anyunit
		Ok(Number::new(left.value * right.value, right.unit))
	} else if lcat == Length && rcat == Length {
		// length * length
		let result = (left.value * left.unit.weight()) * (right.value * right.unit.weight());
		Ok(to_ideal_unit(Number::new(result, SquareMillimeter)))
	} else if lcat == Length && rcat == Area {
		// length * area
		let result = (left.value * left.unit.weight()) * (right.value * right.unit.weight());
		Ok(to_ideal_unit(Number::new(result, CubicMillimeter)))
	} else if lcat == Speed && rcat == Time {
		// 1 km/h * 1h
		let kph_value = left.value * left.unit.weight();
		let hours = convert(right, Hour)?;
		let result = kph_value * hours.value;
		let final_unit = match left.unit {
			KilometersPerHour => Kilometer,
			MetersPerSecond => Meter,
			MilesPerHour => Mile,
			FeetPerSecond => Foot,
			Knot => NauticalMile,
			_ => Meter,
		};
		let kilometers = Number::new(result, Kilometer);
		Ok(convert(kilometers, final_unit)?)
	} else if lcat == DataTransferRate && rcat == Time {
		// 8 megabytes per second * 1 minute
		let data_rate_value = left.value * left.unit.weight();
		let seconds = convert(right, Second)?;
		let result = data_rate_value * seconds.value;
		let final_unit = match left.unit {
			BitsPerSecond => Bit,
			KilobitsPerSecond => Kilobit,
			MegabitsPerSecond => Megabit,
			GigabitsPerSecond => Gigabit,
			TerabitsPerSecond => Terabit,
			PetabitsPerSecond => Petabit,
			ExabitsPerSecond => Exabit,
			ZettabitsPerSecond => Zettabit,
			YottabitsPerSecond => Yottabit,
			KibibitsPerSecond => Kibibit,
			MebibitsPerSecond => Mebibit,
			GibibitsPerSecond => Gibibit,
			TebibitsPerSecond => Tebibit,
			PebibitsPerSecond => Pebibit,
			ExbibitsPerSecond => Exbibit,
			ZebibitsPerSecond => Zebibit,
			YobibitsPerSecond => Yobibit,
			BytesPerSecond => Byte,
			KilobytesPerSecond => Kilobyte,
			MegabytesPerSecond => Megabyte,
			GigabytesPerSecond => Gigabyte,
			TerabytesPerSecond => Terabyte,
			PetabytesPerSecond => Petabyte,
			ExabytesPerSecond => Exabyte,
			ZettabytesPerSecond => Zettabyte,
			YottabytesPerSecond => Yottabyte,
			KibibytesPerSecond => Kibibyte,
			MebibytesPerSecond => Mebibyte,
			GibibytesPerSecond => Gibibyte,
			TebibytesPerSecond => Tebibyte,
			PebibytesPerSecond => Pebibyte,
			ExbibytesPerSecond => Exbibyte,
			ZebibytesPerSecond => Zebibyte,
			YobibytesPerSecond => Yobibyte,
			_ => Bit,
		};
		let data_storage = Number::new(result, Bit);
		Ok(convert(data_storage, final_unit)?)
	} else if lcat == Voltage && rcat == ElectricCurrent {
		// 1 volt * 1 ampere = 1 watt
		let result = (left.value * left.unit.weight()) * (right.value * right.unit.weight());
		Ok(to_ideal_unit(Number::new(result, Watt)))
	} else if lcat == ElectricCurrent && rcat == Resistance {
		// 1 amp * 1 ohm = 1 volt
		let result = (left.value * left.unit.weight()) * (right.value * right.unit.weight());
		Ok(to_ideal_unit(Number::new(result, Watt)))
	} else if lcat == Power && rcat == Time {
		// 1 watt * 1 second = 1 joule
		let result = (left.value * left.unit.weight()) * (right.value * right.unit.weight() / Unit::Second.weight());
		match right.unit {
			Second => Ok(to_ideal_joule_unit(Number::new(result, Joule))),
			_ => Ok(to_ideal_unit(Number::new(result, Joule))),
		}
	} else if swapped {
		Err(format!("Cannot multiply {:?} and {:?}", right.unit, left.unit))
	} else {
		actual_multiply(right, left, true)
	}
}

/// Divide a [`Number`] by another [`Number`]
/// 
/// - Temperatures don't work
/// - If you divide a unit by that same unit, the result has a unit of [`NoType`]
/// - If you divide [`Volume`] by [`Length`], the result has a unit of [`Area`], etc.
/// - If you divide [`Length`] by [`Time`], the result has a unit of [`Speed`]
/// - If you divide [`Length`] by [`Speed`], the result has a unit of [`Time`]
/// - If you divide [`Power`] by [`ElectricCurrent`], the result has a unit of [`Volt`]
/// - If you divide [`Voltage`] by [`ElectricCurrent`], the result has a unit of [`Ohm`]
/// - If you divide [`Voltage`] by [`Resistance`], the result has a unit of [`Ampere`]
/// - If you divide [`Power`] by [`Voltage`], the result has a unit of [`Ampere`]
/// - If you divide [`Energy`] by [`Time`], the result has a unit of [`Power`]
pub fn divide(left: Number, right: Number) -> Result<Number, String> {
	let lcat = left.unit.category();
	let rcat = right.unit.category();
	if left.unit == NoUnit && right.unit == NoUnit {
		// 3 / 2
		Ok(Number::new(left.value / right.value, left.unit))
	} else if lcat == Temperature || rcat == Temperature {
		// if temperature
		Err(format!("Cannot divide {:?} by {:?}", left.unit, right.unit))
	} else if left.unit != NoUnit && right.unit == NoUnit {
		// 1 km / 2
		Ok(Number::new(left.value / right.value, left.unit))
	} else if lcat == rcat {
		// 4 km / 2 km
		let (left, right) = convert_to_lowest(left, right)?;
		Ok(Number::new(left.value / right.value, NoUnit))
	} else if (lcat == Area && rcat == Length) || (lcat == Volume && rcat == Area) {
		// 1 km2 / 1 km, 1 km3 / 1 km2
		let result = (left.value * left.unit.weight()) / (right.value * right.unit.weight());
		Ok(to_ideal_unit(Number::new(result, Millimeter)))
	} else if lcat == Volume && rcat == Length {
		// 1 km3 / 1 km
		let result = (left.value * left.unit.weight()) / (right.value * right.unit.weight());
		Ok(to_ideal_unit(Number::new(result, SquareMillimeter)))
	} else if lcat == Length && rcat == Time {
		// 1 km / 2s
		let final_unit = match (left.unit, right.unit) {
			(Kilometer, Hour) => KilometersPerHour,
			(Meter, Second) => MetersPerSecond,
			(Mile, Hour) => MilesPerHour,
			(Foot, Second) => FeetPerSecond,
			(NauticalMile, Hour) => Knot,
			_ => KilometersPerHour,
		};
		let kilometers = convert(left, Kilometer)?;
		let hours = convert(right, Hour)?;
		let kph = Number::new(kilometers.value / hours.value, KilometersPerHour);
		Ok(convert(kph, final_unit)?)
	} else if lcat == Length && rcat == Speed {
		// 12 km / 100 kph
		let kilometers = convert(left, Kilometer)?;
		let kilometers_per_hour = convert(right, KilometersPerHour)?;
		let hour = Number::new(kilometers.value / kilometers_per_hour.value, Hour);
		Ok(to_ideal_unit(hour))
	} else if lcat == DigitalStorage && rcat == DataTransferRate {
		// 1 kilobit / 1 bit per second
		let bits = convert(left, Bit)?;
		let bits_per_second = convert(right, BitsPerSecond)?;
		let seconds = Number::new(bits.value / bits_per_second.value, Second);
		Ok(to_ideal_unit(seconds))
	} else if lcat == Power && rcat == ElectricCurrent {
		// 1 watt / 1 ampere = 1 volt
		let result = (left.value * left.unit.weight()) / (right.value * right.unit.weight());
		Ok(to_ideal_unit(Number::new(result, Volt)))
	} else if lcat == Voltage && rcat == ElectricCurrent {
		// 1 volt / 1 ampere = 1 ohm
		let result = (left.value * left.unit.weight()) / (right.value * right.unit.weight());
		Ok(to_ideal_unit(Number::new(result, Ohm)))
	} else if lcat == Voltage && rcat == Resistance {
		// 1 volt / 1 ohm = 1 amp
		let result = (left.value * left.unit.weight()) / (right.value * right.unit.weight());
		Ok(to_ideal_unit(Number::new(result, Ampere)))
	} else if lcat == Power && rcat == Voltage {
		// 1 watt / 1 volt = 1 amp
		let result = (left.value * left.unit.weight()) / (right.value * right.unit.weight());
		Ok(to_ideal_unit(Number::new(result, Ampere)))
	} else if lcat == Energy && rcat == Time {
		// 1 joule / 1 second = 1 watt
		let result = (left.value * left.unit.weight()) / (right.value * right.unit.weight() / Unit::Second.weight());
		Ok(to_ideal_unit(Number::new(result, Watt)))
	} else {
		Err(format!("Cannot divide {:?} by {:?}", left.unit, right.unit))
	}
}
/// Modulo a [`Number`] by another [`Number`].
/// 
/// `left` and `right` need to have the same [`UnitType`], and the result will have that same [`UnitType`].
///
/// Temperatures don't work.
pub fn modulo(left: Number, right: Number) -> Result<Number, String> {
	if left.unit.category() == Temperature || right.unit.category() == Temperature {
		// if temperature
		Err(format!("Cannot modulo {:?} by {:?}", left.unit, right.unit))
	} else if left.unit.category() == right.unit.category() {
		// 5 km % 3 m
		let (left, right) = convert_to_lowest(left, right)?;
		Ok(Number::new(left.value % right.value, left.unit))
	} else {
		Err(format!("Cannot modulo {:?} by {:?}", left.unit, right.unit))
	}
}

/// Returns a [`Number`] to the power of another [`Number`]
/// 
/// - If you take [`Length`] to the power of [`NoType`], the result has a unit of [`Area`].
/// - If you take [`Length`] to the power of [`Length`], the result has a unit of [`Area`]
/// - If you take [`Length`] to the power of [`Area`], the result has a unit of [`Volume`]
/// - etc.
pub fn pow(left: Number, right: Number) -> Result<Number, String> {
	let lcat = left.unit.category();
	let rcat = left.unit.category();
	if left.unit == NoUnit && right.unit == NoUnit {
		// 3 ^ 2
		Ok(Number::new(left.value.pow(right.value), left.unit))
	} else if right.value == d128!(1) && right.unit == NoUnit {
		Ok(left)
	} else if lcat == Length && right.unit == NoUnit && right.value == d128!(2) {
		// x km ^ 2
		let result = (left.value * left.unit.weight()).pow(right.value);
		Ok(to_ideal_unit(Number::new(result, SquareMillimeter)))
	} else if lcat == Length && right.unit == NoUnit && right.value == d128!(3) {
		// x km ^ 3
		let result = (left.value * left.unit.weight()).pow(right.value);
		Ok(to_ideal_unit(Number::new(result, CubicMillimeter)))
	} else if lcat == Length && rcat == Length && right.value == d128!(1) {
		// x km ^ 1 km
		Ok(multiply(left, right)?)
	} else if lcat == Length && rcat == Length && right.value == d128!(2) {
		// x km ^ 2 km
		let pow2 = multiply(left, Number::new(d128!(1), right.unit))?;
		let pow3 = multiply(pow2, Number::new(d128!(1), right.unit))?;
		Ok(pow3)
	} else if lcat == Length && rcat == Area && right.value == d128!(1) {
		// x km ^ km2
		Ok(multiply(left, Number::new(d128!(1), right.unit))?)
	} else {
		Err(format!("Cannot multiply {:?} and {:?}", left.unit, right.unit))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	macro_rules! assert_float_eq {
		( $actual:expr, $expected:literal ) => {
			assert!(($actual - $expected).abs() < f64::EPSILON);
		}
	}

	#[test]
	fn test_convert() {
		pub fn convert_test(value: f64, unit: Unit, to_unit: Unit) -> f64 {
			use std::str::FromStr;

			let value_string = &value.to_string();
			let value_d128 = d128::from_str(value_string).unwrap();
			let number = Number::new(value_d128, unit);

			let result = convert(number, to_unit);
			let string_result = &result.unwrap().value.to_string();
			let float_result = f64::from_str(string_result).unwrap();

			return float_result;
		}

		assert_float_eq!(convert_test(1000.0, Nanosecond, Microsecond), 1.0);
		assert_float_eq!(convert_test(1000.0, Nanosecond, Microsecond), 1.0);
		assert_float_eq!(convert_test(1000.0, Microsecond, Millisecond), 1.0);
		assert_float_eq!(convert_test(1000.0, Millisecond, Second), 1.0);
		assert_float_eq!(convert_test(60.0, Second, Minute), 1.0);
		assert_float_eq!(convert_test(60.0, Minute, Hour), 1.0);
		assert_float_eq!(convert_test(24.0, Hour, Day), 1.0);
		assert_float_eq!(convert_test(7.0, Day, Week), 1.0);
		assert_float_eq!(convert_test(30.436875, Day, Month), 1.0);
		assert_float_eq!(convert_test(3.0, Month, Quarter), 1.0);
		assert_float_eq!(convert_test(4.0, Quarter, Year), 1.0);
		assert_float_eq!(convert_test(10.0, Year, Decade), 1.0);
		assert_float_eq!(convert_test(10.0, Decade, Century), 1.0);
		assert_float_eq!(convert_test(10.0, Century, Millenium), 1.0);

		assert_float_eq!(convert_test(10.0, Millimeter, Centimeter), 1.0);
		assert_float_eq!(convert_test(10.0, Centimeter, Decimeter), 1.0);
		assert_float_eq!(convert_test(10.0, Decimeter, Meter), 1.0);
		assert_float_eq!(convert_test(1000.0, Meter, Kilometer), 1.0);
		assert_float_eq!(convert_test(2.54, Centimeter, Inch), 1.0);
		assert_float_eq!(convert_test(12.0, Inch, Foot), 1.0);
		assert_float_eq!(convert_test(3.0, Foot, Yard), 1.0);
		assert_float_eq!(convert_test(1760.0, Yard, Mile), 1.0);
		assert_float_eq!(convert_test(42195.0, Meter, Marathon), 1.0);
		assert_float_eq!(convert_test(1852.0, Meter, NauticalMile), 1.0);
		assert_float_eq!(convert_test(9460730472580800.0, Meter, LightYear), 1.0);
		assert_float_eq!(convert_test(299792458.0, Meter, LightSecond), 1.0);

		assert_float_eq!(convert_test(100.0, SquareMillimeter, SquareCentimeter), 1.0);
		assert_float_eq!(convert_test(100.0, SquareCentimeter, SquareDecimeter), 1.0);
		assert_float_eq!(convert_test(100.0, SquareDecimeter, SquareMeter), 1.0);
		assert_float_eq!(convert_test(1000000.0, SquareMeter, SquareKilometer), 1.0);
		assert_float_eq!(convert_test(645.16, SquareMillimeter, SquareInch), 1.0);
		assert_float_eq!(convert_test(144.0, SquareInch, SquareFoot), 1.0);
		assert_float_eq!(convert_test(9.0, SquareFoot, SquareYard), 1.0);
		assert_float_eq!(convert_test(3097600.0, SquareYard, SquareMile), 1.0);
		assert_float_eq!(convert_test(100.0, SquareMeter, Are), 1.0);
		assert_float_eq!(convert_test(10.0, Are, Decare), 1.0);
		assert_float_eq!(convert_test(10.0, Decare, Hectare), 1.0);
		assert_float_eq!(convert_test(640.0, Acre, SquareMile), 1.0);

		assert_float_eq!(convert_test(1000.0, CubicMillimeter, CubicCentimeter), 1.0);
		assert_float_eq!(convert_test(1000.0, CubicCentimeter, CubicDecimeter), 1.0);
		assert_float_eq!(convert_test(1000.0, CubicDecimeter, CubicMeter), 1.0);
		assert_float_eq!(convert_test(1000000000.0, CubicMeter, CubicKilometer), 1.0);
		assert_float_eq!(convert_test(1728.0, CubicInch, CubicFoot), 1.0);
		assert_float_eq!(convert_test(27.0, CubicFoot, CubicYard), 1.0);
		assert_float_eq!(convert_test(5451776000.0, CubicYard, CubicMile), 1.0);
		assert_float_eq!(convert_test(1.0, Milliliter, CubicCentimeter), 1.0);
		assert_float_eq!(convert_test(10.0, Milliliter, Centiliter), 1.0);
		assert_float_eq!(convert_test(10.0, Centiliter, Deciliter), 1.0);
		assert_float_eq!(convert_test(10.0, Deciliter, Liter), 1.0);
		assert_float_eq!(convert_test(4.92892159375, Milliliter, Teaspoon), 1.0);
		assert_float_eq!(convert_test(3.0, Teaspoon, Tablespoon), 1.0);
		assert_float_eq!(convert_test(2.0, Tablespoon, FluidOunce), 1.0);
		assert_float_eq!(convert_test(8.0, FluidOunce, Cup), 1.0);
		assert_float_eq!(convert_test(2.0, Cup, Pint), 1.0);
		assert_float_eq!(convert_test(2.0, Pint, Quart), 1.0);
		assert_float_eq!(convert_test(4.0, Quart, Gallon), 1.0);
		assert_float_eq!(convert_test(42.0, Gallon, OilBarrel), 1.0);

		assert_float_eq!(convert_test(1000.0, Milligram, Gram), 1.0);
		assert_float_eq!(convert_test(100.0, Gram, Hectogram), 1.0);
		assert_float_eq!(convert_test(1000.0, Gram, Kilogram), 1.0);
		assert_float_eq!(convert_test(1000.0, Kilogram, MetricTon), 1.0);
		assert_float_eq!(convert_test(0.45359237, Kilogram, Pound), 1.0);
		assert_float_eq!(convert_test(16.0, Ounce, Pound), 1.0);
		assert_float_eq!(convert_test(14.0, Pound, Stone), 1.0);
		assert_float_eq!(convert_test(2000.0, Pound, ShortTon), 1.0);
		assert_float_eq!(convert_test(2240.0, Pound, LongTon), 1.0);

		assert_float_eq!(convert_test(1000.0, Bit, Kilobit), 1.0);
		assert_float_eq!(convert_test(1000.0, Kilobit, Megabit), 1.0);
		assert_float_eq!(convert_test(1000.0, Megabit, Gigabit), 1.0);
		assert_float_eq!(convert_test(1000.0, Gigabit, Terabit), 1.0);
		assert_float_eq!(convert_test(1000.0, Terabit, Petabit), 1.0);
		assert_float_eq!(convert_test(1000.0, Petabit, Exabit), 1.0);
		assert_float_eq!(convert_test(1000.0, Exabit, Zettabit), 1.0);
		assert_float_eq!(convert_test(1000.0, Zettabit, Yottabit), 1.0);
		assert_float_eq!(convert_test(1024.0, Bit, Kibibit), 1.0);
		assert_float_eq!(convert_test(1024.0, Kibibit, Mebibit), 1.0);
		assert_float_eq!(convert_test(1024.0, Mebibit, Gibibit), 1.0);
		assert_float_eq!(convert_test(1024.0, Gibibit, Tebibit), 1.0);
		assert_float_eq!(convert_test(1024.0, Tebibit, Pebibit), 1.0);
		assert_float_eq!(convert_test(1024.0, Pebibit, Exbibit), 1.0);
		assert_float_eq!(convert_test(1024.0, Exbibit, Zebibit), 1.0);
		assert_float_eq!(convert_test(1024.0, Zebibit, Yobibit), 1.0);
		assert_float_eq!(convert_test(8.0, Bit, Byte), 1.0);
		assert_float_eq!(convert_test(1000.0, Byte, Kilobyte), 1.0);
		assert_float_eq!(convert_test(1000.0, Kilobyte, Megabyte), 1.0);
		assert_float_eq!(convert_test(1000.0, Megabyte, Gigabyte), 1.0);
		assert_float_eq!(convert_test(1000.0, Gigabyte, Terabyte), 1.0);
		assert_float_eq!(convert_test(1000.0, Terabyte, Petabyte), 1.0);
		assert_float_eq!(convert_test(1000.0, Petabyte, Exabyte), 1.0);
		assert_float_eq!(convert_test(1000.0, Exabyte, Zettabyte), 1.0);
		assert_float_eq!(convert_test(1000.0, Zettabyte, Yottabyte), 1.0);
		assert_float_eq!(convert_test(1024.0, Kibibyte, Mebibyte), 1.0);
		assert_float_eq!(convert_test(1024.0, Mebibyte, Gibibyte), 1.0);
		assert_float_eq!(convert_test(1024.0, Gibibyte, Tebibyte), 1.0);
		assert_float_eq!(convert_test(1024.0, Tebibyte, Pebibyte), 1.0);
		assert_float_eq!(convert_test(1024.0, Pebibyte, Exbibyte), 1.0);
		assert_float_eq!(convert_test(1024.0, Exbibyte, Zebibyte), 1.0);
		assert_float_eq!(convert_test(1024.0, Zebibyte, Yobibyte), 1.0);

		assert_float_eq!(convert_test(1000.0, BitsPerSecond, KilobitsPerSecond), 1.0);
		assert_float_eq!(convert_test(1000.0, KilobitsPerSecond, MegabitsPerSecond), 1.0);
		assert_float_eq!(convert_test(1000.0, MegabitsPerSecond, GigabitsPerSecond), 1.0);
		assert_float_eq!(convert_test(1000.0, GigabitsPerSecond, TerabitsPerSecond), 1.0);
		assert_float_eq!(convert_test(1000.0, TerabitsPerSecond, PetabitsPerSecond), 1.0);
		assert_float_eq!(convert_test(1000.0, PetabitsPerSecond, ExabitsPerSecond), 1.0);
		assert_float_eq!(convert_test(1000.0, ExabitsPerSecond, ZettabitsPerSecond), 1.0);
		assert_float_eq!(convert_test(1000.0, ZettabitsPerSecond, YottabitsPerSecond), 1.0);
		assert_float_eq!(convert_test(1024.0, BitsPerSecond, KibibitsPerSecond), 1.0);
		assert_float_eq!(convert_test(1024.0, KibibitsPerSecond, MebibitsPerSecond), 1.0);
		assert_float_eq!(convert_test(1024.0, MebibitsPerSecond, GibibitsPerSecond), 1.0);
		assert_float_eq!(convert_test(1024.0, GibibitsPerSecond, TebibitsPerSecond), 1.0);
		assert_float_eq!(convert_test(1024.0, TebibitsPerSecond, PebibitsPerSecond), 1.0);
		assert_float_eq!(convert_test(1024.0, PebibitsPerSecond, ExbibitsPerSecond), 1.0);
		assert_float_eq!(convert_test(1024.0, ExbibitsPerSecond, ZebibitsPerSecond), 1.0);
		assert_float_eq!(convert_test(1024.0, ZebibitsPerSecond, YobibitsPerSecond), 1.0);
		assert_float_eq!(convert_test(8.0, BitsPerSecond, BytesPerSecond), 1.0);
		assert_float_eq!(convert_test(1000.0, BytesPerSecond, KilobytesPerSecond), 1.0);
		assert_float_eq!(convert_test(1000.0, KilobytesPerSecond, MegabytesPerSecond), 1.0);
		assert_float_eq!(convert_test(1000.0, MegabytesPerSecond, GigabytesPerSecond), 1.0);
		assert_float_eq!(convert_test(1000.0, GigabytesPerSecond, TerabytesPerSecond), 1.0);
		assert_float_eq!(convert_test(1000.0, TerabytesPerSecond, PetabytesPerSecond), 1.0);
		assert_float_eq!(convert_test(1000.0, PetabytesPerSecond, ExabytesPerSecond), 1.0);
		assert_float_eq!(convert_test(1000.0, ExabytesPerSecond, ZettabytesPerSecond), 1.0);
		assert_float_eq!(convert_test(1000.0, ZettabytesPerSecond, YottabytesPerSecond), 1.0);
		assert_float_eq!(convert_test(1024.0, KibibytesPerSecond, MebibytesPerSecond), 1.0);
		assert_float_eq!(convert_test(1024.0, MebibytesPerSecond, GibibytesPerSecond), 1.0);
		assert_float_eq!(convert_test(1024.0, GibibytesPerSecond, TebibytesPerSecond), 1.0);
		assert_float_eq!(convert_test(1024.0, TebibytesPerSecond, PebibytesPerSecond), 1.0);
		assert_float_eq!(convert_test(1024.0, PebibytesPerSecond, ExbibytesPerSecond), 1.0);
		assert_float_eq!(convert_test(1024.0, ExbibytesPerSecond, ZebibytesPerSecond), 1.0);
		assert_float_eq!(convert_test(1024.0, ZebibytesPerSecond, YobibytesPerSecond), 1.0);

		assert_float_eq!(convert_test(1000.0, Millijoule, Joule), 1.0);
		assert_float_eq!(convert_test(1000.0, Joule, Kilojoule), 1.0);
		assert_float_eq!(convert_test(1.0, NewtonMeter, Joule), 1.0);
		assert_float_eq!(convert_test(1000.0, Kilojoule, Megajoule), 1.0);
		assert_float_eq!(convert_test(1000.0, Megajoule, Gigajoule), 1.0);
		assert_float_eq!(convert_test(1000.0, Gigajoule, Terajoule), 1.0);
		assert_float_eq!(convert_test(4.1868, Joule, Calorie), 1.0);
		assert_float_eq!(convert_test(1000.0, Calorie, KiloCalorie), 1.0);
		assert_float_eq!(convert_test(1055.05585262, Joule, BritishThermalUnit), 1.0);
		assert_float_eq!(convert_test(3600.0, Joule, WattHour), 1.0);
		assert_float_eq!(convert_test(1000.0, WattHour, KilowattHour), 1.0);
		assert_float_eq!(convert_test(1000.0, KilowattHour, MegawattHour), 1.0);
		assert_float_eq!(convert_test(1000.0, MegawattHour, GigawattHour), 1.0);
		assert_float_eq!(convert_test(1000.0, GigawattHour, TerawattHour), 1.0);
		assert_float_eq!(convert_test(1000.0, TerawattHour, PetawattHour), 1.0);

		assert_float_eq!(convert_test(1000.0, Milliwatt, Watt), 1.0);
		assert_float_eq!(convert_test(1000.0, Watt, Kilowatt), 1.0);
		assert_float_eq!(convert_test(1000.0, Kilowatt, Megawatt), 1.0);
		assert_float_eq!(convert_test(1000.0, Megawatt, Gigawatt), 1.0);
		assert_float_eq!(convert_test(1000.0, Gigawatt, Terawatt), 1.0);
		assert_float_eq!(convert_test(1000.0, Terawatt, Petawatt), 1.0);
		assert_float_eq!(convert_test(0.0568690272188, Watt, BritishThermalUnitsPerMinute), 1.0);
		assert_float_eq!(convert_test(60.0, BritishThermalUnitsPerMinute, BritishThermalUnitsPerHour), 1.0);
		assert_float_eq!(convert_test(745.6998715822702, Watt, Horsepower), 1.0);
		assert_float_eq!(convert_test(735.49875, Watt, MetricHorsepower), 1.0);

		assert_float_eq!(convert_test(1000.0, Milliampere, Ampere), 1.0);
		assert_float_eq!(convert_test(1000.0, Ampere, Kiloampere), 1.0);
		assert_float_eq!(convert_test(10.0, Ampere, Abampere), 1.0);

		assert_float_eq!(convert_test(1000.0, Milliohm, Ohm), 1.0);
		assert_float_eq!(convert_test(1000.0, Ohm, Kiloohm), 1.0);

		assert_float_eq!(convert_test(1000.0, Millivolt, Volt), 1.0);
		assert_float_eq!(convert_test(1000.0, Volt, Kilovolt), 1.0);

		assert_float_eq!(convert_test(1000.0, Pascal, Kilopascal), 1.0);
		assert_float_eq!(convert_test(101325.0, Pascal, Atmosphere), 1.0);
		assert_float_eq!(convert_test(100.0, Pascal, Millibar), 1.0);
		assert_float_eq!(convert_test(1000.0, Millibar, Bar), 1.0);
		assert_float_eq!(convert_test(3386.389, Pascal, InchOfMercury), 1.0);
		assert_float_eq!(convert_test(6894.757293168361, Pascal, PoundsPerSquareInch), 1.0);
		assert_float_eq!(convert_test(162.12, Pascal, Torr), 1.0);

		assert_float_eq!(convert_test(1000.0, Hertz, Kilohertz), 1.0);
		assert_float_eq!(convert_test(1000.0, Kilohertz, Megahertz), 1.0);
		assert_float_eq!(convert_test(1000.0, Megahertz, Gigahertz), 1.0);
		assert_float_eq!(convert_test(1000.0, Gigahertz, Terahertz), 1.0);
		assert_float_eq!(convert_test(1000.0, Terahertz, Petahertz), 1.0);
		assert_float_eq!(convert_test(60.0, Hertz, RevolutionsPerMinute), 1.0);

		assert_float_eq!(convert_test(3.6, KilometersPerHour, MetersPerSecond), 1.0);
		assert_float_eq!(convert_test(0.3048, MetersPerSecond, FeetPerSecond), 1.0);
		assert_float_eq!(convert_test(1.609344, KilometersPerHour, MilesPerHour), 1.0);
		assert_float_eq!(convert_test(1.852, KilometersPerHour, Knot), 1.0);

		assert_float_eq!(convert_test(274.15, Kelvin, Celsius), 1.0);
		assert_float_eq!(convert_test(300.0, Kelvin, Fahrenheit), 80.33);
		assert_float_eq!(convert_test(-272.15, Celsius, Kelvin), 1.0);
		assert_float_eq!(convert_test(-15.0, Celsius, Fahrenheit), 5.0);
		assert_float_eq!(convert_test(80.33, Fahrenheit, Kelvin), 300.0);
		assert_float_eq!(convert_test(5.0, Fahrenheit, Celsius), -15.0);
	}
}
