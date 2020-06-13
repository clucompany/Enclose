
/// The old 'enclose' macro behavior is left only to support backward compatibility and to support some specific cases of the old initialization behavior.
#[macro_export]
macro_rules! enclose_dep {
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

