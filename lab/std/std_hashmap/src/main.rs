use std::collections::HashMap;

fn call(number: &str) -> &str {
    match number {
        "798-1364" => "We're sorry, the call cannot be completed as dialed. Please hang up and try again.",
        "645-7689" => "Hello, this is Mr. Awesome's Pizza. My name is Fred. What can I get for you today?",
        _ => "Hi! Who is this again?",
    }
}

fn main() {
    let mut contacts = HashMap::new();
    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");
    contacts.insert("Katie", "435-8291");
    contacts.insert("Robert", "956-1745");

    match contacts.get(&"Daniel") {
        Some(&number) => println!("calling daniel: {}", call(number)),
        _ => println!("do not have daniel's number."),
    }

    match contacts.insert("Daniel", "164-6743") {
        Some(number) => println!("old number: {}", number),
        _ => println!("new"),
    }

    contacts.remove(&"Ashley");

    for (contract, &number) in contacts.iter() {
        println!("calling {}: {}", contract, call(number));
    }
}

