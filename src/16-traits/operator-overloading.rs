// In Rust, many of the operators can be overloaded via traits.
// That is, some operators can be used to accomplish different tasks based on their input arguments.
// This is possible because operators are syntactic sugar for method calls.

// A list of the traits, such as Add, that overload operators can be found in core::ops

use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    // rhs = right hand side
    fn add(self, _rhs: Bar) -> FooBar {
        println!("> Foo.add(Bar) was called");

        FooBar
    }
}

// By reversing the types, we end up implementing non-commutative addition.
// Here, we make `Add<Foo>` - the trait for addition with a RHS of type `Foo`.
// This block implements the operation: Bar + Foo = BarFoo
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> Bar.add(Foo) was called");

        BarFoo
    }
}

fn main() {
    // ops::Add(Bar)
    println!("Foo + Bar = {:?}", Foo + Bar);
    // ops::Add(Foo)
    println!("Bar + Foo = {:?}", Bar + Foo);
}
