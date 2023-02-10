// Unit tests are testing one module in isolation at a time: they're small and can test private code.
// Integration tests are external to your crate and use only its public interface in the same way any other code would.
// Their purpose is to test that many parts of your library work correctly together.

// Each Rust source file in the tests directory is compiled as a separate crate.

// importing common module.
mod common;

#[test]
fn test_add() {
    // using common code.
    common::setup();

    assert_eq!(testing::add(3, 2), 5);
}
