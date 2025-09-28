use std::env;
use std::fs::File;
use std::io::BufReader;
use std::process;

use junit_parser::from_reader;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <junit_file.xml>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file '{}': {}", filename, err);
            process::exit(1);
        }
    };

    let reader = BufReader::new(file);

    match from_reader(reader) {
        Ok(test_suites) => {
            println!("{:#?}", test_suites);
        }
        Err(err) => {
            eprintln!("Error parsing JUnit XML: {}", err);
            process::exit(1);
        }
    }
}
