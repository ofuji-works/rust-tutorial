fn main() {
    {
        // ベクタは単体のデータ構造でありながら複数の値を保持でき、それらの値をメモリ上に隣り合わせに並べます。
        // ここで、型注釈を付けていることに注目してください。
        // なぜなら、このベクタに対して何も値を挿入していないので、コンパイラには私たちがどんなデータを保持させるつもりか推測できないからです。
        let v: Vec<i32> = Vec::new();
    }
    {
        // Rustにはvec!という便利なマクロも用意されています
        // このマクロは与えた値を保持する新しいベクタを生成します。
        let v = vec![1, 2, 3];
    }
    {
        // 中に配置する数値は全てi32型であり、Rustはこのことをデータから推論するので、Vec<i32>という注釈は不要です。
        let mut v = Vec::new();
        // ベクタの更新
        v.push(5);
        v.push(6);
    }
    {
        // ベクタの値へのアクセス
        let v = vec![1, 2, 3, 4, 5];
        // 添字でのアクセス
        // 参照を得る
        // 存在しない場合プログラムをパニックさせます。
        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        // get関数でのアクセス
        // Option<&T>を得るもの
        // 存在しない場合Noneを返す
        match v.get(2) {
            Some(thrid) => println!("The third element is {}", third),
            None => println!("There  is no third element"),
        }
    }
    {
        //ベクタ内の値を順に処理する
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
    }
    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            // 参照はずし
            *i += 50;
        }
    }
    {
        // enumを利用して、違う型の値をベクタに持つ
        // ベクタは同じ型の値しか持つことができない
        // enumという型の値を複数持てる
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
    }
}
