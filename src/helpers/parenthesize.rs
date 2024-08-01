#[macro_export]
macro_rules! parenthesize {
    ($name:expr, $( $val:expr ),* ) => {
        {
            let mut builder = String::new();
            builder.push('(');
            builder.push_str(&$name);
            $(
                builder.push(' ');
                builder.push_str(&$val);
            )*
            builder.push(')');
            builder
        }
    };
}
