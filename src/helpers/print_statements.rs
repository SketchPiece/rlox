use crate::ast::Stmt;

fn gen_indent(indent: usize) -> String {
    "\t".repeat(indent)
}

pub fn print_statements(statements: &[Stmt], indent: usize) {
    println!("Statements:");
    for statement in statements {
        match statement {
            Stmt::Expression(expr) => {
                println!("{}Expression: {}", gen_indent(indent), expr.stringify())
            }
            Stmt::Print(expr) => println!("{}Print: {}", gen_indent(indent), expr.stringify()),
            Stmt::Var(token, value) => println!(
                "{}Var: {} = {}",
                gen_indent(indent),
                token.lexeme,
                if let Some(expr) = value {
                    expr.stringify()
                } else {
                    "nil".to_owned()
                },
            ),
            Stmt::Block(statements) => print_statements(statements, indent + 1),
        }
    }
}
