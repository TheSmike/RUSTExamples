extern crate collatz_ex;
use collatz_ex::compute;

#[test]
fn some_compute() {
    assert_eq!(compute(6), 10);
	assert_eq!(compute(45), 361);
	assert_eq!(compute(260), 18514);
}
