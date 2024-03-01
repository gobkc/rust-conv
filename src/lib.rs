pub mod conv{
    pub fn any<T, U>(value: T) -> U
        where
            T: std::fmt::Display + std::str::FromStr,
            U: std::str::FromStr + std::default::Default,
            <T as std::str::FromStr>::Err: std::fmt::Debug,
            <U as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let value_str = format!("{}", value);
        value_str.parse().unwrap_or_else(|_| {
            eprintln!("Failed to parse value: {}", value_str);
            Default::default()
        })
    }
}