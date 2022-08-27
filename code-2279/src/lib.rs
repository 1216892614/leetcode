pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
    let mut capacity = capacity
        .iter()
        .zip(&rocks)
        .map(|(cap, rock)| *cap - *rock)
        .collect::<Vec<i32>>();
    let mut additional_rocks = additional_rocks.clone();
    let mut ans = 0;

    capacity.sort();
    for cap in capacity {
        if additional_rocks - cap >= 0 {
            additional_rocks -= cap;
            ans += 1;
        } else {
            return ans;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!(
            "{}",
            crate::maximum_bags(vec![2, 3, 4, 5], vec![1, 2, 4, 4], 2)
        )
    }
}
