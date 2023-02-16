// Instead of passing objects by value (T), objects can be passed by reference (&T).
// The compiler statically guarantees (via its borrow checker) that references always point to valid objects.
// That is, while references to an object exist, the object cannot be destroyed.
// References allow you to refer to some value without taking ownership of it.
// A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable.
// Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
// If we don't use the reference the value is moved to the new variable (take the ownership of the value).

// This function takes ownership of a box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// This function borrows an i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
} // Here, borrowed_i32 goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn main() {
    // Create a boxed i32, and a stacked i32
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Borrow the contents of the box. Ownership is not taken,
    // so the contents can be borrowed again.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error!
        // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
        // eat_box_i32(boxed_i32);
        // FIXME ^ Comment out this line

        // Attempt to borrow `_ref_to_i32` after inner value is destroyed
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` goes out of scope and is no longer borrowed.
    }

    // `boxed_i32` can now give up ownership to `eat_box` and be destroyed
    eat_box_i32(boxed_i32);
}
