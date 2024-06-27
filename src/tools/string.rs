#[macro_export]
macro_rules! strip_quotes {
    ($s:expr) => {
        $s.trim_matches('"')
    };
}
