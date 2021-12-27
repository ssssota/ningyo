mod info;
mod pie;

use crate::ast::Diagram;
use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "mermaid.pest"]
struct MermaidParser;

pub fn parse(mermaid: &str) -> Result<Diagram, Error<Rule>> {
    let pair = MermaidParser::parse(Rule::mermaid, mermaid)?
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap();

    match pair.as_rule() {
        Rule::pie_diagram => pie::parse(pair),
        Rule::info => info::parse(pair),
        Rule::EOI
        | Rule::mermaid
        | Rule::WHITESPACE
        | Rule::whitespace_or_newline
        | Rule::info_show
        | Rule::pie_title
        | Rule::pie_title_value
        | Rule::pie_entry
        | Rule::number
        | Rule::string
        | Rule::string_inner
        | Rule::char
        | Rule::diagram => unreachable!(),
    }
}
