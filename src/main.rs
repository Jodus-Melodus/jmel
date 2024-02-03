use std::{
    env, fs::File, io::{BufRead, BufReader}, path::PathBuf
};

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

use crate::environment::Environment;

mod built_in_functions;
mod environment;
mod interpreter;
mod lexer;
mod methods;
mod parser;
mod values;

fn read_file(path: PathBuf) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");
    let content: Vec<String> = BufReader::new(&file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    content
}

fn main() {
    let mut arguments: Vec<String> = env::args().collect();
    arguments.remove(0);

    if let Some(source_file) = arguments.get(0) {
        let source_code = read_file(PathBuf::from(source_file)).join("\n");

        let mut lexer = Lexer::new(source_code);
        let tokens = lexer.tokenize();

        let mut parser = Parser::new(tokens);
        let program = parser.generate_ast();

        let interpreter = Interpreter::new(program);
        let _result = interpreter.interpret(&mut Environment::new(None));
    }
}