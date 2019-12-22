use decimal::d128;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum UnitType {
  NoUnit,
  Time,
  Length,
  Area,
  Volume,
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
  NauticalMile,

  SquareMeter,
  // etc

  CubicMeter,
  // etc
}
use Unit::*;

fn get_info(unit: &Unit) -> (UnitType, d128) {
  match unit {
    Unit::NoUnit => (UnitType::NoUnit, d128!(1)),

    Nanosecond => (Time, d128!(1)),
    Microsecond => (Time, d128!(1000)),
    Millisecond => (Time, d128!(1000000)),
    Second => (Time, d128!(1000000000)),
    Minute => (Time, d128!(60000000000)),
    Hour => (Time, d128!(3600000000000)),
    Day => (Time, d128!(86400000000000)),
    Week => (Time, d128!(604800000000000)),
    Month => (Time, d128!(2629746000000000)),
    Quarter => (Time, d128!(7889238000000000)),
    Year => (Time, d128!(31556952000000000)),
    Decade => (Time, d128!(315569520000000000)),
    Century => (Time, d128!(3155695200000000000)),
    Millenium => (Time, d128!(31556952000000000000)),

    Millimeter => (Length, d128!(10)),
    Centimeter => (Length, d128!(100)),
    Decimeter => (Length, d128!(1000)),
    Meter => (Length, d128!(10000)),
    Kilometer => (Length, d128!(10000000)),
    Inch => (Length, d128!(254)),
    Foot => (Length, d128!(3048)),
    Yard => (Length, d128!(9144)),
    Mile => (Length, d128!(16090000)),
    NauticalMile => (Length, d128!(18520000)),

    SquareMeter => (UnitType::Area, d128!(1)),

    CubicMeter => (UnitType::Volume, d128!(1)),
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
