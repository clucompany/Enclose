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
	
	my_enclose( enclose!((mut clone_data, add_data) move || {
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
	
	my_enclose( enclose!((mut *clone_data, *add_data) move || {
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
///run_enclose!((data.a => mut num_data) move || {
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
///run_enc!((data.a => mut num_data) move || {
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
	/*[( $($tt:tt)* ) $b:expr ] => {{
		$crate::enclose_data! {
			$( $tt )*
		}

		$b
	}};*/
	//old.
	
	
	[( $($d_tt:tt)* ) move || {$($b:tt)*} ] => {{	
		$crate::enclose_data! {
			$( $d_tt )*
		}
		move || {
			$($b)*
		}
	}};
	

	[( $($d_tt:tt)* ) || {$($b:tt)*} ] => {{
		|| {
			$crate::enclose_data! {
				$( $d_tt )*
			}
			
			$($b)*
		}
	}};
	
	
	[( $($d_tt:tt)* ) move |$($all_data:ident),*| {$($b:tt)*} ] => {{	
		$crate::enclose_data! {
			$( $d_tt )*
		}
		move |$($all_data)*| {
			$($b)*
		}
	}};
	

	[( $($d_tt:tt)* ) |$($all_data:ident),*| {$($b:tt)*} ] => {{
		|$($all_data)*| {
			$crate::enclose_data! {
				$( $d_tt )*
			}
			
			$($b)*
		}
	}};
	//Experimental opportunity...
	//Maybe not needed...
	
	
	() => ()
	/*[() $b: expr] => {$b};
	[$b: expr] => {$b};*/
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




#[cfg(test)]
mod tests {
	use std::thread;
	use std::sync::Arc;
	use std::sync::Mutex;
	use std::sync::MutexGuard;
	use std::sync::RwLock;
	
	
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
	
	#[test]
	fn clone_mut_data() {
		let data = 10;
		
		run_enclose!((data => mut new_data) || {
			//let mut new_data = data;
			new_data += 1;
			
			assert_eq!(new_data, 11);
		});
		
		assert_eq!(data, 10);
	}
	
	#[test]
	fn appeal_data() {
		#[derive(Debug, Default)]
		struct StructData {
			a: i32,	
		}
		
		impl StructData {
			fn run_closure<F: Fn(i32)>(&self, f: F) {
				f(0)
			}
		}
		
		let data = StructData::default();
		
		data.run_closure(enclose!((data.a => mut num_data) |num| {
			num_data += 1;
			num_data += num;
			assert_eq!(num_data, 1);
		}));
		
		
		assert_eq!(data.a, 0);
	}
	
	
	#[test]
	fn check_copy_clone_operations() {		
		static mut CHECK_COPY_CLONE_OPERATIONS: u32 = 0;
		
		#[derive(Debug)]
		struct AlwaysClone;

		impl Clone for AlwaysClone {
			#[inline]
			fn clone(&self) -> Self {
				unsafe { 
					CHECK_COPY_CLONE_OPERATIONS += 1; 
				}
				AlwaysClone
			}
		}
		
		impl Copy for AlwaysClone {}
		
		let data = AlwaysClone;
		
		
		assert_eq!(unsafe{ CHECK_COPY_CLONE_OPERATIONS }, 0); //Checking the number of operations
		run_enclose!((data => d) || {
			assert_eq!(unsafe{ CHECK_COPY_CLONE_OPERATIONS }, 1); //Checking the number of operations
			
			std::thread::spawn(enclose!((d) move || {
				assert_eq!(unsafe{ CHECK_COPY_CLONE_OPERATIONS }, 2); //Checking the number of operations
				
				run_enclose!((d) || {
					run_enclose!((d => _d) move || {
							
					});
				});
				assert_eq!(unsafe{ CHECK_COPY_CLONE_OPERATIONS }, 4); //Checking the number of operations
				
				
			})).join().unwrap();
			
		});
		
				
		//Checking the number of operations,
		//Closure = 2
		assert_eq!(unsafe { CHECK_COPY_CLONE_OPERATIONS }, 4);
	}
}
