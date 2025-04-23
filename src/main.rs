
// Senerio 1
// Rectangle Struct (Struct + Method + Borrowing)

//  Create a Rectangle struct and implement a method that checks if it can hold another rectangle.

// struct Rectangle {
//     width : u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// impl Rectangle {
//     fn can_hold (&self,  other: &Rectangle) -> bool{
//         self.width > other.width && self.height > other.height
//     }
    
// }
// fn main() {
// let rect1 = Rectangle{
//     width : 60,
//     height : 80,
// };
// let rect2 = Rectangle{
//     width : 50,
//     height : 70,
// };


// let rect3 = Rectangle{
//     width : 30,
//     height : 90,
// };
// println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

// println!("Can rect1 hold rect2? {}", rect2.can_hold(&rect3));

// }



// Senerio 2
// Problem: Write a function that takes a string slice and returns the first word in the string.



// fn main() {
//     let s = String::from("Hello World");
//     let word = first_word(&s);
//     println!("The first word is: {}", word);
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..] 
// }



// Senerio 3
// Write a function that takes a temperature in Celsius and returns a String telling whether it’s cold, moderate, or hot.
fn main () {
let celsius =5;
let weather = describe_temperature(celsius);
println!("{}",weather);
}
fn describe_temperature(celsius: i32) -> String {
    if (celsius < 10) {
        "Weather is Cold".to_string()
    }else if celsius > 10 && celsius < 25 {
        "Weather is Moderate".to_string()
    }
    else {
        "Weather is Hot".to_string()
    }
}