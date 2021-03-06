#[derive(Clone, PartialEq)]
/// A position in Javascript source code
pub struct Position {
    /// The column number
    pub column_number : u64,
    /// The line number
    pub line_number : u64
}
impl Position {
    /// Create a new position
    pub fn new(line_number: u64, column_number: u64) -> Position {
        Position {
            line_number: line_number,
            column_number: column_number
        }
    }
}
