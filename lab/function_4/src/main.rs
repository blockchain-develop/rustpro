

fn main() {
    fn1();
}

fn call_me<F: Fn()>(f: F) {
    f()
}

fn function() {
    println!("i am a function!");
}

fn fn1() {
    let closure = || println!("i am a closure!");
    call_me(closure);
    call_me(function);
}
