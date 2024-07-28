#[macro_export]
macro_rules! parenthesize {
    ($name:expr, $( $expr:expr ),* ) => {
        {
            let mut builder = String::new();
            builder.push('(');
            builder.push_str(&$name);
            $(
                builder.push(' ');
                builder.push_str(&$expr.accept());
            )*
            builder.push(')');
            builder
        }
    };
}
