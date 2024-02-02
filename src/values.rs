use crate::parser::ASTNode;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    // Primitive datatypes
    Null,
    Integer(i64),
    Real(f64),
    Boolean(bool),
    
    Tuple(Vec<RuntimeValue>),

    // datatypes with methods
    String(
        String,
        HashMap<String, fn(Vec<RuntimeValue>) -> RuntimeValue>,
    ),
    Array(
        Vec<RuntimeValue>,
        HashMap<String, fn(Vec<RuntimeValue>) -> RuntimeValue>,
    ),
    Object(
        HashMap<String, RuntimeValue>,
        HashMap<String, fn(Vec<RuntimeValue>) -> RuntimeValue>,
    ),

    // datatypes the programmer can't access
    BuiltInFunction(fn(Vec<RuntimeValue>) -> RuntimeValue, Vec<RuntimeValue>),
    Method(fn(Vec<RuntimeValue>) -> RuntimeValue, Vec<RuntimeValue>),
    Function(Vec<ASTNode>, ASTNode),
}

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeValue::Integer(i) => write!(f, "{}", i),
            RuntimeValue::Real(r) => write!(f, "{}", r),
            RuntimeValue::Boolean(b) => write!(f, "{}", b),
            RuntimeValue::String(s, _) => write!(f, "{}", s),
            RuntimeValue::Array(a, _) => write!(f, "{:?}", a),
            RuntimeValue::Null => write!(f, "null"),
            RuntimeValue::Object(o, _) => write!(f, "{:?}", o),
            RuntimeValue::BuiltInFunction(c, _) => write!(f, "{:?}", c),
            RuntimeValue::Tuple(t) => write!(f, "{:?}", t),
            RuntimeValue::Function(p, b) => write!(f, "({:?}) {{{:?}}}", p, b),
            RuntimeValue::Method(_, _) => todo!(),
        }
    }
}

impl RuntimeValue {
    // Function to make and add methods to array
    pub fn array(values: Vec<RuntimeValue>) -> Self {
        let mut methods = HashMap::new();
        methods.insert(
            "length".to_string(),
            array_length as fn(Vec<RuntimeValue>) -> RuntimeValue,
        );

        RuntimeValue::Array(values, methods)
    }

    // Function to make and add methods to string
    pub fn string(values: String) -> Self {
        let mut methods = HashMap::new();
        methods.insert(
            "length".to_string(),
            string_length as fn(Vec<RuntimeValue>) -> RuntimeValue,
        );

        RuntimeValue::String(values, methods)
    }

    // Function to make and add methods to object
    pub fn object(values: HashMap<String, RuntimeValue>) -> Self {
        let mut methods = HashMap::new();
        methods.insert(
            "length".to_string(),
            object_length as fn(Vec<RuntimeValue>) -> RuntimeValue,
        );

        RuntimeValue::Object(values, methods)
    }
}

fn array_length(array: Vec<RuntimeValue>) -> RuntimeValue {
    // .length() method for array

    match &array[0] {
        RuntimeValue::Array(a, _) => RuntimeValue::Integer(a.len() as i64),
        _ => RuntimeValue::Null,
    }
}

fn string_length(string: Vec<RuntimeValue>) -> RuntimeValue {
    // .length() method for string

    match &string[0] {
        RuntimeValue::String(s, _) => RuntimeValue::Integer(s.len() as i64),
        _ => RuntimeValue::Null,
    }
}

fn object_length(values: Vec<RuntimeValue>) -> RuntimeValue {
    // .length() method for objects

    match &values[0] {
        RuntimeValue::Object(o, _) => RuntimeValue::Integer(o.len() as i64),
        _ => RuntimeValue::Null,
    }
}
