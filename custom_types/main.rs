// this hides the warning for unused code
#![allow(dead_code)]

// define a custome type of a person.
#[derive(Debug)]
struct Person {
    name: String,
    age: u8, // u8 is a unsigned 8-bit integer
}

// A unit struct
struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32, // f32 is a 32-bit floating point number
    y: f32, // f32 is a 32-bit floating point number
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn main () {
    let name = String::from("Peter"); // String::from is a function that creates a new string from a string literal
    let age = 27; // 27 is an integer literal
    let peter = Person { name, age }; // Person is a struct defined above
    // print the debug struct
    println!("{:?}", peter);

    // Instantiate a Point
    let point: Point = Point { x: 10.3, y: 0.4 };
    let anouther_point: Point = Point { x: 5.2, y: 3.3 }; // variable anouther_point is a Point struct with value Point(5.2, 3.3)

    // Access the fields of the Point struct
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of another Point
    let bottom_right = Point { x: 5.2, ..point }; // bottom_right is a Point struct with value Point(5.2, 0.4)

    println!("second point: ({}, {})", anouther_point.x, anouther_point.y);

    let Point { x: top_edge, y: left_edge } = point; // top_edge is 10.3 and left_edge is 0.4

    let _rectangle = Rectangle {
        top_left: Point { x: top_edge, y: left_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}