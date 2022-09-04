fn main() {
    let mut count = 0;
    // ループ式にラベルを設定できる
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                // 最も内側のループをbreak
                break;
            }
            if count == 2 {
                // ラベルを指定してbreak
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    // 発射！
    println!("LIFTOFF!!!");


    // forループを使ってコレクションの各アイテムに対してコードを実行することができます。
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    // revを使って範囲を逆順にしたカウントダウン
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
