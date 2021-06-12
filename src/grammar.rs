use super::production::Production;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grammar {
    pub variables: Vec<char>,
    pub terminals: Vec<String>,
    pub productions: Vec<Production>,
    pub initial_symbol: char,
}

impl Grammar {
    pub fn production_is_initial(&self, production: &Production) -> bool {
        self.initial_symbol.eq(&production.variable)
    }

    pub fn is_variable(&self, ch: &char) -> bool {
        self.variables.contains(ch)
    }

    pub fn is_terminal(&self, value: &String) -> bool {
        self.terminals.contains(value)
    }
 
    pub fn add_production(&mut self, production: Production) {
        if !self.productions.contains(&production) {
            self.productions.push(production);
        }
    }

    pub fn get_production_by_var(&self, variable: char) -> Option<&Production> {
        self.productions.iter().find(|p| p.variable == variable)
    }

    pub fn get_mut_production_by_var(&mut self, variable: char) -> Option<&mut Production> {
        self.productions.iter_mut().find(|p| p.variable == variable)
    }

    pub fn productions_iter_mut(&mut self) -> std::slice::IterMut<'_, Production> {
        self.productions.iter_mut()
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let divider: &str = &format!("*{:-<15}*{:-<15}*{:-<15}*{:-<15}*\n", "", "", "", "");

        f.write_str(divider)?;
        writeln!(
            f,
            "|{:^15}|{:^15}|{:^15}|{:^15}|",
            "Variavel", "Derivacao", "Firsts", "Follows"
        )?;
        f.write_str(divider)?;

        for production in self.productions.iter() {
            writeln!(f, "{}", production)?;
        }

        f.write_str(divider)
    }
}
