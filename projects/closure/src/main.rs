use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    // ゆっくり計算
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    // クロージャ
    // - クロージャの場合は関数と異なり、型注釈は必須ではない
    // - 自身が定義されたスコープ内の変数をキャプチャする事ができる
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    // let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // valueフィールドは内部的に変更されるので、生成時はNone
    fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation, value: None }
    }

    fn value(&mut self, arg: u32) -> u32 {
        // 以下でキャッシュを実現する
        // - 既に値を持っていた場合はその値を返す
        // - 返す値が存在しない場合はTを実行し、その値を返す
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn capture_closure_example() {
    let x = vec![1, 2, 3];
    // キャプチャ方法: moveに指定している点に注意
    let equal_to_x = move |z| z == x;
    // xは既にmoveされているのでコンパイルエラー
    // println!("can't use x here {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));

    let v = vec![1, 2, 4];
    assert!(equal_to_x(v));
}

fn main() {
    // ユーザの選択したトレーニング強度
    let simulated_user_specified_value = 10;
    // 乱数(生成されるトレーニングにバリエーションをもたせるため)
    let simulated_random_number = 7;

    let return_given = |x| {
        println!("given parameter : {}", x);
        x
    };

    let tmp = return_given(7);
    // let tmp = return_given(String::from("Error"));

    generate_workout(simulated_user_specified_value, simulated_random_number);
    capture_closure_example();
}
