use once_cell::sync::Lazy;
use regex::Regex;

use super::TokenKind;
use crate::TK;

pub(super) struct Rule {
    pub kind: TokenKind,
    pub matches: fn(&str) -> Option<usize>,
}

impl std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} rule", self.kind)
    }
}

fn match_keyword(input: &str, keyword: &str) -> Option<usize> {
    input.starts_with(keyword).then(|| keyword.len())
}

fn match_regex(input: &str, re: &Regex) -> Option<usize> {
    re.find(input).map(|regex_match| regex_match.end())
}

// match '//' then anything except a new line 0 or more times until a newline is met
static WORD_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^[a-zA-z][a-zA-z0-9]*"#).unwrap());
static POSITION_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^\d+ *, *\d+"#).unwrap()); 
static STRING_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(^")(\\"|\\\\|[^\\"\n])*(")"#).unwrap());
static FLOAT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^((\d+(\.\d+)?)|(\.\d+))"#).unwrap());
static COMMENT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"^(//[^\n]*)"#).unwrap());

// the higher the rule the higher its importance
pub(super) fn get_rules() -> Vec<Rule> {
    vec![
        Rule {
            kind: TK![def],
            matches: |input| match_keyword(input, "define"),
        },
        Rule {
            kind: TK![=],
            matches: |input| match_keyword(input, "="),
        },
        Rule {
            kind: TK![Move],
            matches: |input| match_keyword(input, "Move"),
        },
        Rule {
            kind: TK![Tap],
            matches: |input| match_keyword(input, "Tap"),
        },
        Rule {
            kind: TK![Press],
            matches: |input| match_keyword(input, "Press"),
        },
        Rule {
            kind: TK![Release],
            matches: |input| match_keyword(input, "Release"),
        },
        Rule {
            kind: TK![Sleep],
            matches: |input| match_keyword(input, "Sleep"),
        },
        Rule {
            kind: TK![Type],
            matches: |input| match_keyword(input, "Type"),
        },
        Rule {
            kind: TK![,],
            matches: |input| match_keyword(input, ","),
        },
        Rule {
            kind: TK![EOI],
            matches: |input| match_keyword(input, ";"),
        },
        Rule {
            kind: TK![EOI],
            matches: |input| match_keyword(input, "\n"),
        },
        Rule {
            kind: TK![Word],
            matches: |input| match_regex(input, &WORD_RE),
        },
        Rule {
            kind: TK![Position],
            matches: |input| match_regex(input, &POSITION_RE),
        },
        Rule {
            kind: TK![String],
            matches: |input| match_regex(input, &STRING_RE),
        },
        Rule {
            kind: TK![Float],
            matches: |input| match_regex(input, &FLOAT_RE),
        },
        Rule {
            kind: TK![Comment],
            matches: |input| match_regex(input, &COMMENT_RE),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn comment_match() {
        let input: &str = "// This is a simple comment\n";
        assert_eq!(
            match_regex(input, &COMMENT_RE),
            Some(27)
        )
    }
    #[test]
    fn comment_mismatch() {
        let input: &str = "// This is a simple comment\n";
        assert_eq!(
            match_regex(input, &STRING_RE),
            None
        )
    }
}

