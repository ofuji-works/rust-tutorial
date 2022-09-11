// 所有権

fn main() {
    {
        // 変数sはここの行までは有効ではない
        let s = "hello"; // sはここから有効になる
        // 変数sが有効
        println!("{}, world!", s);
    } // スコープが終わり、sがdropされる

    {
        // Rustでは基本的にスタックにローカル変数の値を持つが、
        // コンパイル時にはサイズが不明なものについては、ヒープにメモリを確保する
        let s = String::from("world!");
        println!("Hello, {}", s);
    }

    {        
        let s1 = String::from("hello");
        let s2 = s1; // s1の所有権がs2にmoveしている
        // let s2 = s1.clone(); // cloneであれば値のdeep copyをすることができる
        // println!("Hello, {}", s1); // s1がここ
        println!("Hello, {}", s2);

        let x = 5;
        let y = x; // Copyトレイトの場合、ムーブ後も利用ができる。
        // コンパイル時に
        println!("{}, {}", x, y);
    }

    {
        let s1 = String::from("hello"); // s1が有効になる
        let (s2, len) = calculate_length(s1); // s1の所有権がムーブする
        // ↓以降ではs1は既にムーブしているため無効
        println!("The length of '{}' is {}.", s2, len);
    }
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()メソッドは、Stringの長さを返します
    (s, length)
}