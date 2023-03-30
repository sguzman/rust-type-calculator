pub mod environment;
pub mod types;

use crate::types::type_enum::Type;
use crate::types::type_error::Error;
pub use environment::Environment;

use std::fmt;
use std::io::{self, Write};
use std::str::FromStr;

impl FromStr for Type {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Int" => Ok(Type::Int),
            "Float" => Ok(Type::Float),
            "Bool" => Ok(Type::Bool),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TypeError => write!(f, "Type Error"),
            Error::UndeclaredFunction => write!(f, "Undeclared Function"),
            Error::UndeclaredVariable => write!(f, "Undeclared Variable"),
        }
    }
}

fn call_function(input: &[&str], env: &mut Environment) -> Result<String, Error> {
    if input.len() < 1 {
        return Err(Error::TypeError);
    }

    let func_name = input[0];
    let args = &input[1..];

    let mut converted_args = Vec::new();
    for arg in args {
        if let Ok(var_type) = arg.parse::<Type>() {
            converted_args.push(var_type);
        } else if let Some(var_type) = env.variables.get(&arg.to_string()) {
            converted_args.push(var_type.clone());
        } else {
            return Err(Error::UndeclaredVariable);
        }
    }

    match env.call_function(func_name, &converted_args) {
        Ok(return_type) => Ok(format!(
            "Called function {} with return type {:?}",
            func_name, return_type
        )),
        Err(err) => Err(err),
    }
}

fn declare_variable(input: &[&str], env: &mut Environment) -> Result<String, Error> {
    if input.len() != 2 {
        return Err(Error::TypeError);
    }

    let var_name = input[0];
    let var_type = match input[1] {
        "Int" => Type::Int,
        "Float" => Type::Float,
        "Bool" => Type::Bool,
        _ => return Err(Error::TypeError),
    };
    env.declare_variable(var_name, var_type);
    Ok(format!("{} :: {:?}", var_name, var_type))
}

fn declare_function(input: &[&str], env: &mut Environment) -> Result<String, Error> {
    if input.len() != 3 {
        return Err(Error::TypeError);
    }

    let func_name = input[0];
    let input_type = match input[1] {
        "Int" => Type::Int,
        "Float" => Type::Float,
        "Bool" => Type::Bool,
        _ => return Err(Error::TypeError),
    };
    let output_type = match input[2] {
        "Int" => Type::Int,
        "Float" => Type::Float,
        "Bool" => Type::Bool,
        _ => return Err(Error::TypeError),
    };

    env.declare_function(func_name, input_type, output_type);
    Ok(format!(
        "{} :: {:#?} -> {:?}",
        func_name, input_type, output_type
    ))
}

fn show_declaration(input: &[&str], env: &Environment) -> Result<String, Error> {
    if input.len() != 1 {
        return Err(Error::TypeError);
    }

    let name = input[0];
    if let Some(var_type) = env.variables.get(name) {
        Ok(format!("{} :: {:?}", name, var_type))
    } else if let Some((output_type, input_types)) = env.functions.get(name) {
        let input_types_str = input_types
            .iter()
            .map(|t| format!("{:?}", t))
            .collect::<Vec<String>>()
            .join(" -> ");
        Ok(format!(
            "{} :: {} -> {:?}",
            name, input_types_str, output_type
        ))
    } else {
        Err(Error::UndeclaredVariable)
    }
}

pub fn process_input(input: &str, env: &mut Environment) -> Result<String, Error> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        return Ok(String::new());
    }

    match tokens[0] {
        "declare_var" => declare_variable(&tokens[1..], env),
        "declare_func" => declare_function(&tokens[1..], env),
        "call" => call_function(&tokens[1..], env),
        "show" => show_declaration(&tokens[1..], &*env),
        _ => Err(Error::TypeError),
    }
}
