use crate::lang::{LangError, Span};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind<'a> {
  Symbol(&'a str),
  Number(u64),
  // arithmetic
  Eq,
  Add,
  Sub,
  Mul,
  Div,
  Pow,
  Fact,
  // reserved
  LPar,
  RPar,
  LSqr,
  RSqr,
  // lang
  Keyword(TokenKeyword),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKeyword {
  Diff,
  Int,
  Sum,
  Prod,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
  pub span: Span,
  pub kind: TokenKind<'a>,
}

#[derive(Debug, Clone)]
pub struct Lexer<'a> {
  src: &'a str,
  cur: usize,
}

impl<'a> Lexer<'a> {
  pub fn new(src: &'a str) -> Lexer {
    Lexer {
      // block
      src,
      cur: 0,
    }
  }

  fn peek(&self) -> Option<char> { self.src[self.cur..].chars().next() }

  fn advance(&mut self) -> Option<char> {
    let c = self.peek()?;
    self.cur += c.len_utf8();
    Some(c)
  }

  fn advance_while<P>(
    //.
    &mut self,
    mut predicate: P,
  ) -> Result<(&'a str, Span), LangError>
  where
    P: FnMut(char) -> bool,
  {
    let start = self.cur;

    self
      .src
      .chars()
      // iter
      .skip(self.cur)
      .take_while(|&c| predicate(c))
      .for_each(|c| {
        self.cur += c.len_utf8();
      });

    let end = self.cur;

    if start != end {
      Ok((&self.src[start..end], start..end))
    } else {
      Err(LangError::Lex)
    }
  }

  fn tok(&mut self, kind: TokenKind<'a>) -> Result<Token<'a>, LangError> {
    let start = self.cur;
    self
      .advance()
      //.
      .ok_or(LangError::Lex)?;
    let end = self.cur;

    Ok(Token {
      span: start..end,
      //.
      kind,
    })
  }

  fn number(&mut self) -> Result<Token<'a>, LangError> {
    let (text, span) = self.advance_while(|c| c.is_ascii_digit())?;

    let num = text
      .parse::<u64>() // \in \mathbb{N}
      .map_err(|err| LangError::Integer { err, span: span.clone() })?;

    Ok(Token {
      span,
      kind: TokenKind::Number(
        //.
        num,
      ),
    })
  }

  fn symbol(&mut self) -> Result<Token<'a>, LangError> {
    let (text, span) = self.advance_while(|c| c.is_alphabetic() || c.is_ascii_digit() || c == '_')?;

    let kind = match text {
      "Diff" => TokenKind::Keyword(TokenKeyword::Diff),
      "Int" => TokenKind::Keyword(TokenKeyword::Int),
      "Sum" => TokenKind::Keyword(TokenKeyword::Sum),
      "Prod" => TokenKind::Keyword(TokenKeyword::Prod),
      _ => {
        //.
        TokenKind::Symbol(text)
      }
    };

    Ok(Token {
      span,
      //.
      kind,
    })
  }
}

impl<'a> Iterator for Lexer<'a> {
  type Item = Result<Token<'a>, LangError>;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      return match self.peek()? {
        '=' => Some(self.tok(TokenKind::Eq)),
        '+' => Some(self.tok(TokenKind::Add)),
        '-' => Some(self.tok(TokenKind::Sub)),
        '*' => Some(self.tok(TokenKind::Mul)),
        '/' => Some(self.tok(TokenKind::Div)),
        '^' => Some(self.tok(TokenKind::Pow)),
        '!' => Some(self.tok(TokenKind::Fact)),

        '(' => Some(self.tok(TokenKind::LPar)),
        ')' => Some(self.tok(TokenKind::RPar)),
        '[' => Some(self.tok(TokenKind::LSqr)),
        ']' => Some(self.tok(TokenKind::RSqr)),

        '0'..='9' => {
          Some(self.number()) //.
        }

        'a'..='z' | 'A'..='Z' | '_' => {
          Some(self.symbol()) //.
        }

        c => {
          if c.is_whitespace() {
            self.advance();
            continue;
          } else {
            Some(Err(LangError::Lex))
          }
        }
      };
    }
  }
}
