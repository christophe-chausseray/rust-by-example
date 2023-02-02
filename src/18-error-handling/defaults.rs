// There is more than one way to unpack an Option and fall back on a default if it is None.
// To choose the one that meets our needs, we need to consider the following:

// - do we need eager or lazy evaluation?
// - do we need to keep the original empty value intact, or modify it in place?

// or() is chainable, evaluates eagerly, keeps empty value intact

// or_else() is chainable, evaluates lazily, keeps empty value intact

// get_or_insert() evaluates eagerly, modifies empty value in place

// get_or_insert_with() evaluates lazily, modifies empty value in place


#[derive(Debug)]
enum Fruit { Apple, Orange, Banana, Kiwi, Lemon }

fn main() {
    let apple = Some(Fruit::Apple);
    let orange = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    let first_available_fruit = no_fruit.or(orange).or(apple);
    println!("first_available_fruit: {:?}", first_available_fruit);
    // first_available_fruit: Some(Orange)

    // `or` moves its argument.
    // In the example above, `or(orange)` returned a `Some`, so `or(apple)` was not invoked.
    // But the variable named `apple` has been moved regardless, and cannot be used anymore.
    // println!("Variable apple was moved, so this line won't compile: {:?}", apple);
    // TODO: uncomment the line above to see the compiler error

    let apple = Some(Fruit::Apple);
    let no_fruit: Option<Fruit> = None;
    let get_kiwi_as_fallback = || {
        println!("Providing kiwi as fallback");
        Some(Fruit::Kiwi)
    };
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };

    let first_available_fruit = no_fruit
        .or_else(get_kiwi_as_fallback)
        .or_else(get_lemon_as_fallback);
    println!("first_available_fruit: {:?}", first_available_fruit);
    // Providing kiwi as fallback
    // first_available_fruit: Some(Kiwi)

    println!("Variable apple is not moved in this case: {:?}", apple);

    let mut my_fruit: Option<Fruit> = None;
    let apple = Fruit::Apple;
    let first_available_fruit = my_fruit.get_or_insert(apple);
    println!("my_fruit is: {:?}", first_available_fruit);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    // my_fruit is: Apple
    // first_available_fruit is: Apple
    // println!("Variable named `apple` is moved: {:?}", apple);
    // TODO: uncomment the line above to see the compiler error

    let mut my_fruit: Option<Fruit> = None;
    let get_lemon_as_fallback = || {
        println!("Providing lemon as fallback");
        Fruit::Lemon
    };
    let first_available_fruit = my_fruit
        .get_or_insert_with(get_lemon_as_fallback);
    println!("my_fruit is: {:?}", first_available_fruit);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    // Providing lemon as fallback
    // my_fruit is: Lemon
    // first_available_fruit is: Lemon

    // If the Option has a value, it is left unchanged, and the closure is not invoked
    let mut my_apple = Some(Fruit::Apple);
    let should_be_apple = my_apple.get_or_insert_with(get_lemon_as_fallback);
    println!("should_be_apple is: {:?}", should_be_apple);
    println!("my_apple is unchanged: {:?}", my_apple);
    // The output is a follows. Note that the closure `get_lemon_as_fallback` is not invoked
    // should_be_apple is: Apple
    // my_apple is unchanged: Some(Apple)
 }