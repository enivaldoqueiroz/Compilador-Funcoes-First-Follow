# Compilador - Implementação das funcoes First e Follow

### Função principal da implementação

      mod first;
      mod follow;
      mod grammar;
      mod production;
      mod token;

      use grammar::Grammar;
      use production::Production;


      fn main() {
          let mut grammar = Grammar {
              variables: vec!['E', 'Z', 'T', 'Y', 'X'],
              terminals: vec![
                  "+".to_string(),
                  "*".to_string(),
                  "(".to_string(),
                  "id".to_string(),
                  ")".to_string(),
              ],
              productions: vec![],
              initial_symbol: 'S',
          };

          grammar.add_production(Production::new('E', "TZ".to_string()).unwrap());
          grammar.add_production(Production::new('Z', "+TZ | ε".to_string()).unwrap());
          grammar.add_production(Production::new('T', "XY".to_string()).unwrap());
          grammar.add_production(Production::new('Y', "*XY | ε".to_string()).unwrap());
          grammar.add_production(Production::new('X', "(E) | id".to_string()).unwrap());

          grammar.compute_firsts();
          grammar.compute_follows();
          println!("{}", grammar);
      }


### Saida dos valores apos processamento da função First
![](https://github.com/enivaldoqueiroz/Compilador-Funcoes-First-Follow/blob/main/img/img001.png)

### Saida dos valores apos processamento da função Follow
![](https://github.com/enivaldoqueiroz/Compilador-Funcoes-First-Follow/blob/main/img/img002.png)
