pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn len(&self) -> usize {
        self.span.end - self.span.start
    }

    pub fn text<'a>(&self, input: &'a str) -> &'a str {
        &input[self.span]
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} - <{}, {}>",
            self.kind, self.span.start, self.span.end
        )
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    // inclusive
    pub start: usize,
    // exclusive
    pub end: usize,
}

impl From<Span> for std::ops::Range<usize> {
    fn from(span: Span) -> Self {
        span.start..span.end
    }
}

impl From<std::ops::Range<usize>> for Span {
    fn from(range: std::ops::Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl std::ops::Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[std::ops::Range::<usize>::from(index)]
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Operators
    Define,
    Eq,

    // Actions
    Move,
    Tap,
    Press,
    Release,
    Sleep,
    Type,
    Await,
    Bind,
    Print,
    Println,
    PrintClipboard,

    // Delimiters
    Whitespace,
    Comma,
    LBrace,
    RBrace,
    EOI, // end of instruction
    EOF, // end of file

    // Multiple characters
    Word,
    Position,
    String,
    Float,
    Comment,

    // Misc.
    Error,
}

#[macro_export]
macro_rules! TK {
    // Operators
    [def] => {$crate::compiler::token::TokenKind::Define};
    [=] => {$crate::compiler::token::TokenKind::Eq};

    // Actions
    [Move] => {$crate::compiler::token::TokenKind::Move};
    [Tap] => {$crate::compiler::token::TokenKind::Tap};
    [Press] => {$crate::compiler::token::TokenKind::Press};
    [Release] => {$crate::compiler::token::TokenKind::Release};
    [Sleep] => {$crate::compiler::token::TokenKind::Sleep};
    [Type] => {$crate::compiler::token::TokenKind::Type};
    [Await] => {$crate::compiler::token::TokenKind::Await};
    [Bind] => {$crate::compiler::token::TokenKind::Bind};
    [Print] => {$crate::compiler::token::TokenKind::Print};
    [Println] => {$crate::compiler::token::TokenKind::Println};
    [PrintClipboard] => {$crate::compiler::token::TokenKind::PrintClipboard};

    // Delimiters
    [ws] => {$crate::compiler::token::TokenKind::Whitespace};
    [,] => {$crate::compiler::token::TokenKind::Comma};
    [LBrace] => {$crate::compiler::token::TokenKind::LBrace};
    [RBrace] => {$crate::compiler::token::TokenKind::RBrace};
    [EOI] => {$crate::compiler::token::TokenKind::EOI};
    [EOF] => {$crate::compiler::token::TokenKind::EOF};

    // Multiple characters
    [Word] => {$crate::compiler::token::TokenKind::Word};
    [Position] => {$crate::compiler::token::TokenKind::Position};
    [String] => {$crate::compiler::token::TokenKind::String};
    [Float] => {$crate::compiler::token::TokenKind::Float};
    [Comment] => {$crate::compiler::token::TokenKind::Comment};

    // Misc
    [Error] => {$crate::compiler::token::TokenKind::Error};
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
