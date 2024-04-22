use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MyParser;

fn main() {
    dbg!(MyParser::parse(Rule::init, "abc"));
}
