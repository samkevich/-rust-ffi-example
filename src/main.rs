extern "C" {
    pub fn foo();
}

fn main() {
    unsafe {
	foo();
    }
}
