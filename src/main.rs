use std::env;
use std::str;

//トークンの種類と値
enum Kind {
    //数値はそのまま出力するだけなのでchar型とする
    Number(Vec<char>),
    Operator(char),
}
//トークン列を保存
struct List {
    root: Option<Box<Token>>,
}
struct Token {
    kind: Kind,
    next: Option<Box<Token>>,
}

impl List {
    //リストの末尾にトークンを追加する
    fn push_back(&mut self, value: Kind) {
        //末尾のトークン(None)を返す
        fn findlast(token: &mut Option<Box<Token>>) -> &mut Option<Box<Token>> {
            if let Some(ref mut b) = *token {
                findlast(&mut b.next)
            } else {
                token
            }
        }
        let token = findlast(&mut self.root);
        *token = Some(Box::new(Token {
            kind: value,
            next: None,
        }));
    }
    // 入力文字列argをトークナイズする
    fn tokenize(&mut self, arg: &mut str::Chars) {
        //連続した数字をベクタ型にまとめて返す
        //その次の数字以外の文字も一緒に返す
        fn continue_num(first_c: char, c_iter: &mut str::Chars) -> (Option<char>, Vec<char>) {
            let mut c_vec = vec![first_c];
            //数字以外の文字が出るまでループ
            while let Some(c) = c_iter.next() {
                if c.is_numeric() {
                    //見つかった数字を追加
                    c_vec.push(c);
                } else {
                    //数字以外が見つかったら終了
                    return (Some(c), c_vec);
                }
            }
            //入力文字列が終わった場合
            (None, c_vec)
        }
        while let Some(mut c) = arg.next() {
            //数字の場合
            if c.is_numeric() {
                //連続した数字をVecにまとめ、その次の記号も出す
                let (c_return, numbers) = continue_num(c, arg);
                //数字のトークンを作成
                self.push_back(Kind::Number(numbers));
                //処理すべき記号を更新
                if let Some(d) = c_return {
                    c = d;
                } else {
                    //前の数字で入力文字列が終わっていたら関数終了
                    return;
                }
            }
            //記号の処理
            match c {
                '+' | '-' => self.push_back(Kind::Operator(c)),
                //空白はスキップ
                c if c.is_whitespace() => (),
                _ => panic!(
                    "不正な文字\"{}\"が存在するため、プログラムを終了します。",
                    c
                ),
            }
        }
    }
}

fn main() {
    let arg_vec: Vec<String> = env::args().collect();
    let mut arg = arg_vec[1].chars();

    let mut list = List { root: None };
    //引数の文字列をトークナイズする
    list.tokenize(&mut arg);
    //着目しているトークン
    let mut token: Token;
    if let Some(first_token) = list.root {
        token = *first_token;
    } else {
        panic!("何も入力がされていないため、プログラムを終了します。");
    }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    //最初が数字であるか確認
    if let Kind::Number(numbers) = token.kind {
        print!("  mov rax, ");
        for number in numbers {
            print!("{}", number);
        }
        //改行を入れる
        print!("\n");
    } else {
        panic!("最初の文字が数字ではありません。プログラムを終了します。");
    }

    loop {
        if let Some(cur_token) = token.next {
            //演算子の処理
            if let Kind::Operator(c) = cur_token.kind {
                match c {
                    '+' => print!("  add rax, "),
                    '-' => print!("  sub rax, "),
                    _ => (), //トークナイズの段階で弾いているはず
                }
            }
            token = *cur_token;
        } else {
            //入力の最後が数字なので正常終了
            break;
        }
        if let Some(cur_token) = token.next {
            //数字の処理
            if let Kind::Number(ref numbers) = cur_token.kind {
                for number in numbers {
                    print!("{}", number);
                }
                //改行を入れる
                print!("\n");
            } else {
                panic!("演算子の後に数字以外が存在しています。プログラムを終了します。");
            }
            token = *cur_token;
        } else {
            panic!("入力の最後に演算子があります。プログラムを終了します。");
        }
    }

    println!("\n  ret");
}
