use cpc::eval;
use cpc::units::Unit;

/// cpc CLI interface
fn main() {
  use std::env;
  let args: Vec<String> = env::args().collect();
  let mut verbose = false;
  if args.iter().any(|i| i == "-v" || i == "--verbose") {
    verbose = true;
  }
  if args.len() >= 2 {
    match eval(&args[1], true, Unit::Celsius, verbose) {
      Ok(answer) => {
        if !verbose {
          println!("{} {:?}", answer.value, answer.unit)
        }
      },
      Err(e) => {
        println!("{}", e)
      },
    }
  } else {
    println!("No argument supplied");
  }
}
