use crate::{parenthesize, tokens::Token};

pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

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
    Literal(Literal),
}

impl Expr {
    pub fn accept(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => parenthesize!(operator.lexeme, left, right),
            Expr::Literal(literal) => match literal {
                Literal::Number(num) => num.to_string(),
                Literal::String(str) => str.to_owned(),
                Literal::Bool(value) => {
                    if *value {
                        "true".to_owned()
                    } else {
                        "false".to_owned()
                    }
                }
                Literal::Nil => "nil".to_owned(),
            },
            Expr::Unary { operator, right } => parenthesize!(operator.lexeme, right),
            Expr::Grouping(expr) => parenthesize!("group", expr),
        }
    }
}
