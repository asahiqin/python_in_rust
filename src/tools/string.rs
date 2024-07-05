#[macro_export]
macro_rules! strip_quotes {
    ($s:expr) => {
        $s.trim_matches('"')
    };
}

#[macro_export]
macro_rules! indent_detection {
    ($s:expr) => {
        $s.chars().all(|c| c == ' ' || c == '\t')
    };
}

#[macro_export]
macro_rules! count_char_occurrences {
    ($string:expr, $char:expr) => {
        $string.chars().filter(|&c| c == $char).count()
    };
}

