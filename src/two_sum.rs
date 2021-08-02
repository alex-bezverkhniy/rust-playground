struct Solution;

impl Solution {
// [2, 7, 11, 15] t = 9
// [3, 4, 2] t = 6
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut j = 0;
        
        for (i, &num) in nums.iter().enumerate() {
            if i > 0 { 
                if nums[j] + num == target {
                    return vec![j as i32, i as i32]
                }
                j = i;
            }
        }
    
        vec![]
    }
}

#[test]
fn test1() {
    let expected = vec![0, 1];
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    
    let actual = Solution::two_sum(nums, target);
    
    assert_eq!(expected, actual);
}

#[test]
fn test2() {
    let expected = vec![1, 2];
    let nums = vec![3, 4, 2];
    let target = 6;
    
    let actual = Solution::two_sum(nums, target);
    
    assert_eq!(expected, actual);
}

#[test]
fn test3() {
    let expected = vec![0, 1];
    let nums = vec![3, 4];
    let target = 7;
    
    let actual = Solution::two_sum(nums, target);
    
    assert_eq!(expected, actual);
}