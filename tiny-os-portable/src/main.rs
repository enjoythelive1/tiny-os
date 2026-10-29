extern crate tiny_os_base;

struct Portable;

impl tiny_os_base::Hardware for Portable  {
    fn debug(&self, str: &str) {
        println!("{}", str);
    }
}


fn main() {
    tiny_os_base::start(Portable).expect("Failed to execute")
}
