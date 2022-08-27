pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    let emails = emails
        .into_iter()
        .map(|i| {
            let parts_at: Vec<&str> = i.split(&['@']).collect();

            let parts_add: Vec<&str> = parts_at[0].split(&['+']).collect();

            let parts_dot: String = parts_add[0]
                .to_string()
                .chars()
                .filter(|i| i != &'.')
                .collect();

            [
                parts_dot.chars(),
                "@".chars(),
                parts_at[1].to_string().chars(),
            ]
            .into_iter()
            .flatten()
            .collect::<String>()
        })
        .fold(Vec::new(), |unit, iter| {
            let mut unit = unit;

            if !unit.contains(&iter) {
                unit.push(iter)
            }

            unit
        });

    emails.len() as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!(
            "{:?}",
            crate::num_unique_emails(
                (vec![
                    "test.email+alex@leetcode.com",
                    "test.e.mail+bob.cathy@leetcode.com",
                    "testemail+david@lee.tcode.com"
                ])
                .into_iter()
                .map(|i| i.to_string())
                .collect()
            )
        )
    }

    #[test]
    fn it_works_0() {
        println!(
            "{:?}",
            crate::num_unique_emails(
                (vec![
                    "test.email+alex@leetcode.com",
                    "test.email.leet+alex@code.com"
                ])
                .into_iter()
                .map(|i| i.to_string())
                .collect()
            )
        )
    }
}
