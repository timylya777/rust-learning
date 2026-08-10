fn main() {
    let mut age = 25;
    age = 26;
    const MAX_AGE: u32 = 120;
    println!("Возраст: {}, Максимальный возраст: {}, Разница: {}", age, MAX_AGE, MAX_AGE - age);
}
