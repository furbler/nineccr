//トークンとノードの種類
pub enum Kind {
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
pub struct Node {
    pub kind: Kind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}
