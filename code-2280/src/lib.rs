pub fn minimum_lines(stock_prices: Vec<Vec<i32>>) -> i32 {
    let mut last_slope: Option<(i32, i32)> = None;
    let mut ans = 0;
    let mut stock_prices = stock_prices;
    stock_prices.sort_by_key(|i| i[0]);

    for stock_0 in 0..(stock_prices.len() - 1) {
        let stock_1 = stock_0 + 1;
        let slope = (
            (stock_prices[stock_1][1] - stock_prices[stock_0][1]),
            (stock_prices[stock_1][0] - stock_prices[stock_0][0]),
        );

        match last_slope {
            None => ans += 1,
            Some(last_slope) if slope.0 * last_slope.1 != slope.1 * last_slope.0 => ans += 1,
            _ => (),
        }

        last_slope = Some(slope);
    }
    ans
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!(
            "{}",
            crate::minimum_lines(
                (vec![
                    [1, 7],
                    [2, 6],
                    [3, 5],
                    [4, 4],
                    [5, 4],
                    [6, 3],
                    [7, 2],
                    [8, 1]
                ])
                .into_iter()
                .map(|i| i.to_vec())
                .collect()
            )
        )
    }

    #[test]
    fn it_works_0() {
        println!(
            "{}",
            crate::minimum_lines(
                (vec![
                    [72, 98],
                    [62, 27],
                    [32, 7],
                    [71, 4],
                    [25, 19],
                    [91, 30],
                    [52, 73],
                    [10, 9],
                    [99, 71],
                    [47, 22],
                    [19, 30],
                    [80, 63],
                    [18, 15],
                    [48, 17],
                    [77, 16],
                    [46, 27],
                    [66, 87],
                    [55, 84],
                    [65, 38],
                    [30, 9],
                    [50, 42],
                    [100, 60],
                    [75, 73],
                    [98, 53],
                    [22, 80],
                    [41, 61],
                    [37, 47],
                    [95, 8],
                    [51, 81],
                    [78, 79],
                    [57, 95]
                ])
                .into_iter()
                .map(|i| i.to_vec())
                .collect()
            )
        )
    }

    #[test]
    fn it_works_1() {
        println!(
            "{}",
            crate::minimum_lines(
                (vec![
                    [9, 35],
                    [12, 47],
                    [68, 90],
                    [64, 71],
                    [33, 99],
                    [90, 25],
                    [42, 91],
                    [41, 5],
                    [28, 12],
                    [66, 30],
                    [78, 10],
                    [92, 23],
                    [88, 45],
                    [89, 32],
                    [3, 74],
                    [93, 3],
                    [44, 15],
                    [43, 11],
                    [91, 41]
                ])
                .into_iter()
                .map(|i| i.to_vec())
                .collect()
            )
        )
    }
}
