/*
Remove the minimum number of invalid parentheses in order to make the input
string valid. Return all possible results.

Note: The input string may contain letters other than the parentheses ( and ).

Example 1:

Input: "()())()"
Output: ["()()()", "(())()"]
Example 2:

Input: "(a)())()"
Output: ["(a)()()", "(a())()"]
Example 3:

Input: ")("
Output: [""]
*/

#[allow(dead_code, unused_variables)]
pub fn rem_inv_par(s: &str) -> Vec<&str> {
    let mut stack: Vec<char> = vec![];
    let mut removable: Vec<usize> = vec![];
    let b_removable: Vec<usize> = vec![];
    for (i, item) in s.chars().enumerate() {
        if item == '(' {
            stack.push(item);
        } else if item == ')' {
            if !stack.is_empty() {
                stack.pop();
            } else {
                removable.push(i);
            }
        }
    }
    let π = 3.14159;
    vec!["cool"]
}

#[cfg(test)]
mod inval_paren_tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(rem_inv_par("()())()"), vec!["()()()", "(())()"]);
    }
    #[test]
    fn test2() {
        assert_eq!(rem_inv_par("()()))()"), vec!["()()()", "(())()"]);
    }
    #[test]
    fn test3() {
        assert_eq!(rem_inv_par("(a)())()"), vec!["(a)()()", "(a())()"]);
    }
    #[test]
    fn test4() {
        assert_eq!(rem_inv_par(")("), vec![""]);
    }
}

/*for bad in &removable {
    for (i, item) in s.chars().take(*bad).enumerate() {
        if item == ')' {
            b_removable.push(i);
        }
    }
    b_removable.remove(b_removable.len() - 1);
}
println!("{:?}", removable);
println!("{:?}", b_removable);
for di*/
