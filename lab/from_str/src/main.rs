

fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    // ???
    //let xxx: i32 = "5".into();

    let sum = parsed + turbo_parsed;
    println!("sun: {:?}", sum);
}
