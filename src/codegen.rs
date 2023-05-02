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

    // ラベルに一意に付与する番号
    let mut labelseq: usize = 0;

    for node in nodes {
        //文単位で生成
        labelseq = gen(Some(Box::new(node)), labelseq);
        // 式の評価結果としてスタックに一つの値が残っている
        // はずなので、スタックが溢れないようにポップしておく
        println!("  pop rax");
    }

    // エピローグ
    // 最後の式の結果がRAXに残っているのでそれが返り値になる
    println!(".Lreturn:");
    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}

// 文の処理
#[allow(clippy::too_many_lines)]
fn gen(node: Option<Box<Node>>, mut labelseq: usize) -> usize {
    let node = *node.unwrap();
    match node.kind {
        Kind::Num(numbers) => {
            print!("  push ");
            //数値を出力
            for number in numbers {
                print!("{number}");
            }
            println!();
            //構文木の末尾のノードなので関数終了
            return labelseq;
        }
        // {}の中
        Kind::CurlyBracOpen => {
            if node.lhs.is_none() {
                return labelseq;
            }
            labelseq = gen(node.lhs, labelseq);
            return gen(node.rhs, labelseq);
        }
        Kind::Return => {
            labelseq = gen(node.lhs, labelseq);
            println!("  pop rax");
            println!("  jmp .Lreturn");
            return labelseq;
        }
        Kind::FunCall(func_name, args) => {
            // 引数の入るレジスタ
            let arg_register = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
            // 引数がある場合
            if let Some(args) = args {
                let args_num: usize = if args.len() > arg_register.len() {
                    panic!(
                        "引数はレジスタの数である{}個以下にして下さい。プログラムを終了します。",
                        arg_register.len()
                    );
                } else {
                    args.len()
                };
                // 各引数を評価
                for arg in args {
                    labelseq = gen(Some(Box::new(arg)), labelseq);
                }
                if args_num >= 1 {
                    // 順番に注意
                    for i in (0..args_num).rev() {
                        println!("  pop {}", arg_register[i]);
                    }
                }
            }
            let seq = labelseq;
            labelseq += 1;
            // We need to align RSP to a 16 byte boundary before
            // calling a function because it is an ABI requirement.
            // RAX is set to 0 for variadic function.
            // スタックポインタが16の倍数か確認
            println!("  mov rax, rsp");
            println!("  and rax, 15");

            // if (スタックポインタが16の倍数)
            println!("  jnz .Lcall{seq}");
            // {
            println!("  mov rax, 0");
            println!("  call {func_name}");
            println!("  jmp .Lend{seq}",);
            // } else {
            println!(".Lcall{seq}:");
            println!("  sub rsp, 8");
            println!("  mov rax, 0");
            println!("  call {func_name}");
            println!("  push rax");
            println!("  add rsp, 8");
            // }
            println!(".Lend{seq}:");
            println!(" push rax");
            return labelseq;
        }
        Kind::If(node_cond) => {
            // この関数内でのみ使うラベル番号(ラベル番号を使うすべてのgen関数のラベル番号に対して一意)
            let seq = labelseq;
            // ラベル番号更新
            labelseq += 1;
            if node.rhs.is_some() {
                // else文がある場合
                // 条件式
                labelseq = gen(node_cond, labelseq);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je  .Lelse{seq}");
                // then式
                labelseq = gen(node.lhs, labelseq);
                println!("  jmp .Lend{seq}");
                println!(".Lelse{seq}:");
                // else式
                labelseq = gen(node.rhs, labelseq);
                println!(".Lend{seq}:");
            } else {
                // else文がない場合(rhsがNoneの場合)
                // 条件式
                labelseq = gen(node_cond, labelseq);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je  .Lend{seq}");
                // then式
                labelseq = gen(node.lhs, labelseq);
                println!(".Lend{seq}:");
            }
            return labelseq;
        }
        Kind::While(node_cond) => {
            // この関数内でのみ使うラベル番号(ラベル番号を使うすべてのgen関数のラベル番号に対して一意)
            let seq = labelseq;
            // ラベル番号更新
            labelseq += 1;
            println!(".Lbegin{seq}:");
            // 条件式
            labelseq = gen(node_cond, labelseq);
            println!("  pop rax");
            println!("  cmp rax, 0");
            println!("  je  .Lend{seq}");
            // then式
            labelseq = gen(node.lhs, labelseq);
            println!("  jmp .Lbegin{seq}");
            println!(".Lend{seq}:");
            return labelseq;
        }
        Kind::For(node_init, node_cond, node_inc) => {
            // この関数内でのみ使うラベル番号(ラベル番号を使うすべてのgen関数のラベル番号に対して一意)
            let seq = labelseq;
            // ラベル番号更新
            labelseq += 1;
            if node_init.is_some() {
                // 存在すれば初期化処理
                labelseq = gen(node_init, labelseq);
            }
            println!(".Lbegin{seq}:");
            if node_cond.is_some() {
                // 存在すれば条件式
                labelseq = gen(node_cond, labelseq);
                println!("  pop rax");
                println!("  cmp rax, 0");
                println!("  je  .Lend{seq}");
            }
            // 条件式が真の場合のthen式
            labelseq = gen(node.lhs, labelseq);
            if node_inc.is_some() {
                // 存在すれば変化式
                labelseq = gen(node_inc, labelseq);
            }
            println!("  jmp .Lbegin{seq}");
            println!(".Lend{seq}:");
            return labelseq;
        }

        Kind::Var(ident) => {
            //指定された変数のアドレスをスタックにプッシュする
            push_var_address(ident);
            //変数の中身の値をスタックにプッシュする
            println!("  pop rax");
            println!("  mov rax, [rax]");
            println!("  push rax");
            //構文木の末尾のノードなので関数終了
            return labelseq;
        }
        Kind::Assign => {
            if let Kind::Var(ident) = (node.lhs).as_ref().unwrap().kind {
                //指定された変数のアドレスをスタックにプッシュする
                push_var_address(ident);
                //右辺の値を計算
                gen(node.rhs, labelseq);
                //変数に右辺の値を代入
                println!("  pop rdi");
                println!("  pop rax");
                println!("  mov [rax], rdi");
                println!("  push rdi");
                //代入式が終わったので関数終了
                return labelseq;
            }
            panic!("式の左辺に変数以外があります。プログラムを終了します。");
        }
        //ノードが上記に当てはまらない場合のみ以降の処理に進む
        _ => (),
    }
    //ノードが演算子だった場合
    labelseq = gen(node.lhs, labelseq);
    labelseq = gen(node.rhs, labelseq);

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
    labelseq
}

//指定された変数のアドレスをスタックにプッシュする
fn push_var_address(ident: usize) {
    println!("  mov rax, rbp");
    //オフセット値には変数のサイズ(8byte)を考慮する
    println!("  sub rax, {}", ident * 8);
    println!("  push rax");
}
