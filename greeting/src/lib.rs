pub fn greet(name: Vec<String>) -> String {
    if name.len() == 0 {
        String::from("Hello, my friend.")
    } else if name.len() == 1 {
        let first_name = name.get(0).unwrap();
        if *first_name == first_name.to_uppercase() {
            format!("HELLO {}!", first_name)
        } else {
            format!("Hello, {}.", first_name)
        }
    } else {
        let mut multiple_names_list = name.clone();
        let mut first_part = multiple_names_list
            .splice(0..(name.len() - 1), vec![])
            .collect::<Vec<String>>()
            .join(", ");
        if (name.len() - 1) > 1 {
            first_part = format!("{},", first_part);
        }
        format!(
            "Hello, {} and {}.",
            first_part,
            multiple_names_list.pop().unwrap()
        )
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

    #[test]
    fn requirement_5() {
        let result = greet(vec![
            String::from("Amy"),
            String::from("Brian"),
            String::from("Charlotte"),
        ]);
        assert_eq!(result, String::from("Hello, Amy, Brian, and Charlotte."));
    }
}
