fn main() {
    let y = {
        let x = 3;
        x + 1 // セミコロンがないため、これは式となり値を返す
        // x + 1; // セミコロンが付いているので、これは文となり値を返さない(コンパイルエラー)
    };
    println!("the value of y : {}", y);

    let x = five();
    println!("the value of x : {}", x);
}

fn five() -> i32 {
    5 // セミコロンがないため、これは式となり値を返す(OK)
    return 5; // return で値を返す(OK)
    // 5; //セミコロンが付いているので、これは文となり値を返さない(コンパイルエラー)
