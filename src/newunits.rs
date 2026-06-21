use fastnum::{D128, dec128 as d};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Cpc {
	dimensions: HashMap<String, Dimension>,
	units: HashMap<String, Unit>,
}
impl Cpc {
	pub fn set_dimension(&mut self, name: &str) {
		self.dimensions.insert(
			name.to_string(),
			Dimension {
				name: name.to_string(),
			},
		);
	}
	pub fn add_unit(&mut self, name: &str, plural: &str, weight: D128, definition: &[(&str, i32)]) {
		self.units.insert(
			name.to_string(),
			Unit {
				name: name.to_string(),
				plural: plural.to_string(),
				weight: D128::from(1),
				definition: definition
					.to_vec()
					.iter()
					.map(|(name, exponent)| (name.to_string(), *exponent))
					.collect(),
			},
		);
	}
	/// Examples:
	/// meter, meters = !
	/// metre, metres = meter
	pub fn parse_definition(definition: &str) -> Unit {
		let (name, definition) = definition.split_once("=").unwrap();
		let name = name.trim();
		let definition = definition.trim();
		if definition == "dimension" {
			return Unit {
				name: name.to_string(),
				plural: name.to_string(),
				weight: D128::from(1),
				definition: vec![("dimension".to_string(), 1)],
			};
		};
		todo!();
		// parse "1000meter" as (D128, vec<("meter", 1))
	}
}

pub type DimensionId = String;

#[derive(Clone, Debug)]
pub struct Dimension {
	pub name: DimensionId,
}

pub type UnitId = String;

#[derive(Clone, Debug)]
pub struct Unit {
	pub name: UnitId,
	pub plural: String,
	pub weight: D128,
	pub definition: Vec<(UnitId, i32)>,
}

#[derive(Clone, Debug)]
pub struct UnitWithExponent {
	unit: UnitId,
	exponent: i32,
}

#[derive(Clone, Debug)]
pub struct Number {
	value: D128,
	unit: Vec<UnitWithExponent>,
}

pub fn start() {
	let mut cpc = Cpc {
		dimensions: HashMap::new(),
		units: HashMap::new(),
	};
	cpc.add_unit("meter", "meters", d!(1), vec![("!".to_string(), 1)]);
	Unit {
		name: "meter".to_string(),
		plural: "meters".to_string(),
		weight: D128::from(1),
		definition: vec![("!".to_string(), 1)],
	};
}

pub fn operate(a: Number, b: Number, op: char) -> Number {
	let mut dimensions = a.unit.clone();

	for bd in b.unit {
		let entry = dimensions.iter_mut().find(|d| d.name == bd.name);

		match entry {
			Some(d) => {
				d.exponent += if op == '*' { bd.exponent } else { -bd.exponent };
			}
			None => {
				dimensions.push(Dimension {
					name: bd.name,
					exponent: if op == '*' { bd.exponent } else { -bd.exponent },
				});
			}
		}
	}

	// cleanup zeros
	dimensions.retain(|d| d.exponent != 0);

	let value = match op {
		'*' => a.value * b.value,
		'/' => a.value / b.value,
		_ => panic!("only * and / supported"),
	};

	Number {
		value,
		unit: dimensions,
	}
}
