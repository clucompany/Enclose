#[macro_use]
extern crate enclose;

#[test]
fn appeal_data() {
	#[derive(Debug, Default)]
	struct StructData {
		a: i32,	
	}
	
	impl StructData {
		fn run_closure<F: FnOnce(u64, i32)>(&self, f: F) {
			f(0, 0)
		}
		
		fn run_closure2<F: Fn(u64, i32)>(&self, f: F) {
			f(0,0)
		}
		
		fn null_fn(_a: u64, _b: i32) {}
	}
	
	let data = StructData::default();
	
	data.run_closure(enclose!((data.a => mut num_data) |_, num| {
		num_data += 1;
		num_data += num;
		assert_eq!(num_data, 1);
	}));
	
	data.run_closure(enclose!((data.a => mut num_data) move |_, num| {
		num_data += 1;
		num_data += num;
		assert_eq!(num_data, 1);
	}));
	
	run_enclose!(
		(data.a => _a) || StructData::null_fn
	);
	
	data.run_closure2(
		enclose!(() StructData::null_fn)
	);
	
	assert_eq!(data.a, 0);
}

