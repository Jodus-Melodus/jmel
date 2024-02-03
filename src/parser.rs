use crate::lexer::{Token, TT};

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    StringLiteral(String),
    IntegerLiteral(i64),
    RealLiteral(f64),
    ArrayLiteral(Vec<ASTNode>),
    NullLiteral,
    Identifier(String),

    ConditionalExpression(Box<ASTNode>, String, Box<ASTNode>),
    CallExpression(Box<ASTNode>, Vec<ASTNode>),
    BinaryExpression(Box<ASTNode>, String, Box<ASTNode>),
    AssignmentExpression(Box<ASTNode>, Box<ASTNode>),
    UnaryExpression(String, Box<ASTNode>),
    MemberExpression(Box<ASTNode>, Box<ASTNode>, bool),
    ConversionExpression(Box<ASTNode>, Box<ASTNode>),

    VariableDeclaration(Box<ASTNode>, Box<ASTNode>),
    FunctionDeclaration(Box<ASTNode>, Vec<ASTNode>, Box<ASTNode>),
    IfStatement(Box<ASTNode>, Box<ASTNode>, Box<ASTNode>),
    Program(Vec<ASTNode>),
    CaseStatement(Box<ASTNode>, Vec<ASTNode>),
    Case(Box<ASTNode>, Box<ASTNode>),
}

pub struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }

    fn eat(&mut self) -> Token {
        self.tokens.remove(0)
    }

    fn expect(&mut self, expectation: TT) -> Token {
        let token = &self.peek();
        if token.kind == expectation {
            self.eat()
        } else {
            panic!(
                "Expected a '{:?}' but found a '{:?}'",
                expectation, token.kind
            );
        }
    }

    fn peek(&mut self) -> &Token {
        &self.tokens[0]
    }

    pub fn generate_ast(&mut self) -> ASTNode {
        let mut program_body = Vec::new();

        while self.peek().kind != TT::Eof {
            program_body.push(self.parse());
        }

        ASTNode::Program(program_body)
    }

    fn parse(&mut self) -> ASTNode {
        match self.peek().kind {
            TT::KeyWord => self.parse_statement(),
            _ => self.parse_expression(),
        }
    }

    fn parse_statement(&mut self) -> ASTNode {
        match self.peek().value.as_str() {
            "let" => self.parse_variable_declaration(),
            "if" => self.parse_if_statement(),
            "case" => self.parse_case_statement(),
            "func" => self.parse_function_declaration(),
            _ => panic!("Invalid keyword found '{}'", self.eat().value),
        }
    }

    fn parse_function_declaration(&mut self) -> ASTNode {
        self.eat();

        let name = self.parse_primary_expression();
        assert!(matches!(&name, ASTNode::Identifier(_)));

        if self.peek().kind == TT::OpeningParenthesis {
            let parameters = self.parse_arguments();
            
            if self.peek().kind == TT::OpeningCurlyBrace {
                self.eat();
                let mut body = Vec::new();

                while self.peek().kind != TT::ClosingCurlyBrace {
                    body.push(self.parse());
                };
                self.eat();

                ASTNode::FunctionDeclaration(Box::new(name), parameters, Box::new(ASTNode::Program(body)))
            } else {
                panic!();
            }
        } else {
            panic!("Expected '(', got '{:?}'", self.peek());
        }
    }

    fn parse_case_statement(&mut self) -> ASTNode {
        self.eat();

        let value = self.parse_expression();

        if self.peek().kind == TT::KeyWord && self.peek().value == "of" {
            self.eat();

            let mut cases = Vec::new();

            self.expect(TT::OpeningCurlyBrace);

            while self.peek().kind != TT::ClosingCurlyBrace {
                cases.push(self.parse_case());
            }
            self.expect(TT::ClosingCurlyBrace);
            self.expect(TT::SemiColon);

            ASTNode::CaseStatement(Box::new(value), cases)
        } else {
            panic!("Expected an 'of', got '{:?}'", self.peek());
        }
    }

    fn parse_case(&mut self) -> ASTNode {
        let case = self.parse_expression();
        let mut body = Vec::new();

        self.expect(TT::Colon);

        self.expect(TT::OpeningCurlyBrace);

        while self.peek().kind != TT::ClosingCurlyBrace {
            body.push(self.parse());
        }
        self.eat();

        self.expect(TT::SemiColon);

        ASTNode::Case(Box::new(case), Box::new(ASTNode::Program(body)))
    }

    fn parse_if_statement(&mut self) -> ASTNode {
        self.eat();

        let condition = self.parse_conditional_expression();
        let mut body = Vec::new();
        let mut else_body = Vec::new();

        self.expect(TT::OpeningCurlyBrace);

        while self.peek().kind != TT::ClosingCurlyBrace {
            body.push(self.parse());
        }
        self.eat();

        if self.peek().kind == TT::KeyWord && self.peek().value == "else" {
            self.eat();
            self.expect(TT::OpeningCurlyBrace);

            while self.peek().kind != TT::ClosingCurlyBrace {
                else_body.push(self.parse());
            }

            self.eat();
        };

        ASTNode::IfStatement(
            Box::new(condition),
            Box::new(ASTNode::Program(body)),
            Box::new(ASTNode::Program(else_body)),
        )
    }

    fn parse_variable_declaration(&mut self) -> ASTNode {
        self.eat();

        if self.peek().kind == TT::Identifier || self.peek().kind == TT::OpeningParenthesis {
            let variable_name = match self.peek().kind {
                TT::Identifier => self.parse_primary_expression(),
                _ => self.parse_call_expression(),
            };

            if self.peek().kind == TT::AssignmentOperator {
                self.eat();
                let variable_value = self.parse_expression();

                if self.peek().kind == TT::SemiColon {
                    self.eat();
                    ASTNode::VariableDeclaration(Box::new(variable_name), Box::new(variable_value))
                } else {
                    panic!("Expected ';', got '{:?}'", self.peek());
                }
            } else if self.peek().kind == TT::SemiColon {
                self.eat();
                ASTNode::VariableDeclaration(
                    Box::new(variable_name),
                    Box::new(ASTNode::NullLiteral),
                )
            } else {
                panic!("Expected ';' or an identifier, got '{}'", self.peek().value);
            }
        } else {
            panic!("Expected an identifier, got '{}'", self.peek().value);
        }
    }

    fn parse_expression(&mut self) -> ASTNode {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> ASTNode {
        if self.peek().kind == TT::Identifier || self.peek().kind == TT::OpeningParenthesis {
            let variable = self.parse_call_expression();
            if self.peek().kind == TT::AssignmentOperator {
                self.eat();
                let variable_value = self.parse_expression();

                if self.peek().kind == TT::SemiColon {
                    self.eat();
                    ASTNode::AssignmentExpression(Box::new(variable), Box::new(variable_value))
                } else {
                    panic!("Expected a ';', got a '{:?}'", self.eat());
                }
            } else {
                variable
            }
        } else {
            self.parse_call_expression()
        }
    }

    fn parse_call_expression(&mut self) -> ASTNode {
        let mut calle = self.parse_conditional_expression();

        if self.peek().kind == TT::OpeningParenthesis {
            let arguments = self.parse_arguments();
            calle = ASTNode::CallExpression(Box::new(calle), arguments)
        }
        calle
    }

    fn parse_arguments(&mut self) -> Vec<ASTNode> {
        self.eat();

        let mut arguments = Vec::new();

        while self.peek().kind != TT::ClosingParenthesis {
            let argument = self.parse_expression();
            arguments.push(argument);
            if self.peek().kind == TT::Comma {
                self.eat();
            } else if self.peek().kind != TT::ClosingParenthesis {
                panic!("Expected a ',', or a ')', but found a {:?}.", self.peek());
            }
        }
        self.eat();
        arguments
    }

    fn parse_conditional_expression(&mut self) -> ASTNode {
        let mut left = self.parse_additive_expression();

        while [
            TT::GreaterThan,
            TT::LessThan,
            TT::GreaterThanOrEqualTo,
            TT::LessThanOrEqualTo,
            TT::EqualityOperator,
            TT::InEqualityOperator,
        ]
        .contains(&self.peek().kind)
        {
            let operator = self.eat().value;
            let right = self.parse_additive_expression();
            left = ASTNode::ConditionalExpression(Box::new(left), operator, Box::new(right));
        }

        left
    }

    fn parse_additive_expression(&mut self) -> ASTNode {
        let mut left = self.parse_multiplicative_expression();

        while self.peek().kind == TT::BinaryOperator
            && ["+", "-"].contains(&self.peek().value.as_str())
        {
            let operator = self.eat().value;
            let right = self.parse_multiplicative_expression();
            left = ASTNode::BinaryExpression(Box::new(left), operator, Box::new(right));
        }
        left
    }

    fn parse_multiplicative_expression(&mut self) -> ASTNode {
        let mut left = self.parse_member_expression();

        while self.peek().kind == TT::BinaryOperator
            && ["*", "/", "%"].contains(&self.peek().value.as_str())
        {
            let operator = self.eat().value;
            let right = self.parse_member_expression();
            left = ASTNode::BinaryExpression(Box::new(left), operator, Box::new(right));
        }
        left
    }

    fn parse_member_expression(&mut self) -> ASTNode {
        let mut left = self.parse_conversion_expression();

        while self.peek().kind == TT::Dot || self.peek().kind == TT::OpeningSquareBracket {
            let operator = self.eat();
            let right;

            if operator.kind == TT::Dot {
                right = self.parse_conversion_expression();
            } else {
                right = self.parse_expression();
                self.expect(TT::ClosingSquareBracket);
            }
            left =
                ASTNode::MemberExpression(Box::new(left), Box::new(right), operator.kind == TT::Dot)
        }
        left
    }

    fn parse_conversion_expression(&mut self) -> ASTNode {
        let left = self.parse_primary_expression();

        if self.peek().kind == TT::KeyWord
            && (self.peek().value == "as" || self.peek().value == "to")
        {
            self.eat();

            let right = self.parse_primary_expression();
            ASTNode::ConversionExpression(Box::new(left), Box::new(right))
        } else {
            left
        }
    }

    fn parse_primary_expression(&mut self) -> ASTNode {
        match self.peek().kind {
            TT::Identifier => ASTNode::Identifier(self.eat().value),
            TT::Integer => ASTNode::IntegerLiteral(self.eat().value.parse::<i64>().unwrap()),
            TT::Real => ASTNode::RealLiteral(self.eat().value.parse::<f64>().unwrap()),
            TT::String => ASTNode::StringLiteral(self.eat().value),
            TT::BinaryOperator | TT::Not => match self.peek().value.as_str() {
                "+" | "-" | "!" => {
                    let sign = self.eat().value;
                    let value = self.parse_primary_expression();

                    ASTNode::UnaryExpression(sign, Box::new(value))
                }
                _ => panic!("Invalid token '{:?}' found", self.eat()),
            },
            TT::OpeningSquareBracket => {
                self.eat();
                let mut values = Vec::new();

                while self.peek().kind != TT::ClosingSquareBracket {
                    let node = self.parse_expression();
                    values.push(node);

                    if self.peek().kind == TT::Comma {
                        self.eat();
                    } else {
                        break;
                    };
                }
                self.expect(TT::ClosingSquareBracket);
                ASTNode::ArrayLiteral(values)
            }
            TT::OpeningParenthesis => {
                self.eat();
                if self.peek().kind == TT::ClosingParenthesis {
                    self.eat();
                    return ASTNode::NullLiteral;
                };
                let node = self.parse_expression();
                self.expect(TT::ClosingParenthesis);
                node
            }
            _ => panic!("Invalid token '{:?}' found", self.eat()),
        }
    }
}
