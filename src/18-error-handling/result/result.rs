// Result is a richer version of the Option type that describes possible error instead of possible absence.

// That is, Result<T, E> could have one of two outcomes:

// - Ok(T): An element T was found
// - Err(E): An error was found with element E

// By convention, the expected outcome is Ok while the unexpected outcome is Err.

// fn multiply(first_number_str: &str, second_number_str: &str) -> i32 {
//     // Let's try using `unwrap()` to get the number out. Will it bite us?
//     let first_number = first_number_str.parse::<i32>().unwrap();
//     let second_number = second_number_str.parse::<i32>().unwrap();
//     first_number * second_number
// }

// fn main() {
//     let twenty = multiply("10", "2");
//     println!("double is {}", twenty);

//     let tt = multiply("t", "2");
//     println!("double is {}", tt);
// }

// The Result type can also be the return type of the main function if specified explicitly.
// If an error occurs within the main function it will return an error code and print a debug representation of the error (using the Debug trait).

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    // let number_str = "tt";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        // it prints Error: ParseIntError { kind: InvalidDigit } if not a number
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}
