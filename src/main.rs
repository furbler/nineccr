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
    Equal,     // ==
    NoEqual,   // !=
    LowThan,   // <
    LowEqual,  // <=
    HighThan,  // >
    HighEqual, // >=
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

// expr = equality
fn expr(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
    equality(tokens, progress)
}

// equality = relational ("==" relational | "!=" relational)*
fn equality(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
    //relational
    let (mut node, mut progress) = relational(tokens, progress);
    //("==" relational | "!=" relational)*
    while progress < tokens.len() {
        match tokens[progress] {
            Kind::Equal => {
                let (ret_node, ret_progress) = relational(tokens, progress + 1);
                node = Node {
                    kind: Kind::Equal,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(ret_node)),
                };
                //トークンを進める
                progress = ret_progress;
            }
            Kind::NoEqual => {
                let (ret_node, ret_progress) = relational(tokens, progress + 1);
                node = Node {
                    kind: Kind::NoEqual,
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

// relational = add ("<" add | "<=" add | ">" add | ">=" add)*
fn relational(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
    //add
    let (mut node, mut progress) = add(tokens, progress);
    //("==" relational | "!=" relational)*
    while progress < tokens.len() {
        match tokens[progress] {
            Kind::LowThan => {
                let (ret_node, ret_progress) = add(tokens, progress + 1);
                node = Node {
                    kind: Kind::LowThan,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(ret_node)),
                };
                //トークンを進める
                progress = ret_progress;
            }
            Kind::LowEqual => {
                let (ret_node, ret_progress) = add(tokens, progress + 1);
                node = Node {
                    kind: Kind::LowEqual,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(ret_node)),
                };
                //トークンを進める
                progress = ret_progress;
            }
            Kind::HighThan => {
                let (ret_node, ret_progress) = add(tokens, progress + 1);
                node = Node {
                    //ノードの左右を入れ替えて小なりに統一する
                    kind: Kind::LowThan,
                    lhs: Some(Box::new(ret_node)),
                    rhs: Some(Box::new(node)),
                };
                //トークンを進める
                progress = ret_progress;
            }
            Kind::HighEqual => {
                let (ret_node, ret_progress) = add(tokens, progress + 1);
                node = Node {
                    //ノードの左右を入れ替えて小なりに統一する
                    kind: Kind::LowEqual,
                    lhs: Some(Box::new(ret_node)),
                    rhs: Some(Box::new(node)),
                };
                //トークンを進める
                progress = ret_progress;
            }
            _ => return (node, progress),
        }
    }
    (node, progress)
}

// add = mul ("+" mul | "-" mul)*
fn add(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
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

    let node = expr(&tokens, 0).0;

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    generate(Some(Box::new(node)));

    //結果の値はスタックの一番上に置かれるので、その値をraxレジスタに置く
    println!("  pop rax");
    println!("  ret");
}
