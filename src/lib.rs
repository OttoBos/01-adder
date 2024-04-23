pub fn add(a: &str) -> i32 {
    if a == "" {
        return 0;
    }
    a.parse::<i32>().unwrap()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn empty_string_returns_zero() {
        assert_eq!(add(""), 0);
    }

    #[test]
    fn single_number_runs() {
        assert_eq!(add("1"), 1, "Single number tests failed");
    }
}
