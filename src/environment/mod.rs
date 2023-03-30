use crate::types::type_enum::Type;
use crate::types::type_error::Error;
use std::collections::HashMap;

pub struct Environment {
    pub variables: HashMap<String, Type>,
    pub functions: HashMap<String, (Type, Vec<Type>)>,
}

impl Environment {
    pub fn new() -> Self {
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

    pub fn declare_variable(&mut self, name: &str, var_type: Type) {
        self.variables.insert(name.to_string(), var_type);
    }

    pub fn call_function(&self, name: &str, args: &[Type]) -> Result<Type, Error> {
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

    pub fn declare_function(&mut self, name: &str, input_type: Type, output_type: Type) {
        self.functions
            .insert(name.to_string(), (output_type, vec![input_type]));
    }
}
