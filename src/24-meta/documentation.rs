// Use cargo doc to build documentation in target/doc
// Use cargo test to run all tests (including documentation tests), and cargo test --doc to only run documentation tests.
// These commands will appropriately invoke rustdoc (and rustc) as required

// Doc comments are very useful for big projects that require documentation.
// When running rustdoc, these are the comments that get compiled into documentation.
// They are denoted by a ///, and support Markdown.

#![crate_name = "doc"]

/// A human being is represented here
pub struct Person {
    /// A person must have a name, no matter how much Juliet may hate it
    name: String,
}

impl Person {
    /// Returns a person with the name given them
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the person
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments
    /// // If you pass --test to `rustdoc`, it will even test it for you!
    /// use doc::Person;
    /// let person = Person::new("name");
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name](Person::name)" to the `Person` it is called on.
    pub fn hello(& self) {
        println!("Hello, {}!", self.name);
    }
}

// Used to inline docs, instead of linking out to separate page.
#[doc(inline)]
pub use bar::Bar;

// Used to prevent linking out to separate page or anywhere.
// Example from libcore/prelude
#[doc(no_inline)]
pub use crate::mem::drop;

// Using this tells rustdoc not to include this in documentation
// Example from the futures-rs library
#[doc(hidden)]
pub use self::async_await::*;

/// bar docs
mod bar {
    /// the docs for Bar
    pub struct Bar;
}

fn main() {
    let john = Person::new("John");

    john.hello();
}

// To run the tests, first build the code as a library, then tell rustdoc where to find the library so it can link it into each doctest program
// - rustc ./src/24-meta/documentation.rs --crate-type lib
// - rustdoc --test --extern doc="libdoc.rlib" ./src/24-meta/documentation.rs
