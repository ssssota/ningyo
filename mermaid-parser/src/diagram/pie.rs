use crate::ast::Diagram;
use crate::diagram::Rule;
use pest::error::Error;
use pest::iterators::Pair;

pub fn parse<'a>(pair: Pair<'a, Rule>) -> Result<Diagram<'a>, Error<Rule>> {
    let mut title = None;
    let mut entries = vec![];
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::pie_title => {
                title = Some(p.into_inner().as_str());
            }
            Rule::pie_entry => {
                let mut pairs = p.into_inner();
                entries.push((
                    pairs.next().unwrap().into_inner().next().unwrap().as_str(),
                    pairs.next().unwrap().as_str().parse().unwrap(),
                ))
            }
            Rule::EOI
            | Rule::mermaid
            | Rule::WHITESPACE
            | Rule::whitespace_or_newline
            | Rule::info
            | Rule::info_show
            | Rule::diagram
            | Rule::pie_diagram
            | Rule::pie_title_value
            | Rule::number
            | Rule::string
            | Rule::string_inner
            | Rule::char => unreachable!("in pie"),
        }
    }
    Ok(Diagram::Pie { title, entries })
}
