pub fn add(input: &str) -> i32 {
    let split_input = input.split(&[',', '\n']);
    let numbers = split_input.map(|x| x.parse::<i32>().unwrap_or(0));
    numbers.fold(0, |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_returns_zero() {
        assert_eq!(add(""), 0);
    }

    #[test]
    fn single_number_runs() {
        assert_eq!(add("1"), 1, "Single number tests failed");
    }

    #[test]
    fn two_numbers_run() {
        assert_eq!(add("1,2"), 3, "2 number tests failed");
    }

    #[test]
    fn several_numbers_run() {
        assert_eq!(add("0,1,2,3"), 6, "4 number tests failed");
    }

    #[test]
    fn newline_runs() {
        assert_eq!(add("0,1\n2,3"), 6, "newline failed");
    }
}
