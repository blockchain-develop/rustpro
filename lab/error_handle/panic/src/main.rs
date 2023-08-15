
fn give_princess(gift: &str) {
    if gift == "snake" {
        panic!("aaaaaaaaa!!!!");
    }
    println!("i love {}s!!!!", gift);
}

fn fn1() {
    give_princess("teddy bear");
    give_princess("snake");
}


fn main() {
    fn1();
}
