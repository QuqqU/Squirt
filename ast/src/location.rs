#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Location {
    pub row:    i64,
    pub column: i64,
}

impl Location {
    pub fn new(row: i64, column: i64) -> Self {
        Location { row, column }
    }
}
