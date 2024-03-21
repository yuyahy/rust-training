fn main() {
    let mut s = String::from("hello world");

    // スライスでアクセス
    // 開始と末端は省略可
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let _first_word = first_word(&s);
    println!("first word is {}", _first_word);
    // sはこのスコープ内で不変借用されているので、可変借用はできない
    // s.clear();
    // println!("first word is {}", first_word);
    let literal_str = "This is literal str";
    let _first_word = first_word(&literal_str);
    println!("first word is {}", _first_word);
}

// 文字列から最初の1単語を抽出する関数
// str: 静的な文字列(スタック)を扱う。
// String: 動的な文字列(ヒープ)を扱う。strと互換性を持つ。
// よって、引数を&String→&strとするとstrとStringの両方の引数を受ける事ができる
// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // Note: 以下の処理はASCIIを前提としている(マルチバイト文字の途中でスライスしようとすると予期せぬエラーになるため)
    // i: sのインデックス
    // item: 対応する文字
    for (i, &item) in bytes.iter().enumerate() {
        // 空白かどうかをチェック
        if item == b' ' {
            return &s[..i];
        }
    }

    // 空白が見つからなかった場合は、1単語のみだったと考える
    &s[..]
}
