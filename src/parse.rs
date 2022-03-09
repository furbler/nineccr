use crate::kind::Kind;
use crate::kind::Node;

//外部から呼び出される関数
pub fn program(tokens: &Vec<Kind>) -> Vec<Node> {
    //トークン列から構文木を生成
    let mut progress = 0;
    let mut nodes: Vec<Node> = Vec::new();
    //文単位で保存
    while progress < tokens.len() {
        let (ret_node, ret_progress) = stmt(&tokens, progress);
        nodes.push(ret_node);
        progress = ret_progress;
    }
    nodes
}

// statement
// stmt = expr ";" | "return" expr ";"
fn stmt(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
    let mut node;
    let ret_progress;

    if let Kind::Return = tokens[progress] {
        (node, ret_progress) = expr(tokens, progress + 1);
        node = Node {
            kind: Kind::Return,
            lhs: Some(Box::new(node)),
            rhs: None,
        };
    } else {
        (node, ret_progress) = expr(tokens, progress);
    }

    if tokens.len() <= ret_progress {
        panic!("文の終わりに;が付いていません。プログラムを終了します。");
    }
    if let Kind::Semicolon = tokens[ret_progress] {
        (node, ret_progress + 1)
    } else {
        panic!("文の終わりに;が付いていません。プログラムを終了します。");
    }
}

// expr = assign
fn expr(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
    assign(tokens, progress)
}

// assign = equality ("=" assign)?
fn assign(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
    let (node, progress) = equality(tokens, progress);
    //代入演算子が無い場合
    if tokens.len() <= progress {
        return (node, progress);
    }
    if let Kind::Assign = tokens[progress] {
        let (ret_node, ret_progress) = assign(tokens, progress + 1);
        (
            Node {
                kind: Kind::Assign,
                lhs: Some(Box::new(node)),
                rhs: Some(Box::new(ret_node)),
            },
            ret_progress,
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

// primary = "(" expr ")" | ident | num
fn primary(tokens: &Vec<Kind>, progress: usize) -> (Node, usize) {
    match tokens[progress] {
        Kind::BracOpen => {
            //"(" expr ")"
            let (node, progress) = expr(tokens, progress + 1);
            if let Kind::BracClose = tokens[progress] {
                (node, progress + 1)
            } else {
                panic!("括弧が閉じていません。プログラムを終了します。");
            }
        }
        Kind::Var(index) => (
            Node {
                kind: Kind::Var(index),
                lhs: None,
                rhs: None,
            },
            progress + 1,
        ),
        //num
        Kind::Num(_) => expect_num(tokens, progress),
        _ => panic!("構文木の末端には変数か数値しか置けません。プログラムを終了します。"),
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
