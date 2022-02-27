use std::env;

mod generate;
mod kind;
mod parse;
mod tokenize;

use crate::generate::generate;
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
    let node = parse(&tokens, 0).0;

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    //構文木からアセンブリコードを生成
    generate(Some(Box::new(node)));

    //結果の値はスタックの一番上に置かれるので、その値をraxレジスタに置く
    println!("  pop rax");
    println!("  ret");
}
