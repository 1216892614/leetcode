pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    fn sieve_of_era(top_size: usize, factor_num: usize) -> HashMap<usize, Vec<usize>> {
        let factor_num = factor_num - 2;

        let mut buf = vec![Vec::new(); top_size];
        let mut ans = HashMap::new();

        for i in 2..top_size {
            if buf[i].len() <= factor_num {
                if buf[i].len() == factor_num {
                    ans.insert(i, buf[i].clone());
                }
                let mut j = i + i;
                while j < top_size {
                    buf[j].push(i);
    
                    j += i;
                }
            }
        }

        ans
    }
    let dictionary = sieve_of_era(*nums.iter().max().unwrap() as usize + 1, 4);

    let ans: usize = nums
        .into_iter()
        .map(|i| -> usize {
            let mut a = dictionary
                .get(&(i as usize))
                .unwrap_or(&vec![0])
                .iter()
                .sum();

            if a != 0 {
                a += i as usize + 1;
            }
            
            a
        })
        .sum();

    ans as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{:?}", crate::sum_four_divisors(vec![21, 4, 7]))
    }
}
