use once_cell::sync::Lazy;
use regex::Regex;

use crate::TK;

use super::TokenKind;



pub(super) struct Rule {
    kind: TokenKind,
    matches: fn(&str) -> Option<usize>
}

fn match_keyword(input: &str, keyword: &str) -> Option<usize> {
    input.starts_with(keyword).then(|| keyword.len())
}

// match '//' then anything except a new line 0 or more times and a newline at the end
static COMMENT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^(//[^\n]*\n)"#).unwrap());
static STRING_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^[a-zA-z][a-zA-z0-9]*"#).unwrap());
static NUMBER_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"((\d+(\.\d+)?)|(\.\d+))"#).unwrap());

// the higher the rule the higher its importance
fn get_rules() -> Vec<Rule> {
    vec![
        Rule {
            kind: TK![def],
            matches: |input| match_keyword(input, "define")
        },
    ]
}