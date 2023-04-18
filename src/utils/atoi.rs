pub fn atoi<T: std::str::FromStr + std::default::Default>(str: Option<String>) -> Option<T> {
    str.map(|int_str| int_str.parse::<T>().unwrap_or_default())
}

pub fn atoi64(str: Option<String>) -> Option<i64> {
    atoi::<i64>(str)
}
