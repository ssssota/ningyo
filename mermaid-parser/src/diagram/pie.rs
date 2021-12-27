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
            Rule::EOI => todo!(),
            Rule::mermaid => todo!(),
            Rule::WHITESPACE => todo!(),
            Rule::whitespace_or_newline => todo!(),
            Rule::diagram => todo!(),
            Rule::pie_diagram => todo!(),
            Rule::pie_title_value => todo!(),
            Rule::number => todo!(),
            Rule::string => todo!(),
            Rule::string_inner => todo!(),
            Rule::char => todo!(),
        }
    }
    Ok(Diagram::Pie { title, entries })
}
