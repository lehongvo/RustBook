/**
 * - SpreadsheetCell is an enum with the variants Int(i32), Float(f64), and Text(String).
 * - SpreadsheetCell::Int(1) creates a SpreadsheetCell with the variant Int and the value 1.
 * - SpreadsheetCell::Float(1.0) creates a SpreadsheetCell with the variant Float and the value 1.0.
 * - SpreadsheetCell::Text(String::from("Hello")) creates a SpreadsheetCell with the variant Text and the value "Hello".
 */
#[derive(Debug)]
pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

/**
 * - Vec<T> is a dynamic array that can store values of type T.
 * - vec![1, 2, 3, 4, 5] is a shorthand for creating a Vec<i32> with the values 1, 2, 3, 4, 5.
 * - row is a Vec<SpreadsheetCell> with the values Int(1), Float(1.0), and Text("Hello").
 * - println!("row: {:#?}", row) prints the row vector in a pretty format.
 * - SpreadsheetCell is an enum with the variants Int(i32), Float(f64), and Text(String).
 * - SpreadsheetCell::Int(1) creates a SpreadsheetCell with the variant Int and the value 1.
 * - SpreadsheetCell::Float(1.0) creates a SpreadsheetCell with the variant Float and the value 1.0.
 * - SpreadsheetCell::Text(String::from("Hello")) creates a SpreadsheetCell with the variant Text and the value "Hello".
 */
pub fn lap_code() {
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    println!("vector: {:?}", vector);

    let mut vector_v2 = vec![1, 2, 3, 4, 5];
    vector_v2.push(6);
    println!("vector_v2: {:?}", vector_v2);

    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(1.0),
        SpreadsheetCell::Text(String::from("Hello")),
    ];
    println!("row: {:#?}", row);
}
