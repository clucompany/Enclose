
mod var;
mod prev;
mod ignore_prev;

pub use self::var::*;
pub use self::prev::*;
pub use self::ignore_prev::*;

/// A macro for creating a closure, as well as cloning, copying values â€‹â€‹into the closure.
#[macro_export]
macro_rules! enclose {
	// deprecated method.
	[@deprecated $($all:tt)* ] => {{
		#[cfg(not(disable_dep))]
		$crate::dep_enclose! {
			$($all)*
		}
		
		#[cfg(disable_dep)]
		compile_error!("The 'disable_dep' flag is enabled, it is required not to specify it in cargo.toml.");
	}};
	
	// PREV init methods, deprecated
	[@!prev $($tt:tt)* ] => {{ // prev deprecated
		#[deprecated(
			since = "1.1.9",
			reason = "Use 'ignore_prev' instead of the old '!prev'."
		)]
		$crate::ignore_prev_enclose! {
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
	// END PREV
	
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
	
	/*[ $($unk:tt)+ ] => {
		compile_error!("Undefined entry or unsupported arguments, please double-check input.");
	};*/
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
