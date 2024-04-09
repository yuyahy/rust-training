// 引数によって、x or yのどちらの参照を返却するかが変わるため、
// コンパイラは所有権の生存期間(ライフタイム)が分からない
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn lifetime_example1() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
// このライフタイム注釈は本構造体のインスタンスが、part フィードで保持する参照よりも早く死ぬ事を意味する
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn lifetime_example2() {
    // 僕をイシュマエルとお呼び。何年か前・・・
    let novel = String::from("Call me Ishmael. Some years ago...");
    //                                                  "'.'が見つかりませんでした"
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}

fn lifetime_example3() {
    // 静的ライフタイム
    let s: &'static str = "I have a static lifetime.";
}

fn main() {
    lifetime_example1();
    lifetime_example2();
}
