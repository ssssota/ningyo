type Pos = (usize, usize);

#[derive(Debug, PartialEq)]
pub enum DiagramTerm<'a> {
    // common
    Title {
        content: &'a str,
        posision: Pos,
    },
    Comment {
        content: &'a str,
        posision: Pos,
    },
    // pie chart
    Pie(Pos),
    PieEntry {
        name: &'a str,
        value: f64,
        position: Pos,
    },
    // info
    Info(Pos),
    ShowInfo(Pos),
}
