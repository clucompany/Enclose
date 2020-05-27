

#[doc(hidden)]
#[macro_export]
macro_rules! enclose_var {
	// copy operation
	[ *$a: expr => mut $b: ident $(: $ty:ty)?
		$(, $($tt:tt)+ )?
	] => {
		let mut $b $(: $ty)? = *$a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	
	[ *$a: expr => $b: ident $(: $ty:ty)?
		$(, $($tt:tt)+)?
	] => {
		let $b $(: $ty)? = *$a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	//
	
	// ignore clone
	[ @$a: expr => mut $b: ident $(: $ty:ty)?
		$(, $($tt:tt)+ )?
	] => {
		let mut $b $(: $ty)? = $a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	[ @$a: expr => $b: ident $(: $ty:ty)?
		$(, $($tt:tt)+)?
	] => {
		let $b $(: $ty)? = $a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	//
	
	[ $a: expr => mut $b: ident $(: $ty:ty)?
		$(, $($tt:tt)+ )?
	] => {
		let mut $b $(: $ty)? = $a.clone();
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	[ $a: expr => $b: ident $(: $ty:ty)?
		$(, $($tt:tt)+)?
	] => {
		let $b $(: $ty)? = $a.clone();
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	

	
	[ mut *$a: ident $(: $ty:ty)?
		$(, $($tt:tt)+)?
	] => {
		let mut $a $(: $ty)? = *$a;
		
		$( $crate::enclose_var! {
			$($tt)+ 
		})?
	};
	[ *$a: ident $(: $ty:ty)?
		$(, $($tt:tt)+)?
	] => {
		let $a $(: $ty)? = *$a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	
	
	
	[ mut $a: ident $(: $ty:ty)?
		$(, $($tt:tt)+)?
	] => {
		let mut $a $(: $ty)? = $a.clone();
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	[ $a: ident $(: $ty:ty)?
		$(, $($tt:tt)+)?
	] => {
		let $a $(: $ty)? = $a.clone();
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	
	[] => {};
	[ $($unk:tt)* ] => {
		compile_error!("Undefined entry or unsupported arguments, please double-check the input.");
	};
}
