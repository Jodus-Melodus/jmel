use std::{collections::HashMap, fmt};

use crate::parser::ASTNode;

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    // Primitive datatypes
    Integer(i64),
    Real(f64),
    Boolean(bool),
    String(String),
    
    Array(Vec<RuntimeValue>),
    Null,
    Object(HashMap<String, RuntimeValue>),
    BuiltInFunction(fn(Vec<RuntimeValue>) -> RuntimeValue, Vec<RuntimeValue>),
    Function(Vec<ASTNode>, ASTNode),
    Tuple(Vec<RuntimeValue>),
}

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeValue::Integer(i) => write!(f, "{}", i),
            RuntimeValue::Real(r) => write!(f, "{}", r),
            RuntimeValue::Boolean(b) => write!(f, "{}", b),
            RuntimeValue::String(s) => write!(f, "{}", s),
            RuntimeValue::Array(a) => write!(f, "{:?}", a),
            RuntimeValue::Null => write!(f, "null"),
            RuntimeValue::Object(o) => write!(f, "{:?}", o),
            RuntimeValue::BuiltInFunction(c, _) => write!(f, "{:?}", c),
            RuntimeValue::Tuple(t) => write!(f, "{:?}", t),
            RuntimeValue::Function(p, b) => write!(f, "({:?}) {{{:?}}}", p, b),
        }
    }
}