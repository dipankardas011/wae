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

    #[arg(short='o', long="op", value_parser=[OP_CRYPTO, OP_GITHUBAPI, OP_DEMO, OP_OPENAI], default_value_t=OP_DEMO.to_string())]
    operation: String,
}


const FILE_PATH: &str = "README.md";
const OP_CRYPTO: &str = "crypto";
const OP_GITHUBAPI: &str = "githubapi";
const OP_OPENAI: &str = "openai";
const OP_DEMO: &str = "demo";


use bindings::dipankardas011::{
    crypto::password::generate_random,
    githubapi::releases,
    openai::llm,
};


use anyhow::Result;

async fn hh() {
    println!("@@ Welcome to CLI wonderland @@");
}

#[tokio::main(flavor = "current_thread")] // it is used as wasm32-wasi doesn't support
// multi-threading
async fn main() -> Result<()> {

    let args = CommandToPerform::parse();

    hh().await;

    match args.operation.as_str() {
        OP_OPENAI => {
            println!(" > Enter your Prompt");
            let mut input_org = String::new();
            std::io::stdin().read_line(&mut input_org).expect("Failed to read line");
            let prompt: String = input_org.trim().parse().expect("invalid prompt");
            let generated_output = llm::text_to_text(&prompt);
            println!("Output\n{generated_output}")
            
        }
        OP_CRYPTO => {
            println!(" > Enter Length of Password");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let length_pass: u32 = input.trim().parse().expect("Invalid Input");

            let gen_pass = generate_random(length_pass);

            println!("Created password '{gen_pass}'");
        }
        OP_GITHUBAPI => {

            println!("> Enter [1] latest release [2] contributors [3] stars");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let choice: i32 = input.trim().parse().expect("Invalid Input");
            if choice < 1 || choice > 3 {
                eprintln!("Invalid choice");
            }

            println!(" > Enter Organization Name");
            let mut input_org = String::new();
            std::io::stdin().read_line(&mut input_org).expect("Failed to read line");
            let org: String = input_org.trim().parse().expect("invalid organization");

            println!(" > Enter Repo Name");
            let mut input_proj = String::new();
            std::io::stdin().read_line(&mut input_proj).expect("Failed to read line");
            let proj: String = input_proj.trim().parse().expect("invalid organization");

            if choice == 1 {
                let ver = releases::get_latest_release(&org, &proj);
                println!("Latest version: {ver}");
            }
            else if choice ==2 {
                let contrib = releases::get_contributors(&org, &proj);
                println!("Contributors\n{contrib}");
            }
            else {
                let stars = releases::get_stars(&org, &proj);
                println!("Total Stars: {stars}");
            }

        }
        OP_DEMO => {
            println!("Your Name: {}, Op: {}", args.name, args.operation);

            for (key, value) in env::vars() {
                println!("{key} : {value}");
            }

            println!("In file {FILE_PATH}");

            let contents =
            fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

            println!("Content upto 50 chars: {}\n", &contents[..50]);

            let now = SystemTime::now();

            sleep(Duration::new(2, 0));
            match now.elapsed() {
                Ok(elapsed) => {
                    println!("Sleeped for {}s", elapsed.as_secs());
                }
                Err(e) => {
                    println!("Error: {e:?}");
                }
            }
        }
        _ => eprintln!("Invalid Operation choosen")
    }

    Ok(())
}

