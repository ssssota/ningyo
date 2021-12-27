extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod diagram;

pub use ast::Diagram;
pub use diagram::parse;

#[cfg(test)]
mod pie_chart_test {
    use crate::parse;
    use crate::Diagram::Pie;

    #[test]
    fn basic_pie() {
        let pie = parse(
            r#"
                pie title Language
                "Rust": 80
                "TypeScript": 20
            "#,
        )
        .unwrap();
        match pie {
            Pie { title, entries } => {
                assert_eq!(title, Some("Language"));
                assert_eq!(entries, vec![("Rust", 80.0), ("TypeScript", 20.0),]);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn line_title() {
        let pie = parse(
            r#"
                pie
                title Language Usage Rate
                "Rust": 52
                "TypeScript": 13
            "#,
        )
        .unwrap();
        match pie {
            Pie { title, entries } => {
                assert_eq!(title, Some("Language Usage Rate"));
                assert_eq!(entries, vec![("Rust", 52.0), ("TypeScript", 13.0)]);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn notitle() {
        let pie = parse(
            r#"
                pie
                "Rust": 0.8
                "TypeScript": 0.2
            "#,
        )
        .unwrap();
        match pie {
            Pie { title, entries } => {
                assert_eq!(title, None);
                assert_eq!(entries, vec![("Rust", 0.8), ("TypeScript", 0.2),]);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn noentry() {
        let pie = parse(
            r#"
                pie title Language Usage Rate
            "#,
        )
        .unwrap();
        match pie {
            Pie { title, entries } => {
                assert_eq!(title, Some("Language Usage Rate"));
                assert_eq!(entries, vec![]);
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod info_test {
    use crate::parse;
    use crate::Diagram::Info;

    #[test]
    fn basic_info() {
        let info = parse("info").unwrap();
        match info {
            Info { show_info } => {
                assert_eq!(show_info, false);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn show_info() {
        let info = parse("info showInfo").unwrap();
        match info {
            Info { show_info } => {
                assert_eq!(show_info, true);
            }
            _ => unreachable!(),
        }
    }
}
