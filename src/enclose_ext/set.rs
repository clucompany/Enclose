
#[macro_export]
macro_rules! set_enclose {
	// box def
	[ $i:ident = box $(@$p_tt:tt)? ($($enc_args:tt)*) $($enc_prefix:ident)? || $b:block $(as $as_ty:ty)? ; $($all:tt)* ] => {
		$i = Box::new($crate::enclose! {
			$(@$p_tt)?
			($($enc_args)*)
			
			
			$($enc_prefix)? || $b
		}) $(as $as_ty)?;
		
		$crate::set_enclose! {
			$($all)*
		}
	};
	[ $i:ident = box $(@$p_tt:tt)? ($($enc_args:tt)*) $($enc_prefix:ident)? | $($args:tt),* | $b:block $(as $as_ty:ty)? ; $($all:tt)* ] => {
		$i = Box::new($crate::enclose! {
			$(@$p_tt)?
			($($enc_args)*)
			
			
			$($enc_prefix)? | $($args),* | $b
		}) $(as $as_ty)?;
		
		$crate::set_enclose! {
			$($all)*
		}
	};
	// def
	
	// def
	[ $i:ident = $(& $($l:lifetime)?)? $(@$p_tt:tt)? ($($enc_args:tt)*) $($enc_prefix:ident)? || $b:block $(as $as_ty:ty)? ; $($all:tt)* ] => {
		$i = $(& $($l)?)? $crate::enclose! {
			$(@$p_tt)?
			($($enc_args)*)
			
			
			$($enc_prefix)? || $b
		} $(as $as_ty)?;
		
		$crate::set_enclose! {
			$($all)*
		}
	};
	[ $i:ident = $(& $l:lifetime)? $(@$p_tt:tt)? ($($enc_args:tt)*) $($enc_prefix:ident)? | $($args:tt),* | $b:block $(as $as_ty:ty)? ; $($all:tt)* ] => {
		$i = $(& $l)? $crate::enclose! {
			$(@$p_tt)?
			($($enc_args)*)
			
			
			$($enc_prefix)? | $($args),* | $b
		} $(as $as_ty)?;
		
		$crate::set_enclose! {
			$($all)*
		}
	};
	// def
	
	// block
	[ {$($b:tt)*}; $($all:tt)* ] => {{
		$($b)*
		
		$crate::set_enclose! {
			$($all)*
		}
	}};
	// end block
	
	/*[ $($unk:tt)+ ] => {
		compile_error!("Undefined entry or unsupported arguments, please double-check input.");
	};*/
	[] => {};
}

#[macro_export]
macro_rules! set_enc {
	[ $($all:tt)* ] => {
		$crate::set_enclose! {
			$($all)*
		}
	};
}