pub fn urlify(string: &str, _length: i32) -> String {
    string.trim().replace(" ", "%20")
}

#[cfg(test)]
mod urlify_test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            urlify("Mr John Smith  ", 13),
            "Mr%20John%20Smith".to_string()
        );
    }
}
