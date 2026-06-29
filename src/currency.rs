//! Currency exchange rate fetching and management

use crate::units::Unit;
use fastnum::D128;
use std::collections::HashMap;
use std::sync::RwLock;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Global cache for currency exchange rates relative to EUR
/// Maps currency Unit to its rate relative to EUR
static CURRENCY_CACHE: RwLock<Option<HashMap<Unit, D128>>> = RwLock::new(None);

/// The base currency for all exchange rates (EUR)
pub const BASE_CURRENCY: Unit = Unit::EUR;

/// Initialize the currency cache by fetching rates from the API
pub fn initialize_currency_cache() -> Result<(), String> {
	if CURRENCY_CACHE.read().unwrap().is_some() {
		return Ok(()); // Already initialized
	}

	let rates = fetch_currency_rates()?;
	let mut cache = HashMap::new();

	// Store rates relative to EUR (base currency)
	// EUR to EUR is 1
	cache.insert(BASE_CURRENCY, D128::from(1));

	for (quote_currency, rate) in rates {
		cache.insert(quote_currency, rate);
	}

	*CURRENCY_CACHE.write().unwrap() = Some(cache);
	Ok(())
}

/// Get the exchange rate from one currency to another
/// Both currencies must be currency units
pub fn get_exchange_rate(from: Unit, to: Unit) -> Result<D128, String> {
	if from == to {
		return Ok(D128::from(1));
	}

	// Ensure cache is initialized
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
fn fetch_currency_rates() -> Result<Vec<(Unit, D128)>, String> {
	use reqwest::blocking::get;
	use serde::Deserialize;

	#[derive(Deserialize)]
	struct RateEntry {
		#[allow(dead_code)]
		date: String,
		base: String,
		quote: String,
		rate: f64,
	}

	let url = "https://api.frankfurter.dev/v2/rates?base=EUR";
	let response = get(url).map_err(|e| format!("Failed to fetch currency rates: {}", e))?;

	let rates: Vec<RateEntry> = response
		.json()
		.map_err(|e| format!("Failed to parse currency rates: {}", e))?;

	let mut result = Vec::new();
	for entry in rates {
		if entry.base == "EUR" {
			// Only process currencies that we support
			if let Ok(quote_unit) = currency_code_to_unit(&entry.quote) {
				let rate = D128::from_f64(entry.rate);
				result.push((quote_unit, rate));
			}
		}
	}

	Ok(result)
}

/// Fetch currency rates for WASM target
/// Returns empty - the web app should fetch real rates
/// and call init_currency_cache_with_json
#[cfg(target_arch = "wasm32")]
fn fetch_currency_rates() -> Result<Vec<(Unit, D128)>, String> {
	// Return empty for WASM - the web app must call init_currency_cache_with_json
	// with rates fetched via JavaScript
	Ok(Vec::new())
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
	use serde::Deserialize;

	#[derive(Deserialize)]
	struct RateEntry {
		#[allow(dead_code)]
		date: Option<String>,
		base: String,
		quote: String,
		rate: f64,
	}

	// Parse the JSON array of rate entries
	let rates: Vec<RateEntry> = serde_json::from_str(rates_json)
		.map_err(|e| JsValue::from_str(&format!("Failed to parse JSON: {}", e)))?;

	let mut cache = HashMap::new();

	// Add EUR as base
	cache.insert(BASE_CURRENCY, D128::from(1));

	// Parse individual rates - only accept entries where base is EUR
	for entry in rates {
		if entry.base == "EUR" {
			if let Ok(unit) = currency_code_to_unit(&entry.quote) {
				cache.insert(unit, D128::from_f64(entry.rate));
			}
		}
	}

	*CURRENCY_CACHE.write().unwrap() = Some(cache);
	Ok(())
}
