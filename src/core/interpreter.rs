use std::rc::Rc;

use super::{
    ast::{Expr, Stmt, Value},
    environment::Environment,
    reporter::ErrorReporter,
    tokens::{Token, TokenType},
};

pub enum RuntimeError {
    OperandMustBeNumber(Token),
    OperandsMustBeNumbers(Token),
    OperandsMustBeStrings(Token),
    UndefinedVariable(Token),
    AssignUndefinedVariable(Token),
}

use RuntimeError as RE;
use TokenType as TT;
use Value as V;

#[derive(Default)]
pub struct Interpreter {
    environment: Rc<Environment>,
    reporter: Option<Rc<dyn ErrorReporter>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn attach_reporter<R>(mut self, reporter: Rc<R>) -> Self
    where
        R: ErrorReporter + 'static,
    {
        self.reporter = Some(reporter);
        self
    }

    fn report_runtime_error(&self, error: RuntimeError) {
        if let Some(reporter) = &self.reporter {
            match error {
                RE::OperandMustBeNumber(operator) => {
                    reporter.report_runtime(operator.line, "Operand must be a number.")
                }
                RE::OperandsMustBeNumbers(operator) => {
                    reporter.report_runtime(operator.line, "Operands must be numbers.")
                }
                RE::OperandsMustBeStrings(operator) => {
                    reporter.report_runtime(operator.line, "Operands must be strings.")
                }
                RE::UndefinedVariable(name) => reporter
                    .report_runtime(name.line, &format!("Undefined variable '{}'.", name.lexeme)),
                RE::AssignUndefinedVariable(name) => reporter
                    .report_runtime(name.line, &format!("Undefined variable '{}'.", name.lexeme)),
            }
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) {
        for statement in statements {
            if let Err(error) = self.execute(statement) {
                self.report_runtime_error(error);
                break;
            }
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<Value, RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => self.evaluate_expr(expr),
            Stmt::Print(expr) => {
                let value = self.evaluate_expr(expr)?;
                println!("{}", value.stringify());
                Ok(V::Nil)
            }
            Stmt::Var(name, initializer) => {
                let mut value = Value::Nil;
                if let Some(expr) = initializer {
                    value = self.evaluate_expr(expr)?;
                }

                self.environment.define(name.lexeme.to_owned(), value);
                Ok(V::Nil)
            }
            Stmt::Block(statements) => {
                let previous_env = Rc::clone(&self.environment);

                self.environment = Rc::new(Environment::with_enclosing(Rc::clone(&previous_env)));

                let _ = self.execute_block(statements);

                self.environment = previous_env;

                Ok(V::Nil)
            }
        }
    }

    fn execute_block(&mut self, statements: &Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn evaluate_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Literal(val) => Ok(val.clone()),
            Expr::Grouping(expr) => self.evaluate_expr(expr),
            Expr::Unary { operator, right } => self.evaluate_unary(operator, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => self.evaluate_binary(left, operator, right),
            Expr::Variable(name) => self.evaluate_variable(name),
            Expr::Assign(name, expr) => {
                let value = self.evaluate_expr(expr)?;
                match self
                    .environment
                    .assign(name.lexeme.to_owned(), value.clone())
                {
                    Ok(_) => Ok(value),
                    Err(_) => Err(RE::AssignUndefinedVariable(name.clone())),
                }
            }
        }
    }

    fn evaluate_unary(&mut self, operator: &Token, right: &Expr) -> Result<Value, RuntimeError> {
        let right_value = self.evaluate_expr(right)?;

        match operator.token_type {
            TT::Bang => Ok(V::Bool(!is_truthy(&right_value))),
            TT::Minus => {
                let right_num = expect_number_operand(operator, right_value)?;
                Ok(V::Number(-right_num))
            }
            _ => Ok(V::Nil),
        }
    }

    fn evaluate_variable(&self, name: &Token) -> Result<Value, RuntimeError> {
        match self.environment.get(name) {
            Some(val) => Ok(val.clone()),
            None => Err(RE::UndefinedVariable(name.clone())),
        }
    }

    fn evaluate_binary(
        &mut self,
        left: &Expr,
        operator: &Token,
        right: &Expr,
    ) -> Result<Value, RuntimeError> {
        let left_value = self.evaluate_expr(left)?;
        let right_value = self.evaluate_expr(right)?;

        match operator.token_type {
            TT::BangEqual => Ok(V::Bool(!is_equal(&left_value, &right_value))),
            TT::Equal => Ok(V::Bool(is_equal(&left_value, &right_value))),
            TT::Greater => {
                let (left_num, right_num) =
                    expect_number_operands(operator, left_value, right_value)?;
                Ok(V::Bool(left_num > right_num))
            }
            TT::GreaterEqual => {
                let (left_num, right_num) =
                    expect_number_operands(operator, left_value, right_value)?;
                Ok(V::Bool(left_num >= right_num))
            }
            TT::Less => {
                let (left_num, right_num) =
                    expect_number_operands(operator, left_value, right_value)?;
                Ok(V::Bool(left_num < right_num))
            }
            TT::LessEqual => {
                let (left_num, right_num) =
                    expect_number_operands(operator, left_value, right_value)?;
                Ok(V::Bool(left_num <= right_num))
            }
            TT::Minus => {
                let (left_num, right_num) =
                    expect_number_operands(operator, left_value, right_value)?;
                Ok(V::Number(left_num - right_num))
            }
            TT::Slash => {
                let (left_num, right_num) =
                    expect_number_operands(operator, left_value, right_value)?;
                Ok(V::Number(left_num / right_num))
            }
            TT::Star => {
                let (left_num, right_num) =
                    expect_number_operands(operator, left_value, right_value)?;
                Ok(V::Number(left_num * right_num))
            }
            TT::Plus => {
                if left_value.is_number() && right_value.is_number() {
                    let (left_num, right_num) =
                        expect_number_operands(operator, left_value, right_value)?;
                    Ok(V::Number(left_num + right_num))
                } else {
                    let (left_str, right_str) =
                        expect_string_operands(operator, left_value, right_value)?;
                    Ok(V::String(format!("{}{}", left_str, right_str)))
                }
            }
            _ => Ok(V::Nil),
        }
    }
}

fn is_equal(left: &Value, right: &Value) -> bool {
    if left.is_nil() && right.is_nil() {
        true
    } else if left.is_number() && right.is_number() {
        left.as_number().unwrap() == right.as_number().unwrap()
    } else if left.is_string() && right.is_string() {
        left.as_string().unwrap() == right.as_string().unwrap()
    } else if left.is_bool() && right.is_bool() {
        left.as_bool().unwrap() == right.as_bool().unwrap()
    } else {
        false
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Nil => false,
        Value::Bool(val) => *val,
        _ => true,
    }
}

fn expect_number_operand(operator: &Token, val: Value) -> Result<f64, RuntimeError> {
    val.as_number()
        .ok_or_else(|| RE::OperandMustBeNumber(operator.clone()))
}

fn expect_number_operands(
    operator: &Token,
    left: Value,
    right: Value,
) -> Result<(f64, f64), RuntimeError> {
    let left_num = left
        .as_number()
        .ok_or_else(|| RE::OperandsMustBeNumbers(operator.clone()))?;
    let right_num = right
        .as_number()
        .ok_or_else(|| RE::OperandsMustBeNumbers(operator.clone()))?;
    Ok((left_num, right_num))
}

fn expect_string_operands(
    operator: &Token,
    left: Value,
    right: Value,
) -> Result<(String, String), RuntimeError> {
    let left_str = left
        .as_string()
        .ok_or_else(|| RE::OperandMustBeNumber(operator.clone()))?;
    let right_str = right
        .as_string()
        .ok_or_else(|| RE::OperandsMustBeStrings(operator.clone()))?;

    Ok((left_str, right_str))
}
