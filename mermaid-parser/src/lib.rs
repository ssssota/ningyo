extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod diagram;

pub use ast::DiagramTerm;
pub use diagram::parse;

#[cfg(test)]
mod pie_chart_test {
    use crate::parse;
    use crate::DiagramTerm;

    #[test]
    fn basic_pie() {
        let terms = parse(
            r#"
                # pie chart
                pie title Language
                "Rust": 80 # major language
                "TypeScript": 20
            "#,
        )
        .unwrap();
        assert_eq!(
            terms[0],
            DiagramTerm::Comment {
                content: "pie chart",
                posision: (17, 28)
            }
        );
        assert_eq!(terms[1], DiagramTerm::Pie((45, 48)));
        assert_eq!(
            terms[2],
            DiagramTerm::Title {
                content: "Language",
                posision: (49, 63)
            }
        );
        assert_eq!(
            terms[3],
            DiagramTerm::PieEntry {
                name: "Rust",
                value: 80.0,
                position: (80, 90)
            }
        );
        assert_eq!(
            terms[4],
            DiagramTerm::Comment {
                content: "major language",
                posision: (91, 107)
            }
        );
        assert_eq!(
            terms[5],
            DiagramTerm::PieEntry {
                name: "TypeScript",
                value: 20.0,
                position: (124, 140)
            }
        );
    }
}

#[cfg(test)]
mod info_test {
    use crate::parse;
    use crate::DiagramTerm;

    #[test]
    fn show_info() {
        let terms = parse("info showInfo # comment").unwrap();
        assert_eq!(terms[0], DiagramTerm::Info((0, 4)));
        assert_eq!(terms[1], DiagramTerm::ShowInfo((5, 13)));
        assert_eq!(
            terms[2],
            DiagramTerm::Comment {
                content: "comment",
                posision: (14, 23)
            }
        )
    }
}
