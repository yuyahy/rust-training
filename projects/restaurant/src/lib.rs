// モジュールの定義
mod front_of_house {
    // モジュールの定義
    pub mod hosting {
        // モジュール内の関数(public)
        pub fn add_to_waitlist() {}
        // fn seat_at_table() {}
    }
}

// eat_at_restaurant()とfront_of_houseは同じモジュール内で定義されている(兄弟)ため、
// front_of_houseをpubnにしなくてもeat_at_restaurant()から呼べる
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I would like {} toast please", meal.toast);
    // Breakfast::seasonal_fruitは非公開なのでコンパイルエラー
    // println!("fruit is {}", meal.seasonal_fruit)

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn serve_order() {}

mod back_of_house {
    // 構造体ではフィールドごとに公開非公開を制御できる(デフォルトは非公開)
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    // enumはそれ自体をpubにすると全ての列挙子が公開される
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        // Breakfast::seasonal_fruitは非公開なので、内部的にその値を設定する関数を用意しておかないと、
        // インスタンスが作れない
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // このfix_incorrect_order()の一階層上にアクセスするためにsuperをつける
        super::serve_order();
    }

    fn cook_order() {}
}

//     mod serving {
//         fn take_order() {}
//         fn serve_order() {}
//         fn take_payment() {}
//     }
// }
