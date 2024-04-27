pub fn add(input: &str) -> i32 {
    let delimiters = &get_delimiters(&input)[..];
    let split_input = input.split(delimiters);
    let numbers = split_input.map(|x| x.parse::<i32>().unwrap_or(0));
    panic_on_negatives(numbers.clone());
    numbers.fold(0, |acc, x| acc + x)
}

pub fn getcount() -> i32 {
    1
}

fn get_delimiters(input: &str) -> Vec<char> {
    let mut delimiters = vec![',', '\n'];
    if input.starts_with("//") {
        let delimiter = &input.chars().nth(2).unwrap();
        delimiters.push(*delimiter);
    }
    delimiters
}

fn panic_on_negatives<'a, I>(input: I)
where
    I: Iterator<Item = i32>,
{
    let mut negative_numbers = input.filter(|x| x < &0).peekable();
    if negative_numbers.peek().is_some() {
        let negs = format!("{:?}", negative_numbers.collect::<Vec<_>>());
        panic!("negatives not allowed: {negs}")
    }
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

    #[test]
    fn custom_delimiter_runs() {
        assert_eq!(add("//;\n1;2"), 3, "custom delimiter failed");
    }

    #[test]
    #[should_panic(expected = "negatives not allowed: [-2]")]
    fn negative_thows_exception() {
        add("1,-2");
    }

    #[test]
    #[should_panic(expected = "negatives not allowed: [-2, -4]")]
    fn negatives_thow_exception() {
        add("1,-2,3,-4");
    }

    #[test]
    fn get_count_works() {
        assert_eq!(getcount(), 8)
    }

}
