mod json;
use json::json_reader::{load_json, read_json};
use std::{env, process};

const JSON_ARG: usize = 1;
const INIT_OFFSET: u32 = 0;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: cargo run <json_file>");
        process::exit(1);
    }

    let json_path = &args[JSON_ARG];
    let json = load_json(json_path);

    if let Err(err_msg) = read_json(json, INIT_OFFSET) {
        println!("{}", err_msg);
    }
}
