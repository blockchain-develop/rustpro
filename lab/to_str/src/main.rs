use std::fmt;
use std::string::ToString;

struct Circle {
    radius: i32
}

/*
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "circle of radius {}", self.radius)
    }
}
*/

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("circle of radius {:?}", self.radius)
    }
}

fn main() {
    let circel = Circle{radius: 6};
    println!("{}", circel.to_string());
}
