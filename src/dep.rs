
/// The old 'enclose' macro behavior is left only to support backward compatibility and to support some specific cases of the old initialization behavior.
#[macro_export]
macro_rules! dep_enclose {
	[ ( $($enc_args:tt)* ) $($all:tt)* ] => {{
		$crate::enclose_var! {
			$( $enc_args )*
		}

		$($all)*
	}};
	
	/*[ $($unk:tt)+ ] => {
		compile_error!("Undefined entry or unsupported arguments, please double-check input.");
	};*/
	[] => {};
}

/// The old 'enclose' macro behavior is left only to support backward compatibility and to support some specific cases of the old initialization behavior.
#[macro_export]
macro_rules! dep_enc {
	[ $($all:tt)* ] => {
		$crate::dep_enclose! {
			$($all)*
		}
	};
}