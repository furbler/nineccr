use std::env;
use std::str;

mod kind;
use crate::kind::Kind;
use crate::kind::Node;

mod parse;
use crate::parse::parse;

// 入力文字列argをトークナイズする
fn tokenize(arg: &mut str::Chars) -> Vec<Kind> {
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

//構文木からコードを生成
fn generate(arg_node: Option<Box<Node>>) {
    if let Some(node) = arg_node {
        if let Kind::Num(numbers) = node.kind {
            print!("  push ");
            //数値を出力
            for number in numbers {
                print!("{}", number);
            }
            print!("\n");
            //構文木の末尾のノードなので関数終了
            return;
        } else {
            //ノードが演算子だった場合
            generate(node.lhs);
            generate(node.rhs);

            println!("  pop rdi");
            println!("  pop rax");
            match node.kind {
                Kind::Add => println!("  add rax, rdi"),
                Kind::Sub => println!("  sub rax, rdi"),
                Kind::Mul => println!("  imul rax, rdi"),
                Kind::Div => {
                    println!("  cqo");
                    println!("  idiv rdi");
                }
                Kind::Equal => {
                    println!("  cmp rax, rdi");
                    println!("  sete al");
                    println!("  movzb rax, al");
                }
                Kind::NoEqual => {
                    println!("  cmp rax, rdi");
                    println!("  setne al");
                    println!("  movzb rax, al");
                }
                Kind::LowThan => {
                    println!("  cmp rax, rdi");
                    println!("  setl al");
                    println!("  movzb rax, al");
                }
                Kind::LowEqual => {
                    println!("  cmp rax, rdi");
                    println!("  setle al");
                    println!("  movzb rax, al");
                }
                _ => panic!("不正なノードがあります。プログラムを終了します。"),
            }
            println!("  push rax");
        }
    }
    //引数がNoneの場合は何もしない
}

fn main() {
    //引数を入力文字列として格納
    let arg_vec: Vec<String> = env::args().collect();
    let mut arg = arg_vec[1].chars();

    //引数の文字列をトークナイズする
    let tokens = tokenize(&mut arg);
    //トークン列が空かチェック
    if tokens.len() == 0 {
        panic!("入力がありません。プログラムを終了します。");
    }

    let node = parse(&tokens, 0).0;

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    generate(Some(Box::new(node)));

    //結果の値はスタックの一番上に置かれるので、その値をraxレジスタに置く
    println!("  pop rax");
    println!("  ret");
}
