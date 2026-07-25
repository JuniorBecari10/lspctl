use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'a> {
    #[token("||")]
    PipePipe, // tectonic's nonstandard pipe

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
