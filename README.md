# cpc

calculation + conversion

cpc parses and evaluates strings of math, with support for units and conversion. 128-bit decimal floating points are used for high accuracy.

It also lets you mix units, so for example `1 km - 1m` results in `999 Meter`.

[![Crates.io](https://img.shields.io/crates/v/cpc.svg)](https://crates.io/crates/cpc)
[![Documentation](https://docs.rs/cpc/badge.svg)](https://docs.rs/cpc)

[List of all supported units](https://docs.rs/cpc/latest/cpc/units/enum.Unit.html)

> [!TIP]
> [fend](https://github.com/printfn/fend) is a great alternative to cpc

## CLI Installation
Install using `cargo`:
```
cargo install cpc
```

To install it manually, grab the appropriate binary from the [GitHub Releases page](https://github.com/probablykasper/cpc/releases) and place it wherever you normally place binaries on your OS.

## CLI Usage
```
cpc '2h/3 to min'
```

## API Installation
Add `cpc` as a dependency in `Cargo.toml`.

## API Usage

```rust
use cpc::eval;
use cpc::units::Unit;

match eval("3m + 1cm", true, Unit::Celsius, false) {
    Ok(answer) => {
        // answer: Number { value: 301, unit: Unit::Centimeter }
        println!("Evaluated value: {} {:?}", answer.value, answer.unit)
    },
    Err(e) => {
        println!("{e}")
    }
}
```

## Examples
```
3 + 4 * 2

8 % 3

(4 + 1)km to light years

10m/2s * 5 trillion s

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
- Electric current
- Resistance
- Voltage
- Pressure
- Frequency
- Speed
- Temperature

## Accuracy
cpc uses 128-bit Decimal Floating Point (d128) numbers instead of Binary Coded Decimals for better accuracy. The result cpc gives will still not always be 100% accurate. I would recommend rounding the result to 20 decimals or less.

## Performance
It's pretty fast and scales well. In my case, it usually runs in under 0.1ms. The biggest performance hit is functions like `log()`. `log(12345)` evaluates in 0.12ms, and `log(e)` in 0.25ms.

To see how fast it is, you can pass the `--verbose` flag in CLI, or the `verbose` argument to `eval()`.

## Dev Instructions

### Get started
Install [Rust](https://www.rust-lang.org).

Run cpc with a CLI argument as input:
```
cargo run -- '100ms to s'
```

Run in verbose mode, which shows some extra logs:
```
cargo run -- '100ms to s' --verbose
```

Run tests:
```
cargo test
```

Build:
```
cargo build
```

### Adding a unit

Nice resources for adding units:
- https://github.com/ryantenney/gnu-units/blob/master/units.dat
- https://support.google.com/websearch/answer/3284611 (unit list)
- https://translatorscafe.com/unit-converter (unit conversion)
- https://calculateme.com (unit conversion)
- https://wikipedia.org

#### 1. Add the unit
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

The number associated with a unit is it's "weight". For example, if a second's weight is `1`, then a minute's weight is `60`.

#### 2. Add a test for the unit
Make sure to also add a test for each unit. The tests look like this:
```rs
assert_eq!(convert_test(1000.0, Meter, Kilometer), 1.0);
```
Basically, 1000 Meter == 1 Kilometer.

#### 3. Add the unit to the lexer
Text is turned into tokens (some of which are units) in `lexer.rs`. Here's one example:
```rs
// ...
match string {
  "h" | "hr" | "hrs" | "hour" | "hours" => tokens.push(Token::Unit(Hour)),
  // etc
}
// ...
```

### Potential Improvements
- Support for conversion between Power, Current, Resistance and Voltage. Multiplication and division is currently supported, but not conversions using sqrt or pow.
- Move to pure-rust decimal implementation
  - `rust_decimal`: Only supports numbers up to ~1E+29
  - `bigdecimal`: Lacking math functions
- E notation, like 2E+10
- Unit types
  - Currency: How to go about dynamically updating the weights?
    - https://api.exchangerate-api.com/v4/latest/USD
    - https://www.coingecko.com/en/api
    - https://developers.coinbase.com/api/v2
  - Timezones
  - Binary/octal/decimal/hexadecimal/base32/base64
  - Fuel consumption
  - Data transfer rate
  - Color codes
  - Force
  - Roman numerals
  - Angles
  - Flow rate

### Releasing a new version

1. Update `CHANGELOG.md`
2. Bump the version number in `Cargo.toml`
3. Run `cargo test`
4. Create a git tag in format `v#.#.#`
5. Add release notes to the generated GitHub release and publish it
6. Run `cargo publish`
