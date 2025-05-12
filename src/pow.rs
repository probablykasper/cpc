// This code is translated from https://github.com/Kuuuube/bigdecimal-rs-pow/blob/main/src/main.rs

use malachite::base::num::arithmetic::traits::{Abs, RoundToMultiple};
use malachite::base::num::basic::traits::{One, Zero};
use malachite::base::num::conversion::traits::IsInteger;
use malachite::base::rounding_modes::RoundingMode;
use malachite::rational::Rational;

// calculates integer and decimal bigdecimal powers powers
// be warned that passing none to prec may cause your calculations to take forever
// x is the base, e is the exponent, prec tries to remove effectively infinite calculations, round rounds the inputs of powi to be faster
pub fn powf(
	x: &Rational,
	e: &Rational,
	prec_multiple: Option<&Rational>,
	round_to_multiple: Option<&Rational>,
) -> Rational {
	let is_negative = *e < Rational::ZERO;
	let e_abs = e.abs();
	if e.is_integer() {
		return powi(&x, &e);
	} else {
		let numerator = u32::try_from(e_abs.numerator_ref()).unwrap();
		let denominator = u32::try_from(e_abs.denominator_ref()).unwrap();

		// attempt to chop down the fraction to improve performance if a greatest common denominator can be found
		let gcd = euclid_gcd(numerator, denominator);
		let simplified_numerator = numerator / gcd;
		let simplified_denominator = denominator / gcd;

		let whole_result = if let Some(round_to_multiple) = round_to_multiple {
			powi(
				&x.round_to_multiple(round_to_multiple, RoundingMode::Nearest).0,
				&Rational::from(simplified_numerator),
			)
			.round_to_multiple(round_to_multiple, RoundingMode::Nearest).0
		} else {
			powi(&x, &Rational::from(simplified_numerator))
		};
		let result = root(
			&Rational::from(simplified_denominator),
			&whole_result,
			prec_multiple,
		);

		if is_negative {
			Rational::ONE / result
		} else {
			result
		}
	}
}

// simple greatest common denominator finder
// m is the numerator, n is the denominator
fn euclid_gcd(mut m: u32, mut n: u32) -> u32 {
	while m != 0 {
		let old_m = m;
		m = n % m;
		n = old_m;
	}
	return n;
}

// calculates integer equivalent bigdecimal powers only
// x is the base, e is the exponent
pub fn powi(x: &Rational, e: &Rational) -> Rational {
	if *e < Rational::ZERO {
		return Rational::ONE / powi(x, &(-e));
	}

	let mut r = Rational::ONE;
	let mut i = Rational::ZERO;
	while i < *e {
		r *= x;
		i += Rational::ONE;
	}
	return r;
}

// calculates integer equivalent bigdecimal roots only
// be warned that passing none to prec may cause your calculations to take forever
// n is the root, x is the base, prec tries to remove effectively infinite calculations
pub fn root(n: &Rational, x: &Rational, prec_multiple: Option<&Rational>) -> Rational {
	let mut d: Rational;
	let mut r = Rational::from(1);
	if x == &0 {
		return Rational::ZERO;
	}
	if n < &Rational::from(1) {
		// this if was `if (n < 1 || (x < 0 && !(n&1)))` in C, depending on the use case you may need to find a way to add the rest
		return Rational::ZERO; // substitute for NaN, you may want to convert this function to return an option if you need to handle this case
	}
	loop {
		r = if let Some(prec_multiple) = prec_multiple {
			r.round_to_multiple(prec_multiple, RoundingMode::Nearest).0
		} else {
			r
		}; // looping with round is too expensive, with_prec is used instead
		d = (x / powi(&r, &(n - Rational::ONE)) - &r) / n;
		r += &d;
		if !(&d >= &(Rational::try_from(f64::EPSILON).unwrap() * Rational::from(10))
			|| &d <= &(Rational::try_from(-f64::EPSILON).unwrap() * Rational::from(10)))
		{
			break;
		}
	}
	return r;
}
