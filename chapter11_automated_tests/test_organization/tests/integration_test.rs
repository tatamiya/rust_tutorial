use test_organization;

mod common;

// run only this test by `cargo test --test integration_test`
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_organization::add_two(2));
}
