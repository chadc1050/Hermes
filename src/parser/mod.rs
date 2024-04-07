use self::lexer::Lexer;

mod reader;

mod lexer;

mod token;

/// Parses source code to AST based on [ECMAScript Lexical Grammar](https://262.ecma-international.org/#sec-intro).
pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn init(source: &str) -> Self {
        Parser { lexer: Lexer::init(source) }
    }

    pub fn parse(&self) {
        let tokens = self.lexer.tokenize();

        for token in tokens {}
    }
}

enum StatementType {
    Let,
    Var,
    Const,
}
