use std::collections::{hash_map::Entry, HashMap};

use crate::tokens::{Token, Type};

pub struct Memoria {
    current_scope: u8,
    //                name  scope
    memory: HashMap<(String, u8), Token>,
}

impl Memoria {
    pub fn new() -> Memoria {
        let memory = HashMap::new();
        return Self {
            memory,
            current_scope: 0,
        };
    }

    pub fn scope_forward(&mut self) {
        self.current_scope += 1;
    }

    pub fn scope_backwards(&mut self) {
        self.current_scope -= 1;
    }

    pub fn create(&mut self, name: String, tipo: Type) -> Option<()> {
        match self.memory.entry((name, self.current_scope)) {
            Entry::Occupied(_) => None,
            Entry::Vacant(entry) => {
                let initial_data = match tipo {
                    // default values straight from PSeInt
                    Type::Caracter => Token::String("".to_string()),
                    Type::Entero => Token::Numero(0 as f32, true),
                    Type::Real => Token::Numero(0 as f32, false),
                    Type::Logico => Token::Boolean(false),
                    Type::None => return None,
                };

                entry.insert(initial_data);
                Some(())
            }
        }
    }

    pub fn get(&self, name: String) -> Option<&Token> {
        let data = self.memory.get(&(name, self.current_scope));

        data
    }

    pub fn set(&mut self, name: String, value: Token) -> Option<()> {
        match self.memory.entry((name, self.current_scope)) {
            Entry::Occupied(mut entry) => {
                if !(std::mem::discriminant(entry.get()) == std::mem::discriminant(&value)) {
                    return None;
                }
                entry.insert(value);
                Some(())
            }
            Entry::Vacant(entry) => None,
        }
    }
}
