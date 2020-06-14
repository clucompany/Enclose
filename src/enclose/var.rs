
#[doc(hidden)]
#[macro_export]
macro_rules! enclose_var {
	// ref operation
	[ ref $a: expr => mut $b: ident $(: $ty:ty)?
		$(, $($tt:tt)+ )?
	] => {
		let ref mut $b $(: $ty)? = $a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	[ ref $a: expr => $b: ident $(: $ty:ty)?
		$(, $($tt:tt)+ )?
	] => {
		let ref $b $(: $ty)? = $a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	//
	
	// move operation
	[ move $a: expr => mut $b: ident $(: $ty:ty)?
		$(, $($tt:tt)+ )?
	] => {
		let mut $b $(: $ty)? = $a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	[ move $a: expr => $b: ident $(: $ty:ty)?
		$(, $($tt:tt)+ )?
	] => {
		let $b $(: $ty)? = $a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	//
	
	
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
		$(, $($tt:tt)+ )?
	] => {
		let $b $(: $ty)? = *$a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	//
	
	// expr
	[ @$a: expr => mut $b: ident $(: $ty:ty)?
		$(, $($tt:tt)+ )?
	] => {
		let mut $b $(: $ty)? = $a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	[ @$a: expr => $b: ident $(: $ty:ty)?
		$(, $($tt:tt)+ )?
	] => {
		let $b $(: $ty)? = $a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	//
	
	// ref
	[ ref mut $a: ident $(: $ty:ty)?
		$(, $($tt:tt)+)?
	] => {
		let ref mut $a $(: $ty)? = $a;
		
		$( $crate::enclose_var! {
			$($tt)+ 
		})?
	};
	[ ref $a: ident $(: $ty:ty)?
		$(, $($tt:tt)+)?
	] => {
		let ref $a $(: $ty)? = $a;
		
		$( $crate::enclose_var! {
			$($tt)+ 
		})?
	};
	// end move
	// move
	[ move mut $a: ident $(: $ty:ty)?
		$(, $($tt:tt)+)?
	] => {
		let mut $a $(: $ty)? = $a;
		
		$( $crate::enclose_var! {
			$($tt)+ 
		})?
	};
	[ move $a: ident $(: $ty:ty)?
		$(, $($tt:tt)+)?
	] => {
		let $a $(: $ty)? = $a;
		
		$( $crate::enclose_var! {
			$($tt)+ 
		})?
	};
	// end move
	
	// expr
	[ $a: expr => mut $b: ident $(: $ty:ty)?
		$(, $($tt:tt)+ )?
	] => {
		let mut $b $(: $ty)? = $a.clone();
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	[ $a: expr => $b: ident $(: $ty:ty)?
		$(, $($tt:tt)+ )?
	] => {
		let $b $(: $ty)? = $a.clone();
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	// end expr
	
	
	
	// copy
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
	// end copy
	
	// clone
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
	// end clone
	
	// block
	[ {$($b:tt)*}
		$(, $($tt:tt)+)?
	] => {
		let _hidden = {$($b)*};
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	// end block
	
	/*[ $($unk:tt)+ ] => {
		compile_error!("Undefined entry or unsupported arguments, please double-check the input.")
	};*/
	[] => {};
}
