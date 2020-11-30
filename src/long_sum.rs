use std::time::Instant;

//"dvdf"
#[allow(dead_code)]
pub fn length_of_longest_substring(s: &str) -> i32 {
    if s.len() < 2 {
        return s.len() as i32;
    }
    let mut set: Vec<char> = vec![];
    let mut start = 0;
    let mut end = 0;
    let mut longest = 0;
    let mut ch;

    while end != s.len() {
        ch = s.as_bytes()[end] as char;
        if !set.contains(&ch) {
            set.push(ch);
            end += 1;
            if set.len() > longest {
                longest = set.len();
            }
        } else {
            while dbg!(&s[start..end]).contains(ch) {
                start += 1;
                set.remove(0);
            }
        }
    }
    longest as i32
}

#[allow(dead_code)]
pub fn length_of_longest_substring_2(s: &str) -> i32 {
    let time_start = Instant::now();
    let s = s.as_bytes();
    s.into_iter();
    let mut max_len = 0;
    let mut start = 0usize;
    let mut map = std::collections::HashMap::new();
    for (i, letter) in s.iter().enumerate() {
        if let Some(&left_index) = map.get(letter) {
            start = start.max(left_index + 1);
        }
        max_len = max_len.max(i - start + 1);
        // insert or update
        map.insert(*letter, i);
    }
    let duration = time_start.elapsed();
    println!("Time was: {:?}", duration);
    max_len as i32
}

#[cfg(test)]
mod long_sum_tests {
    use super::*;
    #[test]
    fn long_test1() {
        assert_eq!(length_of_longest_substring_2("abcabcbb"), 3);
    }
    #[test]
    fn long_test2() {
        assert_eq!(length_of_longest_substring_2("bbbbb"), 1);
    }

    #[test]
    fn long_test3() {
        assert_eq!(length_of_longest_substring_2("pwwkew"), 3)
    }

    #[test]
    fn long_test4() {
        assert_eq!(length_of_longest_substring_2(""), 0)
    }

    #[test]
    fn long_test5() {
        assert_eq!(length_of_longest_substring_2(" "), 1)
    }
    #[test]
    fn long_test6() {
        assert_eq!(length_of_longest_substring_2("dvdf"), 3)
    }
}
