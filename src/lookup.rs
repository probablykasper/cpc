use crate::NamedNumber;
use crate::NamedNumber::*;
use malachite::rational::Rational;
use std::str::FromStr;

/// Returns the corresponding [`d128`] of a [`NamedNumber`]
pub fn lookup_named_number(named_number: &NamedNumber) -> Rational {
	match named_number {
		Hundred           => Rational::from(100u128),
		Thousand          => Rational::from(1000u128),
		Million           => Rational::from(1000000u128),
		Billion           => Rational::from(1000000000u128),
		Trillion          => Rational::from(1000000000000u128),
		Quadrillion       => Rational::from(1000000000000000u128),
		Quintillion       => Rational::from(1000000000000000000u128),
		Sextillion        => Rational::from(1000000000000000000000u128),
		Septillion        => Rational::from(1000000000000000000000000u128),
		Octillion         => Rational::from(1000000000000000000000000000u128),
		Nonillion         => Rational::from(1000000000000000000000000000000u128),
		Decillion         => Rational::from(1000000000000000000000000000000000u128),
		Undecillion       => Rational::from(1000000000000000000000000000000000000u128),
		Duodecillion      => Rational::from_str("1E+39").unwrap(),
		Tredecillion      => Rational::from_str("1E+42").unwrap(),
		Quattuordecillion => Rational::from_str("1E+45").unwrap(),
		Quindecillion     => Rational::from_str("1E+48").unwrap(),
		Sexdecillion      => Rational::from_str("1E+51").unwrap(),
		Septendecillion   => Rational::from_str("1E+54").unwrap(),
		Octodecillion     => Rational::from_str("1E+57").unwrap(),
		Novemdecillion    => Rational::from_str("1E+60").unwrap(),
		Vigintillion      => Rational::from_str("1E+63").unwrap(),
		Googol            => Rational::from_str("1E+100").unwrap(),
		Centillion        => Rational::from_str("1E+303").unwrap(),
	}
}
