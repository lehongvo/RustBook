mod error_handling;
mod garden;
mod hourse;
mod math;
mod network;
mod outer;
mod storing;
mod user;
mod vector;

use crate::error_handling::{handle, recoverable};
use crate::garden::vegetables::Asparagus;
use crate::hourse::{
    back_of_house::{Appetizer, kitchen},
    front_of_house::{hosting, serving},
};
use crate::math::cal;
use crate::network::{client, server};
use crate::outer::inner;
use crate::storing::storing as storing_mod;
use crate::user::user_struct::User;
use crate::vector::lap_code::lap_code;

fn main() {
    let numbers = vec![21, 23, 123, 123, 12, 3];
    let value = cal(&numbers);
    println!("{}", value);
    server::start(12);
    client::connect("Vincent");

    let new_user = User::new(String::from("Vincent"), 21);
    new_user.print();

    inner::inner_pub();
    inner::inner_public_create();

    let plant = Asparagus {};
    println!("Plant is {:?}", plant);

    hosting::add_to_waitlist();
    hosting::seat_at_table(1);

    let order1 = Appetizer::Salad;
    let order2 = Appetizer::Soup;

    serving::take_order();
    serving::serve_order(12);
    kitchen::cook_order(12);
    serving::take_payment(12);

    lap_code();
    storing_mod::storing();

    recoverable::recover_error()
}
