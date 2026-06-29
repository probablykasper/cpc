//! Currency exchange rate fetching and management

use crate::units::Unit;
use fastnum::D128;
use std::collections::HashMap;
use std::sync::RwLock;

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

/// Fetch currency rates from the Frankfurter API
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

/// Fetch currency rates using Web API for WASM
#[cfg(target_arch = "wasm32")]
fn fetch_currency_rates() -> Result<Vec<(Unit, D128)>, String> {
	// This is a simplified version - in practice you'd need async handling
	// For now, return some placeholder rates
	Ok(vec![
		(Unit::USD, D128::from(1.1402)),
		(Unit::GBP, D128::from(0.86273)),
		(Unit::JPY, D128::from(184.48)),
	])
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
