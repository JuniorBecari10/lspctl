use std::fmt::Display;

use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'a> {
    #[token("||")]
    PipePipe, // tectonic's non-standard pipe

    #[token("|")]
    Pipe,

    #[token(".")]
    Dot,

    #[token(",")]
    Comma,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[regex(r#""([^"\\]|\\.)*""#)]
    DString(&'a str),

    #[regex(r"'([^'\\]|\\.)*'")]
    SString(&'a str),

    #[regex(r"[A-Za-z_][A-Za-z0-9_]*")]
    Ident(&'a str),
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::PipePipe => write!(f, "||"),
            Token::Pipe => write!(f, "|"),
            Token::Dot => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::DString(s) => write!(f, "{s}"),
            Token::SString(s) => write!(f, "{s}"),
            Token::Ident(s) => write!(f, "{s}"),
        }
    }
}
