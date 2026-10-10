struct Solution;

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x<0 {
            return false;
        }

        let org_no: i32 = x;
        let mut rev_no: i32 = 0;

        while x > 0 {
            let l_digit: i32 = x % 10;
            rev_no = (rev_no * 10) + l_digit;
            x = x/10;
        }

        return org_no == rev_no;
    }
}

fn main(){
    let res: bool = Solution::is_palindrome(151);
    let res2: bool = Solution::is_palindrome(1121);

    println!("{}", res);
    println!("{}", res2);
}
