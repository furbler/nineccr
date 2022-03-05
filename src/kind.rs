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
    Semicolon, // ;
    Assign,    // = 代入演算子
    //変数の1文字目にはアルファベットまたはアンダーバーのみ可
    //2文字目以降はそれに加えて数字も可
    Var(usize), // 変数(識別番号)
    //数値はそのまま出力するだけなのでchar型とする
    Num(Vec<char>),
}
//構文木を構成するノード
pub struct Node {
    pub kind: Kind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}
