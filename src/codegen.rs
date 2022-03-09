use crate::kind::Kind;
use crate::kind::Node;

//構文木からアセンブリコードを生成
pub fn codegen(nodes: Vec<Node>) {
    // アセンブリの前半部分を出力
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    // プロローグ
    // 変数26個分の領域を確保する
    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208"); // 8byte * 26

    for node in nodes {
        //文単位で生成
        gen(Some(Box::new(node)));
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

// 一文の処理
// 引数はnodeのlhsとrhsと揃える
fn gen(node: Option<Box<Node>>) {
    let node = *node.unwrap();
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
        Kind::Return => {
            gen(node.lhs);
            println!("  pop rax");
            println!("  mov rsp, rbp");
            println!("  pop rbp");
            println!("  ret");
            return;
        }

        Kind::Var(ident) => {
            //指定された変数のアドレスをスタックにプッシュする
            push_var_address(ident);
            //変数の中身の値をスタックにプッシュする
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            //構文木の末尾のノードなので関数終了
            return;
        }
        Kind::Assign => {
            if let Kind::Var(ident) = (node.lhs).as_ref().unwrap().kind {
                //指定された変数のアドレスをスタックにプッシュする
                push_var_address(ident);
                //右辺の値を計算
                gen(node.rhs);
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
        //ノードが数値、変数、代入演算子、識別子以外の場合のみ以降の処理に進む
        _ => (),
    }
    //ノードが演算子だった場合
    gen(node.lhs);
    gen(node.rhs);

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

//指定された変数のアドレスをスタックにプッシュする
fn push_var_address(ident: usize) {
    println!("  mov rax, rbp");
    //オフセット値には変数のサイズ(8byte)を考慮する
    println!("  sub rax, {}", ident * 8);
    println!("  push rax");
}
