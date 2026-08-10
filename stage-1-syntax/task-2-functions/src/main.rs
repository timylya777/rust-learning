fn main() {

    println!("{}", celsius_to_fahrenheit(35.0));
    println!("{}", is_even(32));
    println!("{}", describe_number(-10));
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0/5.0 + 32.0
}
fn is_even(n: i32) -> bool {
    n % 2 == 0
}
fn describe_number(n: i32) -> String {
    if n > 0 { "положительное".to_string() }
    else if n < 0 { "отрицательное".to_string() }
    else { "ноль".to_string() } 
}