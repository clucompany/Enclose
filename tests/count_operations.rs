use core::sync::atomic::AtomicU8;
use core::sync::atomic::Ordering;
use enclose::enc;

#[test]
fn clone_count_operations() {
	static CHECK_COUNT_OPERATIONS: AtomicU8 = AtomicU8::new(0);
	struct CleverType;

	impl Clone for CleverType {
		#[inline]
		fn clone(&self) -> Self {
			let _del = CHECK_COUNT_OPERATIONS.fetch_add(1, Ordering::SeqCst);

			CleverType
		}
	}

	let data = CleverType;

	assert_eq!(CHECK_COUNT_OPERATIONS.load(Ordering::SeqCst), 0); //Checking the number of operations
	enc!((data => d) || {
		assert_eq!(CHECK_COUNT_OPERATIONS.load(Ordering::SeqCst), 1); //Checking the number of operations

		std::thread::spawn(enc!((d,) move || {
			assert_eq!(CHECK_COUNT_OPERATIONS.load(Ordering::SeqCst), 2); //Checking the number of operations

			enc!((d) || {
				enc!((d => _d, d => _d2) move || { //'Move 'is not mandatory here, but since the semantics of the macro are different, we will leave it here for tests.

				})();
			})();
			assert_eq!(CHECK_COUNT_OPERATIONS.load(Ordering::SeqCst), 5); //Checking the number of operations


		})).join().unwrap();

	})();

	let _manclone = data.clone();

	//Checking the number of operations,
	//Closure = 2
	assert_eq!(CHECK_COUNT_OPERATIONS.load(Ordering::SeqCst), 6);
}
