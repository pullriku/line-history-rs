#[macro_export]
macro_rules! read_from_file {
    ($path:expr, let $var_src:ident, let $var_history:ident) => {
        let $var_src = std::fs::read_to_string($path).unwrap();
        let $var_history = $crate::parse::parse_history(&$var_src);
    };
}
