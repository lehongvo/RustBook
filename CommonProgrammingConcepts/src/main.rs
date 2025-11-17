fn main() {
    let mut a: [usize; 10] = [5; 10];
    let mut sum = 0;
    for x in &a {
        sum += x;
    }
    println!("{sum}");
}
