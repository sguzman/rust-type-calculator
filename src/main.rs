use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Type {
    Int,
    Float,
    Bool,
}

#[derive(Debug, PartialEq, Eq)]
enum Error {
    TypeError,
    UndeclaredFunction,
    UndeclaredVariable,
}

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

struct Environment {
    variables: HashMap<String, Type>,
    functions: HashMap<String, (Type, Vec<Type>)>,
}

impl Environment {
    fn new() -> Self {
        let mut functions = HashMap::new();
        functions.insert("add".to_string(), (Type::Int, vec![Type::Int]));
        functions.insert("sub".to_string(), (Type::Int, vec![Type::Int]));
        functions.insert("mul".to_string(), (Type::Int, vec![Type::Int]));
        functions.insert("div".to_string(), (Type::Float, vec![Type::Int]));

        // Now and Bool function is added
        functions.insert("and".to_string(), (Type::Bool, vec![Type::Bool]));

        Environment {
            variables: HashMap::new(),
            functions,
        }
    }

    fn declare_variable(&mut self, name: &str, var_type: Type) {
        self.variables.insert(name.to_string(), var_type);
    }

    fn call_function(&self, name: &str, args: &[Type]) -> Result<Type, Error> {
        if let Some((return_type, input_types)) = self.functions.get(name) {
            if input_types.len() != args.len() {
                return Err(Error::TypeError);
            }

            for (i, arg) in args.iter().enumerate() {
                if *arg != input_types[i] {
                    return Err(Error::TypeError);
                }
            }

            Ok(return_type.clone())
        } else {
            Err(Error::UndeclaredFunction)
        }
    }

    fn declare_function(&mut self, name: &str, input_type: Type, output_type: Type) {
        self.functions
            .insert(name.to_string(), (output_type, vec![input_type]));
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

fn process_input(input: &str, env: &mut Environment) -> Result<String, Error> {
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

fn main() {
    let mut env = Environment::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" || input == "exit" {
            break;
        }

        match process_input(input, &mut env) {
            Ok(output) => {
                if !output.is_empty() {
                    println!("{}", output);
                }
            }
            Err(err) => println!("Error: {}", err),
        }
    }
}
