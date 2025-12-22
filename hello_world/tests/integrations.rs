use hello_world::greetings::say_hello;

#[test]
fn greeting_api_works() {
    let result = say_hello("Rust");
    assert!(result.is_ok());
}
