use std::env;
use std::str;

//トークン列を保存
struct List {
    //トークン列の中で処理中のトークン
    proccesing_token: Option<Box<Token>>,
}
struct Token {
    kind: Kind,
    next: Option<Box<Token>>,
}
//トークンとノードの種類
enum Kind {
    Add,
    Sub,
    //数値はそのまま出力するだけなのでchar型とする
    Num(Vec<char>),
}
//構文木を構成するノード
struct Node {
    kind: Kind,
    lhs: Option<Box<Node>>,
    rhs: Option<Box<Node>>,
}

impl List {
    //リストの末尾にトークンを追加する
    fn push_back(&mut self, value: Kind) {
        //末尾のトークン(None)を返す
        fn findlast(token: &mut Option<Box<Token>>) -> &mut Option<Box<Token>> {
            if let Some(ref mut b) = *token {
                findlast(&mut b.next)
            } else {
                token
            }
        }
        let token = findlast(&mut self.proccesing_token);
        *token = Some(Box::new(Token {
            kind: value,
            next: None,
        }));
    }
    // 入力文字列argをトークナイズする
    fn tokenize(&mut self, arg: &mut str::Chars) {
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
        while let Some(mut c) = arg.next() {
            //数字の場合
            if c.is_numeric() {
                //連続した数字をVecにまとめ、その次の記号も出す
                let (c_return, numbers) = continue_num(c, arg);
                //数字のトークンを作成
                self.push_back(Kind::Num(numbers));
                //処理すべき記号を更新
                if let Some(d) = c_return {
                    c = d;
                } else {
                    //前の数字で入力文字列が終わっていたら関数終了
                    return;
                }
            }
            //記号の処理
            match c {
                '+' => self.push_back(Kind::Add),
                '-' => self.push_back(Kind::Sub),
                //空白はスキップ
                c if c.is_whitespace() => (),
                _ => panic!(
                    "不正な文字\"{}\"が存在するため、プログラムを終了します。",
                    c
                ),
            }
        }
    }

    //現在のトークンが数値であれば対応したノードを生成して返す
    //トークンが数値以外または存在しない場合はpanicさせる
    fn expect_num(&mut self) -> Node {
        if let Some(token) = &self.proccesing_token {
            if let Kind::Num(numbers) = &token.kind {
                Node {
                    //proccesing_tokenから所有権を移動させないためにcloneを使う
                    kind: Kind::Num(numbers.clone()),
                    lhs: None,
                    rhs: None,
                }
            } else {
                panic!("演算子の後に数字以外が存在しています。プログラムを終了します。");
            }
        } else {
            panic!("入力の末尾に数値がありません。プログラムを終了します。");
        }
    }
}

fn main() {
    //引数を入力文字列として格納
    let arg_vec: Vec<String> = env::args().collect();
    let mut arg = arg_vec[1].chars();

    let mut list = List {
        proccesing_token: None,
    };
    //引数の文字列をトークナイズする
    list.tokenize(&mut arg);

    let node = expr(list);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    generate(node);

    //結果の値はスタックの一番上に置かれるので、その値をraxレジスタに置く
    println!("  pop rax");
    println!("  ret");
}

// expr = num ("+" num | "-" num)*
fn expr(mut list: List) -> Option<Box<Node>> {
    //num
    let mut node = list.expect_num();
    //トークンの数値を読み取ったらトークンを進める
    list.proccesing_token = next_token(list.proccesing_token);
    //("+" num | "-" num)*
    loop {
        if let Some(token) = &list.proccesing_token {
            match token.kind {
                Kind::Add => {
                    //トークンを進める
                    list.proccesing_token = next_token(list.proccesing_token);
                    node = Node {
                        kind: Kind::Add,
                        lhs: Some(Box::new(node)),
                        rhs: Some(Box::new(list.expect_num())),
                    }
                }
                Kind::Sub => {
                    //トークンを進める
                    list.proccesing_token = next_token(list.proccesing_token);
                    node = Node {
                        kind: Kind::Sub,
                        lhs: Some(Box::new(node)),
                        rhs: Some(Box::new(list.expect_num())),
                    }
                }
                _ => panic!("演算子の後に数字以外が存在しています。プログラムを終了します。"),
            }
        } else {
            //入力の末尾として正常終了
            break;
        }
        //トークンを進める
        list.proccesing_token = next_token(list.proccesing_token);
    }
    Some(Box::new(node))
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
                //この状況はトークン生成時に弾いているはず
                _ => (),
            }
            println!("  push rax");
        }
    }
    //引数がNoneの場合は何もしない
}

//次のトークンを返す
fn next_token(token: Option<Box<Token>>) -> Option<Box<Token>> {
    if let Some(now_token) = token {
        now_token.next
    } else {
        None
    }
}
