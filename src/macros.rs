#[macro_export]
macro_rules! unite {
    () => {
        ValueUnion::new()
    };
    ($($x:expr),*) => {{
        let mut result = ValueUnion::new();
        $(
            result.push_value($x);
        )*
        result
    }};
}
