use syntax::ast::punc::*;
use syntax::ast::token::*;
use std::io::{Error, BufRead, BufReader};
use std::char::from_u32;
use std::num::*;
use std::str::FromStr;
use self::TokenData::*;
use serialize::json::{from_str};
use syntax::ast::punc::Punctuator::{PXor, PAssignXor, PStrictEq, PAssignRightSh, PGreaterThanOrEq, PLessThan, PLeftSh, PRightSh, PURightSh, PAssignURightSh, PNot, PNotEq, PStrictNotEq, PNeg, PGreaterThan, PAssignLeftSh, PArrow, PAssign, PLessThanOrEq};
use syntax::ast::punc::Punctuator::*;
macro_rules! vop(
    ($this:ident, $assign_op:expr, $op:expr) => ({
        let preview = try!($this.preview_next());
        match preview {
            '=' => {
                $this.buffer.consume(1);
                $assign_op
            },
            _ => $op
        }
    });
    ($this:ident, $assign_op:expr, $op:expr, {$($case:pat => $block:expr),+}) => ({
        let preview = try!($this.preview_next());
        match preview {
            '=' => {
                $this.buffer.consume(1);
                $assign_op
            },
            $($case => $block)+,
            _ => $op
        }
    });
    ($this:ident, $op:expr, {$($case:pat => $block:expr),+}) => ({
        let preview = try!($this.preview_next());
        match preview {
            $($case => $block)+,
            _ => $op
        }
    });
);
macro_rules! op(
    ($this:ident, $assign_op:expr, $op:expr) => ({
        let punc = vop!($this, $assign_op, $op);
        $this.push_punc(punc);
    });
    ($this:ident, $assign_op:expr, $op:expr, {$($case:pat => $block:expr),+}) => ({
        let punc = vop!($this, $assign_op, $op, {$($case => $block),+});
        $this.push_punc(punc);
    });
    ($this:ident, $op:expr, {$($case:pat => $block:expr),+}) => ({
        let punc = vop!($this, $op, {$($case => $block),+});
        $this.push_punc();
    });
);
/// A Javascript lexer
pub struct Lexer<B> {
    /// The list of tokens generated so far
    pub tokens : Vec<Token>,
    /// The current line number in the script
    line_number : u64,
    /// The current column number in the script
    column_number : u64,
    /// The reader
    buffer: B
}
impl<B:BufRead> Lexer<B> {
    /// Creates a new lexer with empty buffers
    pub fn new(buffer: B) -> Lexer<B> {
        Lexer {
            tokens: Vec::new(),
            line_number: 1,
            column_number: 0,
            buffer: buffer
        }
    }
    #[inline(always)]
    fn push_token(&mut self, tk:TokenData) {
        self.tokens.push(Token::new(tk, self.line_number, self.column_number))
    }
    #[inline(always)]
    fn push_punc(&mut self, punc:Punctuator) {
        self.push_token(TPunctuator(punc));
    }
    /// Processes an input stream from a string into an array of tokens
    pub fn lex_str(script:&str) -> Vec<Token> {
        let script_bytes:&[u8] = script.as_bytes();
        let reader = BufReader::new(script_bytes);
        let buf_reader = BufReader::new(reader);
        let mut lexer = Lexer::new(buf_reader);
        lexer.lex().unwrap();
        lexer.tokens
    }
    #[inline(always)]
    fn next(&mut self) -> Result<char> {
        self.buffer.read_char()
    }
    fn preview_next(&mut self) -> Result<char> {
        let buf = try!(self.buffer.fill_buf());
        Ok(buf[0] as char)
    }
    fn next_is(&mut self, peek:char) -> Result<bool> {
        let result = try!(self.preview_next()) == peek;
        if result {
            self.buffer.consume(1);
        }
        Ok(result)
    }
    /// Processes an input stream from the `buffer` into a vector of tokens
    pub fn lex(&mut self) -> Result<()> {
        loop {
            let ch = match self.next() {
                Ok(ch) => ch,
                Err(Error {kind: EndOfFile, ..}) => break,
                Err(err) => return Err(err)
            };
            self.column_number += 1;
            match ch {
                '"' | '\'' => {
                    let mut buf = String::new();
                    loop {
                        match try!(self.next()) {
                            '\'' if ch == '\'' => {
                                break;
                            },
                            '"' if ch == '"' => {
                                break;
                            },
                            '\\' => {
                                let escape = try!(self.next());
                                if escape != '\n' {
                                    let escaped_ch = match escape {
                                        'n' => '\n',
                                        'r' => '\r',
                                        't' => '\t',
                                        'b' => '\x08',
                                        'f' => '\x0c',
                                        '0' => '\0',
                                        'x' => {
                                            let mut nums = String::with_capacity(2);
                                            for _ in 0u8..2 {
                                                nums.push_char(try!(self.next()));
                                            }
                                            self.column_number += 2;
                                            let as_num = match u64::from_str_radix(nums.as_slice(), 16) {
                                                Some(v) => v,
                                                None => 0
                                            };
                                            match from_u32(as_num) {
                                                Some(v) => v,
                                                None => panic!("{}:{}: {} is not a valid unicode scalar value", self.line_number, self.column_number, as_num)
                                            }
                                        },
                                        'u' => {
                                            let mut nums = String::new();
                                            for _ in 0u8..4 {
                                                nums.push_char(try!(self.next()));
                                            }
                                            self.column_number += 4;
                                            let as_num = match u64::from_str_radix(nums.as_slice(), 16) {
                                                Some(v) => v,
                                                None => 0
                                            };
                                            match from_u32(as_num) {
                                                Some(v) => v,
                                                None => panic!("{}:{}: {} is not a valid unicode scalar value", self.line_number, self.column_number, as_num)
                                            }
                                        },
                                        '\'' | '"' => escape,
                                        _ => panic!("{}:{}: Invalid escape `{}`", self.line_number, self.column_number, ch)
                                    };
                                    buf.push_char(escaped_ch);
                                }
                            },
                            ch => buf.push_char(ch)
                        }
                    }
                    self.push_token(TStringLiteral(buf))
                },
                '0' => {
                    let mut buf = String::new();
                    let num = if try!(self.next_is('x')) {
                        loop {
                            let ch = try!(self.preview_next());
                            match ch {
                                ch if ch.is_digit_radix(16) => {
                                    self.buffer.consume(1);
                                    buf.push_char(ch)
                                },
                                _ => break
                            }
                        }
                        u64::from_str_radix(buf.as_slice(), 16).unwrap()
                    } else {
                        let mut gone_decimal = false;
                        loop {
                            let ch = try!(self.preview_next());
                            match ch {
                                ch if ch.is_digit_radix(8) => {
                                    buf.push_char(ch);
                                    self.buffer.consume(1);
                                },
                                '8' | '9' | '.' => {
                                    gone_decimal = true;
                                    buf.push_char(ch);
                                    self.buffer.consume(1);
                                },
                                _ =>
                                    break
                            }
                        }
                        if gone_decimal {
                            from_str(buf.as_slice())
                        } else {
                            u64::from_str_radix(buf.as_slice(), 8)
                        }.unwrap()
                    };
                    self.push_token(TNumericLiteral(num))
                },
                _ if ch.is_digit() => {
                    let mut buf = ch.to_string();
                    loop {
                        let ch = try!(self.preview_next());
                        match ch {
                            '.' => {
                                buf.push_char(ch);
                                self.buffer.consume(1);
                            },
                            _ if ch.is_digit() => {
                                buf.push_char(ch);
                                self.buffer.consume(1);
                            },
                            _ => break
                        }
                    }
                    self.push_token(TNumericLiteral(from_str(buf.as_slice()).unwrap()));
                },
                _ if ch.is_alphabetic() || ch == '$' || ch == '_' => {
                    let mut buf = ch.to_string();
                    loop {
                        let ch = try!(self.preview_next());
                        match ch {
                            _ if ch.is_alphabetic() || ch.is_digit() || ch == '_' => {
                                buf.push_char(ch);
                                self.buffer.consume(1);
                            },
                            _ => {
                                break;
                            }
                        }
                    }
                    self.push_token(match buf.as_slice() {
                        "true" => TBooleanLiteral(true),
                        "false" => TBooleanLiteral(false),
                        "null" => TNullLiteral,
                        slice => match FromStr::from_str(slice) {
                            Some(keyword) => TKeyword(keyword),
                            None => TIdentifier(buf.clone())
                        }
                    });
                },
                ';' => self.push_punc(PSemicolon),
                ':' => self.push_punc(PColon),
                '.' => self.push_punc(PDot),
                '(' => self.push_punc(POpenParen),
                ')' => self.push_punc(PCloseParen),
                ',' => self.push_punc(PComma),
                '{' => self.push_punc(POpenBlock),
                '}' => self.push_punc(PCloseBlock),
                '[' => self.push_punc(POpenBracket),
                ']' => self.push_punc(PCloseBracket),
                '?' => self.push_punc(PQuestion),
                '/' => {
                    let token = match try!(self.preview_next()) {
                        '/' => {
                            let comment = try!(self.buffer.read_line());
                            TokenData::TComment(comment.as_slice().slice_to(comment.len() - 1).into_string())
                        },
                        '*' => {
                            let mut buf = String::new();
                            loop {
                                match try!(self.next()) {
                                    '*' =>
                                        if try!(self.next_is('/')) {
                                            break;
                                        } else {
                                            buf.push_char('*');
                                        },
                                    ch =>
                                        buf.push_char(ch)
                                }
                            }
                            TokenData::TComment(buf)
                        },
                        '=' => TokenData::TPunctuator(PAssignDiv),
                        _ => TokenData::TPunctuator(PDiv)
                    };
                    self.push_token(token)
                },
                '*' => op!(self, PAssignMul, PMul),
                '+' => op!(self, PAssignAdd, PAdd, {
                    '+' => PInc
                }),
                '-' => op!(self, PAssignSub, PSub, {
                    '+' => PDec
                }),
                '%' => op!(self, PAssignMod, PMod),
                '|' => op!(self, PAssignOr, POr, {
                    '|' => PBoolOr
                }),
                '&' => op!(self, PAssignAnd, PAnd, {
                    '&' => PBoolAnd
                }),
                '^' => op!(self, PAssignXor, PXor),
                '=' => op!(self, if try!(self.next_is('=')) {
                    PStrictEq
                } else {
                    PEq
                }, PAssign, {
                    '>' => PArrow
                }),
                '<' => op!(self, PLessThanOrEq, PLessThan, {
                    '<' => vop!(self, PAssignLeftSh, PLeftSh)
                }),
                '>' => op!(self, PGreaterThanOrEq, PGreaterThan, {
                    '>' => vop!(self, PAssignRightSh, PRightSh, {
                        '>' => vop!(self, PAssignURightSh, PURightSh)
                    })
                }),
                '!' => op!(self, vop!(self, PStrictNotEq, PNotEq), PNot),
                '~' => self.push_punc(PNeg),
                '\n' | '\u{2028}' | '\u{2029}' => {
                    self.line_number += 1;
                    self.column_number = 0;
                },
                '\r' => {
                    self.column_number = 0;
                },
                " " => (),
                ch => panic!("{}:{}: Unexpected '{}'", self.line_number, self.column_number, ch)
            };
        };
        Ok(())
    }
}
