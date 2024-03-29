use std::mem;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 }
    };

    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        p1: origin(),
        p2: origin()
    });

    let boxed_point: Box<Point> = Box::new(origin());

    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("point occupies {} bytes in the stack", mem::size_of_val(&point));
    println!("rectangle occupies {} bytes in the stack", mem::size_of_val(&rectangle));

    println!("boxed point occupies {} bytes in the stack", mem::size_of_val(&boxed_point));
    println!("boxed rectangle occupies {} bytes in the stack", mem::size_of_val(&boxed_rectangle));
    println!("boxed box occupies {} bytes in the stack", mem::size_of_val(&box_in_a_box));

    let unboxed_point: Point = *boxed_point;
    println!("unboxed point occupies {} bytes in the stack", mem::size_of_val(&unboxed_point));
}

