/*
You are given coins of different denominations and a total amount of money
amount. Write a function to compute the fewest number of coins that you need to
make up that amount. If that amount of money cannot be made up by any
combination of the coins, return -1.

You may assume that you have an infinite number of each kind of coin.
*/

pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
    let mut count = 0;
    let mut total = 0;
    coins.sort_unstable();
    for coin in coins.clone().into_iter().rev() {
        while total + coin <= amount {
            total += coin;
            count += 1;
        }
    }
    if total != amount {
        if coins.is_empty() {
            return -1;
        } else {
            return coin_change(coins[..coins.len() - 1].to_vec(), amount);
        }
    }
    count
}

#[cfg(test)]
mod coin_change_tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn test2() {
        assert_eq!(coin_change(vec![2], 3), -1);
    }

    #[test]
    fn test3() {
        assert_eq!(coin_change(vec![1], 0), 0);
    }

    #[test]
    fn test4() {
        assert_eq!(coin_change(vec![186, 419, 83, 408], 6249), 20)
    }
}
