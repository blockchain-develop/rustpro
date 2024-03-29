use std::path::Path;

fn main() {
    let path = Path::new(".");
    let display = path.display();
    let new_path = path.join("a").join("b");
    match new_path.to_str() {
        None => panic!("new path is not valid"),
        Some(s) => println!("new path is {}", s),
    }
}