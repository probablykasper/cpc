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
		if let Ok(quote_unit) = currency_code_to_unit(&entry.quote) {
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
fn currency_code_to_unit(code: &str) -> Result<Unit, String> {
	match code {
		"USD" => Ok(Unit::USD),
		"EUR" => Ok(Unit::EUR),
		"GBP" => Ok(Unit::GBP),
		"JPY" => Ok(Unit::JPY),
		"CAD" => Ok(Unit::CAD),
		"AUD" => Ok(Unit::AUD),
		"CHF" => Ok(Unit::CHF),
		"CNY" => Ok(Unit::CNY),
		"SEK" => Ok(Unit::SEK),
		"NZD" => Ok(Unit::NZD),
		_ => Err(format!("Unsupported currency code: {}", code)),
	}
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
