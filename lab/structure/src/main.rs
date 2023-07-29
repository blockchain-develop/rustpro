use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.top_left, self.bottom_right)
    }
}


fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle{top_left: Point{x: top_left_x, y: top_left_y}, bottom_right: Point{x: bottom_right_x, y: bottom_right_y}} = rect;
    let x = bottom_right_x - top_left_x;
    let y = top_left_y - bottom_right_y;
    x * y
}

fn square(point: Point, x: f32) -> Rectangle {
    let bottom_right = Point{x: point.x + x, y: point.y - x};
    Rectangle{top_left: point, bottom_right: bottom_right}
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age};
    //
    println!("{:?}", peter);
    //
    let point: Point = Point { x: 10.3, y: 0.4};
    println!("point coordinates: ({}, {})", point.x, point.y);
    //
    let bottom_right = Point { x: 5.2, ..point};
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);
    //
    let Point {x: left_edge, y: top_edge} = point;
    println!("({}, {})", left_edge, top_edge);
    //
    let _rectangle = Rectangle{
        top_left: Point{x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };
    println!("rectangle: {:?}", _rectangle);
    //
    let _unit = Unit;
    //
    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    //
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
    
    //
    let bottom_right = Point{x: 10f32, y: 2f32};
    let top_left = Point{x: 5f32, y: 5f32};
    let _rectangle = Rectangle{top_left: top_left, bottom_right: bottom_right};
    println!("rectangle: {}", _rectangle);
    let area = rect_area(_rectangle);
    println!("area: {}", area);
    // 
    // the following is wrong
    //
    //println!("bottom_right: {}", bottom_right);
    //println!("rectangle: {}", _rectangle);
    //println!("area: {}", rect_area(_rectangle));

    //
    let point = Point{x: 10.0, y: 5.0};
    let _rectangle = square(point, 2.0);
    println!("rectangle: {}", _rectangle);
}
