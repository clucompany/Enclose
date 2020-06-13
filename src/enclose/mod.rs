
mod var;
mod prev;
mod ignore_prev;

pub use self::ignore_prev::*;

/// A macro for creating a closure, as well as cloning, copying values â€‹â€‹into the closure.
#[macro_export]
macro_rules! enclose {
	// deprecated method.
	[@deprecated $($all:tt)* ] => {{
		#[deprecated = "Use 'enclose_dep' with the same parameters."]
		$crate::enclose_dep! {
			$($all)*
		}
	}};
	
	// PREV
	[@!prev $($tt:tt)* ] => {{ // prev deprecated
		#[deprecated = "Use 'ignore_prev' instead of the old '!prev'."]
		crate::ignore_prev_enclose! {
			$($tt)*
		}
	}};
	
	[@prev $($tt:tt)* ] => {{ // empty args
		$crate::prev_enclose! {
			$( $tt )*
		}
	}};
	
	[@ignore_prev $($tt:tt)* ] => {{ // empty args
		$crate::ignore_prev_enclose! {
			$( $tt )*
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
	[( $($enc_args:tt)* ) | $($args:tt),* | $($b:tt)*] => {{ // args
		| $($args),* | {
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
	[( $($enc_args:tt)* ) $p:tt $(:: $($all:tt)+)? ] => {{
		$crate::enclose_var! {
			$( $enc_args )*
		}
		
		$p $(:: $($all)+)?
	}};
	
	[( $($enc_args:tt)* )] => {{ // empty
		$crate::enclose_var! {
			$( $enc_args )*
		}
	}};
	
	[ $($unk:tt)+ ] => {
		compile_error!("Undefined entry or unsupported arguments, please double-check input.");
	};
	[] => {};
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
