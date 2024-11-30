use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    // Javascript keywords
    Var,
    Let,
    Const,
    Function,
    Async,
    Await,
    Static,
    If,
    Else,
    Try,
    Catch,
    Finally,
    While,
    Do,
    For,
    In,
    Of,
    Break,
    Continue,
    Class,
    Abstract,
    Extends,
    Implements,
    New,
    This,
    Super,
    Private,
    Protected,
    Switch,
    Case,
    Default,
    Type,
    Interface,
    True,
    False,

    // Typescript keywords
    StringType,  // : string
    NumberType,  // : number
    BooleanType, // : boolean
    RecordType,  // : Record<>
    ArrayType,   // : Array<>
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Keyword> = {
        let mut m = HashMap::new();

        // JavaScript keywords
        m.insert("var", Keyword::Var);
        m.insert("let", Keyword::Let);
        m.insert("const", Keyword::Const);
        m.insert("function", Keyword::Function);
        m.insert("async", Keyword::Async);
        m.insert("await", Keyword::Await);
        m.insert("static", Keyword::Static);
        m.insert("if", Keyword::If);
        m.insert("else", Keyword::Else);
        m.insert("try", Keyword::Try);
        m.insert("catch", Keyword::Catch);
        m.insert("finally", Keyword::Finally);
        m.insert("while", Keyword::While);
        m.insert("do", Keyword::Do);
        m.insert("for", Keyword::For);
        m.insert("in", Keyword::In);
        m.insert("of", Keyword::Of);
        m.insert("break", Keyword::Break);
        m.insert("continue", Keyword::Continue);
        m.insert("class", Keyword::Class);
        m.insert("abstract", Keyword::Abstract);
        m.insert("extends", Keyword::Extends);
        m.insert("implements", Keyword::Implements);
        m.insert("new", Keyword::New);
        m.insert("this", Keyword::This);
        m.insert("super", Keyword::Super);
        m.insert("private", Keyword::Private);
        m.insert("protected", Keyword::Protected);
        m.insert("switch", Keyword::Switch);
        m.insert("case", Keyword::Case);
        m.insert("default", Keyword::Default);
        m.insert("type", Keyword::Type);
        m.insert("interface", Keyword::Interface);
        m.insert("true", Keyword::True);
        m.insert("false", Keyword::False);

        // TypeScript types
        m.insert("string", Keyword::StringType);
        m.insert("number", Keyword::NumberType);
        m.insert("boolean", Keyword::BooleanType);
        m.insert("Record", Keyword::RecordType);
        m.insert("Array", Keyword::ArrayType);

        m
    };
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Keyword> {
        KEYWORDS.get(s).cloned()
    }
}
