/// Simple `cmp` implementation for `f64`.
/// Panics when given a NaN.
pub fn f64_cmp(x: f64, y: f64) -> std::cmp::Ordering {
	if x.is_nan() || y.is_nan() {
		panic!("Attempted to compare NaN values");
	}

	if x < y {
		std::cmp::Ordering::Less
	}
	else if x > y {
		std::cmp::Ordering::Greater
	}
	else {
		std::cmp::Ordering::Equal
	}
}