// An attribute to hide warnings for unused code.
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
  let Rectangle { top_left: Point { x: top_left_x, y: top_left_y }, bottom_right: Point { x: bottom_right_x, y: bottom_right_y } } = rectangle;

  println!("top left x {} and y {}", top_left_x, top_left_y);
  println!("bottom right x {} and y {}", bottom_right_x, bottom_right_y);

  return (top_left_x - bottom_right_x) * (top_left_y - bottom_right_y).abs();
}

fn square(point: Point, f: f32) -> Rectangle {
    Rectangle {
        top_left: point,
        bottom_right: Point { x: f, y: f },
    }
}


fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
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

    let rectangle = Rectangle {
        top_left: Point { x: 32.4, y: 10.4 },
        bottom_right: Point { x: 22.3, y: 20.5 },
    };

    println!("the rectangle area is {}", rect_area(rectangle));

    let square = square(Point { x: 0.6, y: 0.6 }, 1.2);
    println!("square: {:?}", square);
}
