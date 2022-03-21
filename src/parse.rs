use std::panic;

use crate::kind::Kind;
use crate::kind::Node;

//外部から呼び出される関数
pub fn program(tokens: &Vec<Kind>) -> Vec<Node> {
    //トークン列から構文木を生成
    let mut progress = 0;
    let mut nodes: Vec<Node> = Vec::new();
    let mut ret_node;
    //文単位で保存
    while progress < tokens.len() {
        (ret_node, progress) = stmt(&tokens, progress);
        nodes.push(ret_node);
    }
    nodes
}

// statement
// stmt = expr ";"
// | "{" stmt* "}"
// | "return" expr ";"
// | "if" "(" expr ")" stmt ("else" stmt)?
// | "while" "(" expr ")" stmt
// | "for" "(" expr? ";" expr? ";" expr? ")" stmt
fn stmt(tokens: &Vec<Kind>, mut progress: usize) -> (Node, usize) {
    let mut node;
    match tokens[progress] {
        // "return" expr ";"
        Kind::Return => {
            (node, progress) = expr(tokens, progress + 1);
            node = Node {
                kind: Kind::Return,
                lhs: Some(Box::new(node)),
                rhs: None,
            };

            if tokens.len() <= progress {
                panic!("文の終わりに;が付いていません。プログラムを終了します。");
            }
            if let Kind::Semicolon = tokens[progress] {
                return (node, progress + 1);
            } else {
                panic!("文の終わりに;が付いていません。プログラムを終了します。");
            }
        }
        // "{" stmt* "}"
        Kind::CurlyBracOpen => {
            let mut node = Node {
                // ここは他のノードのkindとかぶらなければ何でも良い
                kind: Kind::CurlyBracOpen,
                // 前の文を保持するノードを指す
                lhs: None,
                // 前の文
                rhs: None,
            };
            progress += 1;
            // } が出てくるまで繰り返す
            loop {
                let node_stmt;
                if let Kind::CurlyBracClose = tokens[progress] {
                    return (node, progress + 1);
                } else {
                    (node_stmt, progress) = stmt(tokens, progress);
                    node = Node {
                        kind: Kind::CurlyBracOpen,
                        lhs: Some(Box::new(node)),
                        rhs: Some(Box::new(node_stmt)),
                    };
                }
            }
        }

        // "if" "(" expr ")" stmt ("else" stmt)?
        Kind::If(_) => {
            // 条件式
            let node_cond;
            // then式
            let node_then;
            // else式
            let node_else;
            if let Kind::RoundBracOpen = tokens[progress + 1] {
                // 条件式
                (node_cond, progress) = expr(tokens, progress + 2);
            } else {
                panic!("if文の条件式は括弧で囲ってください。プログラムを終了します。");
            }
            if let Kind::RoundBracClose = tokens[progress] {
                // 条件式が真のときに実行する部分
                (node_then, progress) = stmt(tokens, progress + 1);
            } else {
                panic!("if文の条件式は括弧で囲ってください。プログラムを終了します。");
            }
            if let Kind::Else = tokens[progress] {
                // 条件式がの偽のときに実行する部分
                (node_else, progress) = stmt(tokens, progress + 1);
                node = Node {
                    kind: Kind::If(Some(Box::new(node_cond))),
                    lhs: Some(Box::new(node_then)),
                    rhs: Some(Box::new(node_else)),
                };
            } else {
                // elseが無い場合
                node = Node {
                    kind: Kind::If(Some(Box::new(node_cond))),
                    lhs: Some(Box::new(node_then)),
                    rhs: None,
                };
            }
            // 条件式が真ならlhsを、偽ならrhsを実行すべし
            return (node, progress);
        }
        // "while" "(" expr ")" stmt
        Kind::While(_) => {
            // 条件式
            let node_cond;
            // then式
            let node_then;
            if let Kind::RoundBracOpen = tokens[progress + 1] {
                // 条件式
                (node_cond, progress) = expr(tokens, progress + 2);
            } else {
                panic!("while文の条件式は括弧で囲ってください。プログラムを終了します。");
            }
            if let Kind::RoundBracClose = tokens[progress] {
                // 条件式が真のときに実行する部分
                (node_then, progress) = stmt(tokens, progress + 1);
            } else {
                panic!("while文の条件式は括弧で囲ってください。プログラムを終了します。");
            }
            node = Node {
                kind: Kind::While(Some(Box::new(node_cond))),
                lhs: Some(Box::new(node_then)),
                rhs: None,
            };

            // 条件式が真ならlhsの処理をループ
            return (node, progress);
        }
        // "for" "(" expr? ";" expr? ";" expr? ")" stmt
        Kind::For(..) => {
            // 初期化式
            let node_init;
            // 条件式
            let node_cond;
            // 変化式
            let node_inc;
            // then式
            let node_then;

            if let Kind::RoundBracOpen = tokens[progress + 1] {
                if let Kind::Semicolon = tokens[progress + 2] {
                    // 初期化式無し
                    node_init = None;
                    progress = progress + 3;
                } else {
                    // 初期化式
                    (node, progress) = expr(tokens, progress + 2);
                    node_init = Some(Box::new(node));
                    // 初期化式と条件式の間のセミコロン
                    if let Kind::Semicolon = tokens[progress] {
                        progress += 1;
                    } else {
                        panic!("for文に;が足りません。プログラムを終了します。");
                    }
                }
            } else {
                panic!("for文の条件式は括弧で囲ってください。プログラムを終了します。");
            }
            if let Kind::Semicolon = tokens[progress] {
                // 条件式無し（無条件ループ）
                node_cond = None;
                progress = progress + 1;
            } else {
                // 条件式
                (node, progress) = expr(tokens, progress);
                node_cond = Some(Box::new(node));
                // 条件式と変化式の間のセミコロン
                if let Kind::Semicolon = tokens[progress] {
                    progress += 1;
                } else {
                    panic!("for文に;が足りません。プログラムを終了します。");
                }
            }
            if let Kind::RoundBracClose = tokens[progress] {
                // 変化式無し
                node_inc = None;
                progress = progress + 1;
            } else {
                // 変化式
                (node, progress) = expr(tokens, progress);
                node_inc = Some(Box::new(node));
                if let Kind::RoundBracClose = tokens[progress] {
                    progress += 1;
                } else {
                    panic!("for文の条件式は括弧で囲ってください。プログラムを終了します。");
                }
            }

            // ループ本体
            (node_then, progress) = stmt(tokens, progress);
            node = Node {
                kind: Kind::For(node_init, node_cond, node_inc),
                lhs: Some(Box::new(node_then)),
                rhs: None,
            };
            return (node, progress);
        }
        // expr ";"
        _ => {
            (node, progress) = expr(tokens, progress);

            if tokens.len() <= progress {
                panic!("文の終わりに;が付いていません。プログラムを終了します。");
            }
            if let Kind::Semicolon = tokens[progress] {
                return (node, progress + 1);
            } else {
                panic!("文の終わりに;が付いていません。プログラムを終了します。");
            }
        }
    }
}

// expr = assign
fn expr(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
    assign(tokens, progress)
}

// assign = equality ("=" assign)?
fn assign(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
    let (node, mut progress) = equality(tokens, progress);
    //代入演算子が無い場合
    if tokens.len() <= progress {
        return (node, progress);
    }
    if let Kind::Assign = tokens[progress] {
        let rhs_node;
        (rhs_node, progress) = assign(tokens, progress + 1);
        (
            Node {
                kind: Kind::Assign,
                lhs: Some(Box::new(node)),
                rhs: Some(Box::new(rhs_node)),
            },
            progress,
        )
    } else {
        (node, progress)
    }
}

// equality = relational ("==" relational | "!=" relational)*
fn equality(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
    //relational
    let (mut node, mut progress) = relational(tokens, progress);
    //("==" relational | "!=" relational)*
    while progress < tokens.len() {
        match tokens[progress] {
            Kind::Equal => {
                let rhs_node;
                (rhs_node, progress) = relational(tokens, progress + 1);
                node = Node {
                    kind: Kind::Equal,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(rhs_node)),
                }
            }
            Kind::NoEqual => {
                let rhs_node;
                (rhs_node, progress) = relational(tokens, progress + 1);
                node = Node {
                    kind: Kind::NoEqual,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(rhs_node)),
                }
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
                let rhs_node;
                (rhs_node, progress) = add(tokens, progress + 1);
                node = Node {
                    kind: Kind::LowThan,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(rhs_node)),
                };
            }
            Kind::LowEqual => {
                let rhs_node;
                (rhs_node, progress) = add(tokens, progress + 1);
                node = Node {
                    kind: Kind::LowEqual,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(rhs_node)),
                };
            }
            Kind::HighThan => {
                let rhs_node;
                (rhs_node, progress) = add(tokens, progress + 1);
                node = Node {
                    //ノードの左右を入れ替えて小なりに統一する
                    kind: Kind::LowThan,
                    lhs: Some(Box::new(rhs_node)),
                    rhs: Some(Box::new(node)),
                };
            }
            Kind::HighEqual => {
                let rhs_node;
                (rhs_node, progress) = add(tokens, progress + 1);
                node = Node {
                    //ノードの左右を入れ替えて小なりに統一する
                    kind: Kind::LowEqual,
                    lhs: Some(Box::new(rhs_node)),
                    rhs: Some(Box::new(node)),
                };
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
                let rhs_node;
                (rhs_node, progress) = mul(tokens, progress + 1);
                node = Node {
                    kind: Kind::Add,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(rhs_node)),
                };
            }
            Kind::Sub => {
                let rhs_node;
                (rhs_node, progress) = mul(tokens, progress + 1);
                node = Node {
                    kind: Kind::Sub,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(rhs_node)),
                };
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
                let rhs_node;
                (rhs_node, progress) = unary(tokens, progress + 1);
                node = Node {
                    kind: Kind::Mul,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(rhs_node)),
                };
            }
            Kind::Div => {
                let rhs_node;
                (rhs_node, progress) = unary(tokens, progress + 1);
                node = Node {
                    kind: Kind::Div,
                    lhs: Some(Box::new(node)),
                    rhs: Some(Box::new(rhs_node)),
                };
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
            let (rhs_node, progress) = primary(tokens, progress + 1);
            // 対応する0のノードを生成
            let zero_node = Node {
                kind: Kind::Num(vec!['0']),
                lhs: None,
                rhs: None,
            };
            (
                Node {
                    kind: Kind::Sub,
                    lhs: Some(Box::new(zero_node)),
                    rhs: Some(Box::new(rhs_node)),
                },
                progress,
            )
        }
        _ => primary(tokens, progress),
    }
}

// primary = "(" expr ")"
// | ident
// | ident func-args?
// | num
fn primary(tokens: &Vec<Kind>, mut progress: usize) -> (Node, usize) {
    match tokens[progress] {
        Kind::RoundBracOpen => {
            //"(" expr ")"
            let (node, progress) = expr(tokens, progress + 1);
            if let Kind::RoundBracClose = tokens[progress] {
                (node, progress + 1)
            } else {
                panic!("括弧が閉じていません。プログラムを終了します。");
            }
        }
        // ident
        Kind::Var(index) => (
            Node {
                kind: Kind::Var(index),
                lhs: None,
                rhs: None,
            },
            progress + 1,
        ),
        // ident "(" func-args? ")"
        Kind::FunCall(ref func_name, _) => {
            progress += 1;
            if let Kind::RoundBracOpen = tokens[progress] {
                if let Kind::RoundBracClose = tokens[progress + 1] {
                    // 引数なし
                    (
                        Node {
                            kind: Kind::FunCall(func_name.clone(), None),
                            lhs: None,
                            rhs: None,
                        },
                        progress + 2,
                    )
                } else {
                    // 引数あり
                    func_args(tokens, progress + 1, func_name.clone())
                }
            } else {
                panic!("関数名の後に括弧がありません。プログラムを終了します。")
            }
        }
        //num
        Kind::Num(_) => expect_num(tokens, progress),
        _ => panic!(
            "構文木の末端には変数か数値しか置けません。\nprogress = {}\nプログラムを終了します。",
            progress
        ),
    }
}

// func-args =  (assign ("," assign)*)?
fn func_args(tokens: &Vec<Kind>, mut progress: usize, func_name: String) -> (Node, usize) {
    let mut args = Vec::new();
    // 引数の1つを評価
    let node;
    (node, progress) = assign(tokens, progress);
    // 引数のリストに追加
    args.push(Box::new(node));

    loop {
        match tokens[progress] {
            Kind::Comma => {
                progress += 1;
            }
            Kind::RoundBracClose => {
                return (
                    Node {
                        kind: Kind::FunCall(
                            func_name.clone(),
                            if args.len() == 0 { None } else { Some(args) },
                        ),
                        lhs: None,
                        rhs: None,
                    },
                    progress + 1,
                );
            }
            _ => panic!("関数の引数の記述が不正です。プログラムを終了します。"),
        }
        let node;
        (node, progress) = assign(tokens, progress);
        // 引数のリストに追加
        args.push(Box::new(node));
    }
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
