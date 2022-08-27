pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
    let mut keys = vec![0; words.len()];

    let len_max = words.iter().map(|x| x.len()).max().unwrap();

    for i in 0..len_max {
        for (wn, w) in words.iter().enumerate() {
            match w.chars().nth(i) {
                Some(w) => {
                    keys[wn] += (order.chars().position(|c| c == w).unwrap() + 1)
                }
                None => continue,
            }
        }

        let mut r = std::cmp::Ordering::Less;
        for i in 0..keys.len() - 1 {
            match keys[i].cmp(&keys[i + 1]) {
                std::cmp::Ordering::Less => continue,
                std::cmp::Ordering::Equal => r = std::cmp::Ordering::Equal,
                std::cmp::Ordering::Greater => r = std::cmp::Ordering::Greater,
            }
        }
        match r {
            std::cmp::Ordering::Less => return true,
            std::cmp::Ordering::Equal => (),
            std::cmp::Ordering::Greater => return false,
        }

        keys = keys.iter().map(|x| x * len_max).collect();
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let words: Vec<String> = (vec!["my", "f"])
            .into_iter()
            .map(|x| x.to_owned())
            .collect();
        let order = "gelyriwxzdupkjctbfnqmsavho".to_owned();

        println!("{}", crate::is_alien_sorted(words, order));
    }

    #[test]
    fn it_works_1() {
        let words: Vec<String> = (vec!["aa", "a"])
            .into_iter()
            .map(|x| x.to_owned())
            .collect();
        let order = "abqwertyuioplkjhgfdszxcvnm".to_owned();

        println!("{}", crate::is_alien_sorted(words, order));
    }
}
