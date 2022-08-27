fn main() {
    let number = 7;

    // Rustでは、論理値以外の値が、自動的に論理値に変換されることはありません
    // 明示し、必ずifには条件式として、論理値を与えなければなりません。
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //  例えば、数値が0以外の時だけifのコードを走らせたいなら、以下のようにif式を変更することができます:
    if number != 0 {
        println!("number was something other than zero");   // 数値は0以外の何かです
    }

    let condition = true;
    // ifは式なので、let文の右辺に持ってくることができます。
    let number = if condition { 5 } else { 6 };

    if number % 4 == 0 {
        // 数値は4で割り切れます
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // 数値は3で割り切れます
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // 数値は2で割り切れます
        println!("number is divisible by 2");
    } else {
        // 数値は4、3、2で割り切れません
        println!("number is not divisible by 4, 3, or 2");
    }
}
