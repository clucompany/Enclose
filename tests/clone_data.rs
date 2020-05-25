use enclose::run_enclose;

#[test]
fn clone_mut_data() {
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