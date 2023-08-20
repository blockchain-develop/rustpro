fn main() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("collected (0..10) into: {:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    println!("initial vector: {:?}", xs);

    println!("push 4 into the vector");
    xs.push(4);
    println!("vector: {:?}", xs);

    //collected_iterator.push(0);

    println!("vector size: {}", xs.len());

    println!("second element: {}", xs[1]);

    println!("pop last element: {:?}", xs.pop());

    //println!("fourth element: {}", xs[3]);

    println!("contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("in position {} we have value {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("update vector: {:?}", xs);
}
