use core::fmt;

use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, EnumIter, PartialEq, Clone)]
pub enum Token {
    None,

    Igual, // = y <-
    Comparacion,

    MenorA,
    MenorOIgual,
    MayorA,
    MayorOIgual,
    Suma,
    Resta,
    Multiplicacion,
    Division,

    Numero(f32, bool),
    String(String),
    Boolean(bool),
    EOL, // ';'

    // Identificador es global, tiene que ser transformado a Variable o Instrucción
    Identificador(String),
    Variable(String),
    Tipo(Type),
    Instruccion(Keyword),
    // El grupo o grouping serian los (), el contenido dentro de estos son mas tokens
    Grupo(Vec<Token>),
    AbrirParentesis,
    CerrarParentesis,
}

#[derive(Default, Debug, EnumIter, PartialEq, Clone)]
pub enum Keyword {
    // default trait needed for EnumIter
    #[default]
    None,

    Algoritmo,
    FinAlgoritmo,
    Escribir,
    Como,
    Definir,
}

#[derive(Default, Debug, EnumIter, PartialEq, Clone, Copy)]
pub enum Type {
    // default trait needed for EnumIter
    #[default]
    None,

    Caracter,
    Real,
    Logico,
    Entero,
}

/// Converts to keyword if it is one, or returns a variable
pub fn convert_to_keyword(text: String) -> Token {
    for keyword in Keyword::iter() {
        if text.to_lowercase() == keyword.to_string().to_lowercase() {
            return Token::Instruccion(keyword);
        }
    }

    for var_type in Type::iter() {
        if text.to_lowercase() == var_type.to_string().to_lowercase() {
            return Token::Tipo(var_type);
        }
    }

    return Token::Identificador(text);
}

pub fn convert_to_type(token: Token) -> Option<Type> {
    match token {
        Token::Tipo(val) => return Some(val),
        _ => return None,
    }
}

impl Token {
    pub fn get_as_string(self) -> String {
        match self {
            Token::Numero(num, rounded) => {
                if rounded {
                    return f32::trunc(num).to_string();
                } else {
                    return num.to_string();
                }
            }
            Token::String(string) => return string,
            Token::Boolean(bool) => return bool.to_string(),
            _ => "".to_string(),
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = format!("{:?}", self);
        let split = name.split("(").collect::<Vec<&str>>();

        write!(f, "{}", split.first().unwrap().trim_matches('"'))
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = format!("{:?}", self);
        let split = name.split("(").collect::<Vec<&str>>();

        write!(f, "{}", split.first().unwrap().trim_matches('"'))
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = format!("{:?}", self);
        let split = name.split("(").collect::<Vec<&str>>();

        write!(f, "{}", split.first().unwrap().trim_matches('"'))
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn igual() {
        let igual = Token::Igual.to_string();
        assert_eq!(igual, "Igual".to_string());
    }

    #[test]
    fn identificador() {
        let identificador = Token::Identificador("Definir".to_string()).to_string();
        assert_eq!(identificador, "Identificador".to_string());
    }
}

// enum Keywords {
//     Algoritmo,
//     FinAlgoritmo,
//     Escribir,
//     Definir,
// }

// enum Types {
//     Caracter,
//     Real,
//     Logico,
//     Entero,
// }
