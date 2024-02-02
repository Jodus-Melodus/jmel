use crate::values::RuntimeValue;

// Array Methods

pub fn array_length(object: RuntimeValue, arguments: Vec<RuntimeValue>) -> RuntimeValue {
    // .length() method for array

    match &object {
        RuntimeValue::Array(a, _) => RuntimeValue::Integer(a.len() as i64),
        _ => RuntimeValue::Null,
    }
}

pub fn array_is_empty(object: RuntimeValue, arguments: Vec<RuntimeValue>) -> RuntimeValue {
    // .is_empty() method for array

    match &object {
        RuntimeValue::Array(a, _) => RuntimeValue::Boolean(a.is_empty()),
        _ => RuntimeValue::Null,
    }
}

// String Methods

pub fn string_length(object: RuntimeValue, arguments: Vec<RuntimeValue>) -> RuntimeValue {
    // .length() method for string

    match &object {
        RuntimeValue::String(s, _) => RuntimeValue::Integer(s.len() as i64),
        _ => RuntimeValue::Null,
    }
}

pub fn string_is_empty(object: RuntimeValue, arguments: Vec<RuntimeValue>) -> RuntimeValue {
    // .is_empty() method for string

    match &object {
        RuntimeValue::String(s, _) => RuntimeValue::Boolean(s.is_empty()),
        _ => RuntimeValue::Null,
    }
}

// Object Methods

pub fn object_length(object: RuntimeValue, arguments: Vec<RuntimeValue>) -> RuntimeValue {
    // .length() method for objects

    match &object {
        RuntimeValue::Object(o, _) => RuntimeValue::Integer(o.len() as i64),
        _ => RuntimeValue::Null,
    }
}

pub fn object_is_empty(object: RuntimeValue, arguments: Vec<RuntimeValue>) -> RuntimeValue {
    // .is_empty() method for object

    match &object {
        RuntimeValue::Object(o, _) => RuntimeValue::Boolean(o.is_empty()),
        _ => RuntimeValue::Null,
    }
}
