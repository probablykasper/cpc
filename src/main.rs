use cpc::eval;
use cpc::units::Unit;
use std::process::exit;
use std::env;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn print_help() {
  println!(concat!(
    "Usage: cpc '<expression>' [options]",
    "\n",
    "\nOptions:",
    "\n    --verbose   Enable verbose logging",
    "\n    --version   Show cpc version",
    "\n    --help      Show this help page",
  ));
}

fn get_args() -> env::Args {
  let mut args = env::args().into_iter();
  args.next(); // skip binary name
  return args;
}

/// CLI interface
fn main() {
  // parse these first so they work if there are unexpected args
  for arg in get_args() {
    match arg.as_str() {
      "--version" => {
        println!("{}", VERSION);
        exit(0);
      },
      "--help" => {
        print_help();
        exit(0);
      },
      _ => {},
    }
  }
  let mut verbose = false;
  let mut expression_opt = None;
  for arg in get_args() {
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
  let expression = match expression_opt {
    Some(expression) => expression,
    None => {
      print_help();
      exit(0);
    },
  };
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
