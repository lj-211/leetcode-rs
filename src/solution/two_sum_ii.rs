/*
 *
 * https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
 *
 */

struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut start: usize = 0;
        let mut end = numbers.len() - 1;

        let mut result: Vec<i32> = Vec::new();

        loop {
            if start < end {
                let sum = numbers[start] + numbers[end];
                if sum < target {
                    start += 1;
                } else if sum > target {
                    end -= 1;
                } else {
                    result.push(start as i32 + 1);
                    result.push(end as i32 + 1);
                    break;
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_ii() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
    }
}
