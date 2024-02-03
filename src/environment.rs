use std::collections::HashMap;

use crate::{built_in_functions, values::RuntimeValue};

#[derive(Clone, Debug)]
pub struct Environment {
    pub parent: Box<Option<Self>>,
    pub variables: HashMap<String, RuntimeValue>,
    pub constants: HashMap<String, RuntimeValue>,
}

impl Environment {
    pub fn new(parent: Option<Self>) -> Self {
        let variables = HashMap::new();
        let mut constants = HashMap::new();

        // Global constants
        constants.insert("null".to_string(), RuntimeValue::Null);
        constants.insert("true".to_string(), RuntimeValue::Boolean(true));
        constants.insert("false".to_string(), RuntimeValue::Boolean(false));
        constants.insert(
            "print".to_string(),
            RuntimeValue::BuiltInFunction(built_in_functions::print, vec![]),
        );
        constants.insert(
            "input".to_string(),
            RuntimeValue::BuiltInFunction(built_in_functions::input, vec![]),
        );
        constants.insert("tup".to_string(), RuntimeValue::BuiltInFunction(built_in_functions::tup, vec![]));

        Environment {
            parent: Box::new(parent),
            variables,
            constants,
        }
    }

    pub fn lookup(&mut self, variable_name: String) -> &RuntimeValue {
        if self.variables.contains_key(&variable_name) {
            &self.variables[&variable_name]
        } else if self.constants.contains_key(&variable_name) {
            &self.constants[&variable_name]
        } else if let Some(parent) = &mut *self.parent {
            parent.lookup(variable_name)
        } else {
            panic!("Variable '{}' undefined.", variable_name)
        }
    }

    pub fn declare_variable(&mut self, variable_name: String, variable_value: RuntimeValue) {
        if variable_name.chars().all(char::is_uppercase) {
            self.constants.insert(variable_name, variable_value);
        } else {
            self.variables.insert(variable_name, variable_value);
        }
    }

    pub fn assign(&mut self, variable_name: String, variable_value: RuntimeValue) {
        if let std::collections::hash_map::Entry::Occupied(mut e) =
            self.variables.entry(variable_name.clone())
        {
            e.insert(variable_value);
        } else if let Some(parent) = &mut *self.parent {
            parent.assign(variable_name, variable_value);
        } else {
            panic!("Variable '{}' undefined", variable_name);
        }
    }
}
