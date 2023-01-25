// To link a crate to this new library you may use rustc's --extern flag.
// All of its items will then be imported under a module named the same as the library.
// use this command to be able to use the external lib : rustc ./src/11-crates/using-library.rs --extern mylib=libmy_lib.rlib --edition=2018
fn main() {
    mylib::public_function();

    // Error! `private_function` is private
    //mylib::private_function();

    mylib::indirect_access();
}
