// A crate is a compilation unit in Rust
// Modules do not get compiled individually, only crates get compiled
// A crate can be compiled into a binary or into a library. By default, rustc will produce a binary from a crate.
// This behavior can be overridden by passing the --crate-type flag to lib
// use this command to create the lib : rustc --crate-type=lib --crate-name=my_lib ./src/11-crates/creating-library.rs
pub fn public_function() {
    println!("called mylib's `public_function()`");
}

fn private_function() {
    println!("called mylib's `private_function()`");
}

pub fn indirect_access() {
    print!("called mylib's `indirect_access()`, that\n> ");

    private_function();
}
