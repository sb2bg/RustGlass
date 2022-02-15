use std::collections::HashMap;
use std::fmt::Display;

pub enum Primitive {
    String(String),
    Number(f64),
    Bool(bool),
    Dictionary(HashMap<Primitive, Primitive>),
    List(Vec<Primitive>),
    TypePrimitive(Type),
    Void,
}

pub enum Type {
    String,
    Number,
    Bool,
    Dictionary,
    List,
    Type,
    Void,
}

impl Type {
    pub fn type_of(primitive: &Primitive) -> Type {
        match primitive {
            Primitive::String(_) => Type::String,
            Primitive::Number(_) => Type::Number,
            Primitive::Bool(_) => Type::Bool,
            Primitive::Dictionary(_) => Type::Dictionary,
            Primitive::List(_) => Type::List,
            Primitive::TypePrimitive(_) => Type::Type,
            Primitive::Void => Type::Void,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Type::String => write!(f, "str"),
            Type::Number => write!(f, "num"),
            Type::Bool => write!(f, "bool"),
            Type::Dictionary => write!(f, "dict"),
            Type::List => write!(f, "list"),
            Type::Type => write!(f, "type"),
            Type::Void => write!(f, "void"),
        }
    }
}