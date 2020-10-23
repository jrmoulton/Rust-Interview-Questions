fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if (nums[i] + nums[j]) as i32 == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}

fn main() {
    two_sum([2, 7, 11, 15].to_vec(), 9);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
    }
    #[test]
    fn test2() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 13), [0, 2]);
    }
    #[test]
    fn test3() {
        assert_eq!(two_sum(vec![4, 7, 6, 19, 2], 10), [0, 2]);
    }
    #[test]
    fn test4() {
        assert_eq!(two_sum(vec![4, 7, 6, 19, 2], 9), [1, 4]);
    }
}
