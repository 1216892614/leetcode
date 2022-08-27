use rand::distributions::{Distribution, Uniform};

struct Solution {
    x: f64,
    y: f64,
    r: f64,
    r_pow: f64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Solution {
            x: x_center,
            y: y_center,
            r: radius,
            r_pow: radius * radius,
        }
    }

    fn rand_point(&self) -> Vec<f64> {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(-self.r..=self.r);
        loop {
            let (x, y): (f64, f64) = (die.sample(&mut rng), die.sample(&mut rng));
            if x * x + y * y <= self.r_pow {
                return vec![x + self.x, y + self.y];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    /**
     * Your Solution object will be instantiated and called as such:
     * let obj = Solution::new(radius, x_center, y_center);
     * let ret_1: Vec<f64> = obj.rand_point();
     */
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
