// 構造体
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// println!で構造体を出力できるようにする
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Rectangleの文脈内で関数を定義するには、impl(implementation; 実装)ブロックを始めます。
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// 複数のimplブロックを存在させることができます
impl Rectangle {
    // 関連関数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
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
    println!("user2 {:#?}", user2);

    let user3 = build_user(String::from("test3@example.com"), String::from("test3"));
    println!("user3 {:#?}", user3);

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

    // 構造体を使ってリファクタリング
    let rect1 = Rectangle {
        width: 32,
        height: 32,
    };
    println!("height * width {}", area(&rect1));

    // デバッグで構造体を出力する
    println!("rect1 {:#?}", rect1);

    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );

        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

        let rect4 = Rectangle::square(30);
        println!(
            "The area of the rectangle is {} square pixels.",
            rect4.area()
        );
    }
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

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
