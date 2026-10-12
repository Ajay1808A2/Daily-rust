struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut low: i32 = 0;
        let mut high: i32 = x;
        let mut ans: i32 = 0;

        while low <= high{
            let mid: i32 = low + (high - low) / 2;
            if mid * mid <= x{
                ans = mid;
                low = mid + 1;
            }else{
                high = mid - 1;
            }
        }

        return ans;
    }
}

fn main(){
    let output: i32 = Solution::my_sqrt(25);
    let output2: i32 = Solution::my_sqrt(16);
    let output3: i32 = Solution::my_sqrt(1000);

    println!("{}", output);
    println!("{}", output2);
    println!("{}", output3);
}
