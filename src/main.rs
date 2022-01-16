use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let print_num: &str = match args.len() {
        1 => "0",      //引数無し
        _ => &args[1], //最初の引数の値
    };

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", print_num);
    println!("  ret");
}
