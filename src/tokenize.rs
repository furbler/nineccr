use crate::kind::Kind;
use std::collections::HashMap;

use std::str;

//出現した変数一覧を保存するハッシュマップのリスト
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
    // 出現した変数名とその識別番号のハッシュマップ
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
            // キーワードまたは変数の場合
            // 先頭が数字の場合は除く
            bravo if is_ident_char(bravo) => {
                //トークンを生成
                let (ret_char, ret_token) = ident_token(bravo, arg, &mut ident_list);

                //変数名に対応した識別番号をトークンに登録
                tokens.push(ret_token);
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
        '(' => tokens.push(Kind::RoundBracOpen),
        ')' => tokens.push(Kind::RoundBracClose),
        '{' => tokens.push(Kind::CurlyBracOpen),
        '}' => tokens.push(Kind::CurlyBracClose),
        ';' => tokens.push(Kind::Semicolon),
        //空白と改行はスキップ（トークンを分ける区切り文字とする）
        ' ' => (),
        '\n' => (),
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

//変数を表す文字列をベクタ型に追加して返す
fn continue_var(mut c_vec: Vec<char>, c_iter: &mut str::Chars) -> (Option<char>, Vec<char>) {
    let mut ret_char: Option<char> = None;
    //変数に使えない文字が出るまでループ
    while let Some(c) = c_iter.next() {
        if is_ident_char(c) || c.is_numeric() {
            //見つかった文字を追加
            c_vec.push(c);
        } else {
            ret_char = Some(c);
            break;
        }
    }
    (ret_char, c_vec)
}

// キーワードか変数が判断して、トークンを生成して返す
fn ident_token(
    first_c: char,
    c_iter: &mut str::Chars,
    ident_list: &mut IdentList,
) -> (Option<char>, Kind) {
    let mut popped_char;
    let c_vec;
    // 変数でない識別子か否か
    let is_ident;

    match first_c {
        'r' => {
            (is_ident, (popped_char, c_vec)) = check_ident("return", c_iter);
            if is_ident {
                // returnキーワード
                return (popped_char, Kind::Return);
            }
        }
        'i' => {
            (is_ident, (popped_char, c_vec)) = check_ident("if", c_iter);
            if is_ident {
                // ifキーワード
                return (popped_char, Kind::If(None));
            }
        }
        'e' => {
            (is_ident, (popped_char, c_vec)) = check_ident("else", c_iter);
            if is_ident {
                // elseキーワード
                return (popped_char, Kind::Else);
            }
        }
        'w' => {
            (is_ident, (popped_char, c_vec)) = check_ident("while", c_iter);
            if is_ident {
                // whileキーワード
                return (popped_char, Kind::While(None));
            }
        }
        'f' => {
            (is_ident, (popped_char, c_vec)) = check_ident("for", c_iter);
            if is_ident {
                // whileキーワード
                return (popped_char, Kind::For(None, None, None));
            }
        }
        _ => {
            // 通常の変数
            (popped_char, c_vec) = continue_var(vec![first_c], c_iter);
        }
    }
    // 変数の文字Vec<char>をStringに変換
    let ident = c_vec.into_iter().collect();

    // 変数名の後に"("があれば関数名
    if let Some(c) = skip_nullity(popped_char, c_iter) {
        if c == '(' {
            // 関数名
            return (Some(c), Kind::FunCall(ident));
        } else {
            // 変数名
            popped_char = Some(c);
        }
    }

    (popped_char, Kind::Var(ident_list.find_ident_index(ident)))
}

// 空白や改行など区切りに使われる無視すべき文字を飛ばす
fn skip_nullity(mut popped_char: Option<char>, c_iter: &mut str::Chars) -> Option<char> {
    while let Some(c) = popped_char {
        match c {
            ' ' => (),
            '\n' => (),
            _ => return Some(c),
        }
        popped_char = c_iter.next();
    }
    None
}

//変数の最初に使える文字なら真を返す
fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
//識別子であれば真を返す
//異なっていたらイテレータで消費した文字をVecにまとめて返す
// 一文字目は変数に使える文字とする
fn check_ident(compare_str: &str, c_iter: &mut str::Chars) -> (bool, (Option<char>, Vec<char>)) {
    //比較対象の文字列の1文字目
    let mut c_vec = vec![compare_str.chars().nth(0).unwrap()];
    //イテレータで取り出して未処理の一文字
    let mut popped_char = None;
    //比較中の文字の位置(2文字目からスタート)
    let mut cnt = 1;
    //イテレータから1文字取り出す
    while let Some(c) = c_iter.next() {
        popped_char = Some(c);
        //比較文字列から1文字取り出す
        if let Some(compare_char) = compare_str.chars().nth(cnt) {
            if compare_char == c {
                c_vec.push(c);
                cnt += 1;
            } else {
                //通常の変数
                if is_ident_char(c) || c.is_numeric() {
                    c_vec.push(c);
                    return (false, continue_var(c_vec, c_iter));
                } else {
                    return (false, (popped_char, c_vec));
                }
            }
        } else {
            // 比較文字列が終了した場合
            if is_ident_char(c) || c.is_numeric() {
                //識別子の文字列の後に変数に使える文字が続いていた場合は通常の変数とみなす
                c_vec.push(c);
                return (false, continue_var(c_vec, c_iter));
            } else {
                //識別子の文字列の後に変数以外の文字があれば識別子であると判定する
                //この場合、返り値の文字のベクタは利用されない(比較文字列と同じなので)
                return (true, (Some(c), c_vec));
            }
        }
    }
    //比較文字列の途中で先にイテレータが無くなった場合
    (false, (popped_char, c_vec))
}
