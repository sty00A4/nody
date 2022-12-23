use crate::*;

#[derive(Clone)]
pub struct Function {
    pub params: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: NodeRef
}
impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn({})", self.params.iter().map(|(_, typ)| typ.to_string()).collect::<Vec<String>>().join(" "))
    }
}
impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn({})", self.params.iter().map(|(_, typ)| typ.to_string()).collect::<Vec<String>>().join(" "))
    }
}
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        if self.params.len() != other.params.len() { return false }
        for i in 0..self.params.len() {
            if self.params[i].1 != other.params[i].1 { return false}
        }
        true
    }
}
#[derive(Clone, PartialEq)]
pub enum Value {
    Int8(i8), Int16(i16), Int32(i32), Int64(i64), Int128(i128), Int(isize),
    UInt8(u8), UInt16(u16), UInt32(u32), UInt64(u64), UInt128(u128), UInt(usize),
    Char(char), Bool(bool), String(String), Vector(Vec<Value>, Option<Type>),
    Function(Function),
    Type(Type)
}
impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Int(v)        => v.to_string(),
            Self::Int8(v)       => v.to_string(),
            Self::Int16(v)      => v.to_string(),
            Self::Int32(v)      => v.to_string(),
            Self::Int64(v)      => v.to_string(),
            Self::Int128(v)     => v.to_string(),
            Self::UInt(v)       => v.to_string(),
            Self::UInt8(v)      => v.to_string(),
            Self::UInt16(v)     => v.to_string(),
            Self::UInt32(v)     => v.to_string(),
            Self::UInt64(v)     => v.to_string(),
            Self::UInt128(v)    => v.to_string(),
            Self::Char(v)       => v.to_string(),
            Self::Bool(v)       => v.to_string(),
            Self::String(v)     => v.to_string(),
            Self::Vector(v, _)  => format!("{v:?}"),
            Self::Function(v)   => v.to_string(),
            Self::Type(v)       => v.to_string()
        })
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Int(v)        => v.to_string(),
            Self::Int8(v)       => v.to_string(),
            Self::Int16(v)      => v.to_string(),
            Self::Int32(v)      => v.to_string(),
            Self::Int64(v)      => v.to_string(),
            Self::Int128(v)     => v.to_string(),
            Self::UInt(v)       => v.to_string(),
            Self::UInt8(v)      => v.to_string(),
            Self::UInt16(v)     => v.to_string(),
            Self::UInt32(v)     => v.to_string(),
            Self::UInt64(v)     => v.to_string(),
            Self::UInt128(v)    => v.to_string(),
            Self::Char(v)       => v.to_string(),
            Self::Bool(v)       => v.to_string(),
            Self::String(v)     => v.to_string(),
            Self::Vector(v, _)  => format!("{v:?}"),
            Self::Function(v)   => v.to_string(),
            Self::Type(v)       => v.to_string()
        })
    }
}
#[derive(Clone, PartialEq)]
pub enum Type {
    Int8, Int16, Int32, Int64, Int128, Int,
    UInt8, UInt16, UInt32, UInt64, UInt128, UInt,
    Char, Bool, String, Vector(Option<Box<Type>>),
    Function(Vec<Type>),
    Type
}
impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Int           => "int".to_string(),
            Self::Int8          => "int8".to_string(),
            Self::Int16         => "int16".to_string(),
            Self::Int32         => "int32".to_string(),
            Self::Int64         => "int64".to_string(),
            Self::Int128        => "int128".to_string(),
            Self::UInt          => "uint".to_string(),
            Self::UInt8         => "uint8".to_string(),
            Self::UInt16        => "uint16".to_string(),
            Self::UInt32        => "uint32".to_string(),
            Self::UInt64        => "uint64".to_string(),
            Self::UInt128       => "uint128".to_string(),
            Self::Char          => "char".to_string(),
            Self::Bool          => "bool".to_string(),
            Self::String        => "str".to_string(),
            Self::Vector(t)     => if let Some(t) = t { format!("vec<{t:?}>") } else { format!("vec") }
            Self::Function(p)   => format!("fn({p:?})"),
            Self::Type          => "type".to_string()
        })
    }
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Int           => "int".to_string(),
            Self::Int8          => "int8".to_string(),
            Self::Int16         => "int16".to_string(),
            Self::Int32         => "int32".to_string(),
            Self::Int64         => "int64".to_string(),
            Self::Int128        => "int128".to_string(),
            Self::UInt          => "uint".to_string(),
            Self::UInt8         => "uint8".to_string(),
            Self::UInt16        => "uint16".to_string(),
            Self::UInt32        => "uint32".to_string(),
            Self::UInt64        => "uint64".to_string(),
            Self::UInt128       => "uint128".to_string(),
            Self::Char          => "char".to_string(),
            Self::Bool          => "bool".to_string(),
            Self::String        => "str".to_string(),
            Self::Vector(t)     => if let Some(t) = t { format!("vec<{t}>") } else { format!("vec") }
            Self::Function(t)   => format!("fn({})", t.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::Type          => "type".to_string()
        })
    }
}