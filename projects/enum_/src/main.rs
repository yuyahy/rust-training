// IPアドレスを表現するenum
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// enumもstructと同様にメソッドを実装できる
impl Message {
    fn call(&self) {
        println!("This is call method in Message enum");
    }
}

// fn main() {
//     let home = IpAddr::V4(127, 0, 0, 1);
//     let loopback = IpAddr::V6(String::from("::1"));

//     let m = Message::Write(String::from("hello"));
//     m.call();
// }

fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    let dividend = 10;
    let divisor = 5;

    // 割り算が成功する場合
    match divide(dividend, divisor) {
        Some(result) => println!("Result of division: {}", result),
        None => println!("Division by zero error!"),
    }

    // 0で割る場合
    let divisor = 0;
    match divide(dividend, divisor) {
        Some(result) => println!("Result of division: {}", result),
        None => println!("Division by zero error!"),
    }
}

// Option<i8>は加算が実装されていないので、エラー// fn invalid_add_to_Option() {
//     let x: i8 = 5;
//     let y: Option<i8> = Some(5);

//     let sum = x + y;
// }
