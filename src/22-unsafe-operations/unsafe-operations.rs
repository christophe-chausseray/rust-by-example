// Unsafe Rust is for operations that the compiler can't guarantee the memory safety
// We should try to minimize the amount of unsafe code in a code base
// But unsafe give 5 superpowers :

// - dereferencing raw pointers (raw pointers can be immutable or mutable and are written as *const T and *mut T)
// - Call an unsafe function or method (including FFI)
// - Access or modify a mutable static variable (ex : static mut COUNTER: u32 = 0;)
// - Implement an unsafe trait (A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify)
// - Access fields of unions (A union is similar to a struct, but only one declared field is used in a particular instance at one time. They are primarily used to interface with unions in C code)

// Raw pointers are different from reference or smart pointers :
// - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
// - Aren’t guaranteed to point to valid memory
// - Are allowed to be null
// - Don’t implement any automatic cleanup
// Dereferencing a raw pointer can only be done through an unsafe block.

use std::slice;

extern "C" {
    // using the function abs from the C standard library (FFI)
    fn abs(input: i32) -> i32;
}

// Tell the Rust compiler not to mangle the name of this function
// Mangling is when a compiler changes the name we’ve given a function to a different name that contains
// more information for other parts of the compilation process to consume but is less human readable.
// call_from_c function is accessible from C code
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Global static variable
// A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory.
// Using the value will always access the same data.
// Constants, on the other hand, are allowed to duplicate their data whenever they’re used.
// Another difference is that static variables can be mutable.
// Accessing and modifying mutable static variables is unsafe.
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        // modifying the static mutable variable
        COUNTER += inc;
    }
}

// unsafe trait
unsafe trait Foo {
    // methods go here
}

// implementing an unsafe trait
unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {
    let mut num = 10;
    let raw_p: *const u32 = &num;
    // can be written like that also
    // let raw_p = &num as *const u32;
    let r2 = &mut num as *mut u32;
    // let r2: *mut u32 = &num;

    unsafe {
        // dereferencing raw pointers is unsafe
        assert!(*raw_p == 10);
        *r2 = 5;
        assert!(*r2 == 5);
    }

    let some_vector = vec![1, 2, 3, 4];

    let pointer = some_vector.as_ptr();
    let length = some_vector.len();

    unsafe {
        // calling an unsafe functions (ex : slice::from_raw_parts)
        // Better to wrap this call in a safe function
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);

        assert_eq!(some_vector.as_slice(), my_slice);
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    add_to_count(3);

    unsafe {
        // accessing the static mutable variable
        println!("COUNTER: {}", COUNTER);
    }
}
