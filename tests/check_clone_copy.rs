use enclose::enclose;
use enclose::run_enclose;

#[test]
fn check_copy_clone_operations() {
	static mut CHECK_COPY_CLONE_OPERATIONS: u32 = 0;
	
	#[derive(Debug)]
	struct AlwaysClone;

	impl Clone for AlwaysClone {
		#[inline]
		fn clone(&self) -> Self {
			unsafe { CHECK_COPY_CLONE_OPERATIONS += 1; }
			
			AlwaysClone
		}
	}
	
	impl Copy for AlwaysClone {}
	
	let data = AlwaysClone;
	
	
	assert_eq!(unsafe { CHECK_COPY_CLONE_OPERATIONS }, 0); //Checking the number of operations
	run_enclose!((data => d) || {
		assert_eq!(unsafe { CHECK_COPY_CLONE_OPERATIONS }, 1); //Checking the number of operations
		
		std::thread::spawn(enclose!((d) move || {
			assert_eq!(unsafe { CHECK_COPY_CLONE_OPERATIONS }, 2); //Checking the number of operations
			
			run_enclose!((d) || {
				run_enclose!((d => _d) move || { //'Move 'is not mandatory here, but since the semantics of the macro are different, we will leave it here for tests.
					
				});
			});
			assert_eq!(unsafe { CHECK_COPY_CLONE_OPERATIONS }, 4); //Checking the number of operations
			
			
		})).join().unwrap();
		
	});
	
			
	//Checking the number of operations,
	//Closure = 2
	assert_eq!(unsafe { CHECK_COPY_CLONE_OPERATIONS }, 4);
}

