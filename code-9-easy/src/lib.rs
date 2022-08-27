pub fn is_palindrome(x: i32) -> bool {
    let x = x.to_string();
    let a:String = (&x[0..x.len()/2]).chars().rev().collect();
    let b = match x.len() % 2 {
        0 => &x[x.len() / 2..x.len()],
        _ => &x[x.len() / 2 + 1..x.len()],
    };

    b == a
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works_0() {
        println!("{:?}", crate::is_palindrome(12321));
    }
}
