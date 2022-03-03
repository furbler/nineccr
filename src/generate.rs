use crate::kind::Kind;
use crate::kind::Node;

//構文木からアセンブリコードを生成
pub fn generate(arg_node: Option<Box<Node>>) {
    if let Some(node) = arg_node {
        match node.kind {
            Kind::Num(numbers) => {
                print!("  push ");
                //数値を出力
                for number in numbers {
                    print!("{}", number);
                }
                print!("\n");
                //構文木の末尾のノードなので関数終了
                return;
            }
            Kind::Var(offset) => {
                //指定された変数のアドレスをスタックにプッシュする
                push_var_address(offset);
                //変数の中身の値をスタックにプッシュする
                println!("  pop rax");
                println!("  mov rax, [rax]");
                println!("  push rax");
                //構文木の末尾のノードなので関数終了
                return;
            }
            Kind::Assign => {
                if let Kind::Var(offset) = (node.lhs).as_ref().unwrap().kind {
                    //指定された変数のアドレスをスタックにプッシュする
                    push_var_address(offset);
                    //右辺の値を計算
                    generate(node.rhs);
                    //変数に右辺の値を代入
                    println!("  pop rdi");
                    println!("  pop rax");
                    println!("  mov [rax], rdi");
                    println!("  push rdi");
                    //代入式が終わったので関数終了
                    return;
                } else {
                    panic!("式の左辺に変数以外があります。プログラムを終了します。");
                }
            }
            //ノードが数値、変数、代入演算子以外の場合のみ以降の処理に進む
            _ => (),
        }
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
    //引数がNoneの場合は何もしない
}

//指定された変数のアドレスをスタックにプッシュする
fn push_var_address(offset: i32) {
    println!("  mov rax, rbp");
    println!("  sub rax, {}", offset);
    println!("  push rax");
}
