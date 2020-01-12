calculation + conversion

Uses Decimal Floating Point numbers instead of Binary Coded Decimals for better accuracy.

# Dev Instructions

## Get started
Install [Rust](https://www.rust-lang.org). This project was built in Rust 1.40.

## Commands
Run cpc with a CLI argument as input:
```
cargo run -- '100ms to s'
```

Run tests:
```
cargo test
```

## Adding a unit
### 1. Add the unit
In `src/units.rs`, units are specified like this:
```rs
pub enum UnitType {
  Time,
  // etc
}

// ...

create_units!(
  Nanosecond:         (Time, d128!(1)),
  Microsecond:        (Time, d128!(1000)),
  // etc
)
```

The number associated with a unit is it's "weight". For example, if a second's weight is `1`, then a minute's weight is `1000`.

I have found [translatorscafe.com](https://www.translatorscafe.com/unit-converter) and [calculateme.com](https://www.calculateme.com/) to be good websites for unit convertion. Wikipedia is worth looking at as well.

### 2. Add a test for the unit
Make sure to also add a test for each unit. The tests look like this:
```rs
assert_eq!(convert_test(1000.0, Meter, Kilometer), 1.0);
```
Basically, 1000 Meter == 1 Kilometer.

### 3. Add the unit to the lexer
Text is turned into tokens (some of which are units) in `lexer.rs`. Here's one example:
```rs
// ...
match string {
  "h" | "hr" | "hrs" | "hour" | "hours" => tokens.push(Token::Unit(Hour)),
  // etc
}
// ...
```

