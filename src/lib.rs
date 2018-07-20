#[macro_export]
macro_rules! enclose {
	(($( $a:ident ),*) $b:expr ) => {{
		$(
			let $a = $a.clone();
		)*

		$b
	}};
}

#[cfg(test)]
mod tests {
	use std::thread;
	use std::sync::Arc;
	use std::sync::Mutex;
	use std::sync::RwLock;

	#[test]
	fn easy() {
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
	}

	#[test]
	fn easy_2() {
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
	}
}
