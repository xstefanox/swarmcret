#[macro_export]
macro_rules! assert_ok {
    ($expected:expr,$actual:expr) => {{
        assert_eq!(Ok($expected), $actual)
    }};
}

#[macro_export]
macro_rules! assert_err {
    ($expected:expr,$actual:expr) => {{
        assert_eq!(Err($expected), $actual)
    }};
}
