enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;
use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //  Rc::clone()を実行すると参照カウントがインクリメントされる
    // a生成後のカウント = {}
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    // b生成後のカウント = {}
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        // c生成後のカウント = {}
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    // cがスコープを抜けた後のカウント = {}
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
