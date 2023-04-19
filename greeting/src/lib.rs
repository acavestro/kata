pub fn greet(name: Option<String>) -> String {
    format!("Hello, {}.", name.unwrap_or(String::from("my friend")))
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
}
