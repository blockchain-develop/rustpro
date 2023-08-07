fn destroy_box(c: Box<i32>) {
    println!("destroying a box that contains {}", c);
}

fn fn1() {
    let x = 5u32;
    let y = x;
    //
    println!("x is {}. y is {}", x, y);

    //
    let a = Box::new(5i32);
    println!("a contains: {}", a);

    //
    let b = a;
    //println!("a contains: {}", a);
    println!("b contains: {}", b);
    
    //
    destroy_box(b);
    //println!("b contains: {}", b);
}

//
fn fn2() {
    let immutable_box = Box::new(5u32);
    println!("immutable box contains {}", immutable_box);

    //*immutable_box = 4;

    //
    let mut mutable_box = immutable_box;
    println!("mutable box contains {}", mutable_box);

    *mutable_box = 4;
    println!("mutable box contains {}", mutable_box);
}

fn main() {
    fn1();
    fn2();
}
