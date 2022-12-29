use crate::*;

#[derive(Clone)]
pub struct Function {
    pub params: Vec<(String, Type, bool)>,
    pub return_type: Option<Type>,
    pub body: NodeRef,
    pub inline: bool
}
impl Function {
    /// return the params as a type vector
    pub fn type_params(&self) -> Vec<Type> {
        let mut types: Vec<Type> = vec![];
        for (_, typ, more) in self.params.iter() { types.push(typ.clone()); }
        types
    }
    /// box the return type
    pub fn return_type_boxed(&self) -> Option<Box<Type>> {
        if let Some(t) = &self.return_type { Some(Box::new(t.clone())) } else { None }
    }
    pub fn pattern_match(&self, pattern: &Vec<Type>) -> bool {
        let mut pattern_idx: usize = 0;
        for i in 0..self.params.len() {
            if pattern.get(pattern_idx) == None { return false }
            let (_, param_type, more) = &self.params[i];
            if *more {
                if let Some(typ) = pattern.get(pattern_idx) { // one of the param_typ type has to be here
                    if typ != param_type { return false }
                    pattern_idx += 1;
                } else { return false }
                while let Some(typ) = pattern.get(pattern_idx) { // skip through the rest
                    if typ != param_type { break }
                    pattern_idx += 1;
                }
            } else {
                if &pattern[pattern_idx] != param_type { return false }
                pattern_idx += 1;
            }
        }
        pattern.get(pattern_idx) == None
    }
}
impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn({})", self.params.iter().map(|(_, typ, more)| typ.to_string()).collect::<Vec<String>>().join(" "))
    }
}
impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn({})", self.params.iter().map(|(_, typ, more)| typ.to_string()).collect::<Vec<String>>().join(" "))
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
    pub params: Vec<(String, Type, bool)>,
    pub return_type: Option<Type>,
    pub body: fn(&mut Context) -> Result<Option<Value>, Error>,
    pub inline: bool
}
impl NativFunction {
    pub fn type_params(&self) -> Vec<Type> {
        let mut types: Vec<Type> = vec![];
        for (_, typ, _) in self.params.iter() { types.push(typ.clone()); }
        types
    }
    pub fn return_type_boxed(&self) -> Option<Box<Type>> {
        if let Some(t) = &self.return_type { Some(Box::new(t.clone())) } else { None }
    }
    pub fn pattern_match(&self, pattern: &Vec<Type>) -> bool {
        let mut pattern_idx: usize = 0;
        for i in 0..self.params.len() {
            if pattern.get(pattern_idx) == None { return false }
            let (_, param_type, more) = &self.params[i];
            if *more {
                if &pattern[pattern_idx] != param_type { return false }
                pattern_idx += 1;
                while let Some(typ) = pattern.get(pattern_idx) {
                    if typ != param_type { return false }
                    pattern_idx += 1;
                }
            } else {
                if &pattern[pattern_idx] != param_type { return false }
                pattern_idx += 1;
            }
        }
        pattern.get(pattern_idx) == None
    }
    pub fn params_match(&self, params: &Vec<(String, Type, bool)>) -> bool {
        if self.params.len() != params.len() { return false }
        for i in 0..self.params.len() {
            if params.get(i) == None { return false }
            let (_, param_type1, more1) = &self.params[i];
            let (_, param_type2, more2) = &params[i];
            if param_type1 != param_type2 || more1 != more2 { return false }
        }
        true
    }
}
impl Debug for NativFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "native-fn({})", self.params.iter().map(|(_, typ, more)| typ.to_string()).collect::<Vec<String>>().join(" "))
    }
}
impl Display for NativFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "native-fn({})", self.params.iter().map(|(_, typ, more)| typ.to_string()).collect::<Vec<String>>().join(" "))
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
    Int(i64), Float(f64), Char(char), Bool(bool),
    String(String), Key(String), Closure(Node), Vector(Vec<Value>, Option<Type>),
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
            Self::Key(_)           => Type::Key,
            Self::Closure(_)       => Type::Closure,
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
            Self::Char(v)          => format!("'{v}'"),
            Self::Bool(v)          => v.to_string(),
            Self::String(v)        => format!("{v:?}"),
            Self::Key(v)           => format!("@{v}"),
            Self::Closure(n)       => format!("#{n:?}"),
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
            Self::Key(v)           => format!("@{v}"),
            Self::Closure(n)       => format!("#{n}"),
            Self::Vector(v, _)     => format!("{v:?}"),
            Self::Function(v)      => v.to_string(),
            Self::NativFunction(v) => v.to_string(),
            Self::Object(_)        => "obj".to_string(),
            Self::Type(v)          => v.to_string()
        })
    }
}
#[derive(Clone)]
pub enum Type {
    Any,
    Int, Float, Char, Bool,
    String, Key, Closure, Vector(Option<Box<Type>>),
    Function(Vec<Type>, Option<Box<Type>>), NativFunction(Vec<Type>, Option<Box<Type>>), Object,
    Type
}
impl Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Any                 => "any".to_string(),
            Self::Int                 => "int".to_string(),
            Self::Float               => "float".to_string(),
            Self::Char                => "char".to_string(),
            Self::Bool                => "bool".to_string(),
            Self::String              => "str".to_string(),
            Self::Key                 => "key".to_string(),
            Self::Closure             => "closure".to_string(),
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
            Self::Any                 => "any".to_string(),
            Self::Int                 => "int".to_string(),
            Self::Float               => "float".to_string(),
            Self::Char                => "char".to_string(),
            Self::Bool                => "bool".to_string(),
            Self::String              => "str".to_string(),
            Self::Key                 => "key".to_string(),
            Self::Closure             => "closure".to_string(),
            Self::Vector(t)           => if let Some(t) = t { format!("vec<{t}>") } else { format!("vec") }
            Self::Function(t, _)      => format!("fn({})", t.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::NativFunction(t, _) => format!("nativ-fn({})", t.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Self::Object              => "obj".to_string(),
            Self::Type                => "type".to_string()
        })
    }
}
impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Any, _) => true, (_, Self::Any) => true,
            (Self::Int, Self::Int)          => true,
            (Self::Float, Self::Float)      => true,
            (Self::Char, Self::Char)        => true,
            (Self::Bool, Self::Bool)        => true,
            (Self::String, Self::String)    => true,
            (Self::Key, Self::Key)          => true,
            (Self::Closure, Self::Closure)  => true,
            (Self::Vector(t1), Self::Vector(t2)) => t1 == t2,
            (Self::Function(p1, t1), Self::Function(p2, t2)) => p1 == p2 && t1 == t2,
            (Self::NativFunction(p1, t1), Self::NativFunction(p2, t2)) => p1 == p2 && t1 == t2,
            (Self::Object, Self::Object)    => true,
            (Self::Type, Self::Type)        => true,
            _ => false
        }
    }
}