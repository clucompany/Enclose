use enclose::enclose;
use std::sync::Arc;

fn main() {
	let clone_data = Arc::new(0);
	let add_data = Arc::new(100);

	my_enclose(enclose!((mut *clone_data, *add_data,) || {
		// (mut *clone_data, *add_data) ->
		// let mut clone_data = *clone_data;
		// let add_data = *add_data;

		println!("#0 {:?}", clone_data);
		clone_data += add_data;
		println!("#1 {:?}", clone_data);

		assert_eq!(clone_data, 100);
	}));

	assert_eq!(*clone_data, 0);
}

#[inline]
fn my_enclose<F: FnOnce() -> R, R>(a: F) -> R {
	a()
}
