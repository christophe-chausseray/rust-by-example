fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // return 20 when it exit the loop
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
    println!("the result is {}", result);
}
