use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, PartialEq)]
#[command(version, about, long_about = None, disable_help_subcommand = true)]
struct Cli {
    /// Verbosity
    #[arg(short, action = clap::ArgAction::Count)]
    verbosity: u8,

    #[arg(short, long)]
    flag: bool,

    #[arg(long, value_name = "ARGS", help = "a list of args", value_parser = validate_args)]
    args: Option<::std::vec::Vec<String>>, // fully qualified type name skips parser auto-translation (Option<Vec<T>> tries to configure this to take in multiple individual strings and some other funky business)

    #[command(subcommand)]
    operation: Option<Operations>,
}

#[derive(Subcommand, Serialize, Deserialize, PartialEq)]
enum Operations {
    /// Counts up from 0 to the given number
    Increment(OpArgs),
    /// Counts down from the given number to 0
    Decrement(OpArgs),
    /// Divides the given number by 2 until it reaches 0
    Split(OpArgs),
}

#[derive(Args, Serialize, Deserialize, PartialEq)]
struct OpArgs {
    /// Number to use
    number: i32,
}

fn validate_args(s: &str) -> Result<Vec<String>, String> {
    let mut parsed: String = String::from(s);
    if !parsed.starts_with("args=") {
        parsed.insert_str(0, "args=");
    }

    #[derive(Serialize, Deserialize)]
    struct Args {
        args: Vec<String>,
    }

    let vec: Args = match toml::from_str(&parsed) {
        Ok(v) => v,
        Err(e) => {
            return Err(format!("{}\n  Expected format: ['arg0', 'arg1', ... ]", e))
        },
    };

    return Ok(vec.args);    
}

fn main() {
    let cli = Cli::parse();

    match cli.verbosity {
        0 => println!("Basic logging"),
        1 => println!("Detailed logging"),
        2 => println!("All logging"),
        _ => println!("You can't get crazier than this"),
    }

    match &cli.args {
        Some(args) => {
            println!("Arg vec: {:?}", args);
        }
        None => {
            println!("No args passed");
        }
    }

    if cli.flag {
        println!("TOML flag enabled\n// BEGIN TOML");
        let toml_cmd = toml::to_string(&cli).expect("Unable to serialize the CLI using TOML");
        println!("{}\n// END TOML", toml_cmd);
        let reparsed: Cli = toml::from_str(&toml_cmd).expect("Unable to deserialize the CLI using TOML");
        if &cli != &reparsed {
            panic!("Reparsed command is not equal to original command");
        }
    }

    match &cli.operation {
        Some(Operations::Increment(args)) => {
            for i in 0..args.number {
                if cli.verbosity > 0 {
                    print!("{}..", i);
                }
            }
            println!("{} done!", args.number);
        },
        Some(Operations::Decrement(args)) => {
            for i in (1..=args.number).rev() {
                if cli.verbosity > 0 {
                    print!("{}..", i);
                }
            }
            println!("{} done!", 0);
        },
        Some(Operations::Split(args)) => {
            let mut n: i32 = args.number;
            while n > 0 {
                if cli.verbosity > 0 {
                    print!("{}..", n);
                }
                n /= 2;
            }
            println!("{} done!", 0);
        },
        None => {
            println!("NOP");
        }
    }
}
