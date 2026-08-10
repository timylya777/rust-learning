fn main() {
    // string_length
    println!("{}", "=".repeat(50));
    let string = String::from("hello world");
    let length = string_length(&string);
    println!("string: '{}', length: {}", string, length);
    // make_uppercase
    println!("{}", "=".repeat(50));
    let mut other_string = String::from("other hello world");
    println!("string before make_uppercase(): {}", other_string);
    make_uppercase(&mut other_string);
    println!("string after make_uppercase(): {}", other_string);

}


fn string_length(s: &String) -> usize {
    s.len()
}
fn make_uppercase(s: &mut String) {
    *s = s.to_uppercase()
}