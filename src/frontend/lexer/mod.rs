mod token;
use token::*;

use miette::{Diagnostic, NamedSource, Report, SourceSpan};
use std::iter::Peekable;
use std::str::CharIndices;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
enum LexError {
    #[error("unclosed string literal")]
    #[diagnostic(
        code(lexer::unclosed_string),
        help("add a closing `\"` to terminate the string")
    )]
    UnclosedString {
        #[source_code]
        src: NamedSource<String>,
        #[label("string starts here and never ends")]
        span: SourceSpan,
    },
}

pub struct Lexer<'a> {
    src: &'a str,
    name: &'a str,
    chars: Peekable<CharIndices<'a>>,
    offset: usize,
    row: u32,
    col: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(name: &'a str, src: &'a str) -> Self {
        Self {
            src,
            name,
            chars: src.char_indices().peekable(),
            offset: 0,
            row: 1,
            col: 0,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, c)| c)
    }

    fn check(
        &mut self,
        expected_char: char,
        default_kind: TokenKind,
        expected_kind: TokenKind,
    ) -> Token {
        if self.peek() == Some(expected_char) {
            let token = self.make_token(expected_kind);
            self.advance();
            return token;
        }
        self.make_token(default_kind)
    }

    fn return_keyword_or_ident(&mut self, ident: String) -> Token {
        let len = ident.len() as u32 - 1;
        self.col -= len;
        let token = match ident.as_str() {
            "archetype" => self.make_token(TokenKind::Archetype),
            "class" => self.make_token(TokenKind::Class),
            "extends" => self.make_token(TokenKind::Extends),
            "fn" => self.make_token(TokenKind::Fn),
            "for" => self.make_token(TokenKind::For),
            "float" => self.make_token(TokenKind::FloatT),
            "int" => self.make_token(TokenKind::IntT),
            "impl" => self.make_token(TokenKind::Impl),
            "new" => self.make_token(TokenKind::New),
            "return" => self.make_token(TokenKind::Return),
            "void" => self.make_token(TokenKind::VoidT),
            "while" => self.make_token(TokenKind::While),
            _ => self.make_token(TokenKind::Ident(ident)),
        };
        self.col += len;
        token
    }

    fn report(&self, err: LexError) {
        eprintln!("{:?}", Report::new(err));
        std::process::exit(1);
    }

    fn named_source(&self) -> NamedSource<String> {
        NamedSource::new(self.name, self.src.to_string())
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.is_at_end() {
            return self.make_token(TokenKind::Eof);
        }
        let c = self.advance();
        match c {
            ',' => self.make_token(TokenKind::Comma),
            '&' => self.make_token(TokenKind::Amp),
            '.' => self.make_token(TokenKind::Dot),
            ';' => self.make_token(TokenKind::SemiColon),
            '(' => self.make_token(TokenKind::LParen),
            ')' => self.make_token(TokenKind::RParen),
            '[' => self.make_token(TokenKind::LBrack),
            ']' => self.make_token(TokenKind::RBrack),
            '{' => self.make_token(TokenKind::LBrace),
            '}' => self.make_token(TokenKind::RBrace),
            '+' => self.make_token(TokenKind::Plus),
            '*' => self.make_token(TokenKind::Star),
            '-' => self.make_token(TokenKind::Minus),
            '/' => self.make_token(TokenKind::Slash),
            '!' => self.check('=', TokenKind::Bang, TokenKind::BangEqual),
            '=' => self.check('=', TokenKind::Equal, TokenKind::EqualEqual),
            '>' => self.check('=', TokenKind::Greater, TokenKind::GreaterEqual),
            '<' => self.check('=', TokenKind::Less, TokenKind::LessEqual),
            '"' => {
                let start = self.offset - 1;
                let mut str = String::new();
                while let Some(x) = self.peek()
                    && x != '"'
                {
                    str.push(x);
                    self.advance();
                }
                if self.is_at_end() {
                    self.report(LexError::UnclosedString {
                        src: self.named_source(),
                        span: (start, self.offset - start).into(),
                    });
                }
                self.advance();
                let len = str.len() as u32 + 1;
                self.col -= len;
                let token = self.make_token(TokenKind::String(str));
                self.col += len;
                token
            }
            '0'..='9' => {
                let mut num = String::new();
                num.push(c);
                while let Some(x) = self.peek()
                    && x.is_numeric()
                {
                    num.push(x);
                    self.advance();
                }
                if self.peek() == Some('.') {
                    num.push('.');
                    self.advance();
                    while let Some(x) = self.peek()
                        && x.is_numeric()
                    {
                        num.push(x);
                        self.advance();
                    }
                }
                let len = num.len() as u32 - 1;
                self.col -= len;
                let token = self.make_token(TokenKind::Number(num));
                self.col += len;
                token
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                ident.push(c);
                while let Some(x) = self.peek()
                    && (x.is_alphanumeric() || x == '_')
                {
                    ident.push(x);
                    self.advance();
                }
                self.return_keyword_or_ident(ident)
            }
            _ => self.make_token(TokenKind::Illegal(c)),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek() {
                None => {
                    self.col += 1;
                    break;
                }
                Some(' ' | '\r' | '\t') => {
                    self.advance();
                }
                Some('\n') => {
                    self.row += 1;
                    self.advance();
                    self.col = 0;
                }
                Some(_) => break,
            }
        }
    }

    fn advance(&mut self) -> char {
        self.col += 1;
        let (i, c) = self.chars.next().expect("advance called at end of input");
        self.offset = i + c.len_utf8();
        c
    }

    fn is_at_end(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    fn make_token(&self, kind: TokenKind) -> Token {
        let pos = Location {
            row: self.row,
            col: self.col,
        };
        Token { kind, pos }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            None
        } else {
            Some(self.next_token())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tok(kind: TokenKind, row: u32, col: u32) -> Token {
        Token {
            kind,
            pos: Location { row, col },
        }
    }

    #[test]
    fn single_punctuations() {
        use TokenKind::*;
        let mut lexer = Lexer::new("test", ";()\n{}=!+/*-.><&,[]");
        assert_eq!(tok(SemiColon, 1, 1), lexer.next_token());
        assert_eq!(tok(LParen, 1, 2), lexer.next_token());
        assert_eq!(tok(RParen, 1, 3), lexer.next_token());
        assert_eq!(tok(LBrace, 2, 1), lexer.next_token());
        assert_eq!(tok(RBrace, 2, 2), lexer.next_token());
        assert_eq!(tok(Equal, 2, 3), lexer.next_token());
        assert_eq!(tok(Bang, 2, 4), lexer.next_token());
        assert_eq!(tok(Plus, 2, 5), lexer.next_token());
        assert_eq!(tok(Slash, 2, 6), lexer.next_token());
        assert_eq!(tok(Star, 2, 7), lexer.next_token());
        assert_eq!(tok(Minus, 2, 8), lexer.next_token());
        assert_eq!(tok(Dot, 2, 9), lexer.next_token());
        assert_eq!(tok(Greater, 2, 10), lexer.next_token());
        assert_eq!(tok(Less, 2, 11), lexer.next_token());
        assert_eq!(tok(Amp, 2, 12), lexer.next_token());
        assert_eq!(tok(Comma, 2, 13), lexer.next_token());
        assert_eq!(tok(LBrack, 2, 14), lexer.next_token());
        assert_eq!(tok(RBrack, 2, 15), lexer.next_token());
    }

    #[test]
    fn compound_punctuations() {
        use TokenKind::*;
        let mut lexer = Lexer::new("test", "==\n!=\n>=\n<=");
        assert_eq!(tok(EqualEqual, 1, 1), lexer.next_token());
        assert_eq!(tok(BangEqual, 2, 1), lexer.next_token());
        assert_eq!(tok(GreaterEqual, 3, 1), lexer.next_token());
        assert_eq!(tok(LessEqual, 4, 1), lexer.next_token());
    }

    #[test]
    fn strings() {
        use TokenKind::*;
        let mut lexer = Lexer::new("test", "\"hello world\"");
        assert_eq!(
            tok(String("hello world".to_string()), 1, 1),
            lexer.next_token()
        );
    }

    #[test]
    fn numbers() {
        use TokenKind::*;
        let mut lexer = Lexer::new("test", "69.420\n73");
        assert_eq!(tok(Number("69.420".to_string()), 1, 1), lexer.next_token());
        assert_eq!(tok(Number("73".to_string()), 2, 1), lexer.next_token());
    }

    #[test]
    fn idents() {
        use TokenKind::*;
        let mut lexer = Lexer::new("test", "ur_mom\nUrDad");
        assert_eq!(tok(Ident("ur_mom".to_string()), 1, 1), lexer.next_token());
        assert_eq!(tok(Ident("UrDad".to_string()), 2, 1), lexer.next_token());
    }

    #[test]
    fn keywords() {
        use TokenKind::*;
        let mut lexer = Lexer::new(
            "test",
            "class\nint\narchetype\nfloat\nfn\nvoid\nimpl\nfor\nwhile\nextends\nnew\nreturn\n",
        );
        assert_eq!(tok(Class, 1, 1), lexer.next_token());
        assert_eq!(tok(IntT, 2, 1), lexer.next_token());
        assert_eq!(tok(Archetype, 3, 1), lexer.next_token());
        assert_eq!(tok(FloatT, 4, 1), lexer.next_token());
        assert_eq!(tok(Fn, 5, 1), lexer.next_token());
        assert_eq!(tok(VoidT, 6, 1), lexer.next_token());
        assert_eq!(tok(Impl, 7, 1), lexer.next_token());
        assert_eq!(tok(For, 8, 1), lexer.next_token());
        assert_eq!(tok(While, 9, 1), lexer.next_token());
        assert_eq!(tok(Extends, 10, 1), lexer.next_token());
        assert_eq!(tok(New, 11, 1), lexer.next_token());
        assert_eq!(tok(Return, 12, 1), lexer.next_token());
    }
}
