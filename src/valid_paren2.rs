pub fn is_valid(s: String) -> bool {
    use std::collections::HashMap;
    let mut stack = vec![];
    let mut matches = HashMap::new();
    matches.insert(')', '(');
    matches.insert('}', '{');
    matches.insert(']', '[');
    for val in s.chars() {
        match val {
            '(' | '{' | '[' => stack.push(val),
            ')' | '}' | ']' => {
                if match stack.pop() {
                    Some(thing) => thing,
                    None => return false,
                } != *matches.get(&val).unwrap()
                // can unwrap because chars in the string is limited
                {
                    return false;
                }
            }
            _ => return false,
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod valid_paren2_tests {
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
