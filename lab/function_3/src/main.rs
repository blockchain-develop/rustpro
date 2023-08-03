

fn main() {
    fn1();
    fn2();
    fn3();
    fn4();
    fn5();
    fn6();
}

fn fn1() {
    fn function(i: i32) -> i32 {
        i + 1
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i | i + 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    //
    let one = || 1;
    println!("closure returning one: {}", one());
}

fn fn2() {
    let color = String::from("green");
    //
    let print = || println!("color: {}", color);
    print();
    //
    let _reborrow = &color;
    print();
    //
    let _color_moved = color;
    //print();
}

fn fn3() {
    //
    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("count: {}", count);
    };
    inc();
    // let _reborrow = &count;
    inc();
    println!("count: {}", count);
    //
    let _count_reborrowed = &mut count;
}

fn fn4() {
    use std::mem;

    let movable = Box::new(3);
    let consume = || {
        println!("movable: {}", movable);
        mem::drop(movable);
    };

    consume();
    //consume();
}

fn fn5() {
    let haystack = vec![1,2,3];
    let contains = move |needle| haystack.contains(needle);
    //
    println!("{}", contains(&1));
    println!("{}", contains(&4));
    //
    //println!("there are {} elements in vec", haystack.len());
}

fn apply<F>(f: F) where
    F: FnOnce() {
        f();
    }

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
        f(3)
    }

fn fn6() {
    use std::mem;
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    //
    let diary = || {
        println!("i said {}.", greeting);
        farewell.push_str("!!!");
        println!("then i screamed {}.", farewell);
        println!("now i can sleep. zzzzz");

        mem::drop(farewell);
    };
    //println!("new farewell {}.", farewell);

    //
    apply(diary);

    //
    let double = |x| 2 * x;
    println!("3 double: {}", apply_to_3(double));
}
