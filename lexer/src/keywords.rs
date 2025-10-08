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
    Interface,
    True,
    False,
    Typeof,
    Throw,

    // Typescript keywords
    StringType,  // : string
    NumberType,  // : number
    BooleanType, // : boolean
    Type,        // type T = ...
    Enum,        // enum Foo {}
    Declare,     // declare enum Foo {}
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out: &'static str = match self {
            // Javascript keywords
            Self::Var => "var",
            Self::Let => "let",
            Self::Const => "const",
            Self::Function => "function",
            Self::Return => "return",
            Self::Async => "async",
            Self::Await => "await",
            Self::Static => "static",
            Self::If => "if",
            Self::Else => "else",
            Self::Try => "try",
            Self::Catch => "catch",
            Self::Finally => "finally",
            Self::While => "while",
            Self::Do => "do",
            Self::For => "for",
            Self::In => "in",
            Self::Of => "of",
            Self::Break => "break",
            Self::Continue => "continue",
            Self::Class => "class",
            Self::Abstract => "abstract",
            Self::Extends => "extends",
            Self::Implements => "implements",
            Self::New => "new",
            Self::This => "this",
            Self::Super => "super",
            Self::Private => "private",
            Self::Protected => "protected",
            Self::Switch => "switch",
            Self::Case => "case",
            Self::Default => "default",
            Self::Interface => "interface",
            Self::True => "true",
            Self::False => "false",
            Self::Typeof => "typeof",
            Self::Throw => "throw",

            // Typescript keywords
            Self::StringType => "string",
            Self::NumberType => "number",
            Self::BooleanType => "boolean",
            Self::Type => "type",
            Self::Enum => "enum",
            Self::Declare => "declare",
        };
        write!(f, "{}", out)
    }
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
    "throw" => Keyword::Throw,

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
