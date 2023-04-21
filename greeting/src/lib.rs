pub fn greet(name: Option<String>) -> String {
    name.map(|n| {
        if n == n.to_uppercase() {
            format!("HELLO {}!", n)
        } else {
            format!("Hello, {}.", n)
        }
    })
    .unwrap_or(String::from("Hello, my friend."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requirement_1() {
        let result = greet(Some(String::from("Bob")));
        assert_eq!(result, String::from("Hello, Bob."));
    }

    #[test]
    fn requirement_2() {
        let result = greet(None);
        assert_eq!(result, String::from("Hello, my friend."));
    }

    #[test]
    fn requirement_3() {
        let result = greet(Some(String::from("JERRY")));
        assert_eq!(result, String::from("HELLO JERRY!"));
    }
}
