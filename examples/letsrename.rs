use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use enclose::enclose;

fn main() {
	let mutex_data = Arc::new(Mutex::new(0));
	let thread = thread::spawn(enclose!((mutex_data => d, @1024 => my_var) move || {
		// (mutex_data => d) ->
		// let d = mutex_data.clone();
		// let my_var = 1024;

		let mut lock = match d.lock() {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		};
		*lock += my_var;
	}));

	thread.join().unwrap();
	{
		let lock = match mutex_data.lock() {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		};
		assert_eq!(*lock, 1024);
	}
}
