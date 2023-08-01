fn main() {
    let mut _mutable_integer = 7i32;
    {
        let _mutable_integer = _mutable_integer;
        let _mutable_integer = 8i32;
        println!("inner {}", _mutable_integer)
        //_mutable_integer = 50;
    }
    println!("outer {}", _mutable_integer);
    _mutable_integer = 3;
    println!("outer {}", _mutable_integer);
}
