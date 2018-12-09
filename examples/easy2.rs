
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use enclose::enc;
use enclose::enclose;

fn main() {
     let v = Arc::new(Mutex::new( 0 ));
	let thread = thread::spawn( enc!((v => MY_LOCKER) move || {
          let mut v_lock = match MY_LOCKER.lock() {
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