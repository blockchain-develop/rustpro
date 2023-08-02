fn main() {
    fn1();
    fn2();
    fn3();
    fn4();
    fn5();
}


fn fn1() {
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }   
}

fn fn2() {
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }   
}

fn fn3() {
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("This is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);
}

fn fn4() {
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("This is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    //println!("names: {:?}", names);
}

fn fn5() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "This is a custacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}