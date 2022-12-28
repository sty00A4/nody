use crate::*;

#[derive(Clone)]
pub struct Function {
    pub params: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: NodeRef
}
impl Function {
    pub fn type_params(&self) -> Vec<Type> {
        let mut types: Vec<Type> = vec![];
        for (_, typ) in self.params.iter() { types.push(typ.clone()); }
        types
    }
    pub fn return_type_boxed(&self) -> Option<Box<Type>> {
        if let Some(t) = &self.return_type { Some(Box::new(t.clone())) } else { None }
    }
    pub fn pattern_match(&self, pattern: &Vec<Type>) -> bool {
        if self.params.len() != pattern.len() { return false }
        for i in 0..pattern.len() {
            if &pattern[i] != &self.params[i].1 {
                return false
            }
        }
        true
    }
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
#[derive(Clone)]
pub struct NativFunction {
    pub params: Vec<(String, Type)>,
    pub return_type: Option<Type>,
    pub body: fn(&mut Context) -> Result<Option<Value>, Error>
}
impl NativFunction {
    pub fn type_params(&self) -> Vec<Type> {
        let mut types: Vec<Type> = vec![];
        for (_, typ) in self.params.iter() { types.push(typ.clone()); }
        types
    }
    pub fn return_type_boxed(&self) -> Option<Box<Type>> {
        if let Some(t) = &self.return_type { Some(Box::new(t.clone())) } else { None }
    }
    pub fn pattern_match(&self, pattern: &Vec<Type>) -> bool {
        if self.params.len() != pattern.len() { return false }
        for i in 0..pattern.len() {
            if &pattern[i] != &self.params[i].1 {
                return false
            }
        }
        true
    }
}
impl Debug for NativFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nativ-fn({})", self.params.iter().map(|(_, typ)| typ.to_string()).collect::<Vec<String>>().join(" "))
    }
}
impl Display for NativFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nativ-fn({})", self.params.iter().map(|(_, typ)| typ.to_string()).collect::<Vec<String>>().join(" "))
    }
}
impl PartialEq for NativFunction {
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
    Int(i64), Float(f64),
    Char(char), Bool(bool), String(String), Vector(Vec<Value>, Option<Type>),
    Function(Function), NativFunction(NativFunction), Object(Scope),
    Type(Type)
}
impl Value {
    pub fn typ(&self) -> Type {
        match self {
            Self::Int(_)           => Type::Int,
            Self::Float(_)         => Type::Float,
            Self::Char(_)          => Type::Char,
            Self::Bool(_)          => Type::Bool,
            Self::String(_)        => Type::String,
            Self::Vector(_, t)     => if let Some(t) = t { Type::Vector(Some(Box::new(t.clone()))) } else { Type::Vector(None) }
            Self::Function(f)      => Type::Function(f.type_params(), f.return_type_boxed()),
            Self::NativFunction(f) => Type::NativFunction(f.type_params(), f.return_type_boxed()),
            Self::Object(_)        => Type::Object,
            Self::Type(_)          => Type::Type
        }
    }
}
impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Int(v)           => v.to_string(),
            Self::Float(v)         => v.to_string(),
            Self::Char(v)          => v.to_string(),
            Self::Bool(v)          => v.to_string(),
            Self::String(v)        => v.to_string(),
            Self::Vector(v, _)     => format!("{v:?}"),
            Self::Function(v)      => v.to_string(),
            Self::NativFunction(v) => v.to_string(),
            Self::Object(_)        => "obj".to_string(),
            Self::Type(v)          => v.to_string()
        })
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Int(v)           => v.to_string(),
            Self::Float(v)         => v.to_string(),
            Self::Char(v)          => v.to_string(),
            Self::Bool(v)          => v.to_string(),
            Self::String(v)        => v.to_string(),
            Self::Vector(v, _)     => format!("{v:?}"),
            Self::Function(v)      => v.to_string(),
            Self::NativFunction(v) => v.to_string(),
            Self::Object(_)        => "obj".to_string(),
            Self::Type(v)          => v.to_string()
        })
    }
}
#[derive(Clone, PartialEq)]
pub enum Type {
    Int, Float,
    Char, Bool, String, Vector(Option<Box<Type>>),
    Function(Vec<Type>, Option<Box<Type>>), NativFunction(Vec<Type>, Option<Box<Type>>), Object,
    Type
}
impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Int                 => "int".to_string(),
            Self::Float               => "float".to_string(),
            Self::Char                => "char".to_string(),
            Self::Bool                => "bool".to_string(),
            Self::String              => "str".to_string(),
            Self::Vector(t)           => if let Some(t) = t { format!("vec<{t:?}>") } else { format!("vec") }
            Self::Function(p, r)      => format!("fn({p:?})"),
            Self::NativFunction(p, r) => format!("nativ-fn({p:?})"),
            Self::Object              => "obj".to_string(),
            Self::Type                => "type".to_string()
        })
    }
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Int                 => "int".to_string(),
            Self::Float               => "float".to_string(),
            Self::Char                => "char".to_string(),
            Self::Bool                => "bool".to_string(),
            Self::String              => "str".to_string(),
            Self::Vector(t)           => if let Some(t) = t { format!("vec<{t}>") } else { format!("vec") }
            Self::Function(t, _)      => format!("fn({})", t.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::NativFunction(t, _) => format!("nativ-fn({})", t.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::Object              => "obj".to_string(),
            Self::Type                => "type".to_string()
        })
    }
}