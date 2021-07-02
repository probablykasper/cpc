use cpc::eval;
use cpc::units::Unit;
use std::process::exit;

/// CLI interface
fn main() {
  use std::env;
  let mut args = env::args().into_iter();
  args.next();
  let mut verbose = false;
  let mut expression_opt = None;
  for arg in args {
    match arg.as_str() {
      "-v" | "--verbose" => verbose = true,
      _ => {
        if expression_opt == None {
          expression_opt = Some(arg);
        } else {
          eprintln!("Unexpected argument: {}", arg);
          exit(1);
        }
      }
    }
  }
  match expression_opt {
    Some(expression) => {
      match eval(&expression, true, Unit::Celsius, verbose) {
        Ok(answer) => {
          if !verbose {
            println!("{}", answer);
          }
        },
        Err(e) => {
          eprintln!("{}", e);
          exit(1);
        },
      }
    }
    None => {
      eprintln!("No argument supplied");
      exit(1);
    }
  }
}
