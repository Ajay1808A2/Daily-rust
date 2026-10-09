struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len(){
            for j in (i+1)..nums.len(){
                if nums[i] + nums[j] == target{
                    return vec![i as i32, j as i32];
                }
            }
        }

        vec![]
    }
}

fn main(){
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let target: i32 = 5;
    println!("Original Array: {:?}", nums);
    println!("Target number: {}", target);
    let result: Vec<i32> = Solution::two_sum(nums, target);
    println!("Two Sum: {:?}", result);
}
