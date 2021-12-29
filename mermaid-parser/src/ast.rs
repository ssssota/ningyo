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
    // user journey
    Journey(Pos),
    JourneyEntry {
        name: &'a str,
        value: f64,
        users: Vec<&'a str>,
        position: Pos,
    },
    JourneySection {
        name: &'a str,
        position: Pos,
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
