// An Iter::map operation might fail, for example:
// fn main() {
//     let strings = vec!["tofu", "93", "18"];
//     let numbers: Vec<_> = strings
//         .into_iter()
//         .map(|s| s.parse::<i32>())
//         .collect();
//     println!("Results: {:?}", numbers);

//     // Results: [Err(ParseIntError { kind: InvalidDigit }), Ok(93), Ok(18)]
// }

// filter_map calls a function and filters out the results that are None.
// fn main() {
//     let strings = vec!["tofu", "93", "18"];
//     let numbers: Vec<_> = strings
//         .into_iter()
//         .filter_map(|s| s.parse::<i32>().ok())
//         .collect();
//     println!("Results: {:?}", numbers);

//     // Results: [93, 18]
// }

// Collect the failed items with map_err() and filter_map()
// map_err calls a function with the error,
// so by adding that to the previous filter_map solution we can save them off to the side while iterating
// fn main() {
//     let strings = vec!["42", "tofu", "93", "999", "18"];
//     let mut errors = vec![];
//     let numbers: Vec<_> = strings
//         .into_iter()
//         .map(|s| s.parse::<u8>())
//         .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
//         .collect();
//     println!("Numbers: {:?}", numbers);
//     println!("Errors: {:?}", errors);

//     // Numbers: [42, 93, 18]
//     // Errors: [ParseIntError { kind: InvalidDigit }, ParseIntError { kind: PosOverflow }]
// }

// Fail the entire operation with collect()
// Result implements FromIterator so that a vector of results (Vec<Result<T, E>>) can be turned into a result with a vector (Result<Vec<T>, E>). Once an Result::Err is found, the iteration will terminate.
// This same technique can be used with Option.
// fn main() {
//     let strings = vec!["tofu", "93", "18"];
//     let numbers: Result<Vec<_>, _> = strings
//         .into_iter()
//         .map(|s| s.parse::<i32>())
//         .collect();
//     println!("Results: {:?}", numbers);

//     // Results: Err(ParseIntError { kind: InvalidDigit })
//     // Results: Ok([93, 18]) -> if no errors
// }

// Collect all valid values and failures with partition()
fn main() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    // Numbers: [Ok(93), Ok(18)]
    // Errors: [Err(ParseIntError { kind: InvalidDigit })]

    // When you look at the results, you'll note that everything is still wrapped in Result
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);

    // Numbers: [93, 18]
    // Errors: [ParseIntError { kind: InvalidDigit }]
}

