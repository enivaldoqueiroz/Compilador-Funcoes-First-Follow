use crate::token::{Token};
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Production {
    pub variable: char,
    pub derivation: String,
    pub firsts: HashSet<Token>,
    pub follows: HashSet<Token>,
}

impl Production {
    pub fn new(variable: char, derivation: String) -> Result<Production, String> {
        if variable.is_uppercase() {
            return Ok(Production {
                variable,
                derivation: derivation.replace(' ', ""),
                firsts: HashSet::new(),
                follows: HashSet::new(),
            });
        }

        Err(format!(
            "the variable '{}' must be a uppercase character.",
            variable
        ))
    }

    pub fn ends_with_variable(&self, variable: char) -> bool {
        self.derivation.ends_with(variable)
    }
  
    pub fn set_firsts(&mut self, firsts: HashSet<Token>) {
        self.firsts = firsts;
    }
   
    fn token_hashset_as_string(hashset: &HashSet<Token>) -> String {
        let mut result = String::new();

        hashset.iter().enumerate().for_each(|(i, v)| {
            result += if i > 0 { "," } else { "" };
            result += &v.to_string()
        });

        result
    }
}

impl fmt::Display for Production {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "|{:^15}|{:^15}|{:^15}|{:^15}|",
            self.variable,
            self.derivation,
            Self::token_hashset_as_string(&self.firsts),
            Self::token_hashset_as_string(&self.follows)
        )
    }
}
