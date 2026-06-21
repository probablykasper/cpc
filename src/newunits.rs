use fastnum::{D128, dec128 as d};
use std::collections::{
	HashMap,
	hash_map::Entry::{Occupied, Vacant},
};

#[derive(Clone, Debug)]
pub struct Cpc {
	units: HashMap<String, Unit>,
}
impl Cpc {
	pub fn add_unit(
		&mut self,
		name: &str,
		plural: &str,
		weight: D128,
		definition: Vec<(&str, i32)>,
	) {
		match self.units.entry(name.to_string()) {
			Occupied(_occupied) => {
				todo!("Unit {} already exists", name);
			}
			Vacant(vacant) => {
				vacant.insert(Unit {
					name: name.to_string(),
					plural: plural.to_string(),
					definition: Number {
						value: weight,
						unit: definition
							.iter()
							.map(|(name, exponent)| (name.to_string(), *exponent))
							.collect(),
					},
				});
			}
		};
	}
}

pub type UnitId = String;

#[derive(Clone, Debug)]
pub struct Unit {
	pub name: UnitId,
	pub plural: String,
	pub definition: Number,
}

#[derive(Clone, Debug)]
pub struct Number {
	pub value: D128,
	pub unit: Vec<(UnitId, i32)>,
}

pub fn start() {
	let mut cpc = Cpc {
		units: HashMap::new(),
	};
	cpc.add_unit("meter", "meters", d!(1), vec![("!", 1)]);
	cpc.add_unit("kilometer", "kilometers", d!(1000), vec![("meter", 1)]);
	cpc.add_unit("liter", "liters", d!(1), vec![("!", 1)]);
}
