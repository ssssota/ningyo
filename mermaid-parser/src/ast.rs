#[derive(Debug, PartialEq)]
pub enum Diagram<'a> {
    Pie {
        title: Option<&'a str>,
        entries: Vec<(&'a str, f64)>,
    },
    None,
}
