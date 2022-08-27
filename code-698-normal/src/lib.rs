pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
    let mut nums = nums;
    let side_length = nums.iter().sum::<i32>();

    fn chose_sticks(matchsticks: &mut Vec<i32>, sides: Vec<i32>) -> bool {
        let side_num = sides.len();
        let mut matchsticks = matchsticks.clone();

        if let Some(chosen_side) = matchsticks.pop() {
            vec![sides; side_num]
                .into_iter()
                .enumerate()
                .map(|(n, i)| {
                    let mut i = i.to_owned();
                    i[n] -= chosen_side;
                    i.sort();
                    (n, i)
                })
                .fold(Vec::new(), |init: Vec<Vec<i32>>, (n, iter)| {
                    let mut init = init;
                    if !init.contains(&iter) && iter[n] >= 0 {
                        init.push(iter);
                    }
                    init
                })
                .into_iter()
                .any(|i| chose_sticks(&mut matchsticks, i))
        } else {
            sides == vec![0; sides.len()]
        }
    }

    nums.sort();

    if side_length % k == 0
        && nums.len() >= k as usize
        && nums.iter().nth(nums.len() - 1).unwrap() <= &(side_length / k)
    {
        let side_length = side_length / k;
        chose_sticks(&mut nums, vec![side_length; k as usize])
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(crate::can_partition_k_subsets(vec![1, 1, 2, 2, 2], 4), true);
    }

    #[test]
    fn it_works_0() {
        assert_eq!(crate::can_partition_k_subsets(vec![1, 2, 3, 4], 3), false);
    }

    #[test]
    fn it_works_1() {
        assert_eq!(
            crate::can_partition_k_subsets(
                vec![
                    724, 3908, 1444, 522, 325, 322, 1037, 5508, 1112, 724, 424, 2017, 1227, 6655,
                    5576, 543
                ],
                4
            ),
            true
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(
            crate::can_partition_k_subsets(
                vec![3, 9, 4, 5, 8, 8, 7, 9, 3, 6, 2, 10, 10, 4, 10, 2],
                10
            ),
            false
        );
    }

    #[test]
    fn it_works_3() {
        assert_eq!(
            crate::can_partition_k_subsets(
                vec![2, 9, 4, 7, 3, 2, 10, 5, 3, 6, 6, 2, 7, 5, 2, 4],
                7
            ),
            false
        );
    }
}
