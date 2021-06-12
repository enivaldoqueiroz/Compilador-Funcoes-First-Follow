use crate::grammar::Grammar;
use crate::production::Production;
use crate::token::{Token, TokenProcessor};
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Item {
    variable: char,
    token: Token,
}

impl Item {
    fn new(variable: char, token: Token) -> Item {
        Item { variable, token }
    }
}

impl Production {
    fn process_follow_variable(
        variable: char,
        grammar: &Grammar,
        next_token: Option<&Token>,
    ) -> Vec<Item> {
        let mut tokens: Vec<Item> = vec![];

        if let Some(token) = next_token {
            match token {
                Token::Variable(ch) => {
                    for token in Self::process_follow_rule2(ch.clone(), grammar) {
                        tokens.push(Item::new(variable, token))
                    }
                }
                Token::Terminal(_) => tokens.push(Item::new(variable, token.clone())),
                _ => (),
            }
        }

        return tokens;
    }

    fn process_follow_rule2(variable: char, grammar: &Grammar) -> HashSet<Token> {
        let prod = grammar.get_production_by_var(variable).unwrap();

        prod.firsts
            .iter()
            .filter_map(|t| {
                if !t.eq(&Token::Epsilon) {
                    return Some(t.clone());
                }

                None
            })
            .collect()
    }

    fn process_follow_rule3(next_token: Option<&Token>, grammar: &Grammar) -> bool {
        if let Some(token) = next_token {
            match token {
                Token::Variable(ch) => {
                    let prod = grammar.get_production_by_var(ch.clone()).unwrap();

                    return prod.firsts.contains(&Token::Epsilon);
                }
                _ => (),
            }
        }

        false
    }

    fn fetch_follows(
        production: &Production,
        grammar: &Grammar,
        token_processor: &TokenProcessor,
    ) -> Vec<Item> {
        let mut tokens: Vec<Item> = vec![];

        // Regra 1
        if grammar.production_is_initial(production) {
            tokens.push(Item::new(production.variable, Token::DollarSign));
        }

        let derivations = token_processor.process_derivation(&production.derivation);

        for slice in derivations {
            let slice_size = slice.tokens.len();

            for (index, token) in slice.tokens.iter().enumerate() {
                match token {
                    Token::Variable(ch) => {
                        let var_tokens = Self::process_follow_variable(
                            ch.clone(),
                            grammar,
                            slice.tokens.get(index + 1),
                        );
                        tokens = [tokens, var_tokens].concat();

                        // check rule 3
                        if index + 1 == slice_size {
                            tokens.push(Item::new(
                                ch.clone(),
                                Token::Placeholder(production.variable),
                            ))
                        }

                        // verifique a regra 3, parte 2
                        if index + 2 == slice_size {
                            if Self::process_follow_rule3(slice.tokens.get(index + 1), grammar) {
                                tokens.push(Item::new(
                                    ch.clone(),
                                    Token::Placeholder(production.variable),
                                ))
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        tokens
    }
}

impl Grammar {
    pub fn compute_follows(&mut self) {
        self.compute_firsts();

        let immut_self = self.clone();
        let processor = TokenProcessor::new(&immut_self);
        let tokens: Vec<Item> = self
            .productions_iter_mut()
            .map(|p| Production::fetch_follows(p, &immut_self, &processor))
            .flatten()
            .collect();

        tokens
            .iter()
            .filter(|item| match item.token {
                Token::Placeholder(_) => false,
                _ => true,
            })
            .for_each(|item| {
                self.get_mut_production_by_var(item.variable)
                    .unwrap()
                    .follows
                    .insert(item.token.clone());
            });

        // marcadores de posição
        tokens
            .iter()
            .filter(|item| match item.token {
                Token::Placeholder(_) => true,
                _ => false,
            })
            .for_each(|item| {
                let mut cl = self.clone();
                let item_prod = self.get_mut_production_by_var(item.variable).unwrap();

                match item.token {
                    Token::Placeholder(ch) => {
                        let placeholder_follows = cl.get_mut_production_by_var(ch).unwrap();
                        for f in placeholder_follows.follows.iter() {
                            item_prod.follows.insert(f.clone());
                        }
                    }
                    _ => (),
                }
            });
    }
}
