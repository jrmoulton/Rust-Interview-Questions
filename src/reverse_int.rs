#[allow(dead_code)]
pub fn reverse_int(mut x: i64) -> i32 {
    let mut new = x % 10;
    x /= 10;
    while x != 0 {
        new *= 10;
        new += x % 10;
        //println!("{}", new);
        x /= 10;
    }
    println!("{}", i32::MAX);
    if new > i32::MAX as i64 - 1 || new < -(i32::MIN as i64) {
        return 0;
    }
    new as i32
}

#[cfg(test)]
mod reverse_int_test {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(reverse_int(123), 321);
    }
    #[test]
    fn test2() {
        assert_eq!(reverse_int(100), 1);
    }
    #[test]
    fn test3() {
        assert_eq!(reverse_int(1534236469), 0)
    }
    //    1534236469
    //-1563847412
    //-2147483412
}
