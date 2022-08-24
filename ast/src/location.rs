use std::fmt;

#[macro_export]
macro_rules! loc {
    ($row: expr, $column: expr) => {
        Location::new($row, $column)
    };
}

#[derive(PartialEq)]
pub struct Location {
    pub row:    i64,
    pub column: i64,
}

impl Location {
    pub fn new(row: i64, column: i64) -> Self {
        Location { row, column }
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

#[cfg(test)]
mod init {
    use crate::Location;

    #[test]
    fn new() {
        let expected = Location {
            row:    1,
            column: 2,
        };
        let loc = Location::new(1, 2);
        assert_eq!(expected, loc)
    }

    #[test]
    fn loc_macro() {
        let expected = Location {
            row:    1,
            column: 2,
        };
        let loc = loc!(1, 2);
        assert_eq!(expected, loc)
    }
}

#[test]
fn fmt_dbg() {
    let expected = "(1, 2)";
    let dbg_str = format!("{:?}", Location::new(1, 2));
    assert_eq!(expected, dbg_str)
}
