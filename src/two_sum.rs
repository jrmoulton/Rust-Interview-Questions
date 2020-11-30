#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if (nums[i] + nums[j]) as i32 == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

#[cfg(test)]
mod two_sum_tests {
    use super::*;
    #[test]
    fn sum_test1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
    }
    #[test]
    fn sum_test2() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 13), [0, 2]);
    }
}
