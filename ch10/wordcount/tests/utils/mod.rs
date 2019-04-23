macro_rules! assert_map {
    ($expr: expr, {$($key: expr => $value:expr),*}) => {
        $(assert_eq!($expr[$key], $value));*
    };
}
