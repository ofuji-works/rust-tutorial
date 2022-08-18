// ライブラリをスコープに入れる
// stdと呼ばれる標準ライブラリの中のio（入出力）ライブラリ
// Rustはデフォルトで、標準ライブラリで定義されているアイテム(prelude（プレリュード）)
// 使いたい型がpreludeにない場合は、その型をuse文で明示的にスコープに入れる必要があります。
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    loop {
        println!("Guess the number!");

        let secret_number = rand::thread_rng().gen_range(1..101);
    
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
    
        // Rustではguessの前の値を新しい値で覆い隠す（shadowする）ことが許されているのです。 
        // シャドーイング（shadowing）は、guess_strとguessのような重複しない変数を二つ作る代わりに、guessという変数名を再利用させてくれるのです。
        // 今のところ、この機能はある型から別の型に値を変換するときによく使われることを知っておいてください。
        let guess: u32 = match guess
            // ユーザは予想を入力したあとread_lineの処理を終えるためにEnterキーを押す必要がありますが、これにより文字列に改行文字が追加されます。 
            // trimメソッドは\nや\r\nを削除する
            .trim()
            //文字列のparseメソッドは文字列をパース（解析）して何らかの数値にします
            .parse()
            {
                Ok(num) => num,
                // アンダースコアの_はすべての値を受け付けます。
                Err(_) => continue,
            };
    
        println!("You guessed: {}", guess); 
    
        let x = 5;
        let y = 10;
        
        println!("x = {} and y = {}", x, y);
    
        // match式は複数のアーム（腕）で構成されます。 各アームはマッチさせるパターンと、matchに与えられた値がそのアームのパターンにマッチしたときに実行されるコードで構成されます。
        // guessとsecret_numberの値に対してcmpを呼んだ結果返されたOrderingの列挙子に基づき、次の動作を決定しています。
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),       //小さすぎ！
            Ordering::Greater => println!("Too big!"),      //大きすぎ！
            Ordering::Equal => {
                println!("You win!"); // やったね！
                break;
            }
        };
    };
}
