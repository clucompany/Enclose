
mod var;
mod dep;
mod run;

pub use self::dep::*;
pub use self::run::*;
pub use self::var::*;

/// A macro for creating a closure, as well as cloning, copying values ​​into the closure.
#[macro_export]
macro_rules! enclose {
	// deprecated method.
	[@deprecated ( $($enc_args:tt)* ) $b:expr ] => {{
		#![deprecated = "Use 'enclose_dep' with the same parameters."]
		$crate::enclose_dep! {
			($enc_args)* $b
		}
	}};
	
	// PREV
	
	[@!prev $($tt:tt)* ] => {{ // prev deprecated
		#![deprecated = "Use 'ignore_prev' instead of the old '!prev'."]
		crate::enclose! {
			@ignore_prev $($tt)*
		}
	}};
	
	[@prev ( $($enc_args:tt)* ) $($add_prefix:tt)? || $($b:tt)* ] => {{ // empty args
		$crate::enclose_var! {
			$( $enc_args )*
		}
		$($add_prefix)? || $($b)*
	}};
	[@prev ( $($enc_args:tt)* ) $($add_prefix:tt)? | $($args:tt),* | $($b:tt)* ] => {{ // args
		$crate::enclose_var! {
			$( $enc_args )*
		}
		$($add_prefix)? | $($args),* | $($b)*
	}};
	
	[@ignore_prev ( $($enc_args:tt)* ) $($add_prefix:tt)? || $($b:tt)* ] => {{ // empty args
		$($add_prefix)? || {
			$crate::enclose_var! {
				$( $enc_args )*
			}
			$($b)*
		}
	}};
	[@ignore_prev ( $($enc_args:tt)* ) $($add_prefix:tt)? | $( $args:tt ),* | $($b:tt)* ] => {{ // args
		$($add_prefix)? | $($args),* | {
			$crate::enclose_var! {
				$( $enc_args )*
			}
			
			$($b)*
		}
	}};
	
	
	// default methods
	[( $($enc_args:tt)* ) move || $($b:tt)* ] => {{ // move, empty args
		$crate::enclose_var! {
			$( $enc_args )*
		}
		move || $($b)*
	}};
	[( $($enc_args:tt)* ) move | $($args:tt),* | $($b:tt)* ] => {{ // move, args
		$crate::enclose_var! {
			$( $enc_args )*
		}
		move | $($args),* | $($b)*
	}};
	
	
	[( $($enc_args:tt)* ) || $($b:tt)* ] => {{ // empty args
		|| {
			$crate::enclose_var! {
				$( $enc_args )*
			}
			
			$($b)*
		}
	}};
	[( $($enc_args:tt)* ) | $($args:tt),* | $($b:tt)* ] => {{ // args
		| $( $args ),* | {
			$crate::enclose_var! {
				$( $enc_args )*
			}
			
			$($b)*
		}
	}};
	
	/*
		data.run_closure2(
			enclose!(() StructData::null_fn)
		);
	*/
	// variable
	[( $($enc_args:tt)* ) $p:tt :: $($all:tt)* ] => {{
		$crate::enclose_var! {
			$( $enc_args )*
		}
		
		$p :: $($all)*
	}};
	
	[( $($enc_args:tt)* )] => {{ // empty
		$crate::enclose_var! {
			$( $enc_args )*
		}
	}};
	
	[] => {};
	[ $($unk:tt)* ] => {
		compile_error!("Undefined entry or unsupported arguments, please double-check input.");
	};
}

///Macro for cloning values to close. Alternative short record.
#[macro_export]
macro_rules! enc {
	[$($tt:tt)*] => {
		$crate::enclose! {
			$($tt)*
		}
	};
}
