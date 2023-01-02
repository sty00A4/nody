use crate::*;

pub type Params = Vec<(String, Type, bool)>;
#[derive(Clone)]
pub struct Function {
    pub params: Params,
    pub return_type: Option<Type>,
    pub body: NodeRef,
    pub inline: bool
}
impl Function {
    pub fn new(params: Params, return_type: Option<Type>, body: NodeRef, inline: bool) -> Self {
        Self { params, return_type, body, inline }
    }
    pub fn type_params(&self) -> Vec<Type> {
        let mut types: Vec<Type> = vec![];
        for (_, typ, more) in self.params.iter() { types.push(typ.clone()); }
        types
    }
    pub fn get_pattern(&self) -> Vec<(Type, bool)> {
        let mut types: Vec<(Type, bool)> = vec![];
        for (_, typ, more) in self.params.iter() { types.push((typ.clone(), *more)); }
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
    pub fn params_match(&self, params: &Params) -> bool {
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
pub type NativeFunctionType = fn(&mut Context) -> Result<(Option<Value>, Return), Error>;
#[derive(Clone)]
pub struct NativFunction {
    pub params: Params,
    pub return_type: Option<Type>,
    pub body: NativeFunctionType,
    pub inline: bool
}
impl NativFunction {
    pub fn new(params: Params, return_type: Option<Type>, body: NativeFunctionType, inline: bool) -> Self {
        Self { params, return_type, body, inline }
    }
    pub fn type_params(&self) -> Vec<Type> {
        let mut types: Vec<Type> = vec![];
        for (_, typ, _) in self.params.iter() { types.push(typ.clone()); }
        types
    }
    pub fn get_pattern(&self) -> Vec<(Type, bool)> {
        let mut types: Vec<(Type, bool)> = vec![];
        for (_, typ, more) in self.params.iter() { types.push((typ.clone(), *more)); }
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
    pub fn params_match(&self, params: &Params) -> bool {
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
pub enum PathWays { Key(String), Path(Box<Path>), Index(Box<Index>) }
impl Debug for PathWays {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(key) => write!(f, "@{}", key),
            Self::Path(path) => write!(f, "{:?}", path),
            Self::Index(index) => write!(f, "{:?}", index),
        }
    }
}
impl Display for PathWays {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(key) => write!(f, "@{}", key),
            Self::Path(path) => write!(f, "{}", path),
            Self::Index(index) => write!(f, "{}", index),
        }
    }
}
#[derive(Clone, PartialEq)]
pub struct Path {
    pub head: PathWays,
    pub sub: String,
}
impl Path {
    pub fn new(head: PathWays, sub: String) -> Self { Self { head, sub } }
    pub fn get_head<'a>(&'a self, context: &'a mut Context) -> Result<Option<&'a Value>, Error> {
        match &self.head {
            PathWays::Key(key) => Ok(context.get_var(&key)),
            PathWays::Path(path) => path.get(context),
            PathWays::Index(index) => index.get(context)
        }
    }
    pub fn get<'a>(&'a self, context: &'a mut Context) -> Result<Option<&'a Value>, Error> {
        let sub = self.sub.clone();
        match self.get_head(context)? {
            Some(value) => match value {
                Value::Object(scope) => match scope.get_var(&sub) {
                    Some(value) => Ok(Some(value)),
                    None => Ok(None)
                }
                _ => {
                    Err(Error::ExpectedType(Type::Object, value.typ()))
                }
            }
            _ => Ok(None)
        }
    }
    pub fn get_head_mut<'a>(&'a mut self, context: &'a mut Context) -> Result<Option<&'a mut Value>, Error> {
        match &mut self.head {
            PathWays::Key(key) => Ok(context.get_var_mut(&key)),
            PathWays::Path(path) => path.get_mut(context),
            PathWays::Index(index) => index.get_mut(context)
        }
    }
    pub fn get_mut<'a>(&'a mut self, context: &'a mut Context) -> Result<Option<&'a mut Value>, Error> {
        let sub = self.sub.clone();
        match self.get_head_mut(context)? {
            Some(value) => match value {
                Value::Object(scope) => match scope.get_var_mut(&sub) {
                    Some(value) => Ok(Some(value)),
                    None => Ok(None)
                }
                _ => {
                    Err(Error::ExpectedType(Type::Object, value.typ()))
                }
            }
            _ => Ok(None)
        }
    }
    pub fn is_mutable(&self, context: &mut Context) -> Result<Option<bool>, Error> {
        match &self.head {
            PathWays::Key(key) => Ok(context.is_mutable(key)),
            PathWays::Path(path) => path.is_mutable(context),
            PathWays::Index(index) => index.is_mutable(context)
        }
    }
}
impl Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}", self.head, self.sub)
    }
}
impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
#[derive(Clone, PartialEq)]
pub struct Index {
    pub head: PathWays,
    pub idx: usize
}
impl Index {
    pub fn new(head: PathWays, idx: usize) -> Self { Self { head, idx } }
    pub fn get_head<'a>(&'a self, context: &'a mut Context) -> Result<Option<&'a Value>, Error> {
        match &self.head {
            PathWays::Key(key) => Ok(context.get_var(&key)),
            PathWays::Path(path) => path.get(context),
            PathWays::Index(index) => index.get(context)
        }
    }
    pub fn get<'a>(&'a self, context: &'a mut Context) -> Result<Option<&'a Value>, Error> {
        let idx = self.idx;
        match self.get_head(context)? {
            Some(value) =>  match value {
                Value::Vector(values, _) => {
                    let len = values.len();
                    match values.get(idx) {
                        Some(value) => Ok(Some(value)),
                        None => {
                            Err(Error::IndexOutOfRange(idx, len))
                        }
                    }
                }
                _ => {
                    Err(Error::ExpectedType(Type::Vector(None), value.typ()))
                }
            }
            _ => Ok(None)
        }
    }
    pub fn get_head_mut<'a>(&'a mut self, context: &'a mut Context) -> Result<Option<&'a mut Value>, Error> {
        match &mut self.head {
            PathWays::Key(key) => Ok(context.get_var_mut(&key)),
            PathWays::Path(path) => path.get_mut(context),
            PathWays::Index(index) => index.get_mut(context)
        }
    }
    pub fn get_mut<'a>(&'a mut self, context: &'a mut Context) -> Result<Option<&'a mut Value>, Error> {
        let idx = self.idx;
        match self.get_head_mut(context)? {
            Some(value) =>  match value {
                Value::Vector(values, _) => {
                    let len = values.len();
                    match values.get_mut(idx) {
                        Some(value) => Ok(Some(value)),
                        None => {
                            Err(Error::IndexOutOfRange(idx, len))
                        }
                    }
                }
                _ => {
                    Err(Error::ExpectedType(Type::Vector(None), value.typ()))
                }
            }
            _ => Ok(None)
        }
    }
    pub fn is_mutable(&self, context: &mut Context) -> Result<Option<bool>, Error> {
        match &self.head {
            PathWays::Key(key) => Ok(context.is_mutable(key)),
            PathWays::Path(path) => path.is_mutable(context),
            PathWays::Index(index) => index.is_mutable(context)
        }
    }
}
impl Debug for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}[{:?}]", self.head, self.idx)
    }
}
impl Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.head, self.idx)
    }
}
#[derive(Clone, PartialEq)]
pub enum Value {
    Int(i64), Float(f64), Char(char), Bool(bool),
    String(String), Vector(Vec<Value>, Option<Type>),
    Key(String), Path(Path), Index(Index),
    Closure(Node), Params(Params),
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
            Self::Key(_)           => Type::Key,
            Self::Path(_)          => Type::Path,
            Self::Index(_)         => Type::Index,
            Self::Closure(_)       => Type::Closure,
            Self::Params(_)        => Type::Params,
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
            Self::Vector(v, _)     => format!("{v:?}"),
            Self::Key(v)           => format!("@{v}"),
            Self::Path(v)          => format!("{v}"),
            Self::Index(v)         => format!("{v}"),
            Self::Closure(n)       => format!("#{n:?}"),
            Self::Params(params)   => format!("$({})",
            params.iter().map(|(id, typ, more)|
                format!("{} {}{}", id, typ, if *more { "*" } else { "" }))
            .collect::<Vec<String>>().join(" ")),
            Self::Function(v)      => v.to_string(),
            Self::NativFunction(v) => v.to_string(),
            Self::Object(scope)    => format!("{{ {} }}", scope.vars.iter().map(|(key, (value, _, _))|format!("{key}={value:?}"))
            .collect::<Vec<String>>().join(" ")),
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
            Self::Key(v)           => format!("@{v}"),
            Self::Path(v)          => format!("{v:?}"),
            Self::Index(v)         => format!("{v:?}"),
            Self::Closure(n)       => format!("#{n}"),
            Self::Params(params)   => format!("$({})",
            params.iter().map(|(id, typ, more)|
                format!("{} {}{}", id, typ, if *more { "*" } else { "" }))
            .collect::<Vec<String>>().join(" ")),
            Self::Function(v)      => v.to_string(),
            Self::NativFunction(v) => v.to_string(),
            Self::Object(scope)    => format!("{{ {} }}", scope.vars.iter().map(|(key, (value, _, _))| format!("{key}={value:?}"))
            .collect::<Vec<String>>().join(" ")),
            Self::Type(v)          => v.to_string()
        })
    }
}
#[derive(Clone)]
pub enum Type {
    Any,
    Int, Float, Char, Bool,
    String, Vector(Option<Box<Type>>),
    Key, Path, Index,
    Closure, Params,
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
            Self::Vector(t)           => if let Some(t) = t { format!("vec<{t:?}>") } else { format!("vec") }
            Self::Key                 => "key".to_string(),
            Self::Path                => "path".to_string(),
            Self::Index               => "index".to_string(),
            Self::Closure             => "closure".to_string(),
            Self::Params              => "params".to_string(),
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
            Self::Vector(t)           => if let Some(t) = t { format!("vec<{t}>") } else { format!("vec") }
            Self::Key                 => "key".to_string(),
            Self::Path                => "path".to_string(),
            Self::Index               => "index".to_string(),
            Self::Closure             => "closure".to_string(),
            Self::Params              => "params".to_string(),
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
            (Self::Vector(t1), Self::Vector(t2)) => match t1 {
                Some(t1) => match t1.as_ref() {
                    Type::Any => match t2 {
                        Some(t2) => match t2.as_ref() {
                            Type::Any => true,
                            _ => false
                        }
                        None => false
                    }
                    _ => match t2 {
                        Some(t2) => t1 == t2,
                        None => true
                    }
                }
                None => true
            }
            (Self::Key, Self::Key)          => true,
            (Self::Path, Self::Path)        => true,
            (Self::Index, Self::Index)      => true,
            (Self::Closure, Self::Closure)  => true,
            (Self::Params, Self::Params)    => true,
            (Self::Function(p1, t1), Self::Function(p2, t2)) => p1 == p2 && t1 == t2,
            (Self::NativFunction(p1, t1), Self::NativFunction(p2, t2)) => p1 == p2 && t1 == t2,
            (Self::Object, Self::Object)    => true,
            (Self::Type, Self::Type)        => true,
            _ => false
        }
    }
}