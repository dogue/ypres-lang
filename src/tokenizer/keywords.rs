use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    Main,
    Return,
    If,
    Else,
    True,
    False,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Nil,
    Bool,
    I32,
    I64,
    U32,
    U64,
    F32,
    F64,
    String,
}

impl FromStr for Type {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nil" => Ok(Self::Nil),
            "bool" => Ok(Self::Bool),
            "i32" => Ok(Self::I32),
            "i64" => Ok(Self::I64),
            "u32" => Ok(Self::U32),
            "u64" => Ok(Self::U64),
            "f32" => Ok(Self::F32),
            "f64" => Ok(Self::F64),
            "string" => Ok(Self::String),
            _ => Err("invalid type".into()),
        }
    }

    type Err = String;
}

impl FromStr for Keyword {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "main" => Ok(Self::Main),
            "return" => Ok(Self::Return),
            "if" => Ok(Self::If),
            "else" => Ok(Self::Else),
            "true" => Ok(Self::True),
            "false" => Ok(Self::False),
            _ => Err("invalid keyword".into()),
        }
    }
}
