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
    eof: bool,
    rules: Vec<Rule>
}

impl<'input> Lexer<'input> {
    fn new(input: &'input str) -> Self {
        Self {
            input,
            position: 0,
            line: 1,
            eof: false,
            rules: get_rules()
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        self.collect()
    }

    fn next_token(&mut self, input: &str) -> Token {
        self.valid_token(input).unwrap_or_else(|| self.invalid_token(input))
    }
    
    fn valid_token(&mut self, input: &str) -> Option<Token> {
        let next = input.chars().next().unwrap();
        if next == '\n' {self.line += 1}
        let (len, kind) = {
            // \n has to be treated seperatly
            if next.is_whitespace() && next != '\n' {
                (input.chars().take_while(|c| c.is_whitespace()).count(), TK![ws])
            } 
            else {
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
        Some(Token{kind, span: Span {start, end: start + len}})
    }

    fn invalid_token(&mut self, input: &str) -> Token {
        let start = self.position;
        let len = input
            .char_indices()
            .find(|(pos, _)| self.valid_token(&input[*pos..]).is_some())
            .map(|(pos, _)| pos)
            .unwrap_or_else(|| input.len());

        self.position = start + len;
        Token{kind: TK![Error], span: Span {start, end: start + len}}
    }
    
    
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position as usize >= self.input.len() {
            if self.eof {
                return None;
            }
            self.eof = true;
            Some(Token {
                kind: TK![EOF],
                span: Span {
                    start: self.position,
                    end:   self.position,
                },
            })
        } else {
            Some(self.next_token(&self.input[self.position as usize..]))
        }
    }
}

pub fn test() {
    let input: &str = "// Simple comment\ndefine RESOLUTION = 1920, 1080\nMove 1910, 1923\n";
    let mut lexer = Lexer::new(input);
    let tokens: Vec<Token> = lexer.tokenize();

    println!("{:#?}", lexer);
    println!("{:#?}", tokens);
}