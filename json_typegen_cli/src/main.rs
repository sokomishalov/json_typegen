extern crate json_typegen_shared;
extern crate clap;

use json_typegen_shared::{codegen, codegen_from_macro, Options, infer_source_type};
use clap::{Arg, App};
use std::io::{self, Read, Write};
use std::fs::OpenOptions;

fn main() {
    let matches = App::new("JSON code generation CLI")
        .version("0.1.0")
        .about("Generate Rust types from JSON samples")
        .arg(
            Arg::with_name("input")
                .help("The input to generate types from. A sample, file, URL, or macro.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .help("Name for the root generated type. Default: Root.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .help("What file to write the output to. Default: standard output.")
                .takes_value(true),
        )
        .get_matches();

    let source = matches.value_of("input").unwrap();

    let input = if source == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
    } else {
        source.to_string()
    };

    let code = if input.trim().starts_with("json_typegen") {
        codegen_from_macro(&input)
    } else {
        let name = matches.value_of("name").unwrap_or("Root");
        codegen(&name, &infer_source_type(&input), Options::default())
    };

    if let Some(filename) = matches.value_of("output") {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)
            .unwrap();

        file.write_all(code.unwrap().as_bytes()).unwrap();
    } else {
        print!("{}", code.unwrap());
    }
}
