use crate::{methods::*, parser::ASTNode};
use std::{
    collections::HashMap,
    fmt::{self, Write},
};

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
        HashMap<String, fn(RuntimeValue, Vec<RuntimeValue>) -> RuntimeValue>,
    ),
    Array(
        Vec<RuntimeValue>,
        HashMap<String, fn(RuntimeValue, Vec<RuntimeValue>) -> RuntimeValue>,
    ),
    Object(
        HashMap<String, RuntimeValue>,
        HashMap<String, fn(RuntimeValue, Vec<RuntimeValue>) -> RuntimeValue>,
    ),

    // datatypes the programmer can't access
    BuiltInFunction(fn(Vec<RuntimeValue>) -> RuntimeValue, Vec<RuntimeValue>),
    Method(
        fn(RuntimeValue, Vec<RuntimeValue>) -> RuntimeValue,
        Box<RuntimeValue>,
        Vec<RuntimeValue>,
    ),
    Function(Vec<ASTNode>, Vec<RuntimeValue>, Box<RuntimeValue>, ASTNode),
}

impl fmt::Display for RuntimeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeValue::Integer(i) => write!(f, "{}", i),
            RuntimeValue::Real(r) => write!(f, "{}", r),
            RuntimeValue::Boolean(b) => write!(f, "{}", b),
            RuntimeValue::String(s, _) => write!(f, "{}", s),
            RuntimeValue::Array(a, _) => {
                let elements: Vec<String> = a.iter().map(|v| format!("{}", v)).collect();
                let res = format!("[{}]", elements.join(", "));
                res.fmt(f)
            }
            RuntimeValue::Null => write!(f, "null"),
            RuntimeValue::Object(o, _) => write!(f, "{:?}", o),
            RuntimeValue::BuiltInFunction(c, _) => write!(f, "{:?}", c),
            RuntimeValue::Tuple(t) => write!(f, "{:?}", t),
            RuntimeValue::Function(p, _, _, b) => write!(f, "({:?}) {{{:?}}}", p, b),
            RuntimeValue::Method(call, object, arguments) => {
                let elements: Vec<String> = arguments.iter().map(|v| format!("{}", v)).collect();
                let res = format!("{}.{:?}({})", object, call, elements.join(", "));
                res.fmt(f)
            },
        }
    }
}

impl RuntimeValue {
    // Function to make and add methods to array
    pub fn array(values: Vec<RuntimeValue>) -> Self {
        let mut methods = HashMap::new();
        methods.insert(
            "length".to_string(),
            array_length as fn(RuntimeValue, Vec<RuntimeValue>) -> RuntimeValue,
        );
        methods.insert(
            "is_empty".to_string(),
            array_is_empty as fn(RuntimeValue, Vec<RuntimeValue>) -> RuntimeValue,
        );

        RuntimeValue::Array(values, methods)
    }

    // Function to make and add methods to string
    pub fn string(values: String) -> Self {
        let mut methods = HashMap::new();
        methods.insert(
            "length".to_string(),
            string_length as fn(RuntimeValue, Vec<RuntimeValue>) -> RuntimeValue,
        );
        methods.insert(
            "is_empty".to_string(),
            string_is_empty as fn(RuntimeValue, Vec<RuntimeValue>) -> RuntimeValue,
        );
        methods.insert(
            "split".to_string(),
            string_split as fn(RuntimeValue, Vec<RuntimeValue>) -> RuntimeValue,
        );

        RuntimeValue::String(values, methods)
    }

    // Function to make and add methods to object
    pub fn object(values: HashMap<String, RuntimeValue>) -> Self {
        let mut methods = HashMap::new();
        methods.insert(
            "length".to_string(),
            object_length as fn(RuntimeValue, Vec<RuntimeValue>) -> RuntimeValue,
        );
        methods.insert(
            "is_empty".to_string(),
            object_is_empty as fn(RuntimeValue, Vec<RuntimeValue>) -> RuntimeValue,
        );

        RuntimeValue::Object(values, methods)
    }
}
