参照：https://doc.rust-jp.rs/book-ja/ch01-03-hello-cargo.html

# 基本

cargo new を使ってプロジェクトを作成できる
cargo build を使ってプロジェクトをビルドできる
cargo run を使うとプロジェクトのビルドと実行を 1 ステップで行える
cargo check を使うとバイナリを生成せずにプロジェクトをビルドして、エラーがないか確認できる
Cargo は、ビルドの成果物をコードと同じディレクトリに保存するのではなく、target/debug ディレクトリに格納する

※ Cargo を使用するもう一つの利点は、どの OS で作業していてもコマンドが同じであることです。 そのため、これ以降は Linux や macOS 向けの手順と、Windows 向けの手順を分けて説明することはありません。

# release build

プロジェクトが最終的にリリースできるようになったら、cargo build --release を使い、最適化した状態でコンパイルできます。
このコマンドは実行ファイルを、target/debug ではなく、target/release に作成します。
