use std::panic::PanicInfo;

struct Foo {
    data: i32
}
impl Foo {
    fn new(val: i32) -> Foo {
        Foo{data: val}
    }
    fn f(&self, a: i32, b: i32) -> i32 {
        self.data + a + b
    }
    fn set_field(&mut self, v: i32) {
        self.data = v;
    }
}
fn main() {
    let foo = Foo::new(30);
    foo.f(10, 10);
    println!("{}", foo.data)
}
