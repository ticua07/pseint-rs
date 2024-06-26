use std::collections::{hash_map::Entry, HashMap};

use crate::{
    error::{CodeError, PossibleErrors},
    tokens::{Token, Type},
};

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

    pub fn get_type(&self, name: String) -> Option<Type> {
        let data = self.memory.get(&(name, self.current_scope))?;

        match data {
            Token::Numero(_, rounded) => {
                if *rounded {
                    return Some(Type::Entero);
                }
                return Some(Type::Real);
            }
            Token::String(_) => return Some(Type::Caracter),
            Token::Boolean(_) => return Some(Type::Logico),
            _ => None,
        }
    }

    pub fn get(&self, name: String) -> Option<&Token> {
        let data = self.memory.get(&(name, self.current_scope));

        data
    }

    pub fn set(&mut self, name: String, value: Token) -> Result<(), CodeError> {
        match self.memory.entry((name, self.current_scope)) {
            Entry::Occupied(mut entry) => {
                if !(std::mem::discriminant(entry.get()) == std::mem::discriminant(&value)) {
                    return Err(CodeError {
                        error: crate::error::PossibleErrors::WrongType,
                    });
                }
                entry.insert(value);
                Ok(())
            }
            Entry::Vacant(entry) => {
                return Err(CodeError {
                    error: PossibleErrors::VariableNotFound(entry.key().0.clone()),
                });
            }
        }
    }
}
