fn main() {
    // loop_label_example();

    while_example();
    for_example();
}

fn loop_label_example() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining : {}", remaining);
            // remaining == 9になったら一番内側のループを終了させる
            if remaining == 9 {
                break;
            }
            // count == 2になったら外側のループラベル: counting_upを終了させる
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
}

fn while_example() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    // forとrev関数を使うと以下
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn for_example() {
    let a = [10, 20, 30, 40, 50];

    for elem in a {
        println!("the value is {}", elem);
    }
}
