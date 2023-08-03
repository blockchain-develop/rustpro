fn main() {
    fn1();
    fn2();
    fn3();
    fn4();
}

fn fn1() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;
    //
    if let Some(i) = number {
        println!("matched {:?}", i);
    }
    
    if let Some(i) = letter {
        println!("matched {:?}", i);
    } else {
        println!("do not matched a number");
    };

    //
    let i_lick_letters = false;
    if let Some(i) = emotion {
        println!("matched {:?}", i);
    } else if i_lick_letters {
        println!("do not match a number");
    } else {
        println!("do not like letters")
    };
}

fn fn2() {
    enum Foo {
        Bar,
        Baz,
        Qux(u32)
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foo bar");
    }

    if let Foo::Bar = b {
        println!("b is foo bar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // todo
    /*
    if Foo::Bar == a {
        println!("a is foo bar");
    }
    */
}

fn fn3() {
    let mut optional = Some(0);
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("greater than 9, quit!");
                    optional = None;
                } else {
                    println!("is is {:?}, try again.", i);
                    optional = Some(i + 1);
                }
            },
            _ => {break;}
        }
    }
}

fn fn4() {
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("greater than 9, quit!");
            optional = None;
        } else {
            println!("is is {:?}, try again.", i);
            optional = Some(i + 1);
        }
    }
}