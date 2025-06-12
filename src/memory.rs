use std::collections::{hash_map::Entry, HashMap};
use std::fmt;

use crate::{
    error::{Code, PossibleErrors},
    tokens::{Token, Type},
};

pub struct Memoria {
    memory: HashMap<String, Token>,
}

impl Memoria {
    pub fn new() -> Memoria {
        let memory = HashMap::new();
        Self { memory }
    }

    pub fn create(&mut self, name: String, tipo: Type) -> Option<()> {
        match self.memory.entry(name) {
            Entry::Occupied(_) => None,
            Entry::Vacant(entry) => {
                let initial_data = match tipo {
                    // default values straight from PSeInt
                    Type::Caracter => Token::String(String::new()),
                    Type::Entero => Token::Numero(0.0, true),
                    Type::Real => Token::Numero(0.0, false),
                    Type::Logico => Token::Boolean(false),
                    Type::None => return None,
                };

                entry.insert(initial_data);
                Some(())
            }
        }
    }

    pub fn get_type(&self, name: String) -> Option<Type> {
        let data = self.memory.get(&name)?;

        match data {
            Token::Numero(_, rounded) => {
                if *rounded {
                    return Some(Type::Entero);
                }
                Some(Type::Real)
            }
            Token::String(_) => Some(Type::Caracter),
            Token::Boolean(_) => Some(Type::Logico),
            _ => None,
        }
    }

    pub fn get(&self, name: String) -> Option<&Token> {
        let data = self.memory.get(&name);

        data
    }

    pub fn set(&mut self, name: String, value: Token) -> Result<(), Code> {
        match self.memory.entry(name) {
            Entry::Occupied(mut entry) => {
                if !(std::mem::discriminant(entry.get()) == std::mem::discriminant(&value)) {
                    return Err(Code {
                        error: crate::error::PossibleErrors::WrongType,
                    });
                }
                entry.insert(value);
                Ok(())
            }
            Entry::Vacant(entry) => {
                return Err(Code {
                    error: PossibleErrors::VariableNotFound(entry.key().clone()),
                });
            }
        }
    }
}

impl fmt::Debug for Memoria {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (name, token) in &self.memory {
            writeln!(f, "{} -> {:?}", name, token)?;
        }
        Ok(())
    }
}
