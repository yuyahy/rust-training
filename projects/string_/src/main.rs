fn main() {
    create_new_string();
    add_string();
    concat_string();
    access_index();
}

// 文字列はUTF-8エンコードされる
fn create_new_string() {
    // 新規文字列を生成
    let mut s = String::new();
    // 文字列の初期値がある場合
    let data = "initial contents";
    let s = data.to_string();
    // 以下もOK
    // let s = "initial contents".to_string();
    let s = String::from("initial contents");
}

fn add_string() {
    let mut s = String::from("foo");
    let s2 = "bar";
    // 文字列を追加する場合はpush_str
    s.push_str(s2);
    println!("s : {}", s);
    // push_str()は所有権の移動は起こらないので、後で使用できる
    println!("s2 : {}", s2);

    // 文字を追加する場合はpush
    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);
}

fn concat_string() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world");
    // s1はここでmoveされ、s2はされない
    let s3 = s1 + &s2;

    let s1 = String::from("one");
    let s2 = String::from("two");
    let s3 = String::from("three");
    // formatは引数の所有権を奪わない
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}

fn access_index() {
    let s1 = "hello".to_string();
    // Rustの文字列は添字アクセスをサポートしていないためエラー
    // let h = s1[0];

    // 文字列の区切りとインデックスは必ずしも対応しない
    let hello = "Здравствуйте";
    // 上記のhelloは一文字が2byteなので2byte区切りでスライスしないとエラー
    // let s = &hello[0..3];
    let s = &hello[0..4];
    println!("{}", s);

    // 人間が考える一文字ずつ処理するにはforが使える
    for c in hello.chars() {
        println!("{}", c);
    }
}
