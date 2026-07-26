use logos::Logos;

use crate::registry::parser::template::{
    ast::{Expr, Filter},
    token::Token,
};

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(inner: &'a str) -> anyhow::Result<Self> {
        let tokens = Token::lexer(inner)
            .collect::<Result<_, _>>()
            .map_err(|_| anyhow::anyhow!("Failed to lex expression: {inner:?}"))?;

        Ok(Self { tokens, pos: 0 })
    }

    fn peek(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.pos)
    }

    fn bump(&mut self) -> Option<Token<'a>> {
        let t = self.tokens.get(self.pos).cloned();
        self.pos += 1;

        t
    }

    pub fn parse_pipeline(&mut self) -> anyhow::Result<Expr> {
        let base = self.parse_primary()?;
        let mut filters = Vec::new();

        while let Some(Token::Pipe) | Some(Token::PipePipe) = self.peek() {
            self.bump();
            filters.push(self.parse_filter()?);
        }

        Ok(if filters.is_empty() {
            base
        } else {
            Expr::Pipeline {
                base: Box::new(base),
                filters,
            }
        })
    }

    fn parse_filter(&mut self) -> anyhow::Result<Filter> {
        let name = self.expect_ident()?;

        let args = if let Some(Token::LParen) = self.peek() {
            self.parse_arglist()?
        } else if let Some(Token::DString(_)) | Some(Token::SString(_)) = self.peek() {
            vec![self.parse_primary()?]
        } else {
            Vec::new()
        };

        Ok(Filter { name, args })
    }

    fn parse_arglist(&mut self) -> anyhow::Result<Vec<Expr>> {
        self.bump(); // (
        let mut args = Vec::new();

        if !matches!(self.peek(), Some(Token::RParen)) {
            loop {
                args.push(self.parse_pipeline()?);

                if let Some(Token::Comma) = self.peek() {
                    self.bump();
                } else {
                    break;
                }
            }
        }

        match self.bump() {
            Some(Token::RParen) => Ok(args),
            other => anyhow::bail!("Expected `)`, got {other:?}"),
        }
    }

    fn parse_primary(&mut self) -> anyhow::Result<Expr> {
        match self.bump() {
            Some(Token::DString(s)) | Some(Token::SString(s)) => Ok(Expr::Str(unquote(s))),

            Some(Token::Ident(first)) => {
                if matches!(self.peek(), Some(Token::LParen)) {
                    return Ok(Expr::Call {
                        name: first.to_string(),
                        args: self.parse_arglist()?,
                    });
                }

                let mut segs = vec![first.to_string()];
                while matches!(self.peek(), Some(Token::Dot)) {
                    self.bump();
                    segs.push(self.expect_ident()?);
                }

                Ok(Expr::Path(segs))
            }

            other => anyhow::bail!("Unexpected token: {other:?}"),
        }
    }

    fn expect_ident(&mut self) -> anyhow::Result<String> {
        match self.bump() {
            Some(Token::Ident(s)) => Ok(s.to_string()),
            other => anyhow::bail!("Expected identifier, got {other:?}"),
        }
    }
}

fn unquote(s: &str) -> String {
    s[1..s.len() - 1].replace("\\\"", "\"").replace("\\'", "'")
}
