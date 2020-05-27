
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
	[ $(@$eprefix:tt)? ($($enc_args:tt)*) move | $($args:ident),* | $b:block as $path:path] => {{
		#[allow(unused_mut)]
		let mut enclose: $path = $crate::enclose!(
			$(@$eprefix)? ($($enc_args)*) $($prefix)?
			move |$($args),*| $b
		);
		
		enclose()
	}};
	[ $(@$eprefix:tt)? ($($enc_args:tt)*) move || $b:block as $path:path] => {{
		#[allow(unused_mut)]
		let mut enclose: $path = $crate::enclose!(
			$(@$eprefix)? ($($enc_args)*)
			move || $b
		);
		
		enclose()
	}};
	// end move
	
	
	[ $(@$eprefix:tt)? ($($enc_args:tt)*) | $($args:ident),* | $b:block as $path:path] => {{
		#[allow(unused_mut)]
		let mut enclose: $path = $crate::enclose!(
			$(@$eprefix)? ($($enc_args)*) $($prefix)?
			|$($args),*| $b
		);
		
		enclose()
	}};
	[ $(@$eprefix:tt)? ($($enc_args:tt)*) || $b:block as $path:path] => {{
		#[allow(unused_mut)]
		let mut enclose: $path = $crate::enclose!(
			$(@$eprefix)? ($($enc_args)*)
			|| $b
		);
		
		enclose()
	}};
	[$($tt:tt)*] => {{
		#[allow(unused_mut)]
		let mut enclose = $crate::enclose!( $($tt)* );
		
		enclose()
	}};
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