// example program that calculates area of a rectangle using structs

#[derive(Debug)] // This annotation allows debugging print of struct
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // Implementation block
    fn area(&self) -> u32 { // Implementation method
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool { // Implementation method
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle { // Associated function (does not take self as a parameter)
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("The area of rect1 is {} u^2.", rect1.area());

    println!("rect1 = {:#?}", rect1); // :#? gives nice formatting

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);

    println!("The area of sq is {} u^2.", sq.area());
    println!("sq = {:#?}", sq); // :#? gives nice formatting
}
