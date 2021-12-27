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
        Rule::EOI => panic!("not impl"),
        Rule::mermaid => panic!("not impl"),
        Rule::WHITESPACE => panic!("not impl"),
        Rule::whitespace_or_newline => panic!("not impl"),
        Rule::pie_title => panic!("not impl"),
        Rule::pie_title_value => panic!("not impl"),
        Rule::pie_entry => panic!("not impl"),
        Rule::number => panic!("not impl"),
        Rule::string => panic!("not impl"),
        Rule::string_inner => panic!("not impl"),
        Rule::char => panic!("not impl"),
        Rule::diagram => unreachable!(),
    }
}
