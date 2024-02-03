use core::panic;

use crate::{environment::Environment, parser::ASTNode, values::RuntimeValue};

pub struct Interpreter {
    program: ASTNode,
}

impl Interpreter {
    pub fn new(program: ASTNode) -> Self {
        Interpreter { program }
    }

    pub fn interpret(&self, environment: &mut Environment) -> RuntimeValue {
        let mut result = RuntimeValue::Null;

        if let ASTNode::Program(body) = &self.program {
            for statement in body.iter() {
                result = self.evaluate(statement.clone(), environment);
            }
        }

        result
    }

    fn evaluate(&self, statement: ASTNode, environment: &mut Environment) -> RuntimeValue {
        match statement {
            ASTNode::ConditionalExpression(left, operand, right) => {
                self.evaluate_conditional_expression(*left, operand, *right, environment)
            }
            ASTNode::BinaryExpression(left, operand, right) => self.evaluate_binary_expression(
                self.evaluate(*left, environment),
                operand,
                self.evaluate(*right, environment),
                environment,
            ),
            ASTNode::CallExpression(calle, arguments) => {
                self.evaluate_call_expression(*calle, arguments, environment)
            }
            ASTNode::UnaryExpression(sign, value) => {
                self.evaluate_unary_expression(sign, *value, environment)
            }
            ASTNode::AssignmentExpression(variable_name, variable_value) => {
                self.evaluate_assignment_expression(*variable_name, *variable_value, environment)
            }
            ASTNode::MemberExpression(object, property, dot) => {
                self.evaluate_member_expression(*object, *property, dot, environment)
            }
            ASTNode::ConversionExpression(left, right) => {
                self.evaluate_conversion_expression(*left, *right, environment)
            }

            ASTNode::StringLiteral(value) => RuntimeValue::string(value),
            ASTNode::IntegerLiteral(value) => RuntimeValue::Integer(value),
            ASTNode::NullLiteral => RuntimeValue::Null,
            ASTNode::Identifier(variable_name) => environment.lookup(variable_name).clone(),
            ASTNode::ArrayLiteral(values) => RuntimeValue::array(
                values
                    .iter()
                    .map(|v| self.evaluate(v.clone(), environment))
                    .collect(),
            ),
            ASTNode::RealLiteral(value) => RuntimeValue::Real(value),

            ASTNode::VariableDeclaration(variable_name, variable_value) => {
                self.evaluate_variable_declaration(*variable_name, *variable_value, environment)
            }
            ASTNode::FunctionDeclaration(name, parameters, parameter_types, return_type, body) => {
                self.evaluate_function_declaration(*name, parameters, parameter_types, *return_type, *body, environment)
            }
            ASTNode::IfStatement(condition, body, else_body) => {
                self.evaluate_if_statement(*condition, *body, *else_body, environment)
            }
            ASTNode::CaseStatement(value, cases) => {
                self.evaluate_case_statement(*value, cases, environment)
            }

            ASTNode::Program(_) => {
                let scope_interpreter = Interpreter::new(statement);
                let mut scope_environment = Environment::new(Some(environment.clone()));

                scope_interpreter.interpret(&mut scope_environment)
            }
            _ => RuntimeValue::Null,
        }
    }

    fn evaluate_function_declaration(
        &self,
        name: ASTNode,
        parameters: Vec<ASTNode>,
        parameter_types: Vec<ASTNode>,
        return_type: ASTNode,
        body: ASTNode,
        environment: &mut Environment,
    ) -> RuntimeValue {
        let mut para_types = Vec::new();
        let return_type = self.evaluate(return_type, environment);

        for parameter_type in parameter_types {
            para_types.push(self.evaluate(parameter_type, environment));
        }

        match name {
            ASTNode::Identifier(function_name) => {
                let function = RuntimeValue::Function(parameters, para_types, Box::new(return_type), body);
                environment.declare_variable(function_name, function.clone());
                function
            }
            _ => panic!(),
        }
    }

    fn evaluate_conversion_expression(
        &self,
        left: ASTNode,
        right: ASTNode,
        environment: &mut Environment,
    ) -> RuntimeValue {
        let l = self.evaluate(left, environment);

        match right {
            ASTNode::Identifier(kind) => match kind.as_str() {
                "integer" => match l {
                    RuntimeValue::Integer(v) => RuntimeValue::Integer(v),
                    RuntimeValue::Real(v) => RuntimeValue::Integer(v as i64),
                    RuntimeValue::Boolean(v) => RuntimeValue::Integer(if v { 1 } else { 0 }),
                    RuntimeValue::String(v, _) => RuntimeValue::Integer(v.parse::<i64>().unwrap()),
                    _ => RuntimeValue::Null,
                },
                "real" => match l {
                    RuntimeValue::Integer(v) => RuntimeValue::Real(v as f64),
                    RuntimeValue::Real(v) => RuntimeValue::Real(v),
                    RuntimeValue::String(v, _) => RuntimeValue::Real(v.parse::<f64>().unwrap()),
                    _ => RuntimeValue::Null,
                },
                "boolean" => match l {
                    RuntimeValue::Integer(v) => RuntimeValue::Boolean(v != 0),
                    RuntimeValue::Real(v) => RuntimeValue::Boolean(v != 0.0),
                    RuntimeValue::Boolean(v) => RuntimeValue::Boolean(v),
                    RuntimeValue::String(v, _) => RuntimeValue::Boolean(!v.is_empty()),
                    RuntimeValue::Array(v, _) => RuntimeValue::Boolean(!v.is_empty()),
                    RuntimeValue::Object(v, _) => RuntimeValue::Boolean(!v.is_empty()),
                    RuntimeValue::Tuple(v) => RuntimeValue::Boolean(!v.is_empty()),
                    _ => RuntimeValue::Null,
                },
                "string" => match l {
                    RuntimeValue::Integer(v) => RuntimeValue::string(v.to_string()),
                    RuntimeValue::Real(v) => RuntimeValue::string(v.to_string()),
                    RuntimeValue::Boolean(v) => RuntimeValue::string(v.to_string()),
                    RuntimeValue::String(v, _) => RuntimeValue::string(v),
                    _ => RuntimeValue::Null,
                },
                _ => RuntimeValue::Null,
            },
            _ => RuntimeValue::Null,
        }
    }

    fn evaluate_case_statement(
        &self,
        value: ASTNode,
        cases: Vec<ASTNode>,
        environment: &mut Environment,
    ) -> RuntimeValue {
        let evaluated_value = self.evaluate(value, environment);

        for case in cases {
            if let ASTNode::Case(c, body) = case {
                let evaluated_case = self.evaluate(*c, environment);
                if (evaluated_case == RuntimeValue::Null) || (evaluated_value == evaluated_case) {
                    return self.evaluate(*body, environment);
                };
            };
        }

        RuntimeValue::Null
    }

    fn evaluate_if_statement(
        &self,
        condition: ASTNode,
        body: ASTNode,
        else_body: ASTNode,
        environment: &mut Environment,
    ) -> RuntimeValue {
        let evaluated_condition = self.evaluate(condition, environment);

        match evaluated_condition {
            RuntimeValue::Boolean(b) => {
                if b {
                    self.evaluate(body, environment)
                } else {
                    self.evaluate(else_body, environment)
                }
            }
            _ => RuntimeValue::Null,
        }
    }

    fn evaluate_conditional_expression(
        &self,
        left: ASTNode,
        operand: String,
        right: ASTNode,
        environment: &mut Environment,
    ) -> RuntimeValue {
        let l = self.evaluate(left, environment);
        let r = self.evaluate(right, environment);

        match (operand.as_str(), l, r) {
            ("<", RuntimeValue::Integer(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Boolean((lhs as f64) < rhs)
            }
            ("<", RuntimeValue::Real(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs < rhs as f64)
            }
            ("<", RuntimeValue::Integer(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs < rhs)
            }
            ("<", RuntimeValue::Real(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Boolean(lhs < rhs)
            }
            ("<", RuntimeValue::String(lhs, _), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs.len() < rhs.try_into().unwrap())
            }
            ("<", RuntimeValue::Array(lhs, _), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs.len() < rhs.try_into().unwrap())
            }
            ("<", RuntimeValue::String(lhs, _), RuntimeValue::String(rhs, _)) => {
                RuntimeValue::Boolean(lhs.len() < rhs.len())
            }
            ("<", RuntimeValue::Array(lhs, _), RuntimeValue::Array(rhs, _)) => {
                RuntimeValue::Boolean(lhs.len() < rhs.len())
            }

            ("<=", RuntimeValue::Integer(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Boolean((lhs as f64) <= rhs)
            }
            ("<=", RuntimeValue::Real(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs <= rhs as f64)
            }
            ("<=", RuntimeValue::Integer(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs <= rhs)
            }
            ("<=", RuntimeValue::Real(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Boolean(lhs <= rhs)
            }
            ("<=", RuntimeValue::String(lhs, _), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs.len() <= rhs.try_into().unwrap())
            }
            ("<=", RuntimeValue::Array(lhs, _), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs.len() <= rhs.try_into().unwrap())
            }
            ("<=", RuntimeValue::String(lhs, _), RuntimeValue::String(rhs, _)) => {
                RuntimeValue::Boolean(lhs.len() <= rhs.len())
            }
            ("<=", RuntimeValue::Array(lhs, _), RuntimeValue::Array(rhs, _)) => {
                RuntimeValue::Boolean(lhs.len() <= rhs.len())
            }

            (">", RuntimeValue::Integer(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Boolean((lhs as f64) > rhs)
            }
            (">", RuntimeValue::Real(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs > rhs as f64)
            }
            (">", RuntimeValue::Integer(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs > rhs)
            }
            (">", RuntimeValue::Real(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Boolean(lhs > rhs)
            }
            (">", RuntimeValue::String(lhs, _), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs.len() > rhs.try_into().unwrap())
            }
            (">", RuntimeValue::Array(lhs, _), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs.len() > rhs.try_into().unwrap())
            }
            (">", RuntimeValue::String(lhs, _), RuntimeValue::String(rhs, _)) => {
                RuntimeValue::Boolean(lhs.len() > rhs.len())
            }
            (">", RuntimeValue::Array(lhs, _), RuntimeValue::Array(rhs, _)) => {
                RuntimeValue::Boolean(lhs.len() > rhs.len())
            }

            (">=", RuntimeValue::Integer(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Boolean((lhs as f64) >= rhs)
            }
            (">=", RuntimeValue::Real(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs >= rhs as f64)
            }
            (">=", RuntimeValue::Integer(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs >= rhs)
            }
            (">=", RuntimeValue::Real(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Boolean(lhs >= rhs)
            }
            (">=", RuntimeValue::String(lhs, _), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs.len() >= rhs.try_into().unwrap())
            }
            (">=", RuntimeValue::Array(lhs, _), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs.len() >= rhs.try_into().unwrap())
            }
            (">=", RuntimeValue::String(lhs, _), RuntimeValue::String(rhs, _)) => {
                RuntimeValue::Boolean(lhs.len() >= rhs.len())
            }
            (">=", RuntimeValue::Array(lhs, _), RuntimeValue::Array(rhs, _)) => {
                RuntimeValue::Boolean(lhs.len() >= rhs.len())
            }

            ("==", RuntimeValue::Integer(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Boolean((lhs as f64) == rhs)
            }
            ("==", RuntimeValue::Real(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs == rhs as f64)
            }
            ("==", RuntimeValue::Integer(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs == rhs)
            }
            ("==", RuntimeValue::Real(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Boolean(lhs == rhs)
            }
            ("==", RuntimeValue::String(lhs, _), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs.len() == rhs.try_into().unwrap())
            }
            ("==", RuntimeValue::Array(lhs, _), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs.len() == rhs.try_into().unwrap())
            }
            ("==", RuntimeValue::String(lhs, _), RuntimeValue::String(rhs, _)) => {
                RuntimeValue::Boolean(lhs == rhs)
            }
            ("==", RuntimeValue::Array(lhs, _), RuntimeValue::Array(rhs, _)) => {
                RuntimeValue::Boolean(lhs == rhs)
            }
            ("==", RuntimeValue::Boolean(lhs), RuntimeValue::Boolean(rhs)) => {
                RuntimeValue::Boolean(lhs == rhs)
            }

            ("!=", RuntimeValue::Integer(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Boolean((lhs as f64) != rhs)
            }
            ("!=", RuntimeValue::Real(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs != rhs as f64)
            }
            ("!=", RuntimeValue::Integer(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs != rhs)
            }
            ("!=", RuntimeValue::Real(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Boolean(lhs != rhs)
            }
            ("!=", RuntimeValue::String(lhs, _), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs.len() != rhs.try_into().unwrap())
            }
            ("!=", RuntimeValue::Array(lhs, _), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Boolean(lhs.len() != rhs.try_into().unwrap())
            }
            ("!=", RuntimeValue::String(lhs, _), RuntimeValue::String(rhs, _)) => {
                RuntimeValue::Boolean(lhs != rhs)
            }
            ("!=", RuntimeValue::Array(lhs, _), RuntimeValue::Array(rhs, _)) => {
                RuntimeValue::Boolean(lhs != rhs)
            }
            ("!=", RuntimeValue::Boolean(lhs), RuntimeValue::Boolean(rhs)) => {
                RuntimeValue::Boolean(lhs != rhs)
            }

            _ => RuntimeValue::Boolean(false),
        }
    }

    fn evaluate_call_expression(
        &self,
        calle: ASTNode,
        arguments: Vec<ASTNode>,
        environment: &mut Environment,
    ) -> RuntimeValue {
        let caller = self.evaluate(calle, environment);
        let mut args = Vec::new();

        for arg in arguments {
            args.push(self.evaluate(arg, environment));
        }

        match caller {
            RuntimeValue::BuiltInFunction(call, _) => call(args),
            RuntimeValue::Method(call, object, _) => call(*object, args),
            RuntimeValue::Function(parameters, parameter_types, return_type, body) => {
                let scope_interpreter = Interpreter::new(body);
                let mut scope_environment = Environment::new(Some(environment.clone()));

                if parameters.len() == args.len() && args.len() == parameter_types.len() {
                    for ((parameter, arg), expected_type) in parameters.iter().zip(args).zip(parameter_types) {
                        if let ASTNode::Identifier(variable_name) = parameter {
                            
                            match (arg.clone(), expected_type.clone()) {
                                (ref arg, _) if std::mem::discriminant(arg) == std::mem::discriminant(&expected_type) => {
                                    scope_environment.declare_variable(variable_name.to_string(), arg.clone())
                                },
                                _ => panic!("Type Error: Expected type '{:?}' but found type '{:?}'", expected_type, arg)
                            }
                        } else {
                            panic!()
                        }
                    }

                    let result = scope_interpreter.interpret(&mut scope_environment);

                    match (result.clone(), return_type.clone()) {
                        (ref result_value, _) if std::mem::discriminant(result_value) == std::mem::discriminant(&return_type) => result,
                        _ => panic!("Type Error: Expected type '{:?}' but found type '{:?}'", return_type, result),
                    }

                } else {
                    panic!(
                        "Wrong number of arguments provided. Expected {} but got {}",
                        parameters.len(),
                        args.len()
                    );
                }
            }
            _ => RuntimeValue::Null
        }
    }

    fn evaluate_member_expression(
        &self,
        object: ASTNode,
        property: ASTNode,
        dot: bool,
        environment: &mut Environment,
    ) -> RuntimeValue {
        let obj = self.evaluate(object, environment);
        let prop = if dot {
            match property {
                ASTNode::Identifier(s) => RuntimeValue::string(s),
                _ => RuntimeValue::Null,
            }
        } else {
            self.evaluate(property, environment)
        };

        match (&obj, prop) {
            // Properties and Methods
            (RuntimeValue::Object(o, methods), RuntimeValue::String(p, _)) => methods
                .get(&p)
                .map(|meth| RuntimeValue::Method(*meth, Box::new(obj.clone()), vec![]))
                .or_else(|| o.get(&p).cloned())
                .expect("Property not found on object"),
            (
                RuntimeValue::Array(_, methods) | RuntimeValue::String(_, methods),
                RuntimeValue::String(method, _),
            ) => methods
                .get(&method)
                .map(|meth| RuntimeValue::Method(*meth, Box::new(obj.clone()), vec![]))
                .expect("Method not found"),

            // Indexing
            (RuntimeValue::String(s, _), RuntimeValue::Integer(i)) => RuntimeValue::string(
                s.chars()
                    .nth(i as usize)
                    .map_or(String::from(" "), |c| c.to_string()),
            ),
            (RuntimeValue::Array(a, _), RuntimeValue::Integer(i)) => {
                a.get(i as usize).cloned().unwrap_or(RuntimeValue::Null)
            }
            _ => RuntimeValue::Null,
        }
    }

    fn evaluate_variable_declaration(
        &self,
        variable: ASTNode,
        variable_value: ASTNode,
        environment: &mut Environment,
    ) -> RuntimeValue {
        let variable_name;

        match (variable, variable_value.clone()) {
            (ASTNode::Identifier(n), _) => {
                variable_name = n;
                let value = self.evaluate(variable_value, environment);
                environment.declare_variable(variable_name, value);
                RuntimeValue::Null
            }
            _ => panic!(),
        }
    }

    fn evaluate_assignment_expression(
        &self,
        variable_name: ASTNode,
        variable_value: ASTNode,
        environment: &mut Environment,
    ) -> RuntimeValue {
        match (variable_name, variable_value.clone()) {
            (ASTNode::Identifier(name), _) => {
                let value = self.evaluate(variable_value, environment);
                environment.assign(name, value);
                RuntimeValue::Null
            }
            _ => panic!("Expected Identifier"),
        }
    }

    fn evaluate_unary_expression(
        &self,
        sign: String,
        value: ASTNode,
        environment: &mut Environment,
    ) -> RuntimeValue {
        let value = self.evaluate(value, environment);
        match sign.as_str() {
            "-" | "!" => match value {
                RuntimeValue::Boolean(b) => RuntimeValue::Boolean(!b),
                RuntimeValue::Integer(i) => RuntimeValue::Integer(-i),
                RuntimeValue::Real(f) => RuntimeValue::Real(-f),
                _ => panic!(""),
            },
            _ => value,
        }
    }

    fn evaluate_binary_expression(
        &self,
        left: RuntimeValue,
        operand: String,
        right: RuntimeValue,
        environment: &mut Environment,
    ) -> RuntimeValue {

        match (operand.as_str(), left, right) {
            // Integer : Integer
            ("+", RuntimeValue::Integer(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Integer(lhs + rhs)
            }
            ("-", RuntimeValue::Integer(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Integer(lhs - rhs)
            }
            ("*", RuntimeValue::Integer(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Integer(lhs * rhs)
            }
            ("/", RuntimeValue::Integer(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Real(lhs as f64 / rhs as f64)
            }
            ("%", RuntimeValue::Integer(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Integer(lhs % rhs)
            }

            // Real : Real
            ("+", RuntimeValue::Real(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Real(lhs + rhs)
            }
            ("-", RuntimeValue::Real(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Real(lhs - rhs)
            }
            ("*", RuntimeValue::Real(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Real(lhs * rhs)
            }
            ("/", RuntimeValue::Real(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Real(lhs / rhs)
            }

            // Integer : Real
            ("+", RuntimeValue::Integer(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Real(lhs as f64 + rhs)
            }
            ("-", RuntimeValue::Integer(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Real(lhs as f64 - rhs)
            }
            ("*", RuntimeValue::Integer(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Real(lhs as f64 * rhs)
            }
            ("/", RuntimeValue::Integer(lhs), RuntimeValue::Real(rhs)) => {
                RuntimeValue::Real(lhs as f64 / rhs)
            }

            // Real : Integer
            ("+", RuntimeValue::Real(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Real(lhs + rhs as f64)
            }
            ("-", RuntimeValue::Real(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Real(lhs - rhs as f64)
            }
            ("*", RuntimeValue::Real(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Real(lhs * rhs as f64)
            }
            ("/", RuntimeValue::Real(lhs), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::Real(lhs / rhs as f64)
            }

            // String : String
            ("+", RuntimeValue::String(lhs, _), RuntimeValue::String(rhs, _)) => {
                RuntimeValue::string(lhs + &rhs)
            }

            // String : Integer
            ("*", RuntimeValue::String(lhs, _), RuntimeValue::Integer(rhs)) => {
                RuntimeValue::string(lhs.repeat(rhs as usize))
            }

            // Array : Any
            ("+", RuntimeValue::Array(mut lhs, _), rhs) => {
                lhs.push(rhs);
                RuntimeValue::array(lhs)
            }

            // Tuple : Tuple
            (op, RuntimeValue::Tuple(lhs), RuntimeValue::Tuple(rhs)) => {
                let res = lhs.iter().zip(rhs).map(|(l, r)| self.evaluate_binary_expression(l.clone(), op.to_string(), r, environment)).collect::<Vec<RuntimeValue>>();
                RuntimeValue::Tuple(res)
            },


            _ => RuntimeValue::Null,
        }
    }
}
