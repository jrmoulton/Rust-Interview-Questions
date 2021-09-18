pub fn is_power_three(n: i32) -> bool {
    let f = (n as f64).log(3.0).fract();
    if !(0.0000000000001..=0.9999999999999).contains(&f) {
        return true;
    }
    false
}

#[allow(dead_code)]
fn is_power_three_fast(n: i32) -> bool {
    let mut n = n;
    if n < 1 {
        false
    } else {
        while n % 3 == 0 {
            n /= 3;
        }
        n == 1
    }
}

#[cfg(test)]
mod power_three_tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(is_power_three_fast(27), true);
    }
    #[test]
    fn test2() {
        assert_eq!(is_power_three_fast(9), true);
    }
    #[test]
    fn test3() {
        assert_eq!(is_power_three_fast(81), true);
    }
    #[test]
    fn test4() {
        assert_eq!(is_power_three_fast(243), true);
    }
}
