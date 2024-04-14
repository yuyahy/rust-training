fn main() {
    println!("Hello, world!");
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    // next()を呼び出す度に、iteratorの内部の状態が変わるのでmutが必要
    let mut v1_iter = v1.iter();

    // iterator内部ではnext()メソッドで次の要素にアクセスしている
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // sum()はiteratorの所有権を奪うため、以降の処理では元のiteratorは使えない(エラー)

    // let size = v1_iter.size_hint();
}

#[test]
fn generate_new_iterator_example() {
    let v1 = vec![1, 2, 3];
    // iteratorは遅延評価のため、実際に使用されるまでは何も処理が実行されない(ワーニング)
    // v1.iter().map(|x| x + 1);
    // 各要素に対してmapで処理した物を新たなコレクションに変換する
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, [2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoe_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter()で値からiteraorを生成→filterで条件に合致する要素のみを取得→コレクションに変換する
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filter_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoe_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    );
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// 1..5をカウントするiterator
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn test_counter_iterator() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None); // 終端
}

#[test]
fn using_other_iterator_trait_methods() {
    // Note: zipは片方のペアがNoneの場合は生成がスキップされるため、最後の(5, None)は生成されない
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1)) // CounterとCounterの先頭要素を削除した物のペア
        .map(|(a, b)| a * b) // ペアの対応するインデックス同士の積
        .filter(|x| x % 3 == 0) // 3で割り切れる要素のみを抽出
        .sum(); // 抽出した要素の合計
    assert_eq!(sum, 18);
}
