use std::env;

mod generate;
mod kind;
mod parse;
mod tokenize;

use crate::generate::generate;
use crate::kind::Node;
use crate::parse::parse;
use crate::tokenize::tokenize;


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
    //トークン列から構文木を生成
    let mut progress = 0;
    let mut nodes: Vec<Node> = Vec::new();
    //文単位で保存
    while progress < tokens.len() {
        let (ret_node, ret_progress) = parse(&tokens, progress);
        nodes.push(ret_node);
        progress = ret_progress;
    }

    // アセンブリの前半部分を出力
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    // プロローグ
    // 変数26個分の領域を確保する
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");

    //構文木からアセンブリコードを生成
    for node in nodes {
        //文単位で生成
        generate(Some(Box::new(node)));
        // 式の評価結果としてスタックに一つの値が残っている
        // はずなので、スタックが溢れないようにポップしておく
        println!("  pop rax");
    }

    // エピローグ
    // 最後の式の結果がRAXに残っているのでそれが返り値になる
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}
