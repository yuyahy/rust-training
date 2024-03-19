fn main() {
    let x = 5;
    let y = x;
    println!("y : {}", y);
    println!("x : {}", x);

    let s = String::from("hello");
    takes_ownership(s);

    // 所有権はtakes_ownership()のsome_stringへ移動しているのでエラー
    // println!("Dead : {}", s);

    makes_copy(x);
    // xとmakes_copy()のsome_integerは別アドレスなのでまだアクセスできる
    println!("Alive : {}", x);
}

fn takes_ownership(some_string: String) {
    // StringはCopyトレイトが実装されていないので、
    // 呼び出し元で渡された値はsome_stringに所有権が移動する
    println!("{}", some_string);
    // このスコープを抜けるとsome_stringが確保していたメモリは解放される
}

fn makes_copy(some_integer: i32) {
    // i32はプリミティブ型なのでCopyトレイトが実装されている
    // よって呼び出し元で渡された値とsome_integerは別のアドレスの変数であり、
    // 所有権は移動していない
    println!("{}", some_integer);
    // このスコープを抜けるとsome_integerが確保していたメモリは解放される
}
