pub fn cook_order(table: u32) {
    println!("Cooking order for table {table}...")
}

fn fix_incorrect_order(table: u32) {
    println!("Fixing order for table {table}...");
    cook_order(table);
}
