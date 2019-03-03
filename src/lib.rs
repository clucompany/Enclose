//Copyright (c) 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

//Permission is hereby granted, free of charge, to any person obtaining a copy
//of this software and associated documentation files (the "Software"), to deal
//in the Software without restriction, including without limitation the rights
//to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//copies of the Software, and to permit persons to whom the Software is
//furnished to do so, subject to the following conditions:

//The above copyright notice and this permission notice shall be included in all
//copies or substantial portions of the Software.

//THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//SOFTWARE.

/*!
A convenient macro for cloning values into a closure.

# Use
```
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
```

# Use 2
```
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
```

*/

///Macro for cloning values to close.
#[macro_export]
macro_rules! enclose {
	(($( $a:ident => $c:ident ),*) $b:expr ) => {{
		$(
			let $c = $a.clone();
		)*

		$b
	}};
	(($( $a:ident ),*) $b:expr ) => {{
		$(
			let $a = $a.clone();
		)*

		$b
	}};
}

///Macro for cloning values to close. Alternative short record.
#[macro_export]
macro_rules! enc {
	($($arg:tt)*) => {
		enclose!( $($arg)* )
	};
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
		let thread = thread::spawn( enc!((v) move || {
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
	fn easy_extract() {
		let v = Arc::new(Mutex::new( 0 ));
		let thread = thread::spawn( enc!((v => my_v) move || {
			let mut v_lock = match my_v.lock() {
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
				thread::spawn( enc!((v, v2) move || {
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
