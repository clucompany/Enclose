
//https://github.com/clucompany/Enclose/issues/1
/*
error: no rules expected the token `.`
   --> src/editview/src/view_item.rs:154:38
       |
154 |             enclose!((edit_view, self.gestures.drag_data => drag_data) move |_, start_x, start_y| {
       |                                      ^ no rules expected this token in macro call

*/

// thank!
// 14.08.2019 13:48 Minsk/Europe UTC+03:00
// UlinKot 1819

use enclose::run_enclose;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct CheckData {
	a: u32,
}

impl CheckData {
	#[inline]
	pub const fn new(a: u32) -> Self {
		Self {
			a: a,
		}
	}
	
	pub fn calculate(&self, mul_num: &u32) -> u32 {
		run_enclose!((*mul_num, self.a => mut num0) move || {
			// (*mul_num, self.a => mut num0) ->
			// let mul_num = *mul_num;
			// let num0 = self.a.clone();
			
			num0 *= mul_num;
			num0 += 1024;
			
			num0
		})
		
		// run_enclose
		//
		// let mut enclose = $crate::enclose!( $($tt)* );
		// enclose()
	}
}

#[test]
fn cogitri_issue() {
	let data = CheckData::new(1024);
	
	let calculate = data.calculate(&2);
	// (data.a * 2) + 1024
	assert_eq!(calculate, 3072);
	
	// check data
	assert_eq!(data.a, 1024);
}