fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    // This borrows each element of the collection through each iteration.
    // Thus leaving the collection untouched and available for reuse after the loop.
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }

    // This consumes the collection so that on each iteration the exact data is provided.
    // Once the collection has been consumed it is no longer available for reuse as it has been 'moved' within the loop.
    // println!("names: {:?}", names); needs to be commented out.
    // for name in names.into_iter() {
    //     match name {
    //         "Ferris" => println!("There is a rustacean among us!"),
    //         _ => println!("Hello {}", name),
    //     }
    // }

    // This mutably borrows each element of the collection, allowing for the collection to be modified in place.
    // let names = vec!["Bob", "Frank", "Ferris"]; needs to be mutable with mut
    // for name in names.iter_mut() {
    //     // deferencing the name to update it
    //     *name  = match name {
    //         // &mut represents a mutable reference
    //         &mut "Ferris" => "There is a rustacean among us!",
    //         _ => "Hello",
    //     }
    // }

    println!("names: {:?}", names);
}
