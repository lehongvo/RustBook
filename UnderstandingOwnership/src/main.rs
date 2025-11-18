fn main() {
    let s: String = String::from("hello world! I'm rust");
    let mut start = 0;
    for index in first_word(&s) {
        println!("{}", &s[start..index]);
        start = index + 1;
    }

    println!("{}", &s[start..]);
}

fn first_word(s: &String) -> Vec<usize> {
    let bytes = s.as_bytes();
    let mut vec_index = vec![];
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            vec_index.push(i);
        }
    }
    vec_index
}
