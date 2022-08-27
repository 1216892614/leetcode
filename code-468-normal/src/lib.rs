use std::net::IpAddr;
pub fn valid_ip_address(ip: String) -> String {
    match ip.parse::<IpAddr>() {
        Ok(IpAddr::V4(x)) => {
            let array: Vec<Vec<char>> = ip.split('.').map(|x| x.chars().collect()).collect();
            for i in array {
                if i[0] == '0' && i.len() > 1 {
                    return String::from("Neither");
                }
            }
            String::from("IPv4")
        }
        Ok(IpAddr::V6(_)) => {
            let array: Vec<Vec<char>> = ip.split(':').map(|x| x.chars().collect()).collect();
            for i in array {
                if i.len() == 0 {
                    return String::from("Neither");
                }
            }
            String::from("IPv6")
        }
        _ => String::from("Neither"),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
