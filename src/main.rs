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
    looping();
    range();
    // fibonacci(secret_number);
    ownership();
}

fn looping() {
    let mut count: i32 = 0;
    'counting_up: loop {
        //'counting_up: はなくてもOKのラベル
        println!("count = {}", count);
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}

fn range() {
    for num in (1..4).rev() {
        println!("{}", num)
    }
    println!("finish!")
}

fn fibonacci(x: u32) -> u32 {
    match x {
        0 => 0, // matchした時に fat arrow を使って戻り値を設定
        1 => 1,
        2..=10 => 10,
        _ => {
            let mut x: u32 = fibonacci(x - 1) + fibonacci(x - 2);
            println!("x is {}", x);
            x
        } // _ はその他
    }
}

fn ownership() {
    // 文字列リテラルはコンパイル時に中身が判明していてサイズが分かっている
    let mut s = "hello";
    println!("s is {}", s);

    // String型は不変かつ伸張可能なので、コンパイル時に不明な量のメモリをヒープ領域に確保する
    // heap領域に確保されると、メモリは実行時にOSに要求されて、使用し終わったらOSに変換される
    let mut t1 = String::from("hello");
    t1.push_str(", world"); // 「hello」に「, world」を追加
    println!("{}", t1);

    // スタックにあるポインタ、長さ、許容量がコピーされる
    // ポインタが指すヒープ上のデータ自体をコピーするわけではない
    let t2 = t1;
    println!("t2 is {}", t2);
    // println!("{}", t1); // 二重開放エラーになる

    let u1 = String::from("hey");
    let u2 = u1.clone(); // deep copy すれば二重開放エラーにならない
    println!("u1 = {}, u2 = {}", u1, u2);

    let v1 = 5;
    let v2 = v1; // deep copy
    println!("v1 = {}, v2 = {}", v1, v2); // 整数ならコンパイル時に値が決まっているものはスタックに保存されるのでエラーにならない

    let len = calculate_length(&t2); // & で借用することでt2の所有権はムーブされない
    println!("The length of {} is {}", t2, len);

    // let w = String::from("hoge");
    // change(&w); // 参照しているものは変更はできない
    let mut y = String::from("hoge");
    change(&mut y);
} // 最後に変数のスコープを外すようdrop関数を呼び出している

fn calculate_length(s: &String) -> usize {
    // 関数の引数で借用 ＝ 借用。所有権なし
    s.len()
}

fn change(word: &mut String) {
    word.push_str(", world"); // 参照しているものは変更はできないので引数をmutableにしている
}
