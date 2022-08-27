fn main() {
    let x = 5;
    println!("The value of x is {}", x);

    let x = x + 1;

    // ブロック式
    // 新しいスコープを作る
    // 返り値は最後の行の;がついていないもの
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
        x
    }

    println!("The value of x is: {}", x);

    let _spaces = "   ";
    let _spaces = _spaces.len();

    let _guess: u32 = "42".parse().expect("Not a number!");


    // rust 文と式の概念
    // 文とは、なんらかの動作をして値を返さない命令です。
    // 式は結果値に評価されます。
    // 文は値を返しません → 変数に入れられない

    // ブロック式では;がついた場合、式ではなくなり文になる。()が返り値になる。
    let y = {
        let x = 3;
        x + 1
    };

}

// 関数ではreturnで返り値を指定できるが、
// セミコロンをつけずに一番最後に書かれた式を暗黙的に返却する
fn five() -> i32 {
    5
}

