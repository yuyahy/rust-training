// traitを用いて、Summary traitの共通の振る舞いを定義する
pub trait Summary {
    // このメソッドは各型で実装する必要がある
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // デフォルトの振る舞いを定義
        // "（{}さんの文章をもっと読む）"
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    // コメントアウトした場合は、Summary traitに定義されたデフォルト動作が使用される
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Summary traitを実装した型を前提とした引数:itemを処理する関数
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// Summary traitを実装した型を前提とした引数:item1, item2を処理する関数
// item1, item2はSummary traitが実装されていれば、同じ型でなくてもOK
//pub fn notify(item1: &impl Summary, item2: &impl Summary) {

// Summary traitを実装した型を前提とした引数:item1, item2を処理する関数
// 上記と異なり、item1, item2は同じ型でなくてはならない
//pub fn notify<T: Summary>(item1: &T, item2: &T) {

// 複数のトレイト境界を定義する
// pub fn notify(item: &(impl Summary + Display)) {}

// where句を使い、複数のトレイト境界を見やすく定義できる
// fn some_function<T, U>(t: &T, u: &U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug

// Summary　traitを実装しているある型を返す
// Note: 処理の分岐によって異なる型を返す事はできない
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
