
// Senerio 1
// Rectangle Struct (Struct + Method + Borrowing)

//  Create a Rectangle struct and implement a method that checks if it can hold another rectangle.

struct Rectangle {
    width : u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold (&self,  other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
    
}
fn main() {
let rect1 = Rectangle{
    width : 60,
    height : 80,
};
let rect2 = Rectangle{
    width : 50,
    height : 70,
};


let rect3 = Rectangle{
    width : 30,
    height : 90,
};
println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect3));

}
