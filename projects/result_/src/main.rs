use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};
fn main() {
    // match_guard_example();
    // unwrap_example();
    read_username_from_file();
}

fn match_guard_example() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // マッチガード
        // Errorの発生原因がErrorKind::NotFoundの場合のハンドリング
        // 'ref'はガード条件式へerrorがムーブされないために必要
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            // ファイルが開けなかったので作成を試みる
            Ok(fc) => fc,
            Err(e) => {
                panic!("Tried to create file but there was a problem: {:?}", e)
            }
        },
        // Errorの発生原因がErrorKind::NotFound以外の場合のハンドリング
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
}

fn unwrap_example() {
    // unwrap()は返り値がOkであればその中身を、Errであればpanic!を実行する
    // let f = File::open("hello.txt").unwrap();
    // expect()を使うと、panic時のためのエラーメッセージが設定できる
    let f = File::open("hello.txt").expect("Failed to open file");
}

// 関数内部で発生したエラーを呼び出し元に返し、ハンドリングを委譲する
fn read_username_from_file() -> Result<String, io::Error> {
    // 丁寧なバージョン
    // let f = File::open("hello.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();

    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    // '?'を使って簡潔に書いたバージョン
    // ※'?'を使うと、受け取ったエラー型が本関数の返り値で定義されている型に変換される
    //  内部で色々なタイプのエラーが発生する可能性があるが、委譲先には特定の1タイプのエラーを渡したい時に有用
    // ※Resultを返す関数に対してのみ使用できる点に注意
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    // チェーンにして更に簡略化
    // let mut s = String::new();

    // File::open("hello.txt")?.read_to_string(&mut s)?;

    // Ok(s)
}
