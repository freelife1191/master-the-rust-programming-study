fn main() {
    // let s = String::from("hello");
    // let t = s;
    // println!("t is {}", t);
    // println!("s is {}", s); // 오류: 's'는 이제 더 이상 유효하지 않습니다.

    // let mut s = String::from("Hello");
    // let t = String::from("World");
    // s = t;
    // println!("s is {}", s);

    let p = get_pointer();
    println!("Value pointed to by p: {}", p);
}

fn get_pointer() -> &'static i32 {
    let value = 5;
    &value
}
