// In order to share some code between integration tests we can make a module with public functions,
// importing and using it within tests.

// Creating the module as tests/common.rs also works,
// but is not recommended because the test runner will treat the file as a test crate and try to run tests inside it.

pub fn setup() {
    // some setup code, like creating required files/directories, starting
    // servers, etc.
}
