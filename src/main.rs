extern crate colored;
extern crate structopt;

use std::time::{Instant};
use std::path::{Path};
use std::process::Command;
use std::fs::{read_dir, read_to_string, File};
use colored::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    binary: Option<String>,
}

fn main() {
    let args = Cli::from_args();

    let binary = if let Some(b) = &args.binary { &b[..] } else { "./target/release/rust" };

    let input_dir_path = Path::new("./in");
    let input_dir = read_dir(&input_dir_path).expect(&format!("Permission error whilie reading {} directory", input_dir_path.to_str().unwrap()));

    let now = Instant::now();

    input_dir.for_each(|de| {
        let input_path = de.unwrap().path();
        let file_name = input_path.file_name().unwrap().to_owned();
        let file_name = file_name.to_str().unwrap();
        let file = File::open(input_path).unwrap();

        let utf8_output = Command::new(binary)
            .stdin(file)
            .output()
            .expect("failed to execute process")
            .stdout;

        let output = String::from_utf8(utf8_output).unwrap();
        let output = output.trim();
        let answer = read_to_string(format!("./out/{}", file_name)).expect("Something went wrong reading the file");
        let answer = answer.trim();

        println!("{}", file_name);
        if output == answer {
            println!("{}", "pass".green());
        } else {
            println!("{} {} {}", "fail".red(), output, answer);
        }
    });

    println!("\nTook {}ms", now.elapsed().as_millis());
}