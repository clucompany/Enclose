

#[doc(hidden)]
#[macro_export]
macro_rules! enclose_var {
	// copy operation
	[ *$a: expr => mut $b: ident
		$(, $($tt:tt)+ )?
	] => {
		let mut $b = *$a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	
	[ *$a: expr => $b: ident
		$(, $($tt:tt)+)?
	] => {
		let $b = *$a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	//
	
	// ignore clone
	[ @$a: expr => mut $b: ident
		$(, $($tt:tt)+ )?
	] => {
		let mut $b = $a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	[ @$a: expr => $b: ident
		$(, $($tt:tt)+)?
	] => {
		let $b = $a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	//
	
	[ $a: expr => mut $b: ident
		$(, $($tt:tt)+ )?
	] => {
		let mut $b = $a.clone();
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	[ $a: expr => $b: ident
		$(, $($tt:tt)+)?
	] => {
		let $b = $a.clone();
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	

	
	[ mut *$a: ident
		$(, $($tt:tt)+)?
	] => {
		let mut $a = *$a;
		
		$( $crate::enclose_var! {
			$($tt)+ 
		})?
	};
	[ *$a: ident
		$(, $($tt:tt)+)?
	] => {
		let $a = *$a;
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	
	
	
	[ mut $a: ident
		$(, $($tt:tt)+)?
	] => {
		let mut $a = $a.clone();
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	[ $a: ident
		$(, $($tt:tt)+)?
	] => {
		let $a = $a.clone();
		
		$( $crate::enclose_var! {
			$($tt)+
		})?
	};
	
	[] => {};
	[ $($unk:tt)* ] => {
		compile_error!("Undefined entry or unsupported arguments, please double-check the input.");
	};
}
