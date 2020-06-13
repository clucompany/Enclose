
#[macro_export]
macro_rules! ignore_prev_enclose {
	[( $($enc_args:tt)* ) $($add_prefix:ident)? || $b:block ] => {{ // empty args
		$($add_prefix)? || {
			$crate::enclose_var! {
				$( $enc_args )*
			}
			
			$b
		}
	}};
	[( $($enc_args:tt)* ) $($add_prefix:ident)? | $($args:tt),* | $b:block ] => {{ // args
		$($add_prefix)? | $($args),* | {
			$crate::enclose_var! {
				$( $enc_args )*
			}
			
			$b
		}
	}};
	
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

#[macro_export]
macro_rules! ignore_prev_enc {
	[$($all:tt)*] => {
		$crate::ignore_prev_enc! {
			$($all)*
		}
	};
}