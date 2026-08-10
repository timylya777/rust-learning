fn main() {
    let some_string = String::from("String");
    println!("some_string: {}", some_string);
    let other_string = some_string.clone();
    println!("some_string after cloning to other_string: {}", some_string);
    let some_integer = 16;
    println!("some_integer: {}", some_integer);
    let other_integer = some_integer;
    println!("some_integer after (idk not mooving//? in other_integer) {}", some_integer); // okay this working
}
