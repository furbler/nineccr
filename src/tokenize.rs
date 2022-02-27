use std::str;

use crate::kind::Kind;

// 入力文字列からトークン列を生成
pub fn tokenize(arg: &mut str::Chars) -> Vec<Kind> {
    //トークン列
    let mut tokens = Vec::new();
    //イテレータで取り出されて未処理の文字
    let mut popped_char: Option<char> = None;
    while let Some(c) = {
        //popped_charに値があればその値を、無ければイテレータから値を取り出す
        if let None = popped_char {
            arg.next()
        } else {
            popped_char
        }
    } {
        //値をリセット
        popped_char = None;
        //記号の処理
        match c {
            '=' => {
                if let Some(next_c) = arg.next() {
                    if '=' == next_c {
                        tokens.push(Kind::Equal);
                    } else {
                        panic!("=単体の演算子は不正です。プログラムを終了します。");
                    }
                }
            }
            '!' => {
                if let Some(next_c) = arg.next() {
                    if '=' == next_c {
                        tokens.push(Kind::NoEqual);
                    } else {
                        panic!("!単体の演算子は不正です。プログラムを終了します。");
                    }
                }
            }
            '<' => {
                if let Some(next_c) = arg.next() {
                    match next_c {
                        // <=
                        '=' => tokens.push(Kind::LowEqual),
                        d if d.is_numeric() => {
                            // <
                            tokens.push(Kind::LowThan);

                            //連続した数字をVecにまとめ、数字のトークンを追加
                            let (ret_char, ret_numbers) = continue_num(d, arg);
                            tokens.push(Kind::Num(ret_numbers));
                            popped_char = ret_char;
                        }
                        _ => {
                            // <
                            tokens.push(Kind::LowThan);
                            tokens = push_token(next_c, tokens)
                        }
                    }
                }
            }
            '>' => {
                if let Some(next_c) = arg.next() {
                    match next_c {
                        // >=
                        '=' => tokens.push(Kind::HighEqual),
                        d if d.is_numeric() => {
                            // >
                            tokens.push(Kind::HighThan);
                            //連続した数字をVecにまとめ、数字のトークンを追加
                            let (ret_char, ret_numbers) = continue_num(d, arg);
                            tokens.push(Kind::Num(ret_numbers));
                            popped_char = ret_char;
                        }
                        _ => {
                            // >
                            tokens.push(Kind::HighThan);
                            tokens = push_token(next_c, tokens)
                        }
                    }
                }
            }
            //数字の場合
            d if d.is_numeric() => {
                //連続した数字をVecにまとめ、数字のトークンを追加
                let (ret_char, ret_numbers) = continue_num(d, arg);
                tokens.push(Kind::Num(ret_numbers));
                popped_char = ret_char;
            }
            _ => tokens = push_token(c, tokens),
        }
    }
    tokens
}

//連続した数字をベクタ型にまとめて返す
fn continue_num(first_c: char, c_iter: &mut str::Chars) -> (Option<char>, Vec<char>) {
    let mut c_vec = vec![first_c];
    let mut ret_char: Option<char> = None;
    //数字以外の文字が出るまでループ
    while let Some(c) = c_iter.next() {
        if c.is_numeric() {
            //見つかった数字を追加
            c_vec.push(c);
        } else {
            //数字以外が見つかったら終了
            ret_char = Some(c);
            break;
        }
    }
    (ret_char, c_vec)
}

//記号に応じたトークンをトークン列に追加
fn push_token(c: char, mut tokens: Vec<Kind>) -> Vec<Kind> {
    match c {
        '+' => tokens.push(Kind::Add),
        '-' => tokens.push(Kind::Sub),
        '*' => tokens.push(Kind::Mul),
        '/' => tokens.push(Kind::Div),
        '(' => tokens.push(Kind::BracOpen),
        ')' => tokens.push(Kind::BracClose),
        //空白はスキップ
        ' ' => (),
        _ => panic!(
            "不正な文字\"{}\"が存在するため、プログラムを終了します。",
            c
        ),
    }
    tokens
}
