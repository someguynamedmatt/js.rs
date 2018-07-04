use std::fmt::{Formatter, Result, Display};
use syntax::ast::pos::Position;
use syntax::ast::punc::Punctuator;
use syntax::ast::keyword::Keyword;
#[derive(Clone, PartialEq)]
/// A single of token of Javascript code including its position
pub struct Token {
    /// The token
    pub data : TokenData,
    /// The token's position
    pub pos : Position
}
impl Token {
    /// Create a new detailed token from the token data, line number and column number
    pub fn new(data: TokenData, line_number: u64, column_number: u64) -> Token {
        Token {
            data: data,
            pos: Position::new(line_number, column_number)
        }
    }
}
impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.data)
    }
}
#[derive(Clone, PartialEq)]
/// A single token of Javacript code - a single word, symbol or constant
pub enum TokenData {
    /// A boolean literal, which is either `true` or `false`
    TBooleanLiteral(bool),
    /// The end of the file
    TEOF,
    /// An identifier
    TIdentifier(String),
    /// A keyword
    TKeyword(Keyword),
    /// A `null` literal
    TNullLiteral,
    /// A numeric literal
    TNumericLiteral(f64),
    /// A piece of punctuation
    TPunctuator(Punctuator),
    /// A string literal
    TStringLiteral(String),
    /// A regular expression
    TRegularExpression(String),
    /// A comment
    TComment(String)
}
impl Display for TokenData {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.clone() {
            TokenData::TBooleanLiteral(val) => write!(f, "{}", val),
            TEOF => write!(f, "end of file"),
            TokenData::TIdentifier(ident) => write!(f, "{}", ident),
            TokenData::TKeyword(word) => write!(f, "{}", word),
            TNullLiteral => write!(f, "null"),
            TokenData::TNumericLiteral(num) => write!(f, "{}", num),
            TokenData::TPunctuator(punc) => write!(f, "{}", punc),
            TokenData::TStringLiteral(lit) => write!(f, "{}", lit),
            TokenData::TRegularExpression(reg) => write!(f, "{}", reg),
            TokenData::TComment(comm) => write!(f, "/*{}*/", comm)
        }
    }
}
