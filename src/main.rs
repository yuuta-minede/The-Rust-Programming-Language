use rand::Rng; //乱数生成  Rngトレイトを使う
use std::cmp::Ordering; // 数字の比較するライブラリ
use std::io; // std: 標準ライブラリ、  I/Oライブラリ（キーボードの入力を読み取る）

fn main() {
    println!("Guess the number!");

    // thread_rng : スレッド固有の乱数生成器
    let secret_number: u32 = rand::thread_rng().gen_range(1..101); // (1..=100)と同義
    println!("The secret number is {}", secret_number);

    println!("Please input your guess.");

    // 新しい空のStringを返す関連関数
    let mut guess = String::new(); // デフォルトでimmutableなのでmutableにする

    // useがなければ srd::io::stdin でも呼び出せる
    // & : 引数 guess は参照したものである。データをメモリにコピーせずに済む。
    io::stdin().read_line(&mut guess).expect("Fail"); // expectでエラーハンドリング

    // trim: 文字列の先頭と末尾の空白を削除（エンターキーを入力したときの\nを削除できる）
    // parse: 文字列を数値にする
    let guess: u32 = guess.trim().parse().expect("please type a number");

    // プレースホルダー {} にデータを入れる
    println!("This is {}", guess);

    // match: 値を比較して、マッチしたパターンに応じてコードを実行する
    match guess.cmp(&secret_number) {
        // cmp: 比較メソッド。guess と secret_numberを比較している
        Ordering::Less => println!("too small"), // 各アーム
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("you win"),
    }
}
