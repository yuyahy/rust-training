fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 1. 可変な文字列を生成
    let mut s1 = String::from("hello");
    // 3. 可変な参照を渡す
    change(&mut s1);
    println!("The length of '{}' is {}.", s1, len);
}

// 2. 可変な借用の関数を用意
fn change(s: &mut String) {
    s.push_str(", world");
}

// 参照しているので本関数に渡した引数のムーブは行われない
// これを借用と呼ぶ
fn calculate_length(s: &String) -> usize {
    s.len()
}

// 不変な借用の場合、その変数を変更することはできない
// fn cannot_change(s: &String) {
//     s.push_str(", world");
// }

// 同じスコープ内である変数に対して複数の可変参照を作ることはできない(データ競合が発生する可能性があるため)
// fn cannot_create_multiple_mutable_reference() {
//     let mut s = String::from("hello");
//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }

fn create_multiple_mutable_reference() {
    let mut s = String::from("hello");
    // このブロックを抜けた後にr1の借用は終了する
    {
        let r1 = &mut s;
    }
    // この時点でsを可変参照している変数はいないのでOK
    let r2 = &mut s;
}

// 同じスコープ内で不変参照と可変参照を組み合わせる事はできない
// fn mix_mutable_immutable_reference() {
//     let mut s = String::from("hello");

//     // 不変参照は同じスコープ内でいくつ存在してもOK
//     let r1 = &s;
//     let r2 = &s;
//     // 可変参照と不変参照を組み合わせるとデータの競合が起こる可能性があるのでNG
//     let r3 = &mut s;
//     change(r3);

//     println!("{}", r3);
//     // Note: 以下の様に不変参照を実際に使用するコードがないとコンパイルエラーに引っかからない
//     println!("{}, {}", r1, r2);
// }

// 本関数のスコープを抜けた時点で、sはメモリから解放されており、
// 返り値の参照は無効なアドレスを指すことになるのでエラー
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// 実体を返す様に変更すればOK
// 所有権が呼び出し元にムーブされるため
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
