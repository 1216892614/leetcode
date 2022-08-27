pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
    let dictionary = remove_duplicate_letters(&target);

    let stickers: Vec<[u8; 15]> = stickers
        .iter()
        .map(|i| str_to_index(&dictionary, i))
        .collect();
    let target = str_to_index(&dictionary, &target);

    match target_match([0; 15], dictionary.chars().count(), stickers, target, 0, 0) {
        Result::Work(n) => n as i32,
        Result::NotWork => -1,
    }
}

enum Result {
    Work(u8),
    NotWork,
}

fn target_match(
    last_remainder: [u8; 15],
    dictionary_len: usize,
    stickers: Vec<[u8; 15]>,
    target: [u8; 15],
    ordinal: usize,
    result: u8,
) -> Result {
    let mut last_remainder = last_remainder;

    for ord in ordinal..dictionary_len {
        if target[ord] > last_remainder[ord] {
            let stickers = eliminated_weaker(dictionary_len, &stickers, ord);
            
            let stickers_available: Vec<&[u8; 15]> =
            stickers.iter().filter(|i| i[ord] > 0).collect();
            
            let result = stickers_available
            .iter()
            .map(|al| {
                let mut remainder = [0; 15];
                
                for i in ord..dictionary_len {
                    remainder[i] = last_remainder[i] + al[i];
                }
                
                target_match(
                    remainder,
                    dictionary_len,
                    stickers.clone(),
                    target,
                    ord,
                    result + 1,
                )
                })
                .min_by(|a, b| {
                    use std::cmp::Ordering;
                    match (a, b) {
                        (Result::Work(a), Result::Work(b)) => a.cmp(b),
                        (Result::Work(_), Result::NotWork) => Ordering::Greater,
                        (Result::NotWork, Result::Work(_)) => Ordering::Less,
                        (Result::NotWork, Result::NotWork) => Ordering::Equal,
                    }
                });

            match result {
                Some(Result::Work(a)) => return Result::Work(a),
                _ => return Result::NotWork,
            }
        }
    }

    Result::Work(result)
}

fn eliminated_weaker(dictionary_len: usize, stickers: &Vec<[u8; 15]>, ord: usize) -> Vec<[u8; 15]> {
    let mut survivors: Vec<[u8; 15]> = Vec::new();

    for (n, s) in stickers.iter().enumerate() {
        let mut time = 0;
        if stickers.iter().all(|c: &[u8; 15]| {
            let mut re = false;

            if time == n {
                re = true;
            } else {
                for i in ord..dictionary_len {
                    if c[i] < s[i] {
                        re = true;
                    }
                }
            }

            time += 1;

            re
        }) {
            survivors.push(*s);
        }
    }

    survivors
}

fn str_to_index(dictionary: &str, word: &str) -> [u8; 15] {
    let mut index = [0; 15];
    for (n, i) in dictionary.chars().enumerate() {
        index[n] = word.chars().filter(|c| *c == i).count() as u8;
    }

    index
}

fn remove_duplicate_letters(s: &str) -> String {
    let size = s.len();

    let mut last_appear_index = [0; 26];
    let mut if_in_stack = [false; 26];

    // 记录每个字符最后一次出现的索引
    for (n, i) in s.bytes().enumerate() {
        last_appear_index[(i - b'a') as usize] = n as i16;
    }

    let mut stack = vec![b'a'];
    for (n, i) in s.bytes().enumerate() {
        if if_in_stack[(i - b'a') as usize] {
            continue;
        }

        while let Some(s) = stack.pop() {
            if s > i && last_appear_index[(s - b'a') as usize] > n as i16 {
                if_in_stack[(s - b'a') as usize] = false;
            } else {
                stack.push(s);
                break;
            }
        }
        stack.push(i);
        if_in_stack[(i - b'a') as usize] = true;
    }
    stack.drain(1..).map(|x| x as char).collect()
}

mod tests {
    #[test]
    fn it_works() {
        let stickers = (vec!["with", "example", "science"])
            .iter()
            .map(|i| i.to_string())
            .collect();
        let target = "thehat".to_string();
        println!("{:?}", crate::min_stickers(stickers, target));
    }

    #[test]
    fn it_works_2() {
        let stickers = (vec![
            "control", "heart", "interest", "stream", "sentence", "soil", "wonder", "them",
            "month", "slip", "table", "miss", "boat", "speak", "figure", "no", "perhaps", "twenty",
            "throw", "rich", "capital", "save", "method", "store", "meant", "life", "oil",
            "string", "song", "food", "am", "who", "fat", "if", "put", "path", "come", "grow",
            "box", "great", "word", "object", "stead", "common", "fresh", "the", "operate",
            "where", "road", "mean",
        ])
        .iter()
        .map(|i| i.to_string())
        .collect();
        let target = "stoodcrease".to_string();
        println!("{:?}", crate::min_stickers(stickers, target));
    }

    #[test]
    fn it_works_3() {
        let stickers = (vec!["city", "would", "feel", "effect", "cell", "paint"])
            .iter()
            .map(|i| i.to_string())
            .collect();
        let target = "putcat".to_string();
        println!("{:?}", crate::min_stickers(stickers, target));
    }

    #[test]
    fn test_dictionary() {
        println!(
            "{:?}",
            crate::remove_duplicate_letters("epogiawhofhiopwahefoih")
        );
    }

    #[test]
    fn test_eliminated_weaker() {
        let stickers = vec!["city", "would", "feel", "effect", "cell", "paint"];

        let dictionary = &crate::remove_duplicate_letters("putcat");

        let stickers: Vec<[u8; 15]> = stickers
            .iter()
            .map(|i| crate::str_to_index(dictionary, i))
            .collect();
        println!(
            "{:?}",
            crate::eliminated_weaker(dictionary.chars().count(), &stickers, 0)
        );
    }

    #[test]
    fn test_index() {
        println!(
            "{:?}",
            crate::str_to_index(
                &crate::remove_duplicate_letters("epogiawhofhiopwahefoih"),
                "epogiawhofhiopwahefoih"
            )
        );
    }

    #[test]
    fn test_index_arr() {
        let stickers = vec![
            "control", "heart", "interest", "stream", "sentence", "soil", "wonder", "them",
            "month", "slip", "table", "miss", "boat", "speak", "figure", "no", "perhaps", "twenty",
            "throw", "rich", "capital", "save", "method", "store", "meant", "life", "oil",
            "string", "song", "food", "am", "who", "fat", "if", "put", "path", "come", "grow",
            "box", "great", "word", "object", "stead", "common", "fresh", "the", "operate",
            "where", "road", "mean",
        ];

        let dictionary = &crate::remove_duplicate_letters("stoodcrease");

        println!("{}", dictionary);

        for i in stickers {
            println!("{:?}", crate::str_to_index(dictionary, i));
        }
    }
}
