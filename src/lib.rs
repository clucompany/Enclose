//Copyright (c) 2019-2025 #UlinProject Denis Kotlyarov (Денис Котляров)

//-----------------------------------------------------------------------------
//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.
//-----------------------------------------------------------------------------

// or

//-----------------------------------------------------------------------------
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

// #Ulin Project 1819 2025

/*!
A convenient macro, for cloning values into a closure.

### Use

Just use it!

```rust
use enclose::enclose;

fn main() {
	let clone_data = 0;
	let add_data = 100;

	my_enclose( enclose!((mut clone_data, add_data) || {
		// (mut clone_data, add_data) ->
		// let mut clone_data = clone_data.clone();
		// let add_data = add_data.clone();

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

### MutexUse

Creating closures for a multi-threaded environment, no extra lines!

```rust
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use enclose::enclose;

fn main() {
	let mutex_data = Arc::new(Mutex::new( 0 ));
	let thread = thread::spawn( enclose!((mutex_data => d) move || {
		// (mutex_data => d) ->
		// let d = mutex_data.clone();

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

### ArcMutexUse

A more complex example of using an enclose macro in a multi-threaded environment.

```rust
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
				// (data1, data2) ->
				// let data1 = data1.clone();
				// let data2 = data2.clone();

				let mut v_lock = match data1.lock() {
					Ok(a) => a,
					Err(e) => e.into_inner(),
				};
				*v_lock += 1;

				drop( data2 ); // ignore warning
			}))
		});
	}
	for a in waits {
		a.join().unwrap();
	}


	{
		// Check data1_lock
		let data1_lock = match data1.lock() {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		};
		assert_eq!(*data1_lock, 5);
	}

	{
		// Check data2_lock
		let data2_lock = match data2.write() {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		};
		assert_eq!(*data2_lock, (0, 2, 3, 4));
	}
}
```

### EasyCopy

Using copy instead of clone.

```rust
use enclose::enclose;
use std::sync::Arc;

fn main() {
	let clone_data = Arc::new(0);
	let add_data = Arc::new(100);

	my_enclose( enclose!((mut *clone_data, *add_data) || {
		// (mut *clone_data, *add_data)
		// let mut clone_data = *clone_data;
		// let add_data = *add_data;

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
#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::needless_doctest_main)]

mod var;

/// A macro for creating a closure, as well as cloning,
/// copying values into the closure.
///
/// ## Args Support
///
/// A list of all possible arguments that can be written, separated by commas,
/// in the arguments to the macro.
///
/// ### `.clone()` operation
/// ```code
/// // (a => mut b: String) will be converted to (let mut b: String = a.clone();)
/// // (b => mut b)
/// // (a => b: usize)
/// // (a => b)
/// // (mut a: String)
/// // (mut a)
/// // (a: String)
/// // (a)
/// ```
///
/// ### `*copy` operation
/// ```code
/// // (*a => mut b: &str) will be converted to (let mut b: &str = *a;)
/// // (*a => mut b)
/// // (*a => b: &str)
/// // (*a => b)
/// // (mut *a: &str)
/// // (mut *a)
/// // (*a: &str)
/// // (*a)
/// ```
///
/// ### `let ref mut a = b;` (`ref`) operation
/// ```code
/// // (ref mut a: String) will be converted to (let ref mut a: String = a;)
/// // (ref mut a)
/// // (ref a: String)
/// // (ref a)
/// // (ref a => mut b: String)
/// // (ref a => mut b)
/// // (ref a => b: String)
/// // (ref a => b)
/// ```
///
/// ### `(1+1)` (expr) operation
/// ```code
/// // (@(1+1) => mut b: usize) will be converted to (let mut b: usize = (1+1);)
/// // (@(1+1) => mut b)
/// // (@(1+1) => b: usize)
/// // (@(1+1) => b)
/// ```
///
/// ### `let a = b;` (move) operation
/// ```code
/// // (move a => mut b: String) will be converted to (let mut b: String = a;)
/// // (move a => mut b)
/// // (move a => mut b)
/// // (move a => b: String)
/// // (move a => b)
/// // (move mut a: String)
/// // (move mut a)
/// // (move a: String)
/// // (move a)
/// ```
///
/// ### `{println!("test");}` (run) operation
/// ```code
/// // { panic!("12"); }
/// ```
///
/// ## Example
///
/// ### JustUse
///
/// ```rust
/// use enclose::enclose;
///
///fn main() {
///	let clone_data = 0;
///	let add_data = 100;
///
///	my_enclose(enclose!((mut clone_data, add_data) || {
///		// (mut clone_data, add_data) ->
///		// let mut clone_data = clone_data.clone();
///		// let add_data = add_data.clone();
///
///		println!("#0 {:?}", clone_data);
///		clone_data += add_data;
///		println!("#1 {:?}", clone_data);
///
///		assert_eq!(clone_data, 100);
///	}));
///
///	assert_eq!(clone_data, 0);
///}
///
///#[inline]
///fn my_enclose<F: FnOnce() -> R, R>(a: F) -> R {
///	a()
///}
///```
///
/// ### Expr
///
///```rust
///use enclose::enclose;
///use std::sync::Arc;
///
///fn main() {
///	let clone_data = Arc::new(0);
///	let add_data = Arc::new(100);
///
///	// I also note that any expressions can be used, but the main thing is to
///	// put the @ symbol at the beginning of the variable, and not forget to assign
///	// a new name to the variable using =>.
///	my_enclose(
///		enclose!((@*clone_data => mut clone_data: usize, @*(add_data.clone()) => add_data) || {
///			// (@*clone_data => mut clone_data, @*(add_data.clone()) => add_data) ->
///			// let mut clone_data = *clone_data;
///			// let add_data = *(add_data.clone());
///
///			println!("#0 {:?}", clone_data);
///			clone_data += add_data;
///			println!("#1 {:?}", clone_data);
///
///			assert_eq!(clone_data, 100);
///		}),
///	);
///
///	assert_eq!(*clone_data, 0);
///}
///
///#[inline]
///fn my_enclose<F: FnOnce() -> R, R>(a: F) -> R {
///	a()
///}
/// ```
///
#[macro_export]
macro_rules! enclose {
	[
		// (a, b) async || true
		( $($enc_args:tt)* ) async move || $($b:tt)*
	] => {{ // async, empty args
		$crate::enclose_var! {
			$( $enc_args )*
		}
		async move || $($b)*
	}};
	[
		// (a, b) async move |c, d| true
		( $($enc_args:tt)* ) async move | $($args:tt),* | $($b:tt)*
	] => {{ // async, args
		$crate::enclose_var! {
			$( $enc_args )*
		}
		async move | $($args),* | $($b)*
	}};
	[
		// (a, b) async || true
		( $($enc_args:tt)* ) async || $($b:tt)*
	] => {{ // async, empty args
		$crate::enclose_var! {
			$( $enc_args )*
		}
		async || $($b)*
	}};
	[
		// (a, b) async |c, d| true
		( $($enc_args:tt)* ) async | $($args:tt),* | $($b:tt)*
	] => {{ // async, args
		$crate::enclose_var! {
			$( $enc_args )*
		}
		async | $($args),* | $($b)*
	}};
	[
		// (a, b) move || true
		( $($enc_args:tt)* ) move || $($b:tt)*
	] => {{ // move, empty args
		$crate::enclose_var! {
			$( $enc_args )*
		}
		move || $($b)*
	}};
	[
		// (a, b) move |c, d| true
		( $($enc_args:tt)* ) move | $($args:tt),* | $($b:tt)*
	] => {{ // move, args
		$crate::enclose_var! {
			$( $enc_args )*
		}
		move | $($args),* | $($b)*
	}};
	[
		// (a, b) || true
		( $($enc_args:tt)* ) || $($b:tt)*
	] => {{ // empty args
		|| {
			$crate::enclose_var! {
				$( $enc_args )*
			}

			$($b)*
		}
	}};
	[
		// (a, b) |c, d| true
		( $($enc_args:tt)* ) | $($args:tt),* | $($b:tt)*
	] => {{ // args
		| $($args),* | {
			$crate::enclose_var! {
				$( $enc_args )*
			}

			$($b)*
		}
	}};

	[
		// (a, b) true
		( $($enc_args:tt)* ) $($all:tt)*
	] => {{ // empty
		$crate::enclose_var! {
			$( $enc_args )*
		}
		$($all)*
	}};

	/*[ $($unk:tt)+ ] => {
		compile_error!("Undefined entry or unsupported arguments, please double-check input.");
	};*/
	[] => {};
}

/// A macro for creating a closure, as well as cloning,
/// copying values into the closure.
///
/// Alternative short record `enclose`.
pub use enclose as enc;