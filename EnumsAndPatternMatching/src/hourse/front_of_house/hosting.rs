#[derive(Debug)]
pub struct Host {
    pub name: String,
}

pub fn add_to_waitlist() {
    println!("Adding a guest to the waitlist...");
}

pub fn seat_at_table(table: u32) {
    println!("Seating guest at table {table}...");
}
