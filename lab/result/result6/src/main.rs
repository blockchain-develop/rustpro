use std::num::ParseIntError;

fn multiply(first_number_str: &str,second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;
    Ok(first_number * second_number)
}

fn myfunction(first_number_str: &str,second_number_str: &str) -> Result<i32, ParseIntError>  {
    multiply(first_number_str, second_number_str)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("error: {}", e),
    }
}

fn main() {
    print(myfunction("t", "2"));
}
