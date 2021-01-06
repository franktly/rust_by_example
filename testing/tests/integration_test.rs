// Unit tests are testing one module in isolation at a time: they're small and can test private
// code
// Integration tests are external to your cate and use only its public interface in the same way
// any other code would
// Test many parts of your library work correctly togeter
mod common; // import common module
use testing::lib_test::add; // import external lib, testing is the project name
#[test]
fn integrate_test_add() {
    common::setup();
    assert_eq!(add(2, 3), 5);
}
