use std::fmt;

#[derive(Debug, Clone)]
pub enum PossibleErrors {
    MissingArguments,
    MissingTypeOrUnvalidType,
    WrongType,
    SyntaxError,
    InvalidInstruction,
    IncompleteAssignment,
    VariableNotFound(String),
}

#[derive(Debug, Clone)]
pub struct CodeError {
    pub error: PossibleErrors,
}

impl fmt::Display for CodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.error {
            PossibleErrors::MissingArguments => write!(f, "ERROR 53: Faltan parámetros."),
            PossibleErrors::WrongType => write!(f, "ERROR 125: No coinciden los tipos."),
            PossibleErrors::MissingTypeOrUnvalidType => {
                write!(f, "ERROR 46: Falta tipo de dato o tipo no válido.")
            }
            PossibleErrors::SyntaxError => write!(f, "ERROR -1: Error de sintaxis."),
            PossibleErrors::InvalidInstruction => write!(f, "ERROR 106: Instrucción no válida."),
            PossibleErrors::IncompleteAssignment => write!(f, "ERROR 89: Asignación incompleta."),
            PossibleErrors::VariableNotFound(var_name) => {
                write!(f, "ERROR 215: Variable no inicializada ({var_name})")
            }
        }
    }
}
