trait MyString {
    fn my_string(self) -> String;
}
impl MyString for i32 {
    fn my_string(self) -> String {
        format!("{}", self)
    }
}

pub fn divide(dividend: i32, divisor: i32) -> i32 {
    let mut positive = true;
    if dividend < 0 {
        positive = !positive;
    }
    if divisor < 0 {
        positive = !positive;
    }
    let dividend = dividend as f64;
    let dividend = dividend.abs();

    let divisor = divisor as f64;
    let divisor = divisor.abs();

    let e = std::f64::consts::E;
    if positive {
        ((e.powf(dividend.log(e) - divisor.log(e)) * 100.0).round() / 100.0) as i32
    } else {
        ((-e.powf(dividend.log(e) - divisor.log(e)) * 100.0).round() / 100.0) as i32
    }
}

#[cfg(test)]
mod divide_int_tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(divide(10, 2), 5);
    }
    #[test]
    fn test2() {
        assert_eq!(divide(-2147483648, -1), 2147483647);
    }
    #[test]
    fn test3() {
        assert_eq!(divide(1, 2), 0)
    }
    #[test]
    fn test4() {
        assert_eq!(divide(2147483647, 1), 2147483647)
    }
}
