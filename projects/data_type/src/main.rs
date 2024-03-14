fn main() {
    // 文字列"42"をパースし、u32型に変換できる場合は42、
    // できない場合は"Not a number!"が出力される
    let _guess: u32 = "42".parse().expect("Not a number!");

    let x = 2.0;
    let y: f32 = 3.0;
    println!("x : {}", x);
    println!("y : {}", y);

    // Rustの文字はシングルクォーテーションで囲む点に注意
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻'; //ハート目の猫
    println!("{c}");
    println!("{z}");
    println!("{heart_eyed_cat}");

    // タプル
    let tup = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    println!("The value of _y is: {_y}");
    // インデックスの0番目にアクセス
    println!("{}", tup.0);

    // 配列(固定長)
    let _array = [1, 2, 3, 4, 5];
    let _months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
}
