use crate::{parenthesize, tokens::Token};

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl Value {
    pub fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(num) => Some(*num),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<String> {
        match self {
            Self::String(str) => Some(str.clone()),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(val) => Some(*val),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    pub fn is_nil(&self) -> bool {
        matches!(self, Self::Nil)
    }

    pub fn stringify(&self) -> String {
        match self {
            Self::Number(num) => num.to_string(),
            Self::String(str) => str.to_owned(),
            Self::Bool(val) => val.to_string(),
            Self::Nil => "nil".to_owned(),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(Value),
    Variable(Token),
    Assign(Token, Box<Expr>),
}

impl Expr {
    pub fn stringify(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => parenthesize!(operator.lexeme, left.stringify(), right.stringify()),
            Expr::Literal(literal) => match literal {
                Value::Number(num) => num.to_string(),
                Value::String(str) => format!("\"{}\"", str.to_owned()),
                Value::Bool(value) => {
                    if *value {
                        "true".to_owned()
                    } else {
                        "false".to_owned()
                    }
                }
                Value::Nil => "nil".to_owned(),
            },
            Expr::Unary { operator, right } => parenthesize!(operator.lexeme, right.stringify()),
            Expr::Grouping(expr) => parenthesize!("group", expr.stringify()),
            Expr::Variable(name) => name.lexeme.to_owned(),
            Expr::Assign(name, expr) => parenthesize!("assign", name.lexeme, expr.stringify()),
        }
    }
}

pub enum Stmt {
    Block(Box<Vec<Stmt>>),
    Expression(Expr),
    Print(Expr),
    Var(Token, Option<Expr>),
}
