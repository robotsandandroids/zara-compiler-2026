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
            "void" => self.make_token(TokenKind::VoidT),
            "while" => self.make_token(TokenKind::While),
            _ =>  self.make_token(TokenKind::Ident(ident)),
        };
        self.col += len;
        return token;
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
                let mut str = String::new();
                while let Some(x) = self.source.peek()
                    && *x != '"'
                {
                    str.push(*x);
                    self.advance();
                }
                if self.is_at_end() {
                    todo!("Use miette to report unclosed string delimeter");
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
                while let Some(x) = self.source.peek()
                    && x.is_numeric()
                {
                    num.push(*x);
                    self.advance();
                }
                if let Some(x) = self.source.peek()
                    && *x == '.'
                {
                    num.push(*x);
                    self.advance();
                    while let Some(x) = self.source.peek()
                        && x.is_numeric()
                    {
                        num.push(*x);
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
                while let Some(x) = self.source.peek()
                    && (x.is_alphanumeric() || *x == '_')
                {
                    ident.push(*x);
                    self.advance();
                }
                self.return_keyword_or_ident(ident)
            }
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
        let test_str = ";()\n{}=!+/*-.><&,[]";
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
        assert_eq!(
            Token {
                kind: Dot,
                pos: Location { col: 9, row: 2 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: Greater,
                pos: Location { col: 10, row: 2 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: Less,
                pos: Location { col: 11, row: 2 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: Amp,
                pos: Location { col: 12, row: 2 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: Comma,
                pos: Location { col: 13, row: 2 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: LBrack,
                pos: Location { col: 14, row: 2 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: RBrack,
                pos: Location { col: 15, row: 2 }
            },
            lexer.next_token()
        );
    }
    #[test]
    fn compound_punctuations() {
        use TokenKind::*;
        let test_str = "==\n!=\n>=\n<=";
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
        assert_eq!(
            Token {
                kind: GreaterEqual,
                pos: Location { col: 1, row: 3 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: LessEqual,
                pos: Location { col: 1, row: 4 }
            },
            lexer.next_token()
        );
    }

    #[test]
    fn strings() {
        use TokenKind::*;
        let test_str = "\"hello world\"";
        let mut lexer = Lexer::new(test_str.chars());
        assert_eq!(
            Token {
                kind: String("hello world".to_string()),
                pos: Location { col: 1, row: 1 }
            },
            lexer.next_token()
        );
    }

    #[test]
    fn numbers() {
        use TokenKind::*;
        let test_str = "69.420\n73";
        let mut lexer = Lexer::new(test_str.chars());
        assert_eq!(
            Token {
                kind: Number("69.420".to_string()),
                pos: Location { col: 1, row: 1 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: Number("73".to_string()),
                pos: Location { col: 1, row: 2 }
            },
            lexer.next_token()
        );
    }

    #[test]
    fn idents() {
        use TokenKind::*;
        let test_str = "ur_mom\nUrDad";
        let mut lexer = Lexer::new(test_str.chars());
        assert_eq!(
            Token {
                kind: Ident("ur_mom".to_string()),
                pos: Location { col: 1, row: 1 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: Ident("UrDad".to_string()),
                pos: Location { col: 1, row: 2 }
            },
            lexer.next_token()
        );
    }

    #[test]
    fn keywords() {
        use TokenKind::*;
        let test_str = "class\nint\narchetype\nfloat\nfn\nvoid\nimpl\nfor\nwhile\nextends\nnew\n";
        let mut lexer = Lexer::new(test_str.chars());
        assert_eq!(
            Token {
                kind: Class,
                pos: Location { col: 1, row: 1 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: IntT,
                pos: Location { col: 1, row: 2 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: Archetype,
                pos: Location { col: 1, row: 3 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: FloatT,
                pos: Location { col: 1, row: 4 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: Fn,
                pos: Location { col: 1, row: 5 }
            },
            lexer.next_token()
        );
        assert_eq!(
            Token {
                kind: VoidT,
                pos: Location { col: 1, row: 6 }
            },
            lexer.next_token()
        );

        assert_eq!(
            Token {
                kind: Impl,
                pos: Location { col: 1, row: 7 }
            },
            lexer.next_token()
        );

        assert_eq!(
            Token {
                kind: For,
                pos: Location { col: 1, row: 8 }
            },
            lexer.next_token()
        );

        assert_eq!(
            Token {
                kind: While,
                pos: Location { col: 1, row: 9 }
            },
            lexer.next_token()
        );

        assert_eq!(
            Token {
                kind: Extends,
                pos: Location { col: 1, row: 10 }
            },
            lexer.next_token()
        );

        assert_eq!(
            Token {
                kind: New,
                pos: Location { col: 1, row: 11 }
            },
            lexer.next_token()
        );
    }
}
