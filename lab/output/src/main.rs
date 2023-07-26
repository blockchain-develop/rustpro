
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_print() {
        let a = 12;
        println!("a is {}", a);
    }

    #[test]
    fn test_shadowing() {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("the value of x is {}", x);
    }

    #[test]
    fn test_tup() {
        let tup = (500, 6.4, 1);
        let (x, y, z) = tup;
        let a = tup.1;
        println!("x {}, y {}, z {}, a {}", x, y, z, a);
    }
}
