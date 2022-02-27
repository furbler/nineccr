use crate::kind::Kind;
use crate::kind::Node;

//構文木からアセンブリコードを生成
pub fn generate(arg_node: Option<Box<Node>>) {
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
