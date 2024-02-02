use std::io::{self, Write};

use crate::values::RuntimeValue;

pub fn print(arguments: Vec<RuntimeValue>) -> RuntimeValue {
    for arg in arguments {
        print!("{}", arg);
    };
    RuntimeValue::Null
}

pub fn input(prompt: Vec<RuntimeValue>) -> RuntimeValue {
    let mut input = String::new();
    print(prompt);

    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    RuntimeValue::String(input.trim_end().to_string())
}
