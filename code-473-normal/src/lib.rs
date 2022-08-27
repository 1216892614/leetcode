pub fn makesquare(matchsticks: Vec<i32>) -> bool {
    let mut matchsticks = matchsticks;
    let side_length = matchsticks.iter().sum::<i32>();

    fn chose_sticks(matchsticks: &mut Vec<i32>, sides: [i32; 4]) -> bool {
        let mut matchsticks = matchsticks.clone();

        if let Some(chosen_side) = matchsticks.pop() {

            vec![sides; 4]
                .into_iter()
                .enumerate()
                .map(|(n, i)| {
                    let mut i = i.to_owned();
                    i[n] -= chosen_side;
                    i.sort();
                    (n, i)
                })
                .fold(Vec::new(), |init: Vec<[i32; 4]>, (n, iter)| {
                    let mut init = init;
                    if init.iter().find(|i| i == &&iter) == None && iter[n] >= 0 {
                        init.push(iter);
                    }
                    init
                })
                .into_iter()
                .any(|i| chose_sticks(&mut matchsticks, i))
        } else {
            sides == [0; 4]
        }
        
    }

    if side_length % 4 == 0 {
        matchsticks.sort();
        chose_sticks(&mut matchsticks, [side_length/4; 4])
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(crate::makesquare(vec![1, 1, 2, 2, 2]), true);
    }

    #[test]
    fn it_works_0() {
        assert_eq!(crate::makesquare(vec![3, 3, 3, 3, 4]), false);
    }

    #[test]
    fn it_works_1() {
        assert_eq!(
            crate::makesquare(vec![5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3]),
            true
        );
    }

    #[test]
    fn it_works_2() {
        assert_eq!(
            crate::makesquare(vec![14, 17, 2, 10, 2, 19, 11, 1, 16, 7, 13, 16, 11, 3, 9]),
            false
        );
    }

    #[test]
    fn it_works_3() {
        assert_eq!(
            crate::makesquare(vec![20, 13, 19, 19, 4, 15, 10, 5, 5, 15, 14, 11, 3, 20, 11]),
            true
        );
    }
}
