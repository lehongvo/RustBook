#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, new_width: u32) -> &mut Self {
        self.width = new_width;
        self
    }

    fn set_height(&mut self, new_height: u32) -> &mut Self {
        self.height = new_height;
        self
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width >= rectangle.width && self.height >= rectangle.height
    }
}

impl Rectangle {
    fn say(&self) {
        println!("Width is {}, Height is {}", self.width, self.height);
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 19,
        height: 20,
    };

    println!("rectangle {:?} has area {}", rect1, rect1.area());

    rect1.set_width(100).set_height(20).area();

    println!("rectangle {:?} has area {}", rect1, rect1.area());

    let rect2 = Rectangle {
        width: 19,
        height: 20,
    };

    let can_hold = rect1.can_hold(&rect2);
    println!("can hold: {}", can_hold);

    // assert_eq!(rect1, rect2); // Removed because Rectangle does not implement PartialEq

    rect1.say();
    rect2.say();
}
