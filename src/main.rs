use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
};

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

use crate::environment::Environment;

mod built_in_functions;
mod environment;
mod interpreter;
mod lexer;
mod parser;
mod values;
mod methods;

fn read_file(path: PathBuf) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");
    let content: Vec<String> = BufReader::new(&file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    content
}

fn read_line(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim_end().to_string()
}

fn main() {
    let source_code = read_file(PathBuf::from(read_line("Enter path > "))).join("\n");
    let mut environment = Environment::new(None);

    let mut lexer = Lexer::new(source_code);
    let tokens = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    let program = parser.generate_ast();

    let interpreter = Interpreter::new(program);
    let _result = interpreter.interpret(&mut environment);
}