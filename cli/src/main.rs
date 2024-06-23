#[allow(warnings)]
mod bindings;

use std::env;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct DemoCmd {
    #[arg(short='n', long="name")]
    name: String,
}

use std::fs;

const FILE_PATH: &str = "README.md";

fn main() {
    let args = DemoCmd::parse();
    println!("Your Name: {}", args.name);

    for (key, value) in env::vars() {
        println!("{key} : {value}");
    }

    println!("In file {FILE_PATH}");

    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
