
use enclose::run_enclose;

fn main() {
	let clone_data = 0;
	let add_data = 100;
	
	run_enclose!((mut clone_data, add_data) || {
		// (mut clone_data, add_data) ->
		// let mut clone_data = clone_data.clone();
		// let add_data = add_data.clone();
		
		println!("#0 {:?}", clone_data);
		clone_data += add_data;
		println!("#1 {:?}", clone_data);
		
		assert_eq!(clone_data, 100);
	});
	
	assert_eq!(clone_data, 0);
}
