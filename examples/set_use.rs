
use enclose::set_enc;

fn main() {
	let num = 1024;
	
	let a: Box<dyn FnOnce()>;
	let b: Box<dyn FnOnce()>;
	let c: Box<dyn FnOnce()>;
	let t: &'static dyn Fn();
	set_enc! {
		a = box (num => mut new_num) move || {
			new_num = new_num + 1024;
			println!("a {:?}", new_num);
		};
		{
			println!("prev TestOk");
		};
		b = box (mut num) move || {
			num += 1024;
			println!("b {:?}", num);
		};
		c = box (move num) move || {
			println!("c {:?}", num);
		};
		t = &() || {
			println!("t");
		};
	};
	
	(a)();
	(b)();
	(c)();
	(t)();
}
