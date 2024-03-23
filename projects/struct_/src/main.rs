struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Note: フィールドごとに可変か不変かを変える事はできない
    //       インスタンスの可変性がフィールドの可変性になる
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("username : {}", user1.username);
    user1.username = String::from("aaa");
    println!("username : {}", user1.username);

    let user2 = create_new_user_from_another(user1);
    println!("username : {}", user2.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    // フィールドとセットする変数名が同じ場合は、その名前だけ書けばOK
        username, // フィールドとセットする変数名が同じ場合は、その名前だけ書けばOK
        active: true,
        sign_in_count: 1,
    }
}

fn create_new_user_from_another(user: User) -> User {
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user // その他のフィールドは引数のuserと同じ値にする
    };
    user2
}

// 以下ほ参照を用いて、引数のムーブを回避し、新たなインスタンスを返すver
// fn create_new_user_from_another(user: &User) -> User {
//     let user2 = User {
//         email: String::from("another@example.com"),
//         username: String::from("anotherusername567"),
//         ..*user // その他のフィールドは引数のuserと同じ値にする(アスタリスクで参照外し)
//     };
//     user2
// }

fn create_tuple_struct() {
    // タプル構造体
    // タブルには名前が付いているが、各フィールドには名前がない
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
