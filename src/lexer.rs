#[derive(Clone, Debug, PartialEq)]
pub enum TT {
    Integer,
    Real,
    String,
    Comma,
    Dot,
    BinaryOperator,
    OpeningCurlyBrace,
    ClosingCurlyBrace,
    OpeningSquareBracket,
    ClosingSquareBracket,
    OpeningParenthesis,
    ClosingParenthesis,
    Eof,
    SemiColon,
    Colon,
    KeyWord,
    Identifier,
    AssignmentOperator,
    EqualityOperator,
    InEqualityOperator,
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    Not,
    And,
    Or,
    Xor,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TT,
    pub value: String,
}

impl Token {
    fn new(kind: TT, value: String) -> Self {
        Token { kind, value }
    }
}

#[derive(Clone, Debug)]
pub struct Lexer {
    source_code: String,
}

impl Lexer {
    pub fn new(source_code: String) -> Self {
        Lexer { source_code }
    }

    fn eat(&mut self) -> char {
        let mut characters = self.source_code.chars();
        let character = characters.next().unwrap();
        self.source_code = characters.collect::<String>();
        character
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        const KEYWORDS: [&str; 9] = ["let", "if", "else", "case", "of", "default", "as", "to", "func"];

        while !self.source_code.is_empty() {
            match self.source_code.chars().next().unwrap() {
                ' ' | '\n' | '\t' => {
                    self.eat();
                }
                '+' | '-' | '*' | '/' | '%' => {
                    if self.source_code.chars().nth(0).unwrap() == '/' {
                        if self.source_code.starts_with("//") {
                            self.eat();
                            self.eat();

                            while !self.source_code.is_empty() {
                                if self.source_code.chars().nth(0).unwrap() == '\n' {
                                    break;
                                };
                                self.eat();
                            }
                        } else {
                            tokens.push(Token::new(TT::BinaryOperator, self.eat().to_string()))
                        };
                    } else {
                        tokens.push(Token::new(TT::BinaryOperator, self.eat().to_string()))
                    }
                }
                ';' => tokens.push(Token::new(TT::SemiColon, self.eat().to_string())),
                ':' => tokens.push(Token::new(TT::Colon, self.eat().to_string())),
                ',' => tokens.push(Token::new(TT::Comma, self.eat().to_string())),
                '.' => tokens.push(Token::new(TT::Dot, self.eat().to_string())),
                '(' => tokens.push(Token::new(TT::OpeningParenthesis, self.eat().to_string())),
                ')' => tokens.push(Token::new(TT::ClosingParenthesis, self.eat().to_string())),
                '{' => tokens.push(Token::new(TT::OpeningCurlyBrace, self.eat().to_string())),
                '}' => tokens.push(Token::new(TT::ClosingCurlyBrace, self.eat().to_string())),
                '[' => tokens.push(Token::new(TT::OpeningSquareBracket, self.eat().to_string())),
                ']' => tokens.push(Token::new(TT::ClosingSquareBracket, self.eat().to_string())),
                '&' => tokens.push(Token::new(TT::And, self.eat().to_string())),
                '^' => tokens.push(Token::new(TT::Xor, self.eat().to_string())),
                '|' => tokens.push(Token::new(TT::Or, self.eat().to_string())),
                '>' => {
                    let (kind, value) = if self.source_code.starts_with(">=") {
                        self.eat();
                        (TT::GreaterThanOrEqualTo, ">=")
                    } else {
                        (TT::GreaterThan, ">")
                    };
                    self.eat();
                    tokens.push(Token::new(kind, value.to_string()));
                }
                '<' => {
                    let (kind, value) = if self.source_code.starts_with("<=") {
                        self.eat();
                        (TT::LessThanOrEqualTo, "<=")
                    } else {
                        (TT::LessThan, "<")
                    };
                    self.eat();
                    tokens.push(Token::new(kind, value.to_string()));
                }
                '=' => {
                    let (kind, value) = if self.source_code.starts_with("==") {
                        self.eat();
                        (TT::EqualityOperator, "==")
                    } else {
                        (TT::AssignmentOperator, "=")
                    };
                    self.eat();
                    tokens.push(Token::new(kind, value.to_string()));
                }
                '!' => {
                    let (kind, value) = if self.source_code.starts_with("!=") {
                        self.eat();
                        (TT::InEqualityOperator, "!=")
                    } else {
                        (TT::Not, "!")
                    };
                    self.eat();
                    tokens.push(Token::new(kind, value.to_string()));
                }
                '\'' | '"' => {
                    let start = self.eat();
                    let mut word = "".to_string();

                    while !self.source_code.is_empty() {
                        match self.source_code.chars().next().unwrap() {
                            c if c == start => {
                                self.eat();
                                break;
                            }
                            _ => word.push(self.eat()),
                        };
                    }
                    tokens.push(Token::new(TT::String, word));
                }
                'a'..='z' | 'A'..='Z' => {
                    let mut word = self.eat().to_string();

                    while !self.source_code.is_empty() {
                        match self.source_code.chars().next().unwrap() {
                            'a'..='z' | 'A'..='Z' | '0'..='9' => word.push(self.eat()),
                            _ => break,
                        };
                    }

                    if KEYWORDS.contains(&word.as_str()) {
                        tokens.push(Token::new(TT::KeyWord, word));
                    } else {
                        tokens.push(Token::new(TT::Identifier, word));
                    };
                }
                '0'..='9' => {
                    let mut number = self.eat().to_string();
                    while !self.source_code.is_empty() {
                        match self.source_code.chars().next().unwrap() {
                            '0'..='9' | '.' => number.push(self.eat()),
                            _ => break,
                        };
                    }
                    if number.contains('.') {
                        tokens.push(Token::new(TT::Real, number));
                    } else {
                        tokens.push(Token::new(TT::Integer, number));
                    };
                }
                _ => panic!("Invalid token '{}' found", self.eat()),
            };
        }
        tokens.push(Token::new(TT::Eof, "".to_string()));
        tokens
    }
}
