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
        _ => panic!()
    }
}

pub fn string_split(object: RuntimeValue, arguments: Vec<RuntimeValue>) -> RuntimeValue {
    let mut splitted_string = Vec::new();

    if arguments.len() == 1 {
        let split_char = match &arguments[0] {
            RuntimeValue::String(s, _) => s,
            _ => panic!()
        };
        let mut split = String::new();

        match object {
            RuntimeValue::String(mut s, _) => {
                while !s.is_empty() {
                    let char = s.remove(0);

                    if char == split_char.as_str().chars().next().unwrap() {
                        splitted_string.push(RuntimeValue::string(split));
                        split = "".to_string();
                    } else {
                        split.push(char);
                    }
                }
            },
            _ => panic!()
        }
    } else {
        panic!("Expected 1 argument, recieved {}", arguments.len())
    }

    RuntimeValue::array(splitted_string)
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
