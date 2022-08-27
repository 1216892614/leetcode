pub fn find_closest(words: Vec<String>, word1: String, word2: String) -> i32 {
    let mut last_words_pos = (None, None);
    let mut distance = words.len();
    for (n, word) in words.iter().enumerate() {
        match word {
            word if word == &word1 => {
                last_words_pos.0 = Some(n);
                if let Some(last_words_pos_1) = last_words_pos.1 {
                    distance = ((n - last_words_pos_1) as usize).min(distance);
                }
            }
            word if word == &word2 => {
                last_words_pos.1 = Some(n);
                if let Some(last_words_pos_0) = last_words_pos.0 {
                    distance = ((n - last_words_pos_0) as usize).min(distance);
                }
            }
            _ => continue,
        }
    }

    distance as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!(
            "{:?}",
            crate::find_closest(
                (vec![
                    "I",
                    "am",
                    "a",
                    "student",
                    "from",
                    "a",
                    "university",
                    "in",
                    "a",
                    "city"
                ])
                .into_iter()
                .map(|x| x.to_string())
                .collect(),
                "a".to_string(),
                "student".to_string()
            )
        )
    }
}
