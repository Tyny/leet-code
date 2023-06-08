struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i1, n1) in nums.iter().enumerate() {
            for (i2, n2) in nums.iter().enumerate() {
                if n1 + n2 == target && i1 != i2 {
                    return vec![i1 as i32, i2 as i32];
                }
            }
        }

        vec![]
    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}
