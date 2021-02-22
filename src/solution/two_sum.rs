/*
 * https://leetcode.com/problems/two-sum/submissions/
 */

use std::collections::HashMap;

pub fn two_sum_v1(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
    nums.sort();

    let mut s: usize = 0;
    let mut e: usize = nums.len() - 1;

    let mut result: Vec<i32> = Vec::new();

    loop {
        if s >= e {
            break;
        }
        let sum = nums[s] + nums[e];

        if sum == target {
            result.push(s as i32);
            result.push(e as i32);
        } else if sum > target {
            e = e - 1;
        } else {
            s = s + 1;
        }
    }

    return result;
}

pub fn two_sum_v2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut index = HashMap::new();

    for (k, v) in nums.iter().enumerate() {
        index.insert(v, k);
    }

    let mut result: Vec<i32> = Vec::new();

    for (k, v) in nums.iter().enumerate() {
        let left = target - v;

        println!("{:?} - {}", index, left);

        match index.get(&left) {
            Some(f) => {
                if k != *f {
                    result.push(k as i32);
                    result.push(*f as i32);

                    break;
                }
            }
            None => (),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_v2() {
        assert_eq!(two_sum_v2(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum_v2(vec![3, 2, 4], 6), vec![1, 2]);
    }
}
