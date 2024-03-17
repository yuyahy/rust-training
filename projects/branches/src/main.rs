fn main() {
    let number = 4;
    let mut condition = false;

    if number < 5 {
        // if number { // Rustでは論理値以外が自動的に論理値に変換されることはないのでエラー
        println!("condition was true");
        condition = true;
    } else {
        println!("condition was false");
        condition = false;
    }

    // 3項演算子
    let number = if condition { number } else { 0 }; // ifは式なので右辺に持ってくる事ができる
    let number = if condition { number } else { "dummy" }; // ただしif-else句で異なる型を取ることはできないのでエラー
    println!("number : {}", number);
}
