fn main() {
    println!("grade_letter(75): {}", grade_letter(75));
    println!("grade_letter(60): {}", grade_letter(60));
    println!("is_weekend(6): {}", is_weekend(6));
    println!("is_weekend(5): {}", is_weekend(5));
}


fn grade_letter(score: i32) -> String {
    match score {
        90..=100 => "A".to_string(),
        80..=89 => "B".to_string(),
        70..=79 => "C".to_string(),
        60..= 69 => "D".to_string(),
        _ => "F".to_string()
    }
}
fn is_weekend(day: i32) -> bool {
    matches!(day, 6|7)
}