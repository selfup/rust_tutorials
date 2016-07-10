fn main() {
    println!("{}", first_function(0));
}

fn first_function(incoming_int: i8) -> String {
    let int_to_str = format!("{}", incoming_int); // i8 -> &str

    if int_to_str == "0" {
        return "is that your best shot?".to_string();
    }

    String::from(int_to_str)
}

#[test]
fn it_can_take_an_int_and_return_a_string() {
    let expected = "12".to_string();

    assert_eq!(expected, first_function(12));
}

#[test]
fn it_returns_a_message_instead_of_a_string_representation() {
    let expected = "is that your best shot?".to_string();

    assert_eq!(expected, first_function(0)); 
}
