fn main() {
    // 以下のいずれかの方法でVec<T>を生成できる
    // let mut v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3, 4, 5];

    // 各要素にアクセスするためには、
    // 添字アクセス or .get()を使う
    {
        let third: i32 = v[2];
        println!("The third element is {}", third);
    }
    println!("{}", v[2]);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    access_out_of_index(&v);

    reference_error_vec(&mut v);
    println!("The fifth element is {}", v[5]);

    for_vector(&mut v);

    use_enum_in_vector();
}

fn access_out_of_index(target_vec: &Vec<i32>) {
    // 以下のコードはpanicでプログラムが終了する
    // let does_not_exist = target_vec[100];

    // 以下はインデックスが範囲外であればNone
    let does_not_exist_by_get = target_vec.get(100);
    match does_not_exist_by_get {
        Some(i) => println!("Index 100 is {}", i),
        None => println!("Invalid index"),
    }
}

fn reference_error_vec(target_vec: &mut Vec<i32>) {
    // 以下のコメントアウトを解除するとコンパイルエラー
    // →可変参照と不変参照が同じスコープに存在するため
    // let first = &target_vec[0];
    target_vec.push(6);

    // println!("The first element is {}", first);
}

fn for_vector(target_vec: &mut Vec<i32>) {
    for i in target_vec {
        // i: &mut Vec<i32>なので参照外しをした上で処理する必要がある
        *i += 50;
        println!("{}", i);
    }
}

fn use_enum_in_vector() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(0.5),
        SpreadsheetCell::Text(String::from("something")),
    ];
    println!("row : {:#?}", row);
}
