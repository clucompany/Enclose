use enclose::enclose;
use enclose::run_enclose;

#[test]
fn check_type_and_runenc() {
	#[derive(Debug, Default)]
	struct StructData {
		a: i32,
	}
	
	impl StructData {
		fn run_enclose<F: FnOnce(u64, i32)>(&self, f: F) {
			f(0, 0)
		}
		
		fn run_enclose2<F: Fn(u64, i32)>(&self, f: F) {
			f(0,0)
		}
		
		fn null_fn(_a: u64, _b: i32) {}
	}
	
	let data = StructData::default();
	
	data.run_enclose(enclose!((data.a => mut num_data) |_, num| {
		num_data += 1;
		num_data += num;
		assert_eq!(num_data, 1);
	}));
	
	data.run_enclose(enclose!(@deprecated(data.a => mut num_data) move |_, num| {
		num_data += 1;
		num_data += num;
		assert_eq!(num_data, 1);
	}));

	
	run_enclose!(
		(data.a => _a) || StructData::null_fn
	);
	
	data.run_enclose2(
		enclose!(() StructData::null_fn)
	);
	
	assert_eq!(data.a, 0);
}

#[test]
fn full_runenc() {
	let data = 10;
	let data2 = 20;
	
	run_enclose!((data => mut new_data, data2 => mut new_data2, data2 => new_data22) || {
		//let mut new_data = data.clone();
		//let mut new_data2 = data2.clone();
		//let mut new_data22 = data2.clone();
		
		new_data += 1;
		new_data2 += 1;
		
		assert_eq!(new_data, 11);
		assert_eq!(new_data2, 21);
		assert_eq!(new_data22, 20);
	});
	
	assert_eq!(data, 10);
	assert_eq!(data2, 20);
}