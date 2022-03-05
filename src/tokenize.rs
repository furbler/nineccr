use crate::kind::Kind;
use std::collections::HashMap;

use std::str;

struct IdentList {
    //変数の識別番号
    list: HashMap<String, usize>,
}

impl IdentList {
    //引数の変数名がリストにあればその識別番号を、無ければリストに追加して識別番号を返す
    fn find_ident_index(&mut self, var_name: String) -> usize {
        if let Some(index) = self.list.get(&var_name) {
            *index
        } else {
            //最初が1で、追加ごとに識別番号を1ずつ増やす
            let index = self.list.len();
            self.list.insert(var_name, index);
            index
        }
    }
}

// 入力文字列からトークン列を生成
pub fn tokenize(arg: &mut str::Chars) -> Vec<Kind> {
    let mut ident_list = IdentList {
        list: HashMap::new(),
    };

    //トークン列
    let mut tokens = Vec::new();
    //イテレータで取り出されて未処理の文字
    let mut popped_char: Option<char> = None;
    while let Some(c) = {
        if let Some(_) = popped_char {
            //popped_charに値があればその値を使う
            popped_char
        } else {
            //無ければイテレータから値を取り出す
            arg.next()
        }
    } {
        //値をリセット
        popped_char = None;
        //記号の処理
        match c {
            '=' => {
                if let Some(next_c) = arg.next() {
                    match next_c {
                        '=' => tokens.push(Kind::Equal),
                        alpha if alpha.is_numeric() => {
                            // =
                            tokens.push(Kind::Assign);

                            //連続した数字をVecにまとめ、数字のトークンを追加
                            let (ret_char, ret_numbers) = continue_num(alpha, arg);
                            tokens.push(Kind::Num(ret_numbers));
                            popped_char = ret_char;
                        }

                        _ => {
                            // =
                            tokens.push(Kind::Assign);
                            popped_char = Some(next_c);
                        }
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
                        alpha if alpha.is_numeric() => {
                            // <
                            tokens.push(Kind::LowThan);

                            //連続した数字をVecにまとめ、数字のトークンを追加
                            let (ret_char, ret_numbers) = continue_num(alpha, arg);
                            tokens.push(Kind::Num(ret_numbers));
                            popped_char = ret_char;
                        }
                        _ => {
                            // <
                            tokens.push(Kind::LowThan);
                            popped_char = Some(next_c);
                        }
                    }
                }
            }
            '>' => {
                if let Some(next_c) = arg.next() {
                    match next_c {
                        // >=
                        '=' => tokens.push(Kind::HighEqual),
                        alpha if alpha.is_numeric() => {
                            // >
                            tokens.push(Kind::HighThan);
                            //連続した数字をVecにまとめ、数字のトークンを追加
                            let (ret_char, ret_numbers) = continue_num(alpha, arg);
                            tokens.push(Kind::Num(ret_numbers));
                            popped_char = ret_char;
                        }
                        _ => {
                            // >
                            tokens.push(Kind::HighThan);
                            popped_char = Some(next_c);
                        }
                    }
                }
            }
            //変数の場合
            bravo if is_ident_char(bravo) => {
                //連続した変数に使える文字列を取得
                let (ret_char, ret_ident) = continue_ident(bravo, arg);
                //変数名に対応した識別番号をトークンに登録
                tokens.push(Kind::Var(ident_list.find_ident_index(ret_ident)));
                popped_char = ret_char;
            }

            //数字の場合
            alpha if alpha.is_numeric() => {
                //連続した数字をVecにまとめ、数字のトークンを追加
                let (ret_char, ret_numbers) = continue_num(alpha, arg);
                tokens.push(Kind::Num(ret_numbers));
                popped_char = ret_char;
            }
            _ => tokens = push_token(c, tokens),
        }
    }
    tokens
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
        ';' => tokens.push(Kind::Semicolon),
        //空白はスキップ
        ' ' => (),
        _ => panic!(
            "不正な文字\"{}\"が存在するため、プログラムを終了します。",
            c
        ),
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

//連続した変数に使える文字を文字列にして返す
fn continue_ident(first_c: char, c_iter: &mut str::Chars) -> (Option<char>, String) {
    let mut c_vec = vec![first_c];
    let mut ret_char: Option<char> = None;
    //変数に使える文字または数字以外が出るまでループ
    while let Some(c) = c_iter.next() {
        if is_ident_char(c) || c.is_numeric() {
            //見つかった文字を追加
            c_vec.push(c);
        } else {
            //変数に使えない文字が見つかったら終了
            ret_char = Some(c);
            break;
        }
    }
    (ret_char, c_vec.into_iter().collect())
}

//変数の最初に使える文字なら真を返す
fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
