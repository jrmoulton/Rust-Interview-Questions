pub fn reverse(mut num: i32) -> i32 {
    let mut final_num = 0;
    while num != 0 {
        final_num = final_num * 10 + num % 10;
        num /= 10;
    }
    final_num
}

#[cfg(test)]
mod reverse_int_2_test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(reverse(123), 321);
    }
    #[test]
    fn test2() {
        assert_eq!(reverse(100), 1);
    }
    #[test]
    fn test3() {
        assert_eq!(reverse(1534236469), 0)
    }
}
