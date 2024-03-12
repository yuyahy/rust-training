const MAX_POINTS: u32 = 100_000;
fn main() {
    // 変数"x"を可変にするために"mut"キーワードを付与
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // シャドーイング
    // 同じスコープ内で以前に宣言された変数名と同じ変数名を利用して、
    // 後者で宣言し直すことができる。※型も違う物に変更できる
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    // 上記のブロック内の"y"は死んでいるので、値はその直前のy + 1に戻る
    println!("The value of y is: {}", y);
    // "mut"を使うと再代入とみなされコンパイルエラー(error[E0308]: mismatched types)
    // let mut spaces = "   "; // &str
    // spaces = spaces.len(); // usize
}
