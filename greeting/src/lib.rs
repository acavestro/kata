pub fn greet(name: Vec<String>) -> String {
    let first = name.first();
    let second = if name.len() == 2 { name.last() } else { None };
    match (first, second) {
        (Some(first_name), None) => {
            if *first_name == first_name.to_uppercase() {
                format!("HELLO {}!", first_name)
            } else {
                format!("Hello, {}.", first_name)
            }
        }
        (Some(first_name), Some(second_name)) => {
            format!("Hello, {} and {}.", first_name, second_name)
        }
        _ => String::from("Hello, my friend."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requirement_1() {
        let result = greet(vec![String::from("Bob")]);
        assert_eq!(result, String::from("Hello, Bob."));
    }

    #[test]
    fn requirement_2() {
        let result = greet(vec![]);
        assert_eq!(result, String::from("Hello, my friend."));
    }

    #[test]
    fn requirement_3() {
        let result = greet(vec![String::from("JERRY")]);
        assert_eq!(result, String::from("HELLO JERRY!"));
    }

    #[test]
    fn requirement_4() {
        let result = greet(vec![String::from("Jill"), String::from("Jane")]);
        assert_eq!(result, String::from("Hello, Jill and Jane."));
    }
}
