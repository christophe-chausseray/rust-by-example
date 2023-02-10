// To generate and open the documentation in a browser run cargo doc --open
// The main purpose of documentation tests is to serve as examples that exercise the functionality, which is one of the most important guidelines.

/// First line is a short summary describing function.
///
/// The next lines present detailed documentation. Code blocks start with
/// triple backquotes and have implicit `fn main()` inside
/// and `extern crate <cratename>`. Assume we're testing `doccomments` crate:
///
/// ```
/// let result = testing::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[allow(dead_code)]
fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

pub fn divide_non_zero_result(a: u32, b: u32) -> u32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    } else if a < b {
        panic!("Divide result is zero");
    }
    a / b
}

/// Usually doc comments may include sections "Examples", "Panics" and "Failures".
///
/// The next function divides two numbers.
///
/// # Examples
///
/// ```
/// let result = testing::div(10, 2);
/// assert_eq!(result, 5);
/// ```
///
/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// testing::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}

// Using ? makes compilation fail since main returns unit.
// The ability to hide some source lines from documentation comes to the rescue:
// one may write fn try_main() -> Result<(), ErrorType>, hide it and unwrap it in hidden main.
// Like it or not, example code is often copied verbatim by users.
// Unwrapping an error should be a conscious decision that the user needs to make.
// A common way of structuring fallible example code is the following.

/// Using hidden `try_main` in doc tests.
///
/// ```
/// # // hidden lines start with `#` symbol, but they're still compilable!
/// # fn try_main() -> Result<(), String> { // line that wraps the body shown in doc
/// let res = testing::try_div(10, 2)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() { // starting main that'll unwrap()
/// #    try_main().unwrap(); // calling try_main and unwrapping
/// #                         // so that test will panic in case of error
/// # }
/// ```
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}

// To run specific tests one may specify the test name to cargo test command
// To run multiple tests one may specify part of a test name that matches all the tests that should be run.

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::assert_eq; // crate for test-only use. Cannot be used in non-test code.

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    // Ignored besause failing
    #[test]
    #[ignore]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }

    #[test]
    fn test_add_hundred() {
        assert_eq!(add(100, 2), 102);
        assert_eq!(add(2, 100), 102);
    }

    // Tests can be marked with the #[ignore] attribute to exclude some tests
    // To run them with command cargo test -- --ignored
    #[test]
    #[ignore]
    fn ignored_test() {
        assert_eq!(add(0, 0), 0);
    }

    // None of the previous unit test examples had a return type.
    // But in Rust 2018, your unit tests can return Result<()>, which lets you use ? in them!
    // This can make them much more concise.
    #[test]
    fn test_sqrt() -> Result<(), String> {
        let x = 4.0;
        // Make it fail because sqrt panic for negative floats
        // let x = -4.0;
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide_non_zero_result(10, 2), 5);
    }

    // To check functions that should panic under certain circumstances, use attribute #[should_panic].
    #[test]
    #[should_panic]
    fn test_any_panic() {
        divide_non_zero_result(1, 0);
    }

    // This attribute accepts optional parameter expected = with the text of the panic message.
    // If your function can panic in multiple ways, it helps make sure your test is testing the correct panic.
    #[test]
    #[should_panic(expected = "Divide result is zero")]
    fn test_specific_panic() {
        divide_non_zero_result(1, 10);
    }
}
