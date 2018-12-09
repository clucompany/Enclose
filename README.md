# Enclose
A convenient macro for cloning values into a closure.

[![Build Status](https://travis-ci.org/clucompany/Enclose.svg?branch=master)](https://travis-ci.org/clucompany/Enclose)
[![Apache licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/enclose)](https://crates.io/crates/enclose)
[![Documentation](https://docs.rs/enclose/badge.svg)](https://docs.rs/enclose)

# Use

	use std::thread;
	use std::sync::Arc;
	use std::sync::Mutex;

	let v = Arc::new(Mutex::new( 0 ));
	let thread = thread::spawn( enclose!((v) move || {
			let mut v_lock = match v.lock() {
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

# Use 2

	use std::thread;
	use std::sync::Arc;
	use std::sync::Mutex;
	use std::sync::RwLock;

	let v = Arc::new(Mutex::new( 0 ));
	let v2 = Arc::new(RwLock::new( (0, 2, 3, 4) ));

	let count_thread = 5;
	let mut wait_all = Vec::with_capacity(count_thread);

	for _a in 0..count_thread {
			wait_all.push({
					thread::spawn( enclose!((v, v2) move || {
							let mut v_lock = match v.lock() {
									Ok(a) => a,
									Err(e) => e.into_inner(),
							};
							*v_lock += 1;

							drop( v2 ); //ignore warning
					}))
			});
	}
	for a in wait_all {
			a.join().unwrap();
	}
	{	
			//Test result
			let v_lock = match v.lock() {
					Ok(a) => a,
					Err(e) => e.into_inner(),
			};
			assert_eq!(*v_lock, 5);
	}

# Use 3

	use std::sync::Arc;
	use std::sync::Mutex;
	use std::thread;

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


# License

Copyright 2018 #UlinProject Денис Котляров

Licensed under the MIT License
