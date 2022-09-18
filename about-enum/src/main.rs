// enumを利用した構造体
enum IpAddrKind {
    v4,
    v6,
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 各enumの列挙子に直接データを格納して、enumを構造体内に使うというよりもenumだけを使って、
// 同じ概念をもっと簡潔な方法で表現することができます。
enum IpAddrEnum {
    v4(String),
    v6(String),
}

// 構造体よりもenumを使うことには、別の利点もあります: 各列挙子に紐付けるデータの型と量は、異なってもいいのです。
// enum列挙子内にいかなる種類のデータでも格納できることを描き出しています
// 例を挙げれば、文字列、数値型、構造体など、他のenumも含める
enum IpAddrEnumDash {
    v4(u8, u8, u8, u8),
    v6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// enumにもメソッドを定義することができる
impl Message {
    fn call(&self) {
        // method body would be defined here
        // メソッド本体はここに定義される
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::v4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::v6,
        address: String::from("::1"),
    };

    {
        let home = IpAddrEnum::v4(String::from("127.0.0.1"));
        let loopback = IpAddrEnum::v6(String::from("::1"));
    };
    {
        let home = IpAddrEnumDash::v4(127, 0, 0, 1);
        let loopback = IpAddrEnumDash::v6(String::from("::1"));
    };
    {
        let m = Message::Write(String::from("hello"));
        m.call();
    };
    {
        // rustにはnullがない
        // 何らかの理由で現在無効、または存在しない値を表現したい場合
        // rustではOptionのenumで表現することができる
        let some_number = Some(5);
        let some_string = Some("a string");

        let absent_number: Option<i32> = None;

        let x: i8 = 5;
        let y: Option<i8> = Some(5);
        // 型が違うのでコンパイルエラーになる
        // let sum = x + y;
    };

    {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }

        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    };

    {
        // _というパターンは、どんな値にもマッチし
        let some_u8_value = 0u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => (),
        };
    };

    {
        let some_u8_value = Some(6);
        if let Some(num) = some_u8_value {
            println!("three {}", num);
        };
    }
}
