#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    // matchの各アームは式
    match coin {
        Coin::Penny => 1,
        // 以下の様に{}をつければ複数行の処理も書ける
        // Coin::Penny => {
        //     println!("Lucky penny!");
        //     1
        // }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn place_holder_example(some_u8_value: u8) {
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        // プレースホルダーは上記のアーム以外の全てのパターンにマッチする
        _ => (),
    }
}

fn if_let_example(some_u8_value: Option<u8>) {
    // ある特定の値のみ処理したい場合は、if letを使う(if let - elseもできる)
    // ※コンパイラによる網羅チェックは行われない事に注意
    if let Some(3) = some_u8_value {
        println!("three by if let")
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let three = Some(3);
    if_let_example(three)
}
