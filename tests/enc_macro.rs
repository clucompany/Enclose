use enclose::enclose;
use enclose::run_enclose;
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
fn thread_onevalue() {
	let mutex_data = Arc::new(MutexSafeData::new(0));
	let thread = thread::spawn( enclose!((mutex_data) move || { //NEW THREAD, NEW ARC!
		//let data = data.clone();
		
		mutex_data.set(10);
	}));

	thread.join().unwrap();
	assert_eq!(*mutex_data.get_mut(), 10);
}

#[test]
fn thread_onevalue_alias() {
	let mutex_data = Arc::new(MutexSafeData::new(0));
	assert_eq!(*mutex_data.get_mut(), 0);
	let thread = thread::spawn( enclose!((mutex_data => new_data) move || { //NEW THREAD, NEW ARC!
		//let data = data.clone();
		
		new_data.set(10);
	}));

	thread.join().unwrap();
	assert_eq!(*mutex_data.get_mut(), 10);
}

#[test]
fn threadpool_twovalue_and_alias() {
	let v1 = Arc::new(MutexSafeData::new(1));
	assert_eq!(*v1.get_mut(), 1);
	
	let v2 = Arc::new(RwLock::new( (0, 2, 3, 4) ));
	
	let count_thread = 5;
	let mut join_all = Vec::with_capacity(count_thread);

	for _a in 0..count_thread {
		join_all.push({
			thread::spawn( enclose!((v1, v2, count_thread => alias_cthreads) move || {
				assert_eq!(alias_cthreads, 5);
				
				*v1.get_mut() += 1;
				
				let mut lock_v2 = match v2.write() {
					Ok(a) => a,
					Err(e) => e.into_inner(),
				};
				lock_v2.0 += 1;
			}))
		});
	}
	for a in join_all {
		a.join().unwrap();
	}
	assert_eq!(*v1.get_mut(), 6);
	{
		let lock_v2 = match v2.write() {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		};
		assert_eq!(lock_v2.0, 5);
	}
}

#[test]
fn appeal_self() {
	#[derive(Debug, Clone, PartialEq, PartialOrd)]
	struct CheckData {
		a: u32,
	}

	impl CheckData {
		#[inline]
		pub const fn new(a: u32) -> Self {
			Self {
				a: a,
			}
		}
		
		pub fn calculate(&self, mul_num: &u32) -> u32 {
			run_enclose!((*mul_num, self.a => mut num0) move || {
				// (*mul_num, self.a => mut num0) ->
				// let mul_num = *mul_num;
				// let num0 = self.a.clone();
				
				num0 *= mul_num;
				num0 += 1024;
				
				num0
			})
			
			// run_enclose
			//
			// let mut enclose = $crate::enclose!( $($tt)* );
			// enclose()
		}
	}

	
	let data = CheckData::new(1024);
	
	// (data.a * 2) + 1024
	assert_eq!(data.calculate(&2), 3072);
	
	// check data
	assert_eq!(data.a, 1024);
}