enum List {
    // 以下で自身であるenum Listを保持しているため、再帰的な型になっている
    // しかし、このままだとコンパイラがenum Listのために確保すべきサイズが判別できないためエラー
    //Cons(i32, List),
    // なので値そのものを保持するのではなく、Box<T>を使ってポインタで保持すればOK
    // Note: ポインタはCPUアーキテクチャによってサイズは固定
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};
fn main() {
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
