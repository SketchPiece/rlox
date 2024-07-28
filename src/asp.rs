use crate::{
    expr::{Expr, Literal},
    tokens::{Token, TokenType},
};

pub fn print_asp(expr: Expr) -> String {
    expr.accept()
}

pub fn print_asp_main() {
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: "-".into(),
                line: 1,
            },
            right: Box::new(Expr::Literal(Literal::Number(123.0))),
        }),
        operator: Token::new(TokenType::Star, "*".into(), 1),
        right: Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Number(
            45.67,
        ))))),
    };

    println!("{}", print_asp(expression));
}
