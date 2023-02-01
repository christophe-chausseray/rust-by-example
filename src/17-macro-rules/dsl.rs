// A DSL is a mini "language" embedded in a Rust macro.
// This allows you to define concise or intuitive syntax for some special functionality (within bounds).
macro_rules! calculate {
    // if we want to use eval keyword when we use the macro
    (eval $e:expr) => {
        {
            let val: usize = $e; // Force types to be integers
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

fn main() {
    calculate! {
        eval 1 + 2 // hehehe `eval` is _not_ a Rust keyword!
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}
