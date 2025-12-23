pub fn say_hello(name: &str) -> Result<String, String> {
    if name.trim().is_empty() {
        return Err("name cannot be empty".to_string());
    }
    Ok(format!("Hello {}!", name))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works_with_name() {
        let result = say_hello("Rust");
        assert_eq!(result.unwrap(), "Hello Rust!");
    }

    #[test]
    fn fails_with_empty() {
        let result = say_hello("");
        assert!(result.is_err());
    }
}
