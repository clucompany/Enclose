#[macro_use]
extern crate enclose;

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