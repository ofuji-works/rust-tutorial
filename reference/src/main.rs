// 参照と借用

fn main() {
    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1); // 参照で変数を渡す
    
        println!("The length of '{}' is {}.", s1, len);
    }

    {
        let s = String::from("hello");
        change(&s);
    }

    {
        // 変更可能な変数
        let mut s = String::from("hello");
        changeable(&mut s); // 変更可能な参照
        // この制約がある利点は、コンパイラがコンパイル時にデータ競合を防ぐことができる点です。 
        // データ競合とは、競合条件と類似していて、これら3つの振る舞いが起きる時に発生します:
        // ・2つ以上のポインタが同じデータに同時にアクセスする。
        // ・少なくとも一つのポインタがデータに書き込みを行っている。
        // ・データへのアクセスを同期する機構が使用されていない。
        // データ競合は未定義の振る舞いを引き起こし、実行時に追いかけようとした時に特定し解決するのが難しい問題です。 
        // しかし、Rustは、データ競合が起こるコードをコンパイルさえしないので、この問題が発生しないようにしてくれるわけです。
    }

    {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            println!("{}", r1);
        } // r1はここでスコープを抜けるので、問題なく新しい参照を作ることができる
        
        let r2 = &mut s;
        println!("{}", r2);
    }

    {
        let mut s = String::from("hello");

        let r1 = &s; // 問題なし
        println!("{}", r1);
        let r2 = &s; // 問題なし
        println!("{}", r2);
        let r3 = &mut s; // 大問題！
        println!("{}", r3);
    }

    {
        let reference_to_nothing = dangle();
    }
}

fn calculate_length(s: &String) -> usize { // sはStringの参照
    s.len() // ここで、sはスコープ外になる。けど、参照しているものの所有権を持っているわけではないので
    // 何も起こらない
}
fn change(some_string: &String) {
    // some_string.push_str(", world"); // ここでの参照元の変更はできない
    println!("{}.", some_string);
}
fn changeable(some_string: &mut String) {
    some_string.push_str(", world");
}
// ダングリングポインタ
// ダングリングポインタとは、 他人に渡されてしまった可能性のあるメモリを指すポインタのことであり、
// その箇所へのポインタを保持している間に、メモリを解放してしまうことで発生します。
fn dangle() -> &String { // dangleはStringへの参照を返す
    let s = String::from("hello"); // sは新しいString
    // &s // String sへの参照を返す
    s // 参照ではないsをreturnするようにする
} // ここで、sはスコープを抜け、ドロップされる。そのメモリは消される。
  // 危険だ