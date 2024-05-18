<div id="header" align="center">

  <b>[enclose]</b>
  
  (A convenient macro, for cloning values into a closure.)
  </br></br>

<div id="badges">
  <a href="./LICENSE_MIT">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/mit.png?raw=true" alt="mit"/>
  </a>
  <a href="./LICENSE_APACHE">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/apache2.png?raw=true" alt="apache2"/>
  </a>
  <a href="https://crates.io/crates/enclose">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/cratesio.png?raw=true" alt="cratesio"/>
  </a>
  <a href="https://docs.rs/enclose">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/docrs.png?raw=true" alt="docrs"/>
  </a>
  <a href="https://github.com/denisandroid">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/uproject.png?raw=true" alt="uproject"/>
  </a>
  <a href="https://github.com/clucompany">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/clulab.png?raw=true" alt="clulab"/>
  </a>
	
  [![CI](https://github.com/clucompany/enclose/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/clucompany/enclose/actions/workflows/CI.yml) 


</div>
</div>

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
enclose = "1.2.0"
```

and this to your source code:

```rust
use enclose::enclose;
```

## Example

### EasyUse

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

<a href="./examples">
  See all
</a>

## License

This project has a dual license according to (LICENSE-MIT) and (LICENSE-APACHE-2-0).

<div align="left">
  <a href="https://github.com/denisandroid">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/uproject.png?raw=true" alt="uproject"/>
  </a>
  <b>&nbsp;Copyright (c) 2019-2024 #UlinProject</b>
	
  <b>&nbsp;(Denis Kotlyarov).</b>
  </br></br></br>
</div>

### Apache License

<div align="left">
  <a href="./LICENSE_APACHE">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/apache2.png?raw=true" alt="apache2"/>
    
  </a>
  <b>&nbsp;Licensed under the Apache License, Version 2.0.</b>
  </br></br></br></br>
</div>

### MIT License

<div align="left">
  <a href="./LICENSE_MIT">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/mit.png?raw=true" alt="mit"/>
  </a>
  <b>&nbsp;Licensed under the MIT License.</b>
  </br></br></br></br>
</div>
