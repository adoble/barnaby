#![allow(dead_code)]

mod model;
use model::repository::Repository;
use parse::troy_parser::TroyParser;

mod parse;

fn main() {
    println!("Hello, Inspector Barnaby!");
    println!("Starting to understand");

    let mut repository = Repository::new();
    TroyParser::build_model("pv Richard Bayly", &mut repository);
}
