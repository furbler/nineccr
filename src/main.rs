use std::env;
use std::str;

//トークンとノードの種類
enum Kind {
    Add,
    Sub,
    Mul,
    Div,
    BracOpen,  //開き括弧
    BracClose, //閉じ括弧
    //数値はそのまま出力するだけなのでchar型とする
    Num(Vec<char>),
}
//構文木を構成するノード
struct Node {
    kind: Kind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

// 入力文字列argをトークナイズする
fn tokenize(arg: &mut str::Chars) -> Vec<Kind> {
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
    //トークン列
    let mut tokens = Vec::new();
    while let Some(mut c) = arg.next() {
        //数字の場合
        if c.is_numeric() {
            //連続した数字をVecにまとめ、その次の記号も出す
            let (c_return, numbers) = continue_num(c, arg);
            //数字のトークンを追加
            tokens.push(Kind::Num(numbers));
            //処理すべき記号を更新
            if let Some(d) = c_return {
                c = d;
            } else {
                //前の数字で入力文字列が終わっていたら関数終了
                return tokens;
            }
        }
        //記号の処理
        match c {
            '+' => tokens.push(Kind::Add),
            '-' => tokens.push(Kind::Sub),
            '*' => tokens.push(Kind::Mul),
            '/' => tokens.push(Kind::Div),
            '(' => tokens.push(Kind::BracOpen),
            ')' => tokens.push(Kind::BracClose),
            //空白はスキップ
            c if c.is_whitespace() => (),
            _ => panic!(
                "不正な文字\"{}\"が存在するため、プログラムを終了します。",
                c
            ),
        }
    }
    tokens
}

//現在のトークンが数値であれば対応したノードを生成して返す
//トークンが数値以外または存在しない場合はpanicさせる
fn expect_num(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
    if let Kind::Num(ref numbers) = tokens[progress] {
        (
            Node {
                //所有権を移動させないためにcloneを使う
                kind: Kind::Num(numbers.clone()),
                lhs: None,
                rhs: None,
            },
            progress + 1,
        )
    } else {
        panic!("数字があるべき箇所に演算子があります。プログラムを終了します。");
    }
}

// expr = mul ("+" mul | "-" mul)*
fn expr(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
    //mul
    let (mut node, mut progress) = mul(tokens, progress);
    //("+" mul | "-" mul)*
    while progress < tokens.len() {
        match tokens[progress] {
            Kind::Add => {
                let (ret_node, ret_progress) = mul(tokens, progress + 1);
                node = Node {
                    kind: Kind::Add,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(ret_node)),
                };
                //トークンを進める
                progress = ret_progress;
            }
            Kind::Sub => {
                let (ret_node, ret_progress) = mul(tokens, progress + 1);
                node = Node {
                    kind: Kind::Sub,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(ret_node)),
                };
                //トークンを進める
                progress = ret_progress;
            }
            _ => return (node, progress),
        }
    }
    (node, progress)
}

//mul  = unary ("*" unary | "/" unary)*
fn mul(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
    //num
    let (mut node, mut progress) = unary(tokens, progress);
    //("*" num | "/" num)*
    while progress < tokens.len() {
        match tokens[progress] {
            Kind::Mul => {
                let (ret_node, ret_progress) = unary(tokens, progress + 1);
                node = Node {
                    kind: Kind::Mul,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(ret_node)),
                };
                //トークンを進める
                progress = ret_progress;
            }
            Kind::Div => {
                let (ret_node, ret_progress) = unary(tokens, progress + 1);
                node = Node {
                    kind: Kind::Div,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(ret_node)),
                };
                //トークンを進める
                progress = ret_progress;
            }
            _ => return (node, progress),
        }
    }
    (node, progress)
}

//unary   = ("+" | "-")? primary
fn unary(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
    match tokens[progress] {
        Kind::Add => primary(tokens, progress + 1),
        Kind::Sub => {
            let (ret_node, ret_progress) = primary(tokens, progress + 1);

            let zero_node = Node {
                kind: Kind::Num(vec!['0']),
                lhs: None,
                rhs: None,
            };
            (
                Node {
                    kind: Kind::Sub,
                    lhs: Some(Box::new(zero_node)),
                    rhs: Some(Box::new(ret_node)),
                },
                ret_progress,
            )
        }
        _ => primary(tokens, progress),
    }
}

//primary = num | "(" expr ")"
fn primary(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
    if let Kind::BracOpen = tokens[progress] {
        //"(" expr ")"
        let (node, progress) = expr(tokens, progress + 1);
        if let Kind::BracClose = tokens[progress] {
            (node, progress + 1)
        } else {
            panic!("括弧が閉じていません。プログラムを終了します。");
        }
    } else {
        //num
        expect_num(tokens, progress)
    }
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
                //この状況はトークン生成時に弾いているはず
                _ => (),
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

    let node = expr(&tokens, 0).0;

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    generate(Some(Box::new(node)));

    //結果の値はスタックの一番上に置かれるので、その値をraxレジスタに置く
    println!("  pop rax");
    println!("  ret");
}
