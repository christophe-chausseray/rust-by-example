// An attribute is metadata applied to some module, crate or item
// This metadata can be used to/for :
// conditional compilation of code
// set crate name, version and type (binary or library)
// disable lints (warnings)
// enable compiler features (macros, glob imports, etc.)
// link to a foreign library
// mark functions as unit tests
// mark functions that will be part of a benchmark
// attribute like macros

// When atributes apply to a whole crate, their syntax is #![crate_attribute]
// When they apply to a module or item, the syntax is #[item_attribute]

// Attributes can take arguments with different syntaxes:
// #[attribute = "value"]
// #[attribute(key = "value")]
// #[attribute(value)]

// Attributes can have multiple values and can be separated over multiple lines
// #[attribute(value, value2)]
// #[attribute(value, value2, value3,
//            value4, value5)]


// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {}

fn main() {}

// This crate is a library
// #![crate_type = "lib"]
// The library is named "rary"
// #![crate_name = "rary"]
// However, it is important to note that both the crate_type and crate_name attributes have no effect whatsoever when using Cargo
