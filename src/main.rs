use std::io;

fn main() {
    let mut args = String::new();
    io::stdin()
        .read_line(&mut args)
        .expect("引数の読み込み失敗しました。");

    let mut line = args.chars();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    print!("  mov rax, ");

    if let Some(c) = line.next() {
        if c.is_numeric() {
            print!("{}", c);
        } else {
            panic!("最初に数字以外の文字があります。プログラムを終了します。");
        }
    }

    while let Some(c) = line.next() {
        if ignore_char(c) {
            continue;
        }
        //数字はそのまま出力
        if c.is_numeric() {
            print!("{}", c);
            continue;
        }

        //演算子の場合
        if translate_operator(c) {
            if let Some(d) = line.next() {
                let mut skip_check = d;
                //無視すべき文字をスキップ
                while ignore_char(skip_check) {
                    if let Some(e) = line.next() {
                        skip_check = e;
                    }
                }
                if skip_check.is_numeric() {
                    print!("{}", skip_check);
                    continue;
                } else {
                    panic!("演算子の後に数字がありません。プログラムを終了します。")
                };
            } else {
                panic!("演算子が最後になっています。プログラムを終了します。")
            }
        } else {
            //数字でも演算子でもない
            panic!("不正な文字が入力されているため、プログラムを終了します。");
        }
    }
}

//無視すべき文字かを判定
fn ignore_char(c: char) -> bool {
    //スペースとタブと改行はすべて無視
    //引数には末尾の改行も含むため
    c == ' ' || c == '\t' || c == '\n'
}
//演算子の判定と表示
fn translate_operator(c: char) -> bool {
    match c {
        '+' => {
            print!("\n  add rax, ");
            true
        }
        '-' => {
            print!("\n  sub rax, ");
            true
        }
        _ => false,
    }
}
