// Vec<T>などと異なり、HashMapを使いたければ自分でuseを書く必要がある
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    let elem1_key = String::from("Blue");
    let elem2_val = 10;

    scores.insert(elem1_key, elem2_val);
    scores.insert(String::from("Yellow"), 50);

    create_hash_map_by_vector();

    // hash_mapへinsertした変数はhash_mapにムーブされるので、以降の呼び出しはエラー
    // println!("elem1: ({}, {})", elem1_key, elem2_val);

    access_hashmap_element();
    update_hashmap_value();
}

fn create_hash_map_by_vector() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // zip()でタプルのベクタを作る→collect()でhashmapに変換
    // collectはHashMap<_, _>から型推論を行い、hashmapを作成する
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}

fn access_hashmap_element() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Blue score : {:#?}", score);

    // forで全要素を処理可能
    for (key, value) in &scores {
        println!("key : {}, value : {}", key, value);
    }
}

fn update_hashmap_value() {
    let mut scores = HashMap::new();

    // 同じkeyに対して複数回valueをinsertすると、
    // 最後にinsertしたvalueが残る
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:#?}", scores);

    // entry()を使って、あるkeyに対するvalueが存在しない場合のみ値を挿入する
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(0);

    println!("{:#?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}
