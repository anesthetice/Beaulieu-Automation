use rules::{Rule, get_rules};

use crate::TK;

use super::token::*;
mod rules;

#[derive(Debug)]
pub struct Lexer<'input> {
    input: &'input str,
    position: usize,
    // for error accuracy
    line: usize,
    // to place an EOI before EOF
    pre_eof: bool,
    eof: bool,
    rules: Vec<Rule>,
}

impl<'input> Lexer<'input> {
    pub(super) fn new(input: &'input str) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            pre_eof: false,
            eof: false,
            rules: get_rules(),
        }
    }

    #[cfg(test)]
    pub(super) fn tokenize(&mut self) -> Vec<Token> {
        self.collect()
    }

    fn next_token(&mut self, input: &str) -> Token {
        self.valid_token(input)
            .unwrap_or_else(|| self.invalid_token(input))
    }

    fn valid_token(&mut self, input: &str) -> Option<Token> {
        let next = input.chars().next().unwrap();
        if next == '\n' {
            self.line += 1
        }
        let (len, kind) = {
            // \n has to be treated seperatly
            if next.is_whitespace() && next != '\n' {
                (
                    input.chars().take_while(|c| c.is_whitespace()).count(),
                    TK![ws],
                )
            } else {
                self.rules
                    .iter()
                    // `max_by_key` returns the last element if multiple rules match,
                    // but we want earlier rules to "win" against later ones
                    .rev()
                    .filter_map(|rule| Some(((rule.matches)(input)?, rule.kind)))
                    .max_by_key(|&(len, _)| len)?
            }
        };

        let start = self.position;
        self.position += len;
        Some(Token {
            kind,
            span: Span {
                start,
                end: start + len,
            },
        })
    }

    fn invalid_token(&mut self, input: &str) -> Token {
        let start = self.position;
        let len = input
            .char_indices()
            .find(|(pos, _)| self.valid_token(&input[*pos..]).is_some())
            .map(|(pos, _)| pos)
            .unwrap_or_else(|| input.len());

        tracing::error!("Unknown token '{}' on line {}", &input[0..len], self.line);
        self.position = start + len;
        Token {
            kind: TK![Error],
            span: Span {
                start,
                end: start + len,
            },
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.input.len() {
            if self.eof {
                return None;
            }
            if self.pre_eof {
                self.eof = true;
                return Some(Token {
                    kind: TK![EOF],
                    span: Span {
                        start: self.position,
                        end: self.position,
                    },
                });
            }

            self.pre_eof = true;
            Some(Token {
                kind: TK![EOI],
                span: Span {
                    start: self.position,
                    end: self.position,
                },
            })
        } else {
            Some(self.next_token(&self.input[self.position..]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, TokenKind};
    use crate::TK;

    #[test]
    fn short() {
        let input: &str = "// Comment abcd\ndefine RESOLUTION = 1920, 1080\nPress LMB\nSleep 4.3";
        let mut lexer = Lexer::new(input);
        let token_kinds: Vec<TokenKind> = lexer
            .tokenize()
            .into_iter()
            .map(|token| token.kind)
            .collect();
        assert_eq!(
            token_kinds,
            vec![
                TK![Comment],
                TK![EOI],
                TK![def],
                TK![ws],
                TK![Word],
                TK![ws],
                TK![=],
                TK![ws],
                TK![Position],
                TK![EOI],
                TK![Press],
                TK![ws],
                TK![Word],
                TK![EOI],
                TK![Sleep],
                TK![ws],
                TK![Float],
                TK![EOI],
                TK![EOF]
            ]
        )
    }

    #[test]
    fn medium() {
        let input: &str = "Bind NR1 {\n  Tap LMB\n  Sleep 0.1\n}\nAwait";
        let mut lexer = Lexer::new(input);
        let token_kinds: Vec<TokenKind> = lexer
            .tokenize()
            .into_iter()
            .map(|token| token.kind)
            .collect();
        assert_eq!(
            token_kinds,
            vec![
                TK![Bind],
                TK![ws],
                TK![Word],
                TK![ws],
                TK![LBrace],
                TK![EOI],
                TK![ws],
                TK![Tap],
                TK![ws],
                TK![Word],
                TK![EOI],
                TK![ws],
                TK![Sleep],
                TK![ws],
                TK![Float],
                TK![EOI],
                TK![RBrace],
                TK![EOI],
                TK![Await],
                TK![EOI],
                TK![EOF],
            ]
        )
    }

    #[test]
    fn long() {
        let input: &str = "// Comment\ndefine RESOLUTION=1920,1080\nMove\nTap\nPress LMB\nRelease LMB\nSleep 4.1\nType \"a simple test\"";
        let mut lexer = Lexer::new(input);
        let token_kinds: Vec<TokenKind> = lexer
            .tokenize()
            .into_iter()
            .map(|token| token.kind)
            .collect();
        assert_eq!(
            token_kinds,
            vec![
                TK![Comment],
                TK![EOI],
                TK![def],
                TK![ws],
                TK![Word],
                TK![=],
                TK![Position],
                TK![EOI],
                TK![Move],
                TK![EOI],
                TK![Tap],
                TK![EOI],
                TK![Press],
                TK![ws],
                TK![Word],
                TK![EOI],
                TK![Release],
                TK![ws],
                TK![Word],
                TK![EOI],
                TK![Sleep],
                TK![ws],
                TK![Float],
                TK![EOI],
                TK![Type],
                TK![ws],
                TK![String],
                TK![EOI],
                TK![EOF]
            ]
        )
    }

    #[test]
    fn misc() {
        let input: &str = "Await nr1\nBind nr2 {\n  Println \"hello world\"\n}\nSleep 0.5 // Comment on same line";
        let mut lexer = Lexer::new(input);
        let token_kinds: Vec<TokenKind> = lexer
            .tokenize()
            .into_iter()
            .map(|token| token.kind)
            .collect();
        assert_eq!(
            token_kinds,
            vec![
                TK![Await],
                TK![ws],
                TK![Word],
                TK![EOI],
                TK![Bind],
                TK![ws],
                TK![Word],
                TK![ws],
                TK![LBrace],
                TK![EOI],
                TK![ws],
                TK![Println],
                TK![ws],
                TK![String],
                TK![EOI],
                TK![RBrace],
                TK![EOI],
                TK![Sleep],
                TK![ws],
                TK![Float],
                TK![ws],
                TK![Comment],
                TK![EOI],
                TK![EOF],
            ],
        )
    }
}
