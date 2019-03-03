
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use enclose::enclose;

fn main() {
	let v = Arc::new(Mutex::new( 0 ));
	let thread = thread::spawn( enclose!((v => my_locker) move || {
		let mut v_lock = match my_locker.lock() {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		};
		*v_lock += 1;
	}));

	thread.join().unwrap();
	{
		let v_lock = match v.lock() {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		};
		assert_eq!(*v_lock, 1);
	}
}