use cpc::eval;
use cpc::units::Unit;

/// cpc CLI interface
fn main() {
  use std::env;
  let mut args: Vec<String> = env::args().collect();
  let mut verbose = false;
  if let Some(pos) = args.iter().position(|x| x == "-v" || x == "--verbose") {
    verbose = true;
    args.remove(pos);
  }
  if args.len() >= 2 {
    match eval(&args[1], true, Unit::Celsius, verbose) {
      Ok(answer) => {
        if !verbose {
          println!("{}", answer);
        }
      },
      Err(e) => {
        eprintln!("{}", e)
      },
    }
  } else {
    eprintln!("No argument supplied");
  }
}
