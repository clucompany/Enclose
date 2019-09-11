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

// #Ulin Project 1819

/*!
A convenient macro for cloning values into a closure.

# Use
```
use enclose::enclose;

fn main() {
	let clone_data = 0;
	let add_data = 100;
	
	my_enclose( enclose!((mut clone_data, add_data) || {
		println!("#0 {:?}", clone_data);
		clone_data += add_data;
		println!("#1 {:?}", clone_data);
		
		assert_eq!(clone_data, 100);
	}));
	
	assert_eq!(clone_data, 0);
}

fn my_enclose<F: FnOnce() -> R, R>(a: F) -> R {
	a()
}
```

# Use 1

```
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use enclose::enclose;

fn main() {
	let mutex_data = Arc::new(Mutex::new( 0 ));
	let thread = thread::spawn( enclose!((mutex_data => d) move || {
		let mut lock = match d.lock() {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		};
		*lock += 1;
	}));

	thread.join().unwrap();
	{
		let lock = match mutex_data.lock() {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		};
		assert_eq!(*lock, 1);
	}
}
```

# Use 2
```

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;
use std::thread;

use enclose::enclose;

fn main() {
	let data1 = Arc::new(Mutex::new( 0 ));
	let data2 = Arc::new(RwLock::new( (0, 2, 3, 4) ));

	let count_thread = 5;
	let mut waits = Vec::with_capacity(count_thread);

	for _a in 0..count_thread {
		waits.push({
			thread::spawn( enclose!((data1, data2) move || {
				//(data1, data2) -> 
				//let data1 = 'root.data1.clone();
				//let data2 = 'root.data2.clone();
				
				let mut v_lock = match data1.lock() {
					Ok(a) => a,
					Err(e) => e.into_inner(),
				};
				*v_lock += 1;

				drop( data2 ); //ignore warning
			}))
		});
	}
	for a in waits {
		a.join().unwrap();
	}
	
	
	{	
		//Check data1_lock
		let data1_lock = match data1.lock() {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		};
		assert_eq!(*data1_lock, 5);
	}
	
	{	
		//Check data2_lock
		let data2_lock = match data2.write() {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		};
		assert_eq!(*data2_lock, (0, 2, 3, 4));
	}
}
```

# Use 3

```
use enclose::enclose;
use std::sync::Arc;

fn main() {
	let clone_data = Arc::new(0);
	let add_data = Arc::new(100);
	
	my_enclose( enclose!((mut *clone_data, *add_data) || {
		println!("#0 {:?}", clone_data);
		clone_data += add_data;
		println!("#1 {:?}", clone_data);
		
		assert_eq!(clone_data, 100);
	}));
	
	assert_eq!(*clone_data, 0);
}

fn my_enclose<F: FnOnce() -> R, R>(a: F) -> R {
	a()
}
```
*/

#![no_std]

///To create and start short circuit.
///```rust
///use enclose::run_enclose;
/// 
///#[derive(Debug, Default)]
///struct StructData {
///	a: i32,	
///}
///
///let data = StructData::default();
///
///run_enclose!((data.a => mut num_data) || {
///	num_data += 1;
///	assert_eq!(num_data, 1);
///});
///
///
///assert_eq!(data.a, 0);
///```
#[macro_export]
macro_rules! run_enclose {
	[$($tt:tt)*] => {{
		#[allow(unused_mut)]
		let mut enclose = $crate::enclose!( $($tt)* );
		
		enclose()
	}}
}

///To create and start short circuit. Alternative short record.
///```rust
///use enclose::run_enc;
/// 
///#[derive(Debug, Default)]
///struct StructData {
///	a: i32,	
///}
///
///let data = StructData::default();
///
///run_enc!((data.a => mut num_data) || {
///	num_data += 1;
///	assert_eq!(num_data, 1);
///});
///
///
///assert_eq!(data.a, 0);
///```
#[macro_export]
macro_rules! run_enc {
	[$($tt:tt)*] => {{
		let mut enclose = $crate::enc!( $($tt)* );
		
		enclose()
	}}
}


///Macro for cloning values to close.
#[macro_export]
macro_rules! enclose {
	[@deprecated ( $($tt:tt)* ) $b:expr ] => {{
		$crate::enclose_data! {
			$( $tt )*
		}

		$b
	}};
	//deprecated method.
	
	[@!prev ( $($d_tt:tt)* ) move || $($b:tt)* ] => {{	
		move || {
			$crate::enclose_data! {
				$( $d_tt )*
			}
			$($b)*	
		}
	}};
	
	[( $($d_tt:tt)* ) move || $($b:tt)* ] => {{	
		$crate::enclose_data! {
			$( $d_tt )*
		}
		move || $($b)*
	}};
	
	
	[@prev ( $($d_tt:tt)* ) || $($b:tt)* ] => {{
		$crate::enclose_data! {
			$( $d_tt )*
		}
		|| $($b)*
	}};

	[( $($d_tt:tt)* ) || $($b:tt)* ] => {{
		|| {
			$crate::enclose_data! {
				$( $d_tt )*
			}
			
			$($b)*
		}
	}};
	//end, empty enclose
	
	[@!prev ( $($d_tt:tt)* ) move |$( $all_data:tt ),*| $($b:tt)* ] => {{
		move |$($all_data),*| {
			$crate::enclose_data! {
				$( $d_tt )*
			}
			
			$($b)*	
		}
	}};
	
	[( $($d_tt:tt)* ) move |$( $all_data:tt ),*| $($b:tt)* ] => {{
		$crate::enclose_data! {
			$( $d_tt )*
		}
		move |$($all_data),*| $($b)*
	}};
	
	
	[@prev ( $($d_tt:tt)* ) |$( $all_data:tt ),*| $($b:tt)* ] => {{
		$crate::enclose_data! {
			$( $d_tt )*
		}
		|$( $all_data ),*| $($b)*
	}};
	
	[( $($d_tt:tt)* ) |$( $all_data:tt ),*| $($b:tt)* ] => {{
		|$( $all_data ),*| {
			$crate::enclose_data! {
				$( $d_tt )*
			}
			
			$($b)*
		}
	}};
	
	
	/*
		data.run_closure2(
			enclose!(() StructData::null_fn)
		);
	*/
	[( $($d_tt:tt)* ) $($b:tt)* ] => {{
		$crate::enclose_data! {
			$( $d_tt )*
		}
		
		$($b)*
	}};
	
	
	() => ()
}

///Macro for cloning values to close. Alternative short record.
#[macro_export]
macro_rules! enc {
	[$($tt:tt)*] => {
		$crate::enclose!{ $($tt)* }
	};
}


#[doc(hidden)]
#[macro_export]
macro_rules! enclose_data {
	[ *$a: expr => mut $b: ident,  $($tt:tt)*] => {
		let mut $b = *$a;
		
		$crate::enclose_data!{ $($tt)* }
	};
	
	[ $a: expr => mut $b: ident,  $($tt:tt)*] => {
		let mut $b = $a.clone();
		
		$crate::enclose_data!{ $($tt)* }
	};
	
	[ *$a: expr => $b: ident,  $($tt:tt)*] => {
		let $b = *$a;
		
		$crate::enclose_data!{ $($tt)* }
	};
	
	[ $a: expr => $b: ident,  $($tt:tt)*] => {
		let $b = $a.clone();
		
		$crate::enclose_data!{ $($tt)* }
	};
	
	[ mut *$a: ident,  $($tt:tt)*] => {
		let mut $a = *$a;
		
		$crate::enclose_data!{ $($tt)* }
	};
	
	[ mut $a: ident,  $($tt:tt)*] => {
		let mut $a = $a.clone();
		
		$crate::enclose_data!{ $($tt)* }
	};
	
	[ *$a: ident,  $($tt:tt)*] => {
		let $a = *$a;
		
		$crate::enclose_data!{ $($tt)* }
	};
	
	[ $a: ident,  $($tt:tt)*] => {
		let $a = $a.clone();
		
		$crate::enclose_data!{ $($tt)* }
	};
	
	
	
	//NO ,!
	[ *$a: expr => mut $b: ident] => {
		let mut $b = *$a;
	};
	
	[ $a: expr => mut $b: ident] => {
		let mut $b = $a.clone();
	};
	
	[ *$a: expr => $b: ident] => {
		let $b = *$a;
	};
	
	[ $a: expr => $b: ident] => {
		let $b = $a.clone();
	};
	
	[ mut *$a: ident] => {
		let $a = *$a;
	};
	
	[ mut $a: ident] => {
		let mut $a = $a.clone();
	};
	
	[ *$a: ident] => {
		let $a = *$a;
	};
	
	[ $a: ident] => {
		let $a = $a.clone();
	};
	
	
	() => ()
}



