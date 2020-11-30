#[allow(dead_code)]
pub fn reverse_string(s: &mut Vec<char>) {
    let l = s.len();
    for i in 0..l / 2 {
        s.swap(i, l - i - 1);
    }
}

#[cfg(test)]
mod reverse_string_tests {
    use super::*;

    #[test]
    fn test1() {
        let s = &mut vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(s);
        assert_eq!(*s, vec!['o', 'l', 'l', 'e', 'h']);
    }
}
