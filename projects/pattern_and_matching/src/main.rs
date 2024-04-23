fn if_let_example() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tues day is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!(
                "Using purple as the background color"
            );
        } else {
            println!(
                "Using orange as the background color"
            );
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let_example() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // 最終的にstack.pop()はNoneを返すのでループが止まる
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_example() {
    let v = vec!['a', 'b', 'c'];

    // forの後に続く要素もパターン
    // →以下の例だと(index, value)の部分
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn let_example() {
    // 実は以下の様なlet構文もパターンを使用している
    // let PATTERN = EXPRESSION;
    let x = 5;

    // 以下は(x,y,z)というパターンと(1,2,3)という値を比較している
    let (x, y, z) = (1, 2, 3);
    // 以下はパターンと値が一致していないのでエラー
    // let (x, y) = (1, 2, 3);
}

// 関数の引数にもパターンが使用できる
fn function_example() {
    // タプルを引数に取り、受取時にそれぞれ個別の変数へパース
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current position : ({}, {})", x, y);
    }
    let point = (3, 5);
    print_coordinates(&point);
}

fn irrefutable_example() {
    // let文では論駁不可能な全ての値を受け入れる様なパターンしか書けない
    // 以下だとNoneのケースをカバーできないのでエラーになる
    // let Some(x) = Some(1);
    // if letならOKだと
    if let Some(x) = Some(1) {
        println!("{}", x);
    }

    // 以下はThe bookだとエラーになると記載されていたが最新のRustだとエラーにはならない
    // if let x = 5 {
    //     println!("{}", x);
    // }
}

fn matching_example() {
    let x = Some(77);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // Note: このスコープ内のyは2行目のyとは別物なので注意
        //        yはx = Some({値})のSome()内のあらゆる値にマッチする
        Some(y) => println!("Matched, y={:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 6;
    match x {
        // '|'を使うとアームにor条件を使用できる
        1 | 2 => println!("one or two"),
        // ..=を使うと特定の範囲の値を条件に使用できる(数値 or char型のみ)
        3..=10 => println!("three to ten"),
        _ => println!("anything"),
    }

    let x = 'e';
    match x {
        'a'..='d' => println!("a to d"),
        'e'..='z' => println!("e to z"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    // 変数pのx, yフィールドに合致する変数a, bを生成する
    // let Point { x: a, y: b } = p;
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point { x, y }| x * x + y * y)
        .sum();
    println!("sum_of_squares : {}", sum_of_squares);
}

fn main() {
    if_let_example();
    while_let_example();
    for_example();
    let_example();
    irrefutable_example();
    matching_example();
}
