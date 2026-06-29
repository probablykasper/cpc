//! Currency exchange rate fetching and management

use crate::units::Unit;
use fastnum::D128;
use fastnum::decimal::Context;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::RwLock;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Global cache for currency exchange rates relative to EUR
/// Maps currency Unit to its rate relative to EUR
static CURRENCY_CACHE: RwLock<Option<HashMap<Unit, D128>>> = RwLock::new(None);

/// The base currency for all exchange rates (EUR)
pub const BASE_CURRENCY: Unit = Unit::EUR;

#[derive(Deserialize, Debug)]
struct CurrencyRate {
	#[allow(dead_code)]
	date: String,
	base: String,
	quote: String,
	rate: serde_json::Number,
}

fn set_currency_cache(rates: Vec<CurrencyRate>) -> Result<(), String> {
	let mut cache = HashMap::with_capacity(rates.len() + 1);

	// Add EUR as base
	cache.insert(BASE_CURRENCY, D128::from(1));

	for entry in rates {
		if entry.base != "EUR" {
			return Err("Exchange rate base currency must be EUR".to_string());
		}
		if let Ok(quote_unit) = currency_code_to_unit(&entry.quote.to_ascii_lowercase()) {
			let rate_str = entry.rate.to_string();
			let rate = D128::parse_str(&rate_str, Context::default());
			cache.insert(quote_unit, rate);
		}
	}

	*CURRENCY_CACHE.write().unwrap() = Some(cache);
	Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn initialize_currency_cache() -> Result<(), String> {
	if CURRENCY_CACHE.read().unwrap().is_some() {
		return Ok(()); // Already initialized
	}

	let rates = fetch_currency_rates()?;
	set_currency_cache(rates)?;
	Ok(())
}

/// Get the exchange rate from one currency to another
/// Both currencies must be currency units
pub fn get_exchange_rate(from: Unit, to: Unit) -> Result<D128, String> {
	if from == to {
		return Ok(D128::from(1));
	}

	// Ensure cache is initialized
	#[cfg(not(target_arch = "wasm32"))]
	initialize_currency_cache()?;

	let cache = CURRENCY_CACHE.read().unwrap();
	let cache = cache.as_ref().unwrap();

	// Get rates relative to EUR
	let from_rate = cache
		.get(&from)
		.ok_or_else(|| format!("No exchange rate found for {:?}", from))?;
	let to_rate = cache
		.get(&to)
		.ok_or_else(|| format!("No exchange rate found for {:?}", to))?;

	// Convert from -> EUR -> to
	// rate = (to / EUR) / (from / EUR) = to / from
	Ok(*to_rate / *from_rate)
}

/// Fetch currency rates from the Frankfurter API (native version)
#[cfg(not(target_arch = "wasm32"))]
fn fetch_currency_rates() -> Result<Vec<CurrencyRate>, String> {
	use reqwest::blocking::get;

	let url = "https://api.frankfurter.dev/v2/rates?base=EUR";
	let response = get(url).map_err(|e| format!("Failed to fetch currency rates: {}", e))?;

	let rates: Vec<CurrencyRate> = response
		.json()
		.map_err(|e| format!("Failed to parse currency rates: {:?}", e))?;

	Ok(rates)
}

/// Convert currency code to Unit enum
pub fn currency_code_to_unit(code: &str) -> Result<Unit, String> {
	let unit = match code {
		"afn" => Unit::AFN,
		"all" => Unit::ALL,
		"amd" => Unit::AMD,
		"ang" => Unit::ANG,
		"aoa" => Unit::AOA,
		"ars" => Unit::ARS,
		"aud" => Unit::AUD,
		"awg" => Unit::AWG,
		"azn" => Unit::AZN,
		"bam" => Unit::BAM,
		"bbd" => Unit::BBD,
		"bdt" => Unit::BDT,
		"bhd" => Unit::BHD,
		"bif" => Unit::BIF,
		"bmd" => Unit::BMD,
		"bnd" => Unit::BND,
		"bob" => Unit::BOB,
		"brl" => Unit::BRL,
		"bsd" => Unit::BSD,
		"btn" => Unit::BTN,
		"bwp" => Unit::BWP,
		"byn" => Unit::BYN,
		"bzd" => Unit::BZD,
		"cad" => Unit::CAD,
		"cdf" => Unit::CDF,
		"chf" => Unit::CHF,
		"clp" => Unit::CLP,
		"cnh" => Unit::CNH,
		"cny" => Unit::CNY,
		"cop" => Unit::COP,
		"crc" => Unit::CRC,
		"cup" => Unit::CUP,
		"cve" => Unit::CVE,
		"czk" => Unit::CZK,
		"djf" => Unit::DJF,
		"dkk" => Unit::DKK,
		"dop" => Unit::DOP,
		"dzd" => Unit::DZD,
		"egp" => Unit::EGP,
		"ern" => Unit::ERN,
		"etb" => Unit::ETB,
		"eur" => Unit::EUR,
		"fjd" => Unit::FJD,
		"fkp" => Unit::FKP,
		"gbp" => Unit::GBP,
		"gel" => Unit::GEL,
		"ggp" => Unit::GGP,
		"ghs" => Unit::GHS,
		"gip" => Unit::GIP,
		"gmd" => Unit::GMD,
		"gnf" => Unit::GNF,
		"gtq" => Unit::GTQ,
		"gyd" => Unit::GYD,
		"hkd" => Unit::HKD,
		"hnl" => Unit::HNL,
		"htg" => Unit::HTG,
		"huf" => Unit::HUF,
		"idr" => Unit::IDR,
		"ils" => Unit::ILS,
		"imp" => Unit::IMP,
		"inr" => Unit::INR,
		"iqd" => Unit::IQD,
		"irr" => Unit::IRR,
		"isk" => Unit::ISK,
		"jep" => Unit::JEP,
		"jmd" => Unit::JMD,
		"jod" => Unit::JOD,
		"jpy" => Unit::JPY,
		"kes" => Unit::KES,
		"kgs" => Unit::KGS,
		"khr" => Unit::KHR,
		"kmf" => Unit::KMF,
		"kpw" => Unit::KPW,
		"krw" => Unit::KRW,
		"kwd" => Unit::KWD,
		"kyd" => Unit::KYD,
		"kzt" => Unit::KZT,
		"lak" => Unit::LAK,
		"lbp" => Unit::LBP,
		"lkr" => Unit::LKR,
		"lrd" => Unit::LRD,
		"lsl" => Unit::LSL,
		"lyd" => Unit::LYD,
		"mad" => Unit::MAD,
		"mdl" => Unit::MDL,
		"mga" => Unit::MGA,
		"mkd" => Unit::MKD,
		"mmk" => Unit::MMK,
		"mnt" => Unit::MNT,
		"mop" => Unit::MOP,
		"mro" => Unit::MRO,
		"mru" => Unit::MRU,
		"mur" => Unit::MUR,
		"mvr" => Unit::MVR,
		"mwk" => Unit::MWK,
		"mxn" => Unit::MXN,
		"myr" => Unit::MYR,
		"mzn" => Unit::MZN,
		"nad" => Unit::NAD,
		"ngn" => Unit::NGN,
		"nio" => Unit::NIO,
		"nok" => Unit::NOK,
		"npr" => Unit::NPR,
		"nzd" => Unit::NZD,
		"omr" => Unit::OMR,
		"pab" => Unit::PAB,
		"pen" => Unit::PEN,
		"pgk" => Unit::PGK,
		"php" => Unit::PHP,
		"pkr" => Unit::PKR,
		"pln" => Unit::PLN,
		"pyg" => Unit::PYG,
		"qar" => Unit::QAR,
		"ron" => Unit::RON,
		"rsd" => Unit::RSD,
		"rub" => Unit::RUB,
		"rwf" => Unit::RWF,
		"sar" => Unit::SAR,
		"sbd" => Unit::SBD,
		"scr" => Unit::SCR,
		"sdg" => Unit::SDG,
		"sek" => Unit::SEK,
		"sgd" => Unit::SGD,
		"shp" => Unit::SHP,
		"sle" => Unit::SLE,
		"sos" => Unit::SOS,
		"srd" => Unit::SRD,
		"ssp" => Unit::SSP,
		"stn" => Unit::STN,
		"svc" => Unit::SVC,
		"syp" => Unit::SYP,
		"szl" => Unit::SZL,
		"thb" => Unit::THB,
		"tjs" => Unit::TJS,
		"tmt" => Unit::TMT,
		"tnd" => Unit::TND,
		"top" => Unit::TOP,
		"try" => Unit::TRY,
		"ttd" => Unit::TTD,
		"twd" => Unit::TWD,
		"tzs" => Unit::TZS,
		"uah" => Unit::UAH,
		"ugx" => Unit::UGX,
		"usd" => Unit::USD,
		"uyu" => Unit::UYU,
		"uzs" => Unit::UZS,
		"ves" => Unit::VES,
		"vnd" => Unit::VND,
		"vuv" => Unit::VUV,
		"wst" => Unit::WST,
		"xaf" => Unit::XAF,
		"xag" => Unit::XAG,
		"xau" => Unit::XAU,
		"xcd" => Unit::XCD,
		"xcg" => Unit::XCG,
		"xdr" => Unit::XDR,
		"xof" => Unit::XOF,
		"xpd" => Unit::XPD,
		"xpf" => Unit::XPF,
		"xpt" => Unit::XPT,
		"yer" => Unit::YER,
		"zar" => Unit::ZAR,
		"zmw" => Unit::ZMW,
		"zwg" => Unit::ZWG,
		_ => return Err(format!("Unsupported currency code: {}", code)),
	};
	Ok(unit)
}

/// Initialize the currency cache with JSON from the API (for WASM use)
/// The web app should fetch from https://api.frankfurter.dev/v2/rates?base=EUR
/// and pass the JSON response (an array of {base, quote, rate} objects) to this function
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_currency_cache_with_json(rates_json: &str) -> Result<(), JsValue> {
	let rates: Vec<CurrencyRate> = serde_json::from_str(rates_json)
		.map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {}", e)))?;

	set_currency_cache(rates)
		.map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {}", e)))?;

	Ok(())
}
