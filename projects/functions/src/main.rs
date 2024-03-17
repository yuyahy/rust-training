fn main() {
    println!("Hello, world!");

    another_functions(5);

    print_labeled_measurement(5, '6');
}

// 後に定義された関数も呼べる
// 関数の引数はかならず型を指定し、意図をコンパイラに伝える必要がある
fn another_functions(x: i32) {
    println!("The value of x is : {}", x);
}
// 引数が2個のパターン
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {} {}", value, unit_label);
}
