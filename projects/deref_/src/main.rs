fn main() {
    let x = 5;
    let y = &x;
    let y = Box::new(x);
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // 参照外し(内部的には*(y.deref())が実行されている)

    let name = String::from("my name is...");
    // 参照外し型矯正がサポートされているので以下の様に書ける
    hello(&name);
    // もしサポートされていなかったら...
    //hello(&(*name)[..]);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

// スマートポインタを自作
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 参照外しをサポートするためには、Derefトレイトを実装する必要がある
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // MyBoxは実質要素を1つだけ持つタプルなので、
        // 先頭要素の参照を返す
        &self.0
    }
}
