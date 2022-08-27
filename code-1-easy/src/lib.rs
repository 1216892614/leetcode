pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i0 in 0..nums.len() {
        for i1 in i0+1..nums.len() {
            if target == nums[i0] + nums[i1] {
                return vec![i0 as i32, i1 as i32];
            }
        }
    }
    Vec::new()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{:?}",crate::two_sum(vec![2,7,11,15], 9));
    }
}
