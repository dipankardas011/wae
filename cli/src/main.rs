#[allow(warnings)]
mod bindings;

use std::env;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CommandToPerform {
    #[arg(short = 'n', long = "name")]
    name: String,

    #[arg(short='o', long="op", value_parser=[OP_GENERATE_RANDOM_PASSWORD, "demo"], default_value_t=String::from("demo"))]
    operation: String,
}


const FILE_PATH: &str = "README.md";
const OP_GENERATE_RANDOM_PASSWORD: &str = "gen_rand_pass";

fn main() {
    let args = CommandToPerform::parse();

    if args.operation == OP_GENERATE_RANDOM_PASSWORD {
        println!(" > Enter Length of Password");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let length_pass: u32 = input.trim().parse().expect("Invalid Input");

        println!("Created password of len: {length_pass}");
    } else {
        println!("Your Name: {}, Op: {}", args.name, args.operation);

        for (key, value) in env::vars() {
            println!("{key} : {value}");
        }

        println!("In file {FILE_PATH}");

        let contents =
            fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

        println!("Content upto 50 chars: {}\n", &contents[..50]);
    }
}
