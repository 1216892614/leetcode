pub fn percentage_letter(s: String, letter: char) -> i32 {
    (s.chars().filter(|i| *i == letter).count() * 100 / s.len()) as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{}", crate::percentage_letter("foobar".to_string(), 'o'))
    }
}
