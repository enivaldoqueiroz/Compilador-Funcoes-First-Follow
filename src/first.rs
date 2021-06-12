use super::grammar::Grammar;
use super::production::Production;
use crate::token::{Token, TokenProcessor};
use std::collections::HashSet;

impl Production {
    fn process_variable(
        variable: char,
        grammar: &Grammar,
        firsts: &mut HashSet<Token>,
        production: &Production,
    ) -> bool {
        if let Some(p) = grammar.get_production_by_var(variable) {
            let variable_firsts = Self::fetch_firsts(p, grammar);
            // continue processando a derivação apenas se a variável firsts contiver um épsilon
            let should_continue = variable_firsts.contains(&Token::Epsilon);

            for token in variable_firsts {
                match token {
                    // regra 3.a
                    Token::Epsilon => {
                        production.ends_with_variable(variable) && firsts.insert(token)
                    }
                    _ => firsts.insert(token),
                };
            }

            return should_continue;
        }

        false
    }

    fn fetch_firsts(production: &Production, grammar: &Grammar) -> HashSet<Token> {
        let mut firsts: HashSet<Token> = HashSet::new();
        let processor = TokenProcessor::new(grammar);

        for slice in processor.process_derivation(&production.derivation) {
            for token in slice.tokens {
                let should_continue = match token {
                    Token::Variable(ch) => {
                        Self::process_variable(ch, grammar, &mut firsts, &production)
                    }
                    _ => {
                        firsts.insert(token);
                        false
                    }
                };

                if !should_continue {
                    break;
                }
            }
        }

        firsts
    }
}

impl Grammar {
    pub fn compute_firsts(&mut self) {
        let immut_self = self.clone();

        for p in self.productions_iter_mut() {
            p.set_firsts(Production::fetch_firsts(p, &immut_self))
        }
    }
}


