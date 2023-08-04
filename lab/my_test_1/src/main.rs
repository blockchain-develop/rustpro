#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

fn cal_length(p: &Point) -> f32 {
    p.x * p.x + p.y * p.y
}

fn process(point: Point) {
    let length = || {
        cal_length(&point) 
    };
    let translate = || {
        let Point { x, y } = point;
        Point { x: x + 1.0, y: y + 1.0}
    };
    //
    let len = length();
    println!("point: {:?}, length: {}", point, len);
    let point = translate();
    let len = length();
    println!("point: {:?}, length: {}", point, len);
}

fn main() {
    let point = Point { x: 1.0, y: 1.0};
    process(point);
}
