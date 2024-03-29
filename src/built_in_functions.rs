use std::io::{self, Write};

use crate::values::RuntimeValue;

pub fn print(arguments: Vec<RuntimeValue>) -> RuntimeValue {
    for arg in arguments {
        print!("{}", arg);
    }
    println!();
    RuntimeValue::Null
}

pub fn input(prompt: Vec<RuntimeValue>) -> RuntimeValue {
    let mut input = String::new();
    print(prompt);

    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    RuntimeValue::string(input.trim_end().to_string())
}

pub fn tup(values: Vec<RuntimeValue>) -> RuntimeValue {
    RuntimeValue::Tuple(values)
}
