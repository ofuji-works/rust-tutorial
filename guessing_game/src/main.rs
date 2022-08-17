// ライブラリをスコープに入れる
// stdと呼ばれる標準ライブラリの中のio（入出力）ライブラリ
// Rustはデフォルトで、標準ライブラリで定義されているアイテム(prelude（プレリュード）)
// 使いたい型がpreludeにない場合は、その型をuse文で明示的にスコープに入れる必要があります。
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thred_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number); 

    println!("Please input your guess.");

    // Rustでは変数はデフォルトで不変（immutable）になります。
    // 変数を可変（mutable）にするには、変数名の前にmutをつけます。
    // Stringは標準ライブラリによって提供される文字列型で、サイズが拡張可能な、UTF-8でエンコードされたテキスト片になります。
    let mut guess = String::new();

    // もし、プログラムの最初にuse std::ioと書いてioライブラリをインポートしていなかったとしても、std::io::stdinのように呼び出せば、この関数を利用できます。 
    io::stdin()
        // read_lineの引数として&mut guessを渡し、ユーザ入力をどの文字列に格納するかを指示しています。
        // この&は、この引数が参照であることを示し、これによりコードの複数の部分が同じデータにアクセスしても、そのデータを何度もメモリにコピーしなくて済みます。
        // 変数のように参照もデフォルトで不変
        // したがって、&guessではなく&mut guessと書いて可変にする必要があります。
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess); 

    let x = 5;
    let y = 10;
    
    println!("x = {} and y = {}", x, y);
}
