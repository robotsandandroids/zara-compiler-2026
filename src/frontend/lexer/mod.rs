mod token;
use token::*;

use std::iter::Peekable;

pub struct Lexer<T: Iterator<Item = char>> {
    source: Peekable<T>,
    row: u32,
    col: u32,
}

impl<T: Iterator<Item = char>> Lexer<T> {
    fn new(source: T) -> Self {
        Self {
            source: source.peekable(),
            row: 1,
            col: 0,
        }
    }

    fn check(
        &mut self,
        expected_char: char,
        default_kind: TokenKind,
        expected_kind: TokenKind,
    ) -> Token {
        if let Some(c) = self.source.peek()
            && *c == expected_char
        {
            let token = self.make_token(expected_kind);
            self.advance();
            return token;
        }
        self.make_token(default_kind)
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        if self.is_at_end() {
            return self.make_token(TokenKind::Eof);
        }
        let c = self.advance();
        match c {
            ';' => self.make_token(TokenKind::SemiColon),
            '(' => self.make_token(TokenKind::LParen),
            ')' => self.make_token(TokenKind::RParen),
            '{' => self.make_token(TokenKind::LBrace),
            '}' => self.make_token(TokenKind::RBrace),
            '+' => self.make_token(TokenKind::Plus),
            '*' => self.make_token(TokenKind::Star),
            '-' => self.make_token(TokenKind::Minus),
            '/' => self.make_token(TokenKind::Slash),
            '!' => self.check('=', TokenKind::Bang, TokenKind::BangEqual),
            '=' => self.check('=', TokenKind::Equal, TokenKind::EqualEqual),
            _ => self.make_token(TokenKind::Illegal(c)),
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            if self.is_at_end() {
                self.col += 1;
                break;
            }
            let c = self.source.peek().unwrap();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.row += 1;
                    self.advance();
                    self.col = 0;
                }
                _ => break,
            }
        }
    }

    fn advance(&mut self) -> char {
        assert!(!self.is_at_end());
        self.col += 1;
        self.source.next().unwrap()
    }

    fn is_at_end(&mut self) -> bool {
        self.source.peek().is_none()
    }

    fn make_token(&self, kind: TokenKind) -> Token {
        let pos = Location {
            row: self.row,
            col: self.col,
        };
        Token { kind, pos }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_punctuations() {
        use TokenKind::*;
        let test_str = ";()\n{}=!+/*-";
        let mut lexer = Lexer::new(test_str.chars());
        assert_eq!(
            Token {
                kind: SemiColon,
                pos: Location { col: 1, row: 1 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: LParen,
                pos: Location { col: 2, row: 1 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: RParen,
                pos: Location { col: 3, row: 1 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: LBrace,
                pos: Location { col: 1, row: 2 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: RBrace,
                pos: Location { col: 2, row: 2 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: Equal,
                pos: Location { col: 3, row: 2 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: Bang,
                pos: Location { col: 4, row: 2 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: Plus,
                pos: Location { col: 5, row: 2 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: Slash,
                pos: Location { col: 6, row: 2 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: Star,
                pos: Location { col: 7, row: 2 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: Minus,
                pos: Location { col: 8, row: 2 }
            },
            lexer.next_token()
        );
    }
    #[test]
    fn compound_punctuations() {
        use TokenKind::*;
        let test_str = "==\n!=";
        let mut lexer = Lexer::new(test_str.chars());
        assert_eq!(
            Token {
                kind: EqualEqual,
                pos: Location { col: 1, row: 1 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: BangEqual,
                pos: Location { col: 1, row: 2 }
            },
            lexer.next_token()
        );
    }
}
