use decimal::d128;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UnitType {
  NoUnit,
  Time,
  Length,
  Area,
  Volume,
  Mass,
  Temperature,
}
use UnitType::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Unit {
  NoUnit,

  Nanosecond,
  Microsecond,
  Millisecond,
  Second,
  Minute,
  Hour,
  Day,
  Week,
  Month,
  Quarter,
  Year,
  Decade,
  Century,
  Millenium,

  Millimeter,
  Centimeter,
  Decimeter,
  Meter,
  Kilometer,
  Inch,
  Foot,
  Yard,
  Mile,
  // 1-dimensional only:
  NauticalMile,
  LightYear,

  SquareMillimeter,
  SquareCentimeter,
  SquareDecimeter,
  SquareMeter,
  SquareKilometer,
  SquareInch,
  SquareFoot,
  SquareYard,
  SquareMile,
  // 2-dimensional only:
  Are,
  Decare,
  Hectare,
  Acre,

  CubicMillimeter,
  CubicCentimeter,
  CubicDecimeter,
  CubicMeter,
  CubicKilometer,
  CubicInch,
  CubicFoot,
  CubicYard,
  CubicMile,
    // 3-dimensional only:
  Milliliter,
  Centiliter,
  Deciliter,
  Liter,
  Teaspoon,
  Tablespoon,
  FluidOunce,
  Cup,
  Pint,
  Quart,
  Gallon,
  OilBarrel,

  Milligram,
  Gram,
  Hectogram,
  Kilogram,
  ShortTon,
  LongTon,
  MetricTon,
  Ounce,
  Pound,
}
use Unit::*;

fn get_info(unit: &Unit) -> (UnitType, d128) {
  match unit {
    Unit::NoUnit => (UnitType::NoUnit, d128!(1)),

    Nanosecond          => (Time, d128!(1)),
    Microsecond         => (Time, d128!(1000)),
    Millisecond         => (Time, d128!(1000000)),
    Second              => (Time, d128!(1000000000)),
    Minute              => (Time, d128!(60000000000)),
    Hour                => (Time, d128!(3600000000000)),
    Day                 => (Time, d128!(86400000000000)),
    Week                => (Time, d128!(604800000000000)),
    Month               => (Time, d128!(2629746000000000)),
    Quarter             => (Time, d128!(7889238000000000)),
    Year                => (Time, d128!(31556952000000000)),
    Decade              => (Time, d128!(315569520000000000)),
    Century             => (Time, d128!(3155695200000000000)),
    Millenium           => (Time, d128!(31556952000000000000)),

    Millimeter          => (Length, d128!(10)),
    Centimeter          => (Length, d128!(100)),
    Decimeter           => (Length, d128!(1000)),
    Meter               => (Length, d128!(10000)),
    Kilometer           => (Length, d128!(10000000)),
    Inch                => (Length, d128!(254)),
    Foot                => (Length, d128!(3048)),
    Yard                => (Length, d128!(9144)),
    Mile                => (Length, d128!(16093440)),
    NauticalMile        => (Length, d128!(18520000)),
    LightYear           => (Length, d128!(94607304725808000000)),

    SquareMillimeter    => (Area, d128!(100)),
    SquareCentimeter    => (Area, d128!(10000)),
    SquareDecimeter     => (Area, d128!(1000000)),
    SquareMeter         => (Area, d128!(100000000)),
    SquareKilometer     => (Area, d128!(100000000000000)),
    SquareInch          => (Area, d128!(64516)),
    SquareFoot          => (Area, d128!(9290304)),
    SquareYard          => (Area, d128!(83612736)),
    SquareMile          => (Area, d128!(258998811033600)),
    Are                 => (Area, d128!(10000000000)),
    Decare              => (Area, d128!(100000000000)),
    Hectare             => (Area, d128!(1000000000000)),
    Acre                => (Area, d128!(404685642240)),

    CubicMillimeter     => (Volume, d128!(1)),
    CubicCentimeter     => (Volume, d128!(1000)),
    CubicDecimeter      => (Volume, d128!(1000000)),
    CubicMeter          => (Volume, d128!(1000000000)),
    CubicKilometer      => (Volume, d128!(1000000000000000000)),
    CubicInch           => (Volume, d128!(16387.064)),
    CubicFoot           => (Volume, d128!(28316846.592)),
    CubicYard           => (Volume, d128!(764554857.984)),
    CubicMile           => (Volume, d128!(4168181825440579584)),
    Milliliter          => (Volume, d128!(1000)),
    Centiliter          => (Volume, d128!(10000)),
    Deciliter           => (Volume, d128!(100000)),
    Liter               => (Volume, d128!(1000000)),
    Teaspoon            => (Volume, d128!(4928.92159375)),
    Tablespoon          => (Volume, d128!(14786.76478125)),
    FluidOunce          => (Volume, d128!(29573.5295625)),
    Cup                 => (Volume, d128!(236588.2365)),
    Pint                => (Volume, d128!(473176.473)),
    Quart               => (Volume, d128!(946352.946)),
    Gallon              => (Volume, d128!(3785411.784)),
    OilBarrel           => (Volume, d128!(158987294.928)),

    Milligram           => (Mass, d128!(0.001)),
    Gram                => (Mass, d128!(1)),
    Hectogram           => (Mass, d128!(100)),
    Kilogram            => (Mass, d128!(1000)),
    MetricTon           => (Mass, d128!(1000000)),
    Ounce               => (Mass, d128!(28.349523125)),
    Pound               => (Mass, d128!(453.59237)),
    ShortTon            => (Mass, d128!(907184.74)),
    LongTon             => (Mass, d128!(1016046.9088)),
  }
}

impl Unit {
  pub fn category(&self) -> UnitType {
    return get_info(self).0
  }
  pub fn weight(&self) -> d128 {
    return get_info(self).1
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_convert() {
    pub fn convert(value: &str, unit: Unit, to_unit: Unit) -> f64 {
      use std::str::FromStr;
      let decimal_value = d128::from_str(value).unwrap();
      let result = decimal_value * unit.weight() / to_unit.weight();
      let string_result = &result.to_string();
      return f64::from_str(string_result).unwrap();
    }
    assert_eq!(convert("1000", Nanosecond, Microsecond), 1.0);
    assert_eq!(convert("1000", Microsecond, Millisecond), 1.0);
    assert_eq!(convert("1000", Millisecond, Second), 1.0);
    assert_eq!(convert("60", Second, Minute), 1.0);
    assert_eq!(convert("60", Minute, Hour), 1.0);
    assert_eq!(convert("24", Hour, Day), 1.0);
    assert_eq!(convert("7", Day, Week), 1.0);
    assert_eq!(convert("30.436875", Day, Month), 1.0);
    assert_eq!(convert("3", Month, Quarter), 1.0);
    assert_eq!(convert("4", Quarter, Year), 1.0);
    assert_eq!(convert("10", Year, Decade), 1.0);
    assert_eq!(convert("10", Decade, Century), 1.0);
    assert_eq!(convert("10", Century, Millenium), 1.0);

    assert_eq!(convert("10", Millimeter, Centimeter), 1.0);
    assert_eq!(convert("10", Centimeter, Decimeter), 1.0);
    assert_eq!(convert("10", Decimeter, Meter), 1.0);
    assert_eq!(convert("1000", Meter, Kilometer), 1.0);
    assert_eq!(convert("2.54", Centimeter, Inch), 1.0);
    assert_eq!(convert("12", Inch, Foot), 1.0);
    assert_eq!(convert("3", Foot, Yard), 1.0);
    assert_eq!(convert("1760", Yard, Mile), 1.0);
    assert_eq!(convert("1852", Meter, NauticalMile), 1.0);
    assert_eq!(convert("9460730472580800", Meter, LightYear), 1.0);
    
    assert_eq!(convert("10", Millimeter, Centimeter), 1.0);
    assert_eq!(convert("10", Centimeter, Decimeter), 1.0);
    assert_eq!(convert("10", Decimeter, Meter), 1.0);
    assert_eq!(convert("1000", Meter, Kilometer), 1.0);
    assert_eq!(convert("2.54", Centimeter, Inch), 1.0);
    assert_eq!(convert("12", Inch, Foot), 1.0);
    assert_eq!(convert("3", Foot, Yard), 1.0);
    assert_eq!(convert("1760", Yard, Mile), 1.0);
    assert_eq!(convert("1852", Meter, NauticalMile), 1.0);
    assert_eq!(convert("9460730472580800", Meter, LightYear), 1.0);
    
    assert_eq!(convert("100", SquareMillimeter, SquareCentimeter), 1.0);
    assert_eq!(convert("100", SquareCentimeter, SquareDecimeter), 1.0);
    assert_eq!(convert("100", SquareDecimeter, SquareMeter), 1.0);
    assert_eq!(convert("1000000", SquareMeter, SquareKilometer), 1.0);
    assert_eq!(convert("645.16", SquareMillimeter, SquareInch), 1.0);
    assert_eq!(convert("144", SquareInch, SquareFoot), 1.0);
    assert_eq!(convert("9", SquareFoot, SquareYard), 1.0);
    assert_eq!(convert("3097600", SquareYard, SquareMile), 1.0);
    assert_eq!(convert("100", SquareMeter, Are), 1.0);
    assert_eq!(convert("10", Are, Decare), 1.0);
    assert_eq!(convert("10", Decare, Hectare), 1.0);
    assert_eq!(convert("640", Acre, SquareMile), 1.0);

    assert_eq!(convert("1000", CubicMillimeter, CubicCentimeter), 1.0);
    assert_eq!(convert("1000", CubicCentimeter, CubicDecimeter), 1.0);
    assert_eq!(convert("1000", CubicDecimeter, CubicMeter), 1.0);
    assert_eq!(convert("1000000000", CubicMeter, CubicKilometer), 1.0);
    assert_eq!(convert("1728", CubicInch, CubicFoot), 1.0);
    assert_eq!(convert("27", CubicFoot, CubicYard), 1.0);
    assert_eq!(convert("5451776000", CubicYard, CubicMile), 1.0);
    assert_eq!(convert("1", Milliliter, CubicCentimeter), 1.0);
    assert_eq!(convert("10", Milliliter, Centiliter), 1.0);
    assert_eq!(convert("10", Centiliter, Deciliter), 1.0);
    assert_eq!(convert("10", Deciliter, Liter), 1.0);
    assert_eq!(convert("4.92892159375", Milliliter, Teaspoon), 1.0);
    assert_eq!(convert("3", Teaspoon, Tablespoon), 1.0);
    assert_eq!(convert("2", Tablespoon, FluidOunce), 1.0);
    assert_eq!(convert("8", FluidOunce, Cup), 1.0);
    assert_eq!(convert("2", Cup, Pint), 1.0);
    assert_eq!(convert("2", Pint, Quart), 1.0);
    assert_eq!(convert("4", Quart, Gallon), 1.0);
    assert_eq!(convert("42", Gallon, OilBarrel), 1.0);

    assert_eq!(convert("1000", Milligram, Gram), 1.0);
    assert_eq!(convert("100", Gram, Hectogram), 1.0);
    assert_eq!(convert("1000", Gram, Kilogram), 1.0);
    assert_eq!(convert("1000", Kilogram, MetricTon), 1.0);
    assert_eq!(convert("0.45359237", Kilogram, Pound), 1.0);
    assert_eq!(convert("16", Ounce, Pound), 1.0);
    assert_eq!(convert("2000", Pound, ShortTon), 1.0);
    assert_eq!(convert("2240", Pound, LongTon), 1.0);
  }
}
