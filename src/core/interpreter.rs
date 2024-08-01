use super::{
    expr::{Expr, Value},
    tokens::{Token, TokenType},
};

use TokenType as TT;
use Value as V;

pub enum InterpreterError {
    OperandMustBeNumber(Token),
}

pub struct Interpreter {
    expr: Expr,
}

// ! refactor to return error instead of expect
impl Interpreter {
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }

    pub fn interpret(&self) {
        // ! handle error here
        let value = self.evaluate_expr(&self.expr);
        println!("{}", stringify(&value));
    }

    fn evaluate_expr(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal(val) => val.clone(),
            Expr::Grouping(expr) => self.evaluate_expr(expr),
            Expr::Unary { operator, right } => self.evaluate_unary(operator, right),
            Expr::Binary {
                left,
                operator,
                right,
            } => self.evaluate_binary(left, operator, right),
        }
    }

    fn evaluate_unary(&self, operator: &Token, right: &Expr) -> Value {
        let right_value = self.evaluate_expr(right);

        match operator.token_type {
            TT::Bang => V::Bool(!is_truthy(&right_value)),
            TT::Minus => {
                // let right_num = expect_number_operand(&operator, right_value)?;
                // V::Number(-right_num)
                V::Number(
                    -right_value
                        .as_number()
                        .expect("Using minus operator with non number value"),
                )
            }
            _ => V::Nil,
        }
    }

    fn evaluate_binary(&self, left: &Expr, operator: &Token, right: &Expr) -> Value {
        let left_value = self.evaluate_expr(left);
        let right_value = self.evaluate_expr(right);

        match operator.token_type {
            TT::BangEqual => V::Bool(!is_equal(&left_value, &right_value)),
            TT::Equal => V::Bool(is_equal(&left_value, &right_value)),
            TT::Greater => {
                let left_num = left_value
                    .as_number()
                    .expect("Left expression is not a number");
                let right_num = right_value
                    .as_number()
                    .expect("Right expression is not a number");
                V::Bool(left_num > right_num)
            }
            TT::GreaterEqual => {
                let left_num = left_value
                    .as_number()
                    .expect("Left expression is not a number");
                let right_num = right_value
                    .as_number()
                    .expect("Right expression is not a number");
                V::Bool(left_num >= right_num)
            }
            TT::Less => {
                let left_num = left_value
                    .as_number()
                    .expect("Left expression is not a number");
                let right_num = right_value
                    .as_number()
                    .expect("Right expression is not a number");
                V::Bool(left_num < right_num)
            }
            TT::LessEqual => {
                let left_num = left_value
                    .as_number()
                    .expect("Left expression is not a number");
                let right_num = right_value
                    .as_number()
                    .expect("Right expression is not a number");
                V::Bool(left_num <= right_num)
            }
            TT::Minus => {
                let left_num = left_value
                    .as_number()
                    .expect("Left expression is not a number");
                let right_num = right_value
                    .as_number()
                    .expect("Right expression is not a number");
                V::Number(left_num - right_num)
            }
            TT::Slash => {
                let left_num = left_value
                    .as_number()
                    .expect("Left expression is not a number");
                let right_num = right_value
                    .as_number()
                    .expect("Right expression is not a number");
                V::Number(left_num / right_num)
            }
            TT::Star => {
                let left_num = left_value
                    .as_number()
                    .expect("Left expression is not a number");
                let right_num = right_value
                    .as_number()
                    .expect("Right expression is not a number");
                V::Number(left_num * right_num)
            }
            TT::Plus => {
                if left_value.is_number() && right_value.is_number() {
                    let left_num = left_value
                        .as_number()
                        .expect("Left expression is not a number");
                    let right_num = right_value
                        .as_number()
                        .expect("Right expression is not a number");
                    V::Number(left_num + right_num)
                } else {
                    let left_str = left_value
                        .as_string()
                        .expect("Left expression is not a string");
                    let right_str = right_value
                        .as_string()
                        .expect("Right expression is not a string");
                    V::String(format!("{}{}", left_str, right_str))
                }
            }
            _ => V::Nil,
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
// todo: delete empty decimal for number
fn stringify(value: &Value) -> String {
    match value {
        V::Number(num) => num.to_string(),
        V::String(str) => str.to_owned(),
        V::Bool(val) => val.to_string(),
        V::Nil => "nil".to_owned(),
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Nil => false,
        Value::Bool(val) => *val,
        _ => true,
    }
}

// fn expect_number_operand(operator: &Token, val: Value) -> Result<f64, InterpreterError> {
//     val.as_number()
//         .ok_or_else(|| InterpreterError::OperandMustBeNumber(operator.clone()))
// }

// todo: implement reporting
// fn report_runtime_error() {}
