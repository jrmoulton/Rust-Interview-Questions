pub fn reverse(x: i32) -> i32 {
    let mut x = x as i64;
    let mut new: i64 = x % 10;
    x /= 10;
    while x != 0 {
        new *= 10;
        new += x % 10;
        x /= 10;
    }
    if new > std::i32::MAX as i64 - 1 || new < std::i32::MIN as i64 {
        return 0;
    }
    new as i32
}

#[cfg(test)]
mod reverse_int_test {
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
