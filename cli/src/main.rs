#[allow(warnings)]
mod bindings;

use std::env;
use clap::Parser;
use std::fs;
use std::time::{Duration, SystemTime};
use std::thread::sleep;
use bindings::dipankardas011::{
    crypto::password::generate_random,
    githubapi::releases,
    openai::llm,
};
use anyhow::Result;
use ansi_term;
use ansi_term::Colour::{Cyan,Black,Red, Green, Blue, Yellow};

const FILE_PATH: &str = "README.md";
const OP_CRYPTO: &str = "crypto";
const OP_GITHUBAPI: &str = "githubapi";
const OP_OPENAI: &str = "openai";
const OP_DEMO: &str = "demo";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CommandToPerform {
    #[arg(short = 'n', long = "name")]
    name: String,

    #[arg(short='o', long="op", value_parser=[OP_CRYPTO, OP_GITHUBAPI, OP_DEMO, OP_OPENAI], default_value_t=OP_DEMO.to_string())]
    operation: String,
}

async fn hh(name: &str) {
    println!("{} {} {}",
        Yellow.bold().paint("@@"),
        format!("{} {} {}", Green.paint("Welcome"), Blue.bold().paint(name), Green.paint("🚀 to WASI CLI based on Component Model")),
        Yellow.bold().paint("@@"),
    );
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {

    let args = CommandToPerform::parse();

    hh(&args.name).await;

    match args.operation.as_str() {
        OP_OPENAI => {
            println!("{}", Cyan.paint("> Enter [1] text-to-text [2] text-to-image"));
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let choice: i32 = input.trim().parse().expect("Invalid Input");
            if choice < 1 || choice > 2 {
                eprintln!("{}", Red.bold().paint("Invalid choice"));
            }
            if choice == 1 {
                llm::text_to_text();
            } else if choice == 2 {
                llm::text_to_image();
            }
        }
        OP_CRYPTO => {
            println!("{}", Cyan.paint("> Enter Length of Password"));
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let length_pass: u32 = input.trim().parse().expect("Invalid Input");

            let gen_pass = generate_random(length_pass);

            println!("{} pass={}", Green.bold().paint("Created password"),gen_pass);
        }
        OP_GITHUBAPI => {

            println!("{}", Cyan.paint("> Enter [1] latest release [2] contributors [3] stars"));
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let choice: i32 = input.trim().parse().expect("Invalid Input");
            if choice < 1 || choice > 3 {
                eprintln!("{}", Red.bold().paint("Invalid choice"));
            }

            println!("{}", Cyan.paint(" > Enter Organization Name"));
            let mut input_org = String::new();
            std::io::stdin().read_line(&mut input_org).expect("Failed to read line");
            let org: String = input_org.trim().parse().expect("invalid organization");

            println!("{}", Cyan.paint(" > Enter Repo Name"));
            let mut input_proj = String::new();
            std::io::stdin().read_line(&mut input_proj).expect("Failed to read line");
            let proj: String = input_proj.trim().parse().expect("invalid organization");

            if choice == 1 {
                let ver = releases::get_latest_release(&org, &proj);
                println!("{}: {ver}", Green.bold().paint("Latest Release"));
            }
            else if choice ==2 {
                let contrib = releases::get_contributors(&org, &proj);
                println!("{}\n{contrib}", Green.bold().paint("Contributors"));
            }
            else {
                let stars = releases::get_stars(&org, &proj);
                println!("{}={stars}", Green.bold().paint("Total Stars"));
            }

        }
        OP_DEMO => {
            println!("{}", Black.paint(format!("Your Name: {}, Op: {}", args.name, args.operation)));

            for (key, value) in env::vars() {
                println!("{}", Black.paint(format!("{key} : {value}")));
            }

            println!("{}", Blue.paint(format!("In file {FILE_PATH}")));

            let contents =
            fs::read_to_string(FILE_PATH).expect("Should have been able to read the file");

            println!("Content upto 50 chars: \n-----\n{}\n-----\n", &contents[..50]);

            let now = SystemTime::now();

            sleep(Duration::new(2, 0));
            match now.elapsed() {
                Ok(elapsed) => {
                    println!("{}", Black.paint(format!("Sleeped for {}s", elapsed.as_secs())));
                }
                Err(e) => {
                    eprintln!("{}", Red.bold().paint(format!("Error: {e:?}")));
                }
            }
        }
        _ => eprintln!("{}", Red.bold().paint("Invalid Operation choosen"))
    }

    Ok(())
}

