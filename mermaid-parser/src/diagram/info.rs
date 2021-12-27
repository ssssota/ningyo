use crate::ast::Diagram;
use crate::diagram::Rule;
use pest::error::Error;
use pest::iterators::Pair;

pub fn parse<'a>(pair: Pair<'a, Rule>) -> Result<Diagram<'a>, Error<Rule>> {
    let mut show_info = false;
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::info_show => show_info = true,
            Rule::EOI
            | Rule::mermaid
            | Rule::WHITESPACE
            | Rule::whitespace_or_newline
            | Rule::diagram
            | Rule::info
            | Rule::pie_title
            | Rule::pie_entry
            | Rule::pie_diagram
            | Rule::pie_title_value
            | Rule::number
            | Rule::string
            | Rule::string_inner
            | Rule::char => unreachable!("in info"),
        }
    }
    Ok(Diagram::Info { show_info })
}
