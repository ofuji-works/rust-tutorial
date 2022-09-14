// 構造体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // 構造体のインスタンスの生成
    let mut user1 = User {
        username: String::from("test"),
        email: String::from("test@example.com"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("test2@example.com");

    // 構造体更新記法
    // 前のインスタンスの値を使用しつつ、変更する箇所もある形で新しいインスタンスを生成
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // タプル構造体
    // フィールドに紐づけられた名前がなく、むしろフィールドの型だけ
    // タプル構造体のインスタンスは、 タプルと同じように振る舞います
    // 分配して個々の部品にしたり、.と添え字を使用して個々の値にアクセスするなどです。
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // blackとoriginの値は、違う型である
    // 異なるタプル構造体のインスタンスだから
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // ユニット様構造体
    // ある型にトレイトを実装するけれども、 型自体に保持させるデータは一切ない場面に有効になります
}

fn build_user(email: String, username: String) -> User {
    User {
        // 省略で書ける email: email → email
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
