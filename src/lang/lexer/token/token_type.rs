use strum_macros::IntoStaticStr;

#[derive(Copy, Clone, PartialEq, IntoStaticStr)]
pub enum TokenType {
    Plus,
    Minus,
    Times,
    Divide,
    Mod,
    PlusEquals,
    MinusEquals,
    TimesEquals,
    DivideEquals,
    ModEquals,
    PowEquals,
    Not,
    Or,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    NotEqual,
    Number,
    Identifier,
    Period,
    Rparen,
    Equal,
    DoubleEqual,
    Lparen,
    Lbracket,
    Rbracket,
    Lbrace,
    Rbrace,
    Lambda,
    Num,
    Str,
    String,
    Func,
    Function,
    Bool,
    True,
    False,
    Print,
    Println,
    Void,
    List,
    Dict,
    If,
    In,
    End,
    Newline,
    Pow,
    Typeof,
    Comma,
    Colon,
}