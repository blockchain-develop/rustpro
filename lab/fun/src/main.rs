fn main() {
    println!("Hello, world!");
}

fn add1(a: i32, b: i32) -> i32 {
    return a + b;
}

fn add2(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let x = add1(1,2);
        let y = add2(3,4);
        println!("x is {}, y is {}", x, y);
    }
}