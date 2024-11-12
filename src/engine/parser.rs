use std::{
    error::Error,
    fmt::{self, Display},
    mem::take,
};

#[derive(Debug)]
pub enum AST {
    Char(char),
    Plus(Box<AST>),
    Star(Box<AST>),
    Question(Box<AST>),
    Or(Box<AST>, Box<AST>),
    Seq(Vec<AST>),
}

#[derive(Debug)]
pub enum ParseError {
    InvalidEscape(usize, char),
    InvalidRightParen(usize),
    NoPrev(usize),
    NoRightParen(usize),
    Empty,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidEscape(pos, c) => {
                write!(f, "Invalid escape sequence at position {}: {}", pos, c)
            }
            ParseError::InvalidRightParen(pos) => {
                write!(f, "Invalid right parenthesis at position {}", pos)
            }
            ParseError::NoPrev(pos) => write!(f, "No previous character at position {}", pos),
            ParseError::NoRightParen(pos) => write!(f, "No right parenthesis at position {}", pos),
            ParseError::Empty => write!(f, "Empty expression"),
        }
    }
}

impl Error for ParseError {}

fn parse_escape(pos: usize, c: char) -> Result<AST, ParseError> {
    match c {
        '\\' | '(' | ')' | '*' | '+' | '?' | '|' => Ok(AST::Char(c)),
        _ => {
            let err = ParseError::InvalidEscape(pos, c);
            Err(err)
        }
    }
}

enum PSQ {
    Plus,
    Star,
    Question,
}

fn parse_plus_star_question(
    seq: &mut Vec<AST>,
    ast_type: PSQ,
    pos: usize,
) -> Result<(), ParseError> {
    if let Some(prev) = seq.pop() {
        let ast = match ast_type {
            PSQ::Plus => AST::Plus(Box::new(prev)),
            PSQ::Star => AST::Star(Box::new(prev)),
            PSQ::Question => AST::Question(Box::new(prev)),
        };
        seq.push(ast);
        Ok(())
    } else {
        Err(ParseError::NoPrev(pos))
    }
}

fn parse_or(mut seq_or: Vec<AST>) -> AST {
    // 本実装においては必ずseq_orの要素数が2つ以上の場合でのみ呼ばれる
    let mut ast = seq_or.pop().unwrap();
    // 最右から順に結合させたいためreverse
    seq_or.reverse();
    for s in seq_or {
        ast = AST::Or(Box::new(s), Box::new(ast));
    }
    return ast;
}

pub fn parse(expr: &str) -> Result<AST, ParseError> {
    enum ParseState {
        Char,
        Escape,
    }

    let mut state = ParseState::Char;
    let mut seq = Vec::new(); // 今現在構築している系列
    let mut seq_or = Vec::new(); // orの演算対象となる系列
    let mut stack = Vec::new(); // （）登場時にこれまでのseqとseq_orを退避する

    for (i, c) in expr.chars().enumerate() {
        match &state {
            ParseState::Char => match c {
                '\\' => {
                    state = ParseState::Escape;
                }
                '+' => {
                    parse_plus_star_question(&mut seq, PSQ::Plus, i)?;
                }
                '*' => {
                    parse_plus_star_question(&mut seq, PSQ::Star, i)?;
                }
                '?' => {
                    parse_plus_star_question(&mut seq, PSQ::Question, i)?;
                }
                '|' => {
                    if seq.is_empty() {
                        return Err(ParseError::NoPrev(i));
                    } else {
                        // これまでの系列をor演算対象として保存し、新しい系列を構築する
                        let prev = take(&mut seq);
                        seq_or.push(AST::Seq(prev));
                    }
                }

                '(' => {
                    // コンテキストスイッチ
                    stack.push((take(&mut seq), take(&mut seq_or)));
                }
                ')' => {
                    // コンテキストスイッチしつつ、or演算をする
                    if let Some((mut prev_seq, prev_seq_or)) = stack.pop() {
                        // コンテキスト内でor演算があった場合
                        if !seq_or.is_empty() {
                            if seq.is_empty() {
                                return Err(ParseError::NoPrev(i));
                            } else {
                                let prev = take(&mut seq);
                                seq_or.push(AST::Seq(prev));
                                prev_seq.push(parse_or(seq_or));
                            }
                        }
                        // コンテキスト内でor演算がなかった場合
                        else {
                            // ただの()のとき -> なにもしない
                            if seq.is_empty() {
                            }
                            // ただの()でないとき
                            else {
                                prev_seq.push(AST::Seq(seq));
                            }
                        }

                        seq = prev_seq;
                        seq_or = prev_seq_or;
                    } else {
                        return Err(ParseError::InvalidRightParen(i));
                    }
                }

                _ => {
                    seq.push(AST::Char(c));
                }
            },
            ParseState::Escape => {
                let ast = parse_escape(i, c)?;
                seq.push(ast);
                state = ParseState::Char;
            }
        }
    }
    // stackが解放されていない、すなわち（）が閉じられていない場合
    if !stack.is_empty() {
        return Err(ParseError::NoRightParen(expr.len()));
    }
    // or演算が残っている場合
    if !seq_or.is_empty() {
        if seq.is_empty() {
            return Err(ParseError::NoPrev(expr.len()));
        } else {
            let prev = take(&mut seq);
            seq_or.push(AST::Seq(prev));
            return Ok(parse_or(seq_or));
        }
    }
    // or演算が残っていない場合
    else {
        if seq.is_empty() {
            return Err(ParseError::Empty);
        } else {
            return Ok(AST::Seq(seq));
        }
    }
}
