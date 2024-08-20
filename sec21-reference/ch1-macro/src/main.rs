use std::cell::RefCell;

fn main() {
    // Used as an expression.
    let x = vec![1,2,3];

    // Used as a statement.
    println!("Hello!");

    // Used in a pattern.
    macro_rules! pat {
        ($i:ident) => (Some($i))
    }

    if let pat!(x) = Some(1) {
        assert_eq!(x, 1);
    }

    // Used in a type.
    macro_rules! Tuple {
        { $A:ty, $B:ty } => { ($A, $B) };
    }

    type N2 = Tuple!(i32, i32);

    // Used as an item.
    thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

    // Used as an associated item.
    macro_rules! const_maker {
        ($t:ty, $v:tt) => { const CONST: $t = $v; };
    }
    trait T {
        const_maker!{i32, 7}
    }

    // Macro calls within macros.
    macro_rules! example {
        () => { println!("Macro call in a macro!") };
    }
    // Outer macro `example` is expanded, then inner macro `println` is expanded.
    example!();

    macro_rules! foo {
        ($l:expr) => { bar!($l); }
    // ERROR:               ^^ no rules expected this token in macro call
    }
    macro_rules! bar {
        (3) => {}
    }
    foo!(3);

    // compiles OK
    macro_rules! foo {
        ($l:tt) => { bar!($l); }
    }
    macro_rules! bar {
        (3) => {}
    }
    foo!(3);

}