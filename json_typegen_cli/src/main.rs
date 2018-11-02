use json_typegen_shared::{codegen, codegen_from_macro, Options, parse, OutputMode};
use clap::{Arg, App};
use std::io::{self, Read, Write};
use std::fs::OpenOptions;

fn main_with_result() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("json_typegen CLI")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Generate Rust types from JSON samples")
        .arg(
            Arg::with_name("input")
                .help(concat!(
                    "The input to generate types from. A sample, file, URL, or macro. To read ",
                    "from standard input, a dash, '-', can be used as the input argument."))
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
        .arg(
            Arg::with_name("options")
                .long("options")
                .help(concat!(
                    "Options for code generation, in the form of an options block. If input is a ",
                    "macro, this option is ignored."))
                .takes_value(true)
        )
        .arg(
            Arg::with_name("output-mode")
                .long("output-mode")
                .short("-O")
                .possible_values(&["rust", "typescript", "kotlin", "json_schema", "shape"])
                .help("What to output.")
                .takes_value(true)
        )
        .get_matches();

    let source = matches.value_of("input").ok_or("Input argument is required")?;

    let input = if source == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        source.to_string()
    };

    let code = if input.trim().starts_with("json_typegen") {
        codegen_from_macro(&input)
    } else {
        let name = matches.value_of("name").unwrap_or("Root");
        let mut options = match matches.value_of("options") {
            Some(block) => parse::options(block)?,
            None => Options::default(),
        };
        if let Some(output_mode) = matches.value_of("output-mode") {
            options.output_mode = OutputMode::from_str(output_mode).ok_or("Invalid output mode")?;
        }
        codegen(&name, &input, options)
    };

    if let Some(filename) = matches.value_of("output") {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;

        file.write_all(code?.as_bytes())?;
    } else {
        print!("{}", code?);
    }

    Ok(())
}

fn main() {
    let result = main_with_result();

    if let Err(e) = result {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
