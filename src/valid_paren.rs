use std::collections::HashMap;

#[allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    let open = vec!['(', '[', '{'];
    let mut stack: Vec<char> = vec![];
    let pair: HashMap<char, char> = [(')', '('), (']', '['), ('}', '{')]
        .iter()
        .cloned()
        .collect();
    for chr in s.chars() {
        if open.contains(&chr) {
            stack.push(chr);
        } else {
            if pair.get(&chr) == stack.last() {
                stack.truncate(stack.len() - 1);
            } else {
                return false;
            }
        }
    }
    if stack.len() != 0 {
        return false;
    }
    true
}

#[cfg(test)]
mod valid_paren_tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(is_valid("([)]".to_string()), false)
    }
    #[test]
    fn test2() {
        assert_eq!(is_valid("[".to_string()), false);
    }
}
