use std::collections::HashMap;

pub fn storing() {
    println!("=========================");
    let mut greeting = String::from("Hello!");
    greeting.push_str("world");
    greeting.push_str("!");
    println!("Print {:#?}", greeting);

    let mut first = String::from("Foo");
    let suffix = "bar";
    first.push_str(suffix);
    println!("suffix: {suffix}");

    let s1 = String::from("Hello");
    let s2 = String::from("world");
    let combined = s1 + " " + &s2;
    println!("{combined}");

    let hello = "Здравствуйте";

    for c in hello.chars() {
        println!("Value is {:#?}", c);
    }

    for c in hello.bytes() {
        println!("Value is {:#?}", c);
    }

    let original = String::from("Rust1? Rust.");
    let excited = original.replace("Rust1", "Vincent");
    println!("original is {:?}", original);
    println!("excited is {:?}", excited);
    println!("=========================");

    println!("=========================");
    let mut scores = HashMap::new();
    scores.insert(String::from("Vincent-Vo1"), 10);
    scores.insert(String::from("Vincent-Vo2"), 11);
    scores.insert(String::from("Vincent-Vo3"), 12);
    scores.insert(String::from("Vincent-Vo4"), 13);
    scores.insert(String::from("Blue"), 25);
    scores.insert(String::from("Yellow"), 50);

    println!("Value is {:#?}", scores);
    let first_name = String::from("Vincent-Vo1");
    let current_score = scores.get(&first_name).copied().unwrap_or(0);
    println!("current_score is: {:?}", current_score);

    for (name, point) in &scores {
        println!("Name is {name}, point is {point}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut new_map = HashMap::new();
    new_map.insert(field_name, field_value);

    scores.insert(String::from("Blue"), 25);
    scores.entry(String::from("Green")).or_insert(40);
    scores.entry(String::from("Yellow")).or_insert(999);

    println!("current_score is: {:?}", scores);
    println!("Count is: {:?}", scores.iter().count());

    println!("=========================");

    println!("=========================");
    println!(" panic...");
    panic!("crash and burn");
    println!("=========================");
}
