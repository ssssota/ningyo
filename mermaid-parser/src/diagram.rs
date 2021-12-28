use std::collections::VecDeque;

use pest::error::Error;
use pest::iterators::Pair;
use pest::{Parser, Span};

use crate::DiagramTerm;

#[derive(Parser)]
#[grammar = "mermaid.pest"]
struct MermaidParser;

fn span_to_pos(span: &Span) -> (usize, usize) {
    (span.start(), span.end())
}

pub fn parse(mermaid: &str) -> Result<Vec<DiagramTerm>, Error<Rule>> {
    let mut queue: VecDeque<Pair<Rule>> = VecDeque::new();
    MermaidParser::parse(Rule::mermaid, mermaid)?.for_each(|p| queue.push_back(p));

    let mut result: Vec<DiagramTerm> = vec![];
    while queue.len() > 0 {
        let pair = queue.pop_front().unwrap();
        let span = pair.as_span();
        match pair.as_rule() {
            Rule::mermaid => pair.into_inner().rev().for_each(|p| queue.push_front(p)),
            Rule::pie_start => {
                pair.into_inner().rev().for_each(|p| queue.push_front(p));
                result.push(DiagramTerm::Pie(span_to_pos(&span)));
            }
            Rule::pie_title => {
                result.push(DiagramTerm::Title {
                    content: pair.into_inner().next().unwrap().as_str(),
                    posision: span_to_pos(&span),
                });
            }
            Rule::pie_entry => {
                let mut entry = pair.into_inner();
                result.push(DiagramTerm::PieEntry {
                    name: entry.next().unwrap().into_inner().next().unwrap().as_str(),
                    value: entry.next().unwrap().as_str().parse().unwrap(),
                    position: span_to_pos(&span),
                });
            }
            Rule::info_start => {
                pair.into_inner().rev().for_each(|p| queue.push_front(p));
                result.push(DiagramTerm::Info(span_to_pos(&span)));
            }
            Rule::info_show => {
                result.push(DiagramTerm::ShowInfo(span_to_pos(&span)));
            }
            Rule::comment => {
                result.push(DiagramTerm::Comment {
                    content: pair.into_inner().next().unwrap().as_str(),
                    posision: span_to_pos(&span),
                });
            }
            Rule::EOI => {}
            Rule::number
            | Rule::quoted_string
            | Rule::string_inner
            | Rule::char
            | Rule::pie_title_value
            | Rule::comment_inner
            | Rule::comment_start
            | Rule::comment_or_newline
            | Rule::diagram
            | Rule::pie_diagram
            | Rule::info
            | Rule::WHITESPACE => unreachable!(),
        };
    }

    Ok(result)
}
