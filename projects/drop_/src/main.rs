struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        // CustomSmartPointerをデータ`{}`とともにドロップするよ
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    }; // 俺のもの

    // 明示的にデストラクタを呼ぶことはできない(エラー)
    // c.drop();
    // drop()を使うと解放できる
    drop(c);

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    }; // 別のもの
    println!("CustomSmartPointers created."); // CustomSmartPointerが生成された
}
