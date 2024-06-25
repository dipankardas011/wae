// cargo_component_bindings::generate!();

#[allow(warnings)]
mod bindings;

use std::env;
use clap::Parser;
use std::fs;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

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


use bindings::dipankardas011::crypto::password::generate_random;

fn main() {
    let args = CommandToPerform::parse();
    let now = SystemTime::now();

    // we sleep for 2 seconds
    sleep(Duration::new(2, 0));
    match now.elapsed() {
        Ok(elapsed) => {
            println!("{}", elapsed.as_secs());
        }
        Err(e) => {
            println!("Error: {e:?}");
        }
    }
    if args.operation == OP_GENERATE_RANDOM_PASSWORD {
        println!(" > Enter Length of Password");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let length_pass: u32 = input.trim().parse().expect("Invalid Input");

        let gen_pass: String = generate_password(length_pass as usize);
        let gen_pass_1 = generate_random(length_pass);

        println!("Created password {gen_pass} {gen_pass_1} of len: {length_pass}");
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


use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn generate_password(length: usize) -> String {
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    password
}
