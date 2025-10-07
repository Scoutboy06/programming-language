use phf::phf_map;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    // Javascript keywords
    Var,
    Let,
    Const,
    Function,
    Return,
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
    Typeof,

    // Typescript keywords
    StringType,  // : string
    NumberType,  // : number
    BooleanType, // : boolean
    Enum,        // enum Foo {}
    Declare,     // declare enum Foo {}
}

static KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "var" => Keyword::Var,
    "let" => Keyword::Let,
    "const" => Keyword::Const,
    "function" => Keyword::Function,
    "return" => Keyword::Return,
    "async" => Keyword::Async,
    "await" => Keyword::Await,
    "static" => Keyword::Static,
    "if" => Keyword::If,
    "else" => Keyword::Else,
    "try" => Keyword::Try,
    "catch" => Keyword::Catch,
    "finally" => Keyword::Finally,
    "while" => Keyword::While,
    "do" => Keyword::Do,
    "for" => Keyword::For,
    "in" => Keyword::In,
    "of" => Keyword::Of,
    "break" => Keyword::Break,
    "continue" => Keyword::Continue,
    "class" => Keyword::Class,
    "abstract" => Keyword::Abstract,
    "extends" => Keyword::Extends,
    "implements" => Keyword::Implements,
    "new" => Keyword::New,
    "this" => Keyword::This,
    "super" => Keyword::Super,
    "private" => Keyword::Private,
    "protected" => Keyword::Protected,
    "switch" => Keyword::Switch,
    "case" => Keyword::Case,
    "default" => Keyword::Default,
    "type" => Keyword::Type,
    "interface" => Keyword::Interface,
    "true" => Keyword::True,
    "false" => Keyword::False,
    "typeof" => Keyword::Typeof,

    // TypeScript keywords
    "string" => Keyword::StringType,
    "number" => Keyword::NumberType,
    "boolean" => Keyword::BooleanType,
    "enum" => Keyword::Enum,
    "declare" => Keyword::Declare
};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKeyword {
    String,
    Number,
    Boolean,
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Keyword> {
        KEYWORDS.get(s).cloned()
    }

    pub fn as_type_keyword(&self) -> Option<TypeKeyword> {
        match self {
            Self::StringType => Some(TypeKeyword::String),
            Self::NumberType => Some(TypeKeyword::Number),
            Self::BooleanType => Some(TypeKeyword::Boolean),
            _ => None,
        }
    }
}
