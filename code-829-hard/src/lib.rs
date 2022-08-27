pub fn consecutive_numbers_sum(n: i32) -> i32 {
    (1..)
        .take_while(|k| k * (k + 1) <= 2 * n)
        .filter(|k| 2 * n % k == 0 && (2 * n / k - k) & 1 != 0)
        .count() as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{:?}", crate::consecutive_numbers_sum(15));
    }
}
