//トークンとノードの種類
pub enum Kind {
    Add,                                                          // +
    Sub,                                                          // -
    Mul,                                                          // *
    Div,                                                          // /
    RoundBracOpen,                                                // (
    RoundBracClose,                                               // )
    CurlyBracOpen,                      // トークンでは{, ノードでは{}内の文を表す
    CurlyBracClose,                     // }
    Equal,                              // ==
    NoEqual,                            // !=
    LowThan,                            // <
    LowEqual,                           // <=
    HighThan,                           // >
    HighEqual,                          // >=
    Semicolon,                          // ;
    Assign,                             // = 代入演算子
    Return,                             // return
    Comma,                              // ,
    FunCall(String, Option<Vec<Node>>), // 関数呼び出し (関数名, 引数(トークンでは常にNoneとする))
    If(Option<Box<Node>>),              // if(条件式のノード)
    While(Option<Box<Node>>),           // while(条件式のノード)
    For(Option<Box<Node>>, Option<Box<Node>>, Option<Box<Node>>), // for(初期化式;条件式;変化式)
    Else,                               //else
    //変数の1文字目にはアルファベットまたはアンダーバーのみ可
    //2文字目以降はそれに加えて数字も可
    Var(usize), // 変数(変数を一意に指す識別番号。1からの連番)
    //数値はそのまま出力するだけなのでchar型とする
    Num(Vec<char>),
}

//構文木を構成するノード
pub struct Node {
    pub kind: Kind,
    pub lhs: Option<Box<Node>>,
    pub rhs: Option<Box<Node>>,
}
