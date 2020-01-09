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

macro_rules! create_units {
  ( $( $x:ident : $y:expr ),*, ) => {
    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Unit {
      $($x),*  
    }
    use Unit::*;

    fn get_info(unit: &Unit) -> (UnitType, d128) {
      match unit {
        $(
          Unit::$x => $y
        ),*
      }
    }
  }
}

create_units!(
  NoUnit:             (UnitType::NoUnit, d128!(1)),

  Nanosecond:         (Time, d128!(1)),
  Microsecond:        (Time, d128!(1000)),
  Millisecond:        (Time, d128!(1000000)),
  Second:             (Time, d128!(1000000000)),
  Minute:             (Time, d128!(60000000000)),
  Hour:               (Time, d128!(3600000000000)),
  Day:                (Time, d128!(86400000000000)),
  Week:               (Time, d128!(604800000000000)),
  Month:              (Time, d128!(2629746000000000)),
  Quarter:            (Time, d128!(7889238000000000)),
  Year:               (Time, d128!(31556952000000000)),
  Decade:             (Time, d128!(315569520000000000)),
  Century:            (Time, d128!(3155695200000000000)),
  Millenium:          (Time, d128!(31556952000000000000)),

  Millimeter:         (Length, d128!(10)),
  Centimeter:         (Length, d128!(100)),
  Decimeter:          (Length, d128!(1000)),
  Meter:              (Length, d128!(10000)),
  Kilometer:          (Length, d128!(10000000)),
  Inch:               (Length, d128!(254)),
  Foot:               (Length, d128!(3048)),
  Yard:               (Length, d128!(9144)),
  Mile:               (Length, d128!(16093440)),
  NauticalMile:       (Length, d128!(18520000)),
  LightYear:          (Length, d128!(94607304725808000000)),

  SquareMillimeter:   (Area, d128!(100)),
  SquareCentimeter:   (Area, d128!(10000)),
  SquareDecimeter:    (Area, d128!(1000000)),
  SquareMeter:        (Area, d128!(100000000)),
  SquareKilometer:    (Area, d128!(100000000000000)),
  SquareInch:         (Area, d128!(64516)),
  SquareFoot:         (Area, d128!(9290304)),
  SquareYard:         (Area, d128!(83612736)),
  SquareMile:         (Area, d128!(258998811033600)),
  Are:                (Area, d128!(10000000000)),
  Decare:             (Area, d128!(100000000000)),
  Hectare:            (Area, d128!(1000000000000)),
  Acre:               (Area, d128!(404685642240)),

  CubicMillimeter:    (Volume, d128!(1)),
  CubicCentimeter:    (Volume, d128!(1000)),
  CubicDecimeter:     (Volume, d128!(1000000)),
  CubicMeter:         (Volume, d128!(1000000000)),
  CubicKilometer:     (Volume, d128!(1000000000000000000)),
  CubicInch:          (Volume, d128!(16387.064)),
  CubicFoot:          (Volume, d128!(28316846.592)),
  CubicYard:          (Volume, d128!(764554857.984)),
  CubicMile:          (Volume, d128!(4168181825440579584)),
  Milliliter:         (Volume, d128!(1000)),
  Centiliter:         (Volume, d128!(10000)),
  Deciliter:          (Volume, d128!(100000)),
  Liter:              (Volume, d128!(1000000)),
  Teaspoon:           (Volume, d128!(4928.92159375)),
  Tablespoon:         (Volume, d128!(14786.76478125)),
  FluidOunce:         (Volume, d128!(29573.5295625)),
  Cup:                (Volume, d128!(236588.2365)),
  Pint:               (Volume, d128!(473176.473)),
  Quart:              (Volume, d128!(946352.946)),
  Gallon:             (Volume, d128!(3785411.784)),
  OilBarrel:          (Volume, d128!(158987294.928)),

  Milligram:          (Mass, d128!(0.001)),
  Gram:               (Mass, d128!(1)),
  Hectogram:          (Mass, d128!(100)),
  Kilogram:           (Mass, d128!(1000)),
  MetricTon:          (Mass, d128!(1000000)),
  Ounce:              (Mass, d128!(28.349523125)),
  Pound:              (Mass, d128!(453.59237)),
  ShortTon:           (Mass, d128!(907184.74)),
  LongTon:            (Mass, d128!(1016046.9088)),
  Kelvin:             (Temperature, d128!(0)),
  Celcius:            (Temperature, d128!(0)),
  Fahrenheit:         (Temperature, d128!(0)),
);

impl Unit {
  pub fn category(&self) -> UnitType {
    return get_info(self).0
  }
  pub fn weight(&self) -> d128 {
    return get_info(self).1
  }
}

fn get_convertion_factor(unit: Unit, to_unit: Unit) -> d128 {
  return unit.weight() / to_unit.weight();
}

pub fn convert(value: d128, unit: Unit, to_unit: Unit) -> Result<d128, String> {
  if unit.category() == UnitType::Temperature {
    match (unit, to_unit) {
      (Kelvin, Kelvin) => Ok(value),
      (Kelvin, Celcius) => Ok(value-d128!(273.15)),
      (Kelvin, Fahrenheit) => Ok(value*d128!(1.8)-d128!(459.67)),
      (Celcius, Celcius) => Ok(value),
      (Celcius, Kelvin) => Ok(value+d128!(273.15)),
      (Celcius, Fahrenheit) => Ok(value*d128!(1.8)+d128!(32)),
      (Fahrenheit, Fahrenheit) => Ok(value),
      (Fahrenheit, Kelvin) => Ok((value+d128!(459.67))*d128!(5)/d128!(9)),
      (Fahrenheit, Celcius) => Ok((value-d128!(32))/d128!(1.8)),
      _ => Err(format!("Error converting temperature {:?} to {:?}", unit, to_unit)),
    }
  } else {
    let convertion_factor = get_convertion_factor(unit, to_unit);
    Ok(value * convertion_factor)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_convert() {
    pub fn convert_test(value: f64, unit: Unit, to_unit: Unit) -> f64 {
      use std::str::FromStr;

      let value_string = &value.to_string();
      let value_d128 = d128::from_str(value_string).unwrap();
      
      let result = convert(value_d128, unit, to_unit);
      let string_result = &result.unwrap().to_string();
      let float_result = f64::from_str(string_result).unwrap();

      return float_result;
    }
    assert_eq!(convert_test(1000.0, Nanosecond, Microsecond), 1.0);
    assert_eq!(convert_test(1000.0, Nanosecond, Microsecond), 1.0);
    assert_eq!(convert_test(1000.0, Microsecond, Millisecond), 1.0);
    assert_eq!(convert_test(1000.0, Millisecond, Second), 1.0);
    assert_eq!(convert_test(60.0, Second, Minute), 1.0);
    assert_eq!(convert_test(60.0, Minute, Hour), 1.0);
    assert_eq!(convert_test(24.0, Hour, Day), 1.0);
    assert_eq!(convert_test(7.0, Day, Week), 1.0);
    assert_eq!(convert_test(30.436875, Day, Month), 1.0);
    assert_eq!(convert_test(3.0, Month, Quarter), 1.0);
    assert_eq!(convert_test(4.0, Quarter, Year), 1.0);
    assert_eq!(convert_test(10.0, Year, Decade), 1.0);
    assert_eq!(convert_test(10.0, Decade, Century), 1.0);
    assert_eq!(convert_test(10.0, Century, Millenium), 1.0);

    assert_eq!(convert_test(10.0, Millimeter, Centimeter), 1.0);
    assert_eq!(convert_test(10.0, Centimeter, Decimeter), 1.0);
    assert_eq!(convert_test(10.0, Decimeter, Meter), 1.0);
    assert_eq!(convert_test(1000.0, Meter, Kilometer), 1.0);
    assert_eq!(convert_test(2.54, Centimeter, Inch), 1.0);
    assert_eq!(convert_test(12.0, Inch, Foot), 1.0);
    assert_eq!(convert_test(3.0, Foot, Yard), 1.0);
    assert_eq!(convert_test(1760.0, Yard, Mile), 1.0);
    assert_eq!(convert_test(1852.0, Meter, NauticalMile), 1.0);
    assert_eq!(convert_test(9460730472580800.0, Meter, LightYear), 1.0);
    
    assert_eq!(convert_test(10.0, Millimeter, Centimeter), 1.0);
    assert_eq!(convert_test(10.0, Centimeter, Decimeter), 1.0);
    assert_eq!(convert_test(10.0, Decimeter, Meter), 1.0);
    assert_eq!(convert_test(1000.0, Meter, Kilometer), 1.0);
    assert_eq!(convert_test(2.54, Centimeter, Inch), 1.0);
    assert_eq!(convert_test(12.0, Inch, Foot), 1.0);
    assert_eq!(convert_test(3.0, Foot, Yard), 1.0);
    assert_eq!(convert_test(1760.0, Yard, Mile), 1.0);
    assert_eq!(convert_test(1852.0, Meter, NauticalMile), 1.0);
    assert_eq!(convert_test(9460730472580800.0, Meter, LightYear), 1.0);
    
    assert_eq!(convert_test(100.0, SquareMillimeter, SquareCentimeter), 1.0);
    assert_eq!(convert_test(100.0, SquareCentimeter, SquareDecimeter), 1.0);
    assert_eq!(convert_test(100.0, SquareDecimeter, SquareMeter), 1.0);
    assert_eq!(convert_test(1000000.0, SquareMeter, SquareKilometer), 1.0);
    assert_eq!(convert_test(645.16, SquareMillimeter, SquareInch), 1.0);
    assert_eq!(convert_test(144.0, SquareInch, SquareFoot), 1.0);
    assert_eq!(convert_test(9.0, SquareFoot, SquareYard), 1.0);
    assert_eq!(convert_test(3097600.0, SquareYard, SquareMile), 1.0);
    assert_eq!(convert_test(100.0, SquareMeter, Are), 1.0);
    assert_eq!(convert_test(10.0, Are, Decare), 1.0);
    assert_eq!(convert_test(10.0, Decare, Hectare), 1.0);
    assert_eq!(convert_test(640.0, Acre, SquareMile), 1.0);

    assert_eq!(convert_test(1000.0, CubicMillimeter, CubicCentimeter), 1.0);
    assert_eq!(convert_test(1000.0, CubicCentimeter, CubicDecimeter), 1.0);
    assert_eq!(convert_test(1000.0, CubicDecimeter, CubicMeter), 1.0);
    assert_eq!(convert_test(1000000000.0, CubicMeter, CubicKilometer), 1.0);
    assert_eq!(convert_test(1728.0, CubicInch, CubicFoot), 1.0);
    assert_eq!(convert_test(27.0, CubicFoot, CubicYard), 1.0);
    assert_eq!(convert_test(5451776000.0, CubicYard, CubicMile), 1.0);
    assert_eq!(convert_test(1.0, Milliliter, CubicCentimeter), 1.0);
    assert_eq!(convert_test(10.0, Milliliter, Centiliter), 1.0);
    assert_eq!(convert_test(10.0, Centiliter, Deciliter), 1.0);
    assert_eq!(convert_test(10.0, Deciliter, Liter), 1.0);
    assert_eq!(convert_test(4.92892159375, Milliliter, Teaspoon), 1.0);
    assert_eq!(convert_test(3.0, Teaspoon, Tablespoon), 1.0);
    assert_eq!(convert_test(2.0, Tablespoon, FluidOunce), 1.0);
    assert_eq!(convert_test(8.0, FluidOunce, Cup), 1.0);
    assert_eq!(convert_test(2.0, Cup, Pint), 1.0);
    assert_eq!(convert_test(2.0, Pint, Quart), 1.0);
    assert_eq!(convert_test(4.0, Quart, Gallon), 1.0);
    assert_eq!(convert_test(42.0, Gallon, OilBarrel), 1.0);

    assert_eq!(convert_test(1000.0, Milligram, Gram), 1.0);
    assert_eq!(convert_test(100.0, Gram, Hectogram), 1.0);
    assert_eq!(convert_test(1000.0, Gram, Kilogram), 1.0);
    assert_eq!(convert_test(1000.0, Kilogram, MetricTon), 1.0);
    assert_eq!(convert_test(0.45359237, Kilogram, Pound), 1.0);
    assert_eq!(convert_test(16.0, Ounce, Pound), 1.0);
    assert_eq!(convert_test(2000.0, Pound, ShortTon), 1.0);
    assert_eq!(convert_test(2240.0, Pound, LongTon), 1.0);

    assert_eq!(convert_test(274.15, Kelvin, Celcius), 1.0);
    assert_eq!(convert_test(300.0, Kelvin, Fahrenheit), 80.33);
    assert_eq!(convert_test(-272.15, Celcius, Kelvin), 1.0);
    assert_eq!(convert_test(-15.0, Celcius, Fahrenheit), 5.0);
    assert_eq!(convert_test(80.33, Fahrenheit, Kelvin), 300.0);
    assert_eq!(convert_test(5.0, Fahrenheit, Celcius), -15.0);
  }
}
