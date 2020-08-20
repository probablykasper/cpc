# cpc
calculation + conversion

cpc parses and evaluates strings of math, with support for units and conversion. 128-bit decimal floating points are used for high accuracy.

cpc lets you mix units, so for example `1 km - 1m` results in `Number { value: 999, unit: Meter }`.

## Usage
```rs
use cpc::{eval, Unit::*}

match eval("3m + 1cm", true, Celcius) {
    Ok(answer) => {
        // answer: Number { value: 301, unit: Unit::cm }
        println!("Evaluated value: {} {:?}", answer.value, answer.unit)
    },
    Err(e) => {
        println!(e)
    }
}
```

## Examples
```
3 + 4 * 2

8 % 3

(4 + 1)km to light years

10m/2s * 5s

1 lightyear * 0.001mm in km2

1m/s + 1mi/h in kilometers per h

round(sqrt(2)^4)! liters

10% of abs(sin(pi)) horsepower to watts
```

## Supported unit types
- Normal numbers
- Time
- Length
- Area
- Volume
- Mass
- Digital storage (bytes etc)
- Energy
- Power
- Pressure
- Speed
- Temperature

## Accuracy
cpc Uses 128-bit Decimal Floating Point (d128) numbers instead of Binary Coded Decimals for better accuracy. The result cpc gives will still not always be 100% accurate. I would recommend rounding the result to 20 decimals or less.

## Performance
In my case, I can expect `eval()` to take 100-200ms, and this scales pretty alright. However, putting numbers with a lot of digits into functions result in pretty poor performance. `log(e)` is one of the worst, and takes 500ms for me.

## Errors
cpc returns `Result`s with basic strings as errors. Just to be safe, you may want to handle panics (You can do that using `std::panic::catch_unwind`).

# Dev Instructions

## Get started
Install [Rust](https://www.rust-lang.org). This project was built in Rust 1.45.

## Commands
Run cpc with a CLI argument as input:
```
cargo run -- '100ms to s'
```

Run with debugging, which shows some extra logs:
```
cargo run -- '100ms to s' --debug
```

Run tests:
```
cargo test
```

Build:
```
cargo build
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

## Potential Improvements
### General
- Support for math in `6'4"` syntax, like `3'+2'4"`. Currently needs to be written like `3'+3'+4"`
- The functions in units.rs have a lot of manual if statements. This could probably be replaced with a pretty advanced macro.
- Support for lexing words, like `one billion`
### Potential unit types
Nice list of units: https://support.google.com/websearch/answer/3284611
- Currency: How would you go about dynamically updating the weights?
- Fuel consumption
- Data transfer rate
- Color codes
- Force
- Roman numerals
- Angles
- Electric current, capacitance, charge, conductance, volts
- Flow rate
- Frequency
