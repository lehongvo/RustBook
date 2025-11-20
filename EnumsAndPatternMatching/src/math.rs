pub fn cal(numbers: &[i32]) -> f32 {
    if numbers.is_empty() {
        0.0
    } else {
        let sum: i32 = numbers.iter().sum();
        let length = numbers.len();
        sum as f32 / length as f32
    }
}
