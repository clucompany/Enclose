
#[macro_use]
extern crate enclose;
extern crate std;

use std::sync::MutexGuard;
use std::sync::Mutex;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

struct MutexSafeData(Mutex<usize>);
		
impl MutexSafeData {
	#[inline]
	pub fn new(def: usize) -> Self {
		MutexSafeData(Mutex::new(def))
	}
	pub fn set(&self, size: usize) {
		*self.get_mut() = size;
	}
	pub fn get_mut<'a>(&'a self) -> MutexGuard<'a, usize> {
		match self.0.lock() {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		}
	}
}

#[test]
fn easy() {
	let mutex_data = Arc::new(MutexSafeData::new(0));
	let thread = thread::spawn( enclose!((mutex_data) move || { //NEW THREAD, NEW ARC!
		//let data = data.clone();
		
		mutex_data.set(10);
	}));

	thread.join().unwrap();
	assert_eq!(*mutex_data.get_mut(), 10);
}

#[test]
fn easy_extract() {
	let mutex_data = Arc::new(MutexSafeData::new(0));
	let thread = thread::spawn( enclose!((mutex_data => new_data) move || { //NEW THREAD, NEW ARC!
		//let data = data.clone();
		
		new_data.set(10);
	}));

	thread.join().unwrap();
	assert_eq!(*mutex_data.get_mut(), 10);
}

#[test]
fn easy_2name() {
	let safe_data = Arc::new(MutexSafeData::new(0));
	let v2 = Arc::new(RwLock::new( (0, 2, 3, 4) ));

	let count_thread = 5;
	let mut join_all = Vec::with_capacity(count_thread);

	for _a in 0..count_thread {
		join_all.push({
			thread::spawn( enclose!((safe_data, v2) move || {
				*safe_data.get_mut() += 1;

				drop( v2 ); //ignore warning
			}))
		});
	}
	for a in join_all {
		a.join().unwrap();
	}
	assert_eq!(*safe_data.get_mut(), 5);
}

