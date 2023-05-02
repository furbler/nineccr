#![warn(clippy::all, clippy::pedantic)]
use std::env;

mod codegen;
mod kind;
mod parse;
mod tokenize;

use crate::codegen::codegen;
use crate::parse::program;
use crate::tokenize::tokenize;

fn main() {
    //引数を入力文字列として格納
    let arg_vec: Vec<String> = env::args().collect();
    let mut arg = arg_vec[1].chars();

    //引数の文字列をトークナイズする
    let tokens = tokenize(&mut arg);
    //トークン列が空(入力が空)ならばエラー
    if tokens.is_empty() {
        panic!("入力がありません。プログラムを終了します。");
    }
    // トークン列から構文木を生成
    let nodes = program(&tokens);

    //構文木からアセンブリコードを出力
    codegen(nodes);
}
