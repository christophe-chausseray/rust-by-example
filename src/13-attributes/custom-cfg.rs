// We can have custom conditional configuration
// However for custom we need to have the flag cfg passed to rustc
// rustc --cfg some_condition ./src/13-attributes/custom-cfg.rs && ./custom-cfg
// Otherwise it can't compile the conditional function
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}

