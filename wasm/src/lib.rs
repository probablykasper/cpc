use decimal::d128;
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct NumberWasm {
	pub value: String,
	pub quantity: String,
	pub unit: String,
}

#[wasm_bindgen]
pub fn eval_wasm(input: &str) -> Result<JsValue, String> {
	let number = cpc::eval(input, true, false)?;
	// 0.2/0.01 results in 2E+1, but if we add zero it becomes 20
	let quantity = number.value + d128!(0);
	let word = match quantity == d128!(1) {
		true => number.unit.singular(),
		false => number.unit.plural(),
	};
	let output_text = match word {
		"" => format!("{quantity}"),
		_ => format!("{quantity} {word}"),
	};
	let number_wasm = NumberWasm {
		value: output_text,
		quantity: quantity.to_string(),
		unit: word.into(),
	};
	let js_value = serde_wasm_bindgen::to_value(&number_wasm).unwrap();
	Ok(js_value)
}
