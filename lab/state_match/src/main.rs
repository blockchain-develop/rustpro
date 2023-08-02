fn main() {
    fn1();
    fn2();
    fn3();
    fn4();
    fn5();
    fn6();
    fn7();
    fn8();
    fn9();
    fn10();
    fn11();
}

fn fn1() {
    let number = 19;
    println!("tell me about {}", number);
    match number {
        1 => println!("One!"),
        2|3|5|7|11 => println!("This is a prime"),
        13..=19 => println!("a teen"),
        _ => println!("ain's special"),
    }
}

fn fn2() {
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}

fn fn3() {
    let triple = (0, -2, 3);
    println!("tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("x is 0, y is {}, z is {}", y, z),
        (1, ..) => println!("x is 1 and the rest doesnot matter"),
        _ => println!("it doesnot matter what they are"),
    }
}

enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn fn4() {
    let color = Color::RGB(122, 17, 40);
    println!("what color is it?");

    match color {
        Color::Red => println!("red!"),
        Color::Blue => println!("blue!"),
        Color::Green => println!("green!"),
        Color::RGB(r, g, b) => println!("r: {}, g: {}, b: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("h: {}, s: {}, v: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("h: {}, s: {}, l: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("c: {}, m: {}, y: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!("c: {}, m: {}, y: {}, k: {}!", c, m, y, k),
    }
}

fn fn5() {
    let reference = &4;
    match reference {
        &val => println!("got a value: {:?}", val),
    }
}

fn fn6() {
    let reference = &4;
    match *reference {
        val => println!("got a value: {:?}", val),
    }
}

fn fn7() {
    let value = 5;
    match value {
        ref r => println!("got a reference to a value: {:?}", r),
    }
}

// todo
// 为什么没有出现生命期问题
//
fn fn8() {
    let value = 5;
    match value {
        x if x < 6 => println!("{}", x),
        _ => println!("other"),
    }
    println!("value: {}", value);
}

fn fn9() {
    let mut mut_value = 6;
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("value: {:?}", m);
        }
    }
    println!("value: {:?}", mut_value);
}

fn fn10() {
    struct Foo {x: (u32, u32), y: u32}
    //
    let foo = Foo {x: (1, 2), y: 3};
    let Foo{x: (a, b), y} = foo;
    println!("a = {}, b = {}, y = {}", a, b, y);

    //
    let Foo {y: i, x: j} = foo;
    println!("i = {:?}, j = {:?}", i, j);

    //
    let Foo {y, ..} = foo;
    println!("y = {}", y);
}

fn fn11() {
    let pair = (2, -2);
    println!("tell me {:?}", pair);
    match pair {
        (x, y) if x == y => println!("twins"),
        (x, y) if x + y == 0 => println!("kaboom"),
        (x, _) if x % 2 == 0 => println!("odd"),
        _ => println!("no......"),
    }
}
