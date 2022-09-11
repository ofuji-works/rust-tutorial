fn main() {
    println!("Hello, world!");
    {
        let mut s = String::from("hello world");
        let word = first_word(&s); // word will get the value 5
                                   // wordの中身は、値5になる

        s.clear(); // this empties the String, making it equal to ""
                   // Stringを空にする。つまり、""と等しくする
        // word still has the value 5 here, but there's no more string that
        // we could meaningfully use the value 5 with. word is now totally invalid!
        // wordはまだ値5を保持しているが、もうこの値を正しい意味で使用できる文字列は存在しない。
        // wordは今や完全に無効なのだ！
    }

    {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];
    }

    {
        let s = String::from("hello world");

        // 一部の参照
        let hello = &s[0..5];
        let world = &s[6..11];
    }

    {

        let s = String::from("hello");

        // 下記は同義　
        // let slice = &s[0..2];
        // let slice = &s[..2];

        // 下記も同義
        // Stringの最後のバイトをスライスが含む
        // let len = s.len();
        // let slice = &s[3..len];
        // let slice = &s[3..];
    }
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}