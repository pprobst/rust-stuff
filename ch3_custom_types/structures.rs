#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct.
struct Nil;

// A tuple struct.
struct Pair(i32, f32);

// A struct with two fields.
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct.
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        top_left:      Point { x: x1, y: y1 },
        bottom_right:  Point { x: x2, y: y2 },
    } = rect;

    (x1 - x2).abs() * (y1-y2).abs()
}

fn square(point: Point, n: f32) -> Rectangle {
    let (lower_l_x, lower_l_y) = (point.x, point.y);
    Rectangle {
        top_left: point,
        bottom_right: Point { x: lower_l_x + n, y: lower_l_y + n }
    }
}

fn main() {
    // Create struct with field init shorthand.
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct.
    println!("{:?}", peter);

    // Instantiate a 'Point'.
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point.
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the field of our other one.
    let bottom_right = Point { x: 5.2, ..point };

    // 'bottom_right.y' will be the same as 'point.y' because we used that field from 'point'.
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a 'let' binding.
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // Struct instantiation is an expression too.
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct.
    let _nil = Nil;

    // Instantiate a tuple struct.
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct.
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct.
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    // Area of a rectangle
    println!("rect area: {}", rect_area(_rectangle));

    // Square func
    let _rect = square(Point { x: 2.0, y: 2.0 }, 5.0);
    println!("square area: {}", rect_area(_rect))
}
