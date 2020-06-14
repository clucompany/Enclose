
/// To create and start short circuit.
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
	[ $(@$prefix:tt $($prefix2:tt)? )? ($($args:tt)*) $($all:tt)*] => {{
		#[allow(unused_mut)]
		let mut enclose = $crate::enclose! (
			$(@$prefix $($prefix2)? )?
			($($args)*)
			
			$($all)*
		);
		
		enclose()
	}};
	// end move
	
	/*[ $($unk:tt)+ ] => {
		compile_error!("Undefined entry or unsupported arguments, please double-check input.");
	};*/
	[] => {};
}



/// To create and start short circuit. Alternative short record.
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
		$crate::run_enclose! {
			$($tt)*
		}
	}};
}