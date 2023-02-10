// Rust, like many programming languages, has the concept of "keywords"
// These identifiers mean something to the language, and so you cannot use them in places like variable names, function names, and other places.
// Raw identifiers let you use keywords where they would not normally be allowed.

mod foo {
  pub fn try() {
    println!("This is try function")
  }
}

fn main() {
    // Needs raw identifier because try is a reserved keyword since the 2018 edition
    foo::r#try();
}
