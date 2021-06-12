use crate::grammar::Grammar;

pub const EPSILON: &str = "ε";
pub const DOLLAR_SIGN: &str = "$";

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub enum Token {
    Variable(char),
    Terminal(String),
    Epsilon,
    DollarSign,
    Placeholder(char),
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Variable(ch) => {
                let mut s = String::new();
                s.push(ch.to_owned());
                s
            }
            Token::Terminal(s) => s.to_owned(),
            Token::Epsilon => EPSILON.to_string(),
            Token::DollarSign => DOLLAR_SIGN.to_string(),
            Token::Placeholder(ch) => {
                let mut s = String::new();
                s.push(ch.to_owned());
                s
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct DerivationTokenSlice {
    pub tokens: Vec<Token>,
}

impl DerivationTokenSlice {
    fn new(tokens: Vec<Token>) -> DerivationTokenSlice {
        DerivationTokenSlice { tokens }
    }
}

#[derive(Debug)]
pub struct TokenProcessor<'a> {
    grammar: &'a Grammar,
}

impl TokenProcessor<'_> {
    pub fn new(grammar: &Grammar) -> TokenProcessor {
        TokenProcessor { grammar }
    }

    pub fn process_derivation(&self, derivation: &str) -> Vec<DerivationTokenSlice> {
        derivation
            .split("|")
            .map(|slice| DerivationTokenSlice::new(self.get_token_vec(slice.trim())))
            .collect()
    }

    fn get_token_vec(&self, input: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let mut buffer = String::new();

        for ch in input.chars() {
            buffer.push(ch);
            if buffer.as_str().eq(EPSILON) {
                tokens.push(Token::Epsilon);
                buffer.clear()
            } else if self.grammar.is_variable(&ch) {
                tokens.push(Token::Variable(ch));
                buffer.clear()
            } else if self.grammar.is_terminal(&buffer) {
                tokens.push(Token::Terminal(buffer.clone()));
                buffer.clear()
            }
        }

        tokens
    }
}

