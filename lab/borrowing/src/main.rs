// 编译器静态保证了引用总是指向有效的对象
// 在存在引用指向一个对象时，该对象不能被销毁
//

fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("destroying box: {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("this is: {}", borrowed_i32);
}

fn fn1() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;
    //
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    //
    {
        let _ref_to_i32: &i32 = &boxed_i32;
        // 
        //eat_box_i32(boxed_i32);
        //
        borrow_i32(_ref_to_i32);
    }
    //
    eat_box_i32(boxed_i32);
}

// 可变借用
#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("imutable borrowed {} - {}", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("mutable borrowed {} - {}", book.title, book.year);
}

fn fn2() {
    let immutable_book = Book {
        author: "Douglas Hofstadter",
        title: "Godel, Escher, bach",
        year: 1979,
    };
    let mut mutable_book = immutable_book;
    //
    borrow_book(&immutable_book);
    borrow_book(&mutable_book);
    new_edition(&mut mutable_book);
    //new_edition(&mut immutable_book);
}

// 同一时间内只允许一次可变借用
// 当最后一次使用可变借用之后，数据可以再次借用
struct Point { x: i32, y: i32, z: i32}

fn fn3() {
    let mut point = Point { x: 0, y: 0, z: 0};
    let borrowed_point = &point;
    let another_borrow = &point;
    println!("point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z);
    //
    //let mutable_borrow = &mut point;
    println!("point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_borrow.y, point.z);

    // 不可变引用不再被使用，可以被重新借用
    let mutable_borrow = &mut point;
    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;
    
    //
    //let y = &point;
    //println!("point z coordinate is {}", point.z);

    //
    println!("point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);  

    //
    println!("point has coordinates: ({}, {}, {})",
        point.x, point.y, point.z);     
    //
    let new_borrow_point = &point;
    println!("point has coordinates: ({}, {}, {})",
        new_borrow_point.x, new_borrow_point.y, new_borrow_point.z);            
}

fn main() {
    fn1();
    fn2();
    fn3();
}
