// Rust provides a powerful macro system that allows metaprogramming.
// As you've seen in previous chapters, macros look like functions, except that their name ends with a bang !,
// but instead of generating a function call, macros are expanded into source code that gets compiled with the rest of the program

// macros are useful for :
// - Don't repeat yourself
// - Domain-specific languages
// - Variadic interface (interface that takes a variable number of arguments)

// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    () => {
        // The macro will expand into the contents of this block.
        println!("Hello!")
    };
}

fn main() {
    // This call will expand into `println!("Hello");`
    say_hello!()
}
