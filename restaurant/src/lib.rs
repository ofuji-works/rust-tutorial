// mod front_of_houseの後にブロックではなくセミコロンを使うと、
// Rustにモジュールの中身をモジュールと同じ名前をした別のファイルから読み込むように命令します。
mod front_of_house;

pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
