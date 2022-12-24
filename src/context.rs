use crate::*;

#[derive(Debug, Clone)]
pub struct Scope {
    vars: HashMap<String, (Value, bool, Position)>,
    funcs: HashMap<String, Vec<(Function, Position)>>,
    nativ_funcs: HashMap<String, Vec<(NativFunction, Position)>>,
    subs: HashMap<String, Scope>
}
impl Scope {
    pub fn new() -> Self {
        Scope {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            nativ_funcs: HashMap::new(),
            subs: HashMap::new()
        }
    }
    // create
    pub fn create_var(&mut self, id: String, value: Value, mutable: bool, pos: Position) -> Result<(), Error> {
        if self.vars.contains_key(&id) { return Err(Error::AlreadyDefined(id)) }
        self.vars.insert(id, (value, mutable, pos));
        Ok(())
    }
    pub fn create_fn(&mut self, id: String, func: Function, pos: Position) -> Result<(), Error> {
        match self.get_fn_mut(&id, &func.type_params()) {
            None => if self.funcs.contains_key(&id) {
                match self.funcs.get_mut(&id) {
                    Some(funcs) => {
                        funcs.push((func, pos));
                        Ok(())
                    }
                    None => {
                        self.funcs.insert(id, vec![(func, pos)]);
                        Ok(())
                    }
                }
            } else {
                self.funcs.insert(id, vec![(func, pos)]);
                Ok(())
            }
            Some(_) => Err(Error::AlreadyDefined(id))
        }
    }
    pub fn create_nativ_fn(&mut self, id: String, func: NativFunction, pos: Position) -> Result<(), Error> {
        match self.get_nativ_fn_mut(&id, &func.type_params()) {
            None => if self.nativ_funcs.contains_key(&id) {
                match self.nativ_funcs.get_mut(&id) {
                    Some(defs) => {
                        defs.push((func, pos));
                        Ok(())
                    }
                    None => {
                        self.nativ_funcs.insert(id, vec![(func, pos)]);
                        Ok(())
                    }
                }
            } else {
                self.nativ_funcs.insert(id, vec![(func, pos)]);
                Ok(())
            }
            Some(_) => Err(Error::AlreadyDefined(id))
        }
    }

    pub fn del_var(&mut self, id: &String) -> Option<(Value, bool, Position)> {
        self.vars.remove(id)
    }

    pub fn change(&mut self, id: String, value: Value) -> Result<(), Error> {
        match self.vars.get_mut(&id) {
            Some((old_value, mutable, pos)) => if *mutable {
                *old_value = value;
                Ok(())
            } else {
                Err(Error::Immutable(id))
            }
            None => Err(Error::NotDefined(id))
        }
    }
    // get var
    pub fn get_var(&self, id: &String) -> Option<&Value> {
        match self.vars.get(id) {
            Some((value, _, _)) => Some(value),
            None => None
        }
    }
    pub fn is_mutable(&self, id: &String) -> Option<bool> {
        match self.vars.get(id) {
            Some((_, mutable, _)) => Some(*mutable),
            None => None
        }
    }
    pub fn get_var_mut(&mut self, id: &String) -> Option<&mut Value> {
        match self.vars.get_mut(id) {
            Some((value, _, _)) => Some(value),
            None => None
        }
    }
    // get fn
    pub fn get_fn(&self, id: &String, pattern: &Vec<Type>) -> Option<&Function> {
        match self.funcs.get(id) {
            Some(defs) => todo!("get_fn"),
            None => None
        }
    }
    pub fn get_fn_mut(&mut self, id: &String, pattern: &Vec<Type>) -> Option<&mut Function> {
        match self.funcs.get_mut(id) {
            Some(defs) => todo!("get_fn_mut"),
            None => None
        }
    }
    pub fn get_fn_first(&self, id: &String) -> Option<&Function> {
        match self.funcs.get(id) {
            Some(defs) => Some(&defs.first().unwrap().0),
            None => None
        }
    }
    pub fn get_fn_first_mut(&mut self, id: &String) -> Option<&mut Function> {
        match self.funcs.get_mut(id) {
            Some(defs) => Some(&mut defs.first_mut().unwrap().0),
            None => None
        }
    }
    // get nativ fn
    pub fn get_nativ_fn(&self, id: &String, pattern: &Vec<Type>) -> Option<&NativFunction> {
        match self.nativ_funcs.get(id) {
            Some(defs) => todo!("get_nativ_fn"),
            None => None
        }
    }
    pub fn get_nativ_fn_mut(&mut self, id: &String, pattern: &Vec<Type>) -> Option<&mut NativFunction> {
        match self.nativ_funcs.get_mut(id) {
            Some(defs) => todo!("get_nativ_fn_mut"),
            None => None
        }
    }
    pub fn get_nativ_fn_first(&self, id: &String) -> Option<&NativFunction> {
        match self.nativ_funcs.get(id) {
            Some(defs) => Some(&defs.first().unwrap().0),
            None => None
        }
    }
    pub fn get_nativ_fn_first_mut(&mut self, id: &String) -> Option<&mut NativFunction> {
        match self.nativ_funcs.get_mut(id) {
            Some(defs) => Some(&mut defs.first_mut().unwrap().0),
            None => None
        }
    }
}
impl PartialEq for Scope {
    fn eq(&self, other: &Self) -> bool { false }
}
#[derive(Debug, Clone)]
pub struct Context {
    pub scopes: Vec<Scope>,
    pub global: Scope,
    pub trace: Vec<Position>
}
impl Context {
    pub fn new() -> Self { Self { scopes: vec![Scope::new()], global: Scope::new(), trace: vec![] } }
    pub fn push(&mut self) { self.scopes.push(Scope::new()) }
    pub fn pop(&mut self) -> Option<Scope> { self.scopes.pop() }
    pub fn trace_push(&mut self, pos: &Position) { self.trace.push(pos.clone()); }
    pub fn trace_pop(&mut self) -> Option<Position> { self.trace.pop() }
    // scope of var
    pub fn get_scope_var(&self, id: &String) -> Option<&Scope> {
        for scope in self.scopes.iter() {
            if scope.get_var(id).is_some() { return Some(scope) }
        }
        if self.global.get_var(id).is_some() { return Some(&self.global) }
        None
    }
    pub fn get_scope_var_mut(&mut self, id: &String) -> Option<&mut Scope> {
        for scope in self.scopes.iter_mut() {
            if scope.get_var(id).is_some() { return Some(scope) }
        }
        if self.global.get_var(id).is_some() { return Some(&mut self.global) }
        None
    }
    // scope of fn
    pub fn get_scope_fn(&self, id: &String, pattern: &Vec<Type>) -> Option<&Scope> {
        for scope in self.scopes.iter() {
            if scope.get_fn(id, pattern).is_some() { return Some(scope) }
        }
        if self.global.get_fn(id, pattern).is_some() { return Some(&self.global) }
        None
    }
    pub fn get_scope_fn_mut(&mut self, id: &String, pattern: &Vec<Type>) -> Option<&mut Scope> {
        for scope in self.scopes.iter_mut() {
            if scope.get_fn_mut(id, pattern).is_some() { return Some(scope) }
        }
        if self.global.get_fn_mut(id, pattern).is_some() { return Some(&mut self.global) }
        None
    }
    pub fn get_scope_fn_first(&self, id: &String) -> Option<&Scope> {
        for scope in self.scopes.iter() {
            if scope.get_fn_first(id).is_some() { return Some(scope) }
        }
        if self.global.get_fn_first(id).is_some() { return Some(&self.global) }
        None
    }
    pub fn get_scope_fn_first_mut(&mut self, id: &String) -> Option<&mut Scope> {
        for scope in self.scopes.iter_mut() {
            if scope.get_fn_first_mut(id).is_some() { return Some(scope) }
        }
        if self.global.get_fn_first_mut(id).is_some() { return Some(&mut self.global) }
        None
    }
    // scope of nativ fn
    pub fn get_scope_nativ_fn(&self, id: &String, pattern: &Vec<Type>) -> Option<&Scope> {
        for scope in self.scopes.iter() {
            if scope.get_nativ_fn(id, pattern).is_some() { return Some(scope) }
        }
        if self.global.get_nativ_fn(id, pattern).is_some() { return Some(&self.global) }
        None
    }
    pub fn get_scope_nativ_fn_mut(&mut self, id: &String, pattern: &Vec<Type>) -> Option<&mut Scope> {
        for scope in self.scopes.iter_mut() {
            if scope.get_nativ_fn_mut(id, pattern).is_some() { return Some(scope) }
        }
        if self.global.get_nativ_fn_mut(id, pattern).is_some() { return Some(&mut self.global) }
        None
    }
    pub fn get_scope_nativ_fn_first(&self, id: &String) -> Option<&Scope> {
        for scope in self.scopes.iter() {
            if scope.get_nativ_fn_first(id).is_some() { return Some(scope) }
        }
        if self.global.get_nativ_fn_first(id).is_some() { return Some(&self.global) }
        None
    }
    pub fn get_scope_nativ_fn_first_mut(&mut self, id: &String) -> Option<&mut Scope> {
        for scope in self.scopes.iter_mut() {
            if scope.get_nativ_fn_first_mut(id).is_some() { return Some(scope) }
        }
        if self.global.get_nativ_fn_first_mut(id).is_some() { return Some(&mut self.global) }
        None
    }
    
    // create
    pub fn create_var(&mut self, id: String, value: Value, mutable: bool, pos: Position) -> Result<(), Error> {
        match self.get_scope_var_mut(&id) {
            None => self.scopes.last_mut().unwrap().create_var(id, value, mutable, pos),
            Some(_) => Err(Error::AlreadyDefined(id))
        }
    }
    pub fn create_fn(&mut self, id: String, func: Function, pos: Position) -> Result<(), Error> {
        match self.get_scope_fn_mut(&id, &func.type_params()) {
            None => self.scopes.last_mut().unwrap().create_fn(id, func, pos),
            Some(scope) => scope.create_fn(id, func, pos)
        }
    }
    pub fn create_nativ_fn(&mut self, id: String, func: NativFunction, pos: Position) -> Result<(), Error> {
        self.global.create_nativ_fn(id, func, pos)
    }
    
    pub fn del_var(&mut self, id: &String) -> Option<(Value, bool, Position)> {
        self.get_scope_var_mut(&id)?.del_var(id)
    }
    
    pub fn change(&mut self, id: String, value: Value) -> Result<(), Error> {
        match self.global.vars.get_mut(&id) {
            Some((old_value, mutable, pos)) => if *mutable {
                *old_value = value;
                return Ok(())
            } else {
                return Err(Error::Immutable(id))
            }
            None => {}
        }
        for scope in self.scopes.iter_mut().rev() {
            match scope.vars.get_mut(&id) {
                Some((old_value, mutable, pos)) => if *mutable {
                    *old_value = value;
                    return Ok(())
                } else {
                    return Err(Error::Immutable(id))
                }
                None => {}
            }
        }
        Err(Error::NotDefined(id))
    }
    // get var
    pub fn get_var(&self, id: &String) -> Option<&Value> {
        match self.get_scope_var(id) {
            Some(scope) => scope.get_var(id),
            None => self.global.get_var(id)
        }
    }
    pub fn is_mutable(&self, id: &String) -> Option<bool> {
        match self.get_scope_var(id) {
            Some(scope) => scope.is_mutable(id),
            None => self.global.is_mutable(id)
        }
    }
    pub fn get_var_mut(&mut self, id: &String) -> Option<&mut Value> {
        for scope in self.scopes.iter_mut() {
            match scope.get_var_mut(id) {
                Some(value) => return Some(value),
                None => {}
            }
        }
        match self.global.get_var_mut(id) {
            Some(value) => Some(value),
            None => None
        }
    }
    // get fn
    pub fn get_fn(&self, id: &String, pattern: &Vec<Type>) -> Option<&Function> {
        self.get_scope_fn(id, pattern)?.get_fn(id, pattern)
    }
    pub fn get_fn_mut(&mut self, id: &String, pattern: &Vec<Type>) -> Option<&mut Function> {
        self.get_scope_fn_mut(id, pattern)?.get_fn_mut(id, pattern)
    }
    pub fn get_fn_first(&self, id: &String) -> Option<&Function> {
        self.get_scope_fn_first(id)?.get_fn_first(id)
    }
    pub fn get_fn_first_mut(&mut self, id: &String) -> Option<&mut Function> {
        self.get_scope_fn_first_mut(id)?.get_fn_first_mut(id)
    }
    // get nativ fn
    pub fn get_nativ_fn(&self, id: &String, pattern: &Vec<Type>) -> Option<&NativFunction> {
        self.get_scope_nativ_fn(id, pattern)?.get_nativ_fn(id, pattern)
    }
    pub fn get_nativ_fn_mut(&mut self, id: &String, pattern: &Vec<Type>) -> Option<&mut NativFunction> {
        self.get_scope_nativ_fn_mut(id, pattern)?.get_nativ_fn_mut(id, pattern)
    }
    pub fn get_nativ_fn_first(&self, id: &String) -> Option<&NativFunction> {
        self.get_scope_nativ_fn_first(id)?.get_nativ_fn_first(id)
    }
    pub fn get_nativ_fn_first_mut(&mut self, id: &String) -> Option<&mut NativFunction> {
        self.get_scope_nativ_fn_first_mut(id)?.get_nativ_fn_first_mut(id)
    }
}

fn _add_int(context: &mut Context) -> Result<Option<Value>, Error> {
    let a = context.get_var(&"a".to_string()).unwrap();
    let b = context.get_var(&"b".to_string()).unwrap();
    if let Value::Int(a) = a {
        if let Value::Int(b) = b {
            Ok(Some(Value::Int(*a + *b)))
        } else { panic!("type checking doesn't work"); }
    } else { panic!("type checking doesn't work"); }
}
fn _add_float(context: &mut Context) -> Result<Option<Value>, Error> {
    let a = context.get_var(&"a".to_string()).unwrap();
    let b = context.get_var(&"b".to_string()).unwrap();
    if let Value::Float(a) = a {
        if let Value::Float(b) = b {
            Ok(Some(Value::Float(*a + *b)))
        } else { panic!("type checking doesn't work"); }
    } else { panic!("type checking doesn't work"); }
}
fn _add_str(context: &mut Context) -> Result<Option<Value>, Error> {
    let a = context.get_var(&"a".to_string()).unwrap();
    let b = context.get_var(&"b".to_string()).unwrap();
    if let Value::String(a) = a {
        if let Value::String(b) = b {
            Ok(Some(Value::String(a.clone() + b)))
        } else { panic!("type checking doesn't work"); }
    } else { panic!("type checking doesn't work"); }
}
pub fn std_context() -> Context {
    let mut context = Context::new();
    let pos = Position::new(0..0, 0..0, &String::from("<STD>"));
    context.create_nativ_fn(String::from("+"), NativFunction {
        params: vec![("a".to_string(), Type::Int), ("b".to_string(), Type::Int)],
        return_type: Some(Type::Int),
        body: _add_int
    }, pos.clone());
    context.create_nativ_fn(String::from("+"), NativFunction {
        params: vec![("a".to_string(), Type::Float), ("b".to_string(), Type::Float)],
        return_type: Some(Type::Float),
        body: _add_float
    }, pos.clone());
    context.create_nativ_fn(String::from("+"), NativFunction {
        params: vec![("a".to_string(), Type::String), ("b".to_string(), Type::String)],
        return_type: Some(Type::String),
        body: _add_str
    }, pos.clone());
    context
}