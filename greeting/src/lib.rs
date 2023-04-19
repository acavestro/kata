pub fn greet(name: String) -> String {
    format!("Hello, {}.", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requirement_1() {
        let result = greet(String::from("Bob"));
        assert_eq!(result, String::from("Hello, Bob."));
    }
}
