/// A macro that generates code for working with variables based on input data.
///
/// (usually for internal use only)
///
/// ## Support
///
/// ### `.clone()` operation
/// ```code
/// // (a => mut b: String) will be converted to (let mut b: String = a.clone();)
/// // (b => mut b)
/// // (a => b: usize)
/// // (a => b)
/// // (mut a: String)
/// // (mut a)
/// // (a: String)
/// // (a)
/// ```
///
/// ### `*copy` operation
/// ```code
/// // (*a => mut b: &str) will be converted to (let mut b: &str = *a;)
/// // (*a => mut b)
/// // (*a => b: &str)
/// // (*a => b)
/// // (mut *a: &str)
/// // (mut *a)
/// // (*a: &str)
/// // (*a)
/// ```
///
/// ### `let ref mut a = b;`(`ref`) operation
/// ```code
/// // (ref mut a: String) will be converted to (let ref mut a: String = a;)
/// // (ref mut a)
/// // (ref a: String)
/// // (ref a)
/// // (ref a => mut b: String)
/// // (ref a => mut b)
/// // (ref a => b: String)
/// // (ref a => b)
/// ```
///
/// ### `(1+1)` (expr) operation
/// ```code
/// // (@(1+1) => mut b: usize) will be converted to (let mut b: usize = (1+1);)
/// // (@(1+1) => mut b)
/// // (@(1+1) => b: usize)
/// // (@(1+1) => b)
/// ```
///
/// ### `let a = b;` (move) operation
/// ```code
/// // (move a => mut b: String) will be converted to (let mut b: String = a;)
/// // (move a => mut b)
/// // (move a => mut b)
/// // (move a => b: String)
/// // (move a => b)
/// // (move mut a: String)
/// // (move mut a)
/// // (move a: String)
/// // (move a)
/// ```
///
/// ### `{println!("test");}` (run) operation
/// ```code
/// // { panic!("12"); }
/// ```
#[macro_export]
macro_rules! enclose_var {
	[
		// ref operation
		// (ref a => mut b: String)
		// (ref a => mut b)
		ref $a: expr => mut $b: ident $(: $ty:ty)?

		$(, $($tt:tt)+ )?
	] => {
		let ref mut $b $(: $ty)? = $a;

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};
	[
		// ref operation
		// (ref a => b: String)
		// (ref a => b)
		ref $a: expr => $b: ident $(: $ty:ty)?

		$(, $($tt:tt)+ )?
	] => {
		let ref $b $(: $ty)? = $a;

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};
	//

	[
		// move operation
		// (move a => mut b: String)
		// (move a => mut b)
		move $a: expr => mut $b: ident $(: $ty:ty)?

		$(, $($tt:tt)+ )?
	] => {
		let mut $b $(: $ty)? = $a;

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};

	[
		// move operation
		// (move a => b: String)
		// (move a => b)
		move $a: expr => $b: ident $(: $ty:ty)?

		$(, $($tt:tt)+ )?
	] => {
		let $b $(: $ty)? = $a;

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};
	//

	[
		// copy operation
		// (*a => mut b: &str)
		// (*a => mut b)
		*$a: expr => mut $b: ident $(: $ty:ty)?

		$(, $($tt:tt)+ )?
	] => {
		let mut $b $(: $ty)? = *$a;

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};

	[
		// copy operation
		// (*a => b: String)
		// (*a => b)
		*$a: expr => $b: ident $(: $ty:ty)?

		$(, $($tt:tt)+ )?
	] => {
		let $b $(: $ty)? = *$a;

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};
	//

	[
		// expr
		// (@(1+1) => mut b: usize)
		// (@(1+1) => mut b)
		@$a: expr => mut $b: ident $(: $ty:ty)?

		$(, $($tt:tt)+ )?
	] => {
		let mut $b $(: $ty)? = $a;

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};

	[
		// expr
		// (@(1+1) => b: usize)
		// (@(1+1) => b)
		@$a: expr => $b: ident $(: $ty:ty)?

		$(, $($tt:tt)+ )?
	] => {
		let $b $(: $ty)? = $a;

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};
	//

	[
		// ref
		// (ref mut a: String)
		// (ref mut a)
		ref mut $a: ident $(: $ty:ty)?

		$(, $($tt:tt)+)?
	] => {
		let ref mut $a $(: $ty)? = $a;

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};

	[
		// ref
		// (ref a: String)
		// (ref a)
		ref $a: ident $(: $ty:ty)?

		$(, $($tt:tt)+)?
	] => {
		let ref $a $(: $ty)? = $a;

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};
	// end

	[
		// move
		// (move mut a: String)
		// (move mut a)
		move mut $a: ident $(: $ty:ty)?

		$(, $($tt:tt)+)?
	] => {
		let mut $a $(: $ty)? = $a;

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};
	[
		// move
		// (move a: String)
		// (move a)
		move $a: ident $(: $ty:ty)?

		$(, $($tt:tt)+)?
	] => {
		let $a $(: $ty)? = $a;

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};
	// end

	[
		// clone
		// (a => mut b: usize)
		// (a => mut b)
		$a: expr => mut $b: ident $(: $ty:ty)?

		$(, $($tt:tt)+ )?
	] => {
		let mut $b $(: $ty)? = $a.clone();

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};
	[
		// clone
		// (a => b: usize)
		// (b => b)
		$a: expr => $b: ident $(: $ty:ty)?

		$(, $($tt:tt)+ )?
	] => {
		let $b $(: $ty)? = $a.clone();

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};
	// end

	[
		// copy
		// (mut *a: String)
		// (mut *a)
		mut *$a: ident $(: $ty:ty)?

		$(, $($tt:tt)+)?
	] => {
		let mut $a $(: $ty)? = *$a;

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};
	[
		// copy
		// (*a: String)
		// (*a)
		*$a: ident $(: $ty:ty)?

		$(, $($tt:tt)+)?
	] => {
		let $a $(: $ty)? = *$a;

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};
	// end copy

	[
		// clone
		// (mut a: String)
		// (mut a)
		mut $a: ident $(: $ty:ty)?

		$(, $($tt:tt)+)?
	] => {
		let mut $a $(: $ty)? = $a.clone();

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};
	[
		// clone
		// (a: String)
		// (a)
		$a: ident $(: $ty:ty)?

		$(, $($tt:tt)+)?
	] => {
		let $a $(: $ty)? = $a.clone();

		$(
			$crate::enclose_var! {
				$($tt)+
			}
		)?
	};
	// end clone

	[
		// run_block
		// { panic!("12"); }
		{$($b:tt)*}

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
	[ $(,)? ] => {};
}
