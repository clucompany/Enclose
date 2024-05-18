use enclose::enclose;

fn main() {
	let mut clone_data = 2;
	my_enclose(enclose!((ref clone_data => mut ref_clone_data) || {
		// (ref clone_data => mut ref_clone_data) ->
		// let ref mut ref_clone_data = clone_data;

		println!("#0 {:?}", ref_clone_data);
		*ref_clone_data += 2;
		println!("#1 {:?}", clone_data);

		assert_eq!(clone_data, 4);
	}));

	assert_eq!(clone_data, 4);
}

#[inline]
fn my_enclose<F: FnOnce() -> R, R>(a: F) -> R {
	a()
}
