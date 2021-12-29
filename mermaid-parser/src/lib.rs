extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod diagram;

pub use ast::DiagramTerm;
pub use diagram::parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn journey() {
        let terms = parse(
            r#"
                # user journey
                journey
                title Daily tasks
                Take a bread: 5: Me
                section At the office
                sleep: 0.1
            "#,
        )
        .unwrap();
        assert_eq!(
            terms[0],
            DiagramTerm::Comment {
                content: "user journey",
                posision: (17, 31)
            }
        );
        assert_eq!(terms[1], DiagramTerm::Journey((48, 55)));
        assert_eq!(
            terms[2],
            DiagramTerm::Title {
                content: "Daily tasks",
                posision: (72, 89)
            }
        );
        assert_eq!(
            terms[3],
            DiagramTerm::JourneyEntry {
                name: "Take a bread",
                value: 5.0,
                users: vec!["Me"],
                position: (106, 125)
            }
        );
        assert_eq!(
            terms[4],
            DiagramTerm::JourneySection {
                name: "At the office",
                position: (142, 163)
            }
        );
        assert_eq!(
            terms[5],
            DiagramTerm::JourneyEntry {
                name: "sleep",
                value: 0.1,
                users: vec![],
                position: (180, 190)
            }
        );
    }

    #[test]
    fn pie() {
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

    #[test]
    fn info() {
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
