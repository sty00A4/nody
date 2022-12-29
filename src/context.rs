use crate::*;

#[derive(Debug, Clone)]
pub struct Scope {
    vars: HashMap<String, (Value, bool, Position)>,
    funcs: HashMap<String, Vec<(Function, Position)>>,
    native_funcs: HashMap<String, Vec<(NativFunction, Position)>>,
    subs: HashMap<String, Scope>
}
impl Scope {
    pub fn new() -> Self {
        Scope {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            native_funcs: HashMap::new(),
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
    pub fn create_native_fn(&mut self, id: String, func: NativFunction, pos: Position) -> Result<(), Error> {
        match self.get_native_fn_params_mut(&id, &func.params) {
            None => if self.native_funcs.contains_key(&id) {
                match self.native_funcs.get_mut(&id) {
                    Some(defs) => {
                        defs.push((func, pos));
                        Ok(())
                    }
                    None => {
                        self.native_funcs.insert(id, vec![(func, pos)]);
                        Ok(())
                    }
                }
            } else {
                self.native_funcs.insert(id, vec![(func, pos)]);
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
            Some(defs) => {
                for (func, _) in defs.iter() {
                    if func.pattern_match(pattern) {
                        return Some(func)
                    }
                }
                None
            }
            None => None
        }
    }
    pub fn get_fn_mut(&mut self, id: &String, pattern: &Vec<Type>) -> Option<&mut Function> {
        match self.funcs.get_mut(id) {
            Some(defs) => {
                for (func, _) in defs.iter_mut() {
                    if func.pattern_match(pattern) {
                        return Some(func)
                    }
                }
                None
            }
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
    pub fn fn_exists(&self, id: &String) -> bool {
        self.funcs.contains_key(id)
    }
    // get native fn
    pub fn get_native_fn(&self, id: &String, pattern: &Vec<Type>) -> Option<&NativFunction> {
        match self.native_funcs.get(id) {
            Some(defs) => {
                for (func, _) in defs.iter() {
                    if func.pattern_match(pattern) {
                        return Some(func)
                    }
                }
                None
            }
            None => None
        }
    }
    pub fn get_native_fn_mut(&mut self, id: &String, pattern: &Vec<Type>) -> Option<&mut NativFunction> {
        match self.native_funcs.get_mut(id) {
            Some(defs) => {
                for (func, _) in defs.iter_mut() {
                    if func.pattern_match(pattern) {
                        return Some(func)
                    }
                }
                None
            }
            None => None
        }
    }
    pub fn get_native_fn_params(&self, id: &String, params: &Vec<(String, Type, bool)>) -> Option<&NativFunction> {
        match self.native_funcs.get(id) {
            Some(defs) => {
                for (func, _) in defs.iter() {
                    if func.params_match(params) {
                        return Some(func)
                    }
                }
                None
            }
            None => None
        }
    }
    pub fn get_native_fn_params_mut(&mut self, id: &String, params: &Vec<(String, Type, bool)>) -> Option<&mut NativFunction> {
        match self.native_funcs.get_mut(id) {
            Some(defs) => {
                for (func, _) in defs.iter_mut() {
                    if func.params_match(params) {
                        return Some(func)
                    }
                }
                None
            }
            None => None
        }
    }
    pub fn get_native_fn_first(&self, id: &String) -> Option<&NativFunction> {
        match self.native_funcs.get(id) {
            Some(defs) => Some(&defs.first().unwrap().0),
            None => None
        }
    }
    pub fn get_native_fn_first_mut(&mut self, id: &String) -> Option<&mut NativFunction> {
        match self.native_funcs.get_mut(id) {
            Some(defs) => Some(&mut defs.first_mut().unwrap().0),
            None => None
        }
    }
    pub fn native_fn_exists(&self, id: &String) -> bool {
        self.native_funcs.contains_key(id)
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
    pub fn call(context: &Context, inline: bool) -> Self {
        let mut scopes = if inline { context.scopes.clone() } else { vec![Scope::new()] };
        if inline { scopes.push(Scope::new()); }
        Self { scopes, global: context.global.clone(), trace: context.trace.clone() }
    }
    pub fn after_call(&mut self, context: Context) {
        self.global = context.global;
        self.trace = context.trace;
    }
    pub fn push(&mut self) { self.scopes.push(Scope::new()) }
    pub fn pop(&mut self) -> Option<Scope> { self.scopes.pop() }
    pub fn trace_push(&mut self, pos: &Position) { self.trace.push(pos.clone()); }
    pub fn trace_pop(&mut self) -> Option<Position> { self.trace.pop() }
    // scope of var
    pub fn get_scope_var(&self, id: &String) -> Option<&Scope> {
        for scope in self.scopes.iter().rev() {
            if scope.get_var(id).is_some() { return Some(scope) }
        }
        if self.global.get_var(id).is_some() { return Some(&self.global) }
        None
    }
    pub fn get_scope_var_mut(&mut self, id: &String) -> Option<&mut Scope> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.get_var(id).is_some() { return Some(scope) }
        }
        if self.global.get_var(id).is_some() { return Some(&mut self.global) }
        None
    }
    // scope of fn
    pub fn get_scope_fn(&self, id: &String, pattern: &Vec<Type>) -> Option<&Scope> {
        for scope in self.scopes.iter().rev() {
            if scope.get_fn(id, pattern).is_some() { return Some(scope) }
        }
        if self.global.get_fn(id, pattern).is_some() { return Some(&self.global) }
        None
    }
    pub fn get_scope_fn_mut(&mut self, id: &String, pattern: &Vec<Type>) -> Option<&mut Scope> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.get_fn_mut(id, pattern).is_some() { return Some(scope) }
        }
        if self.global.get_fn_mut(id, pattern).is_some() { return Some(&mut self.global) }
        None
    }
    // scope of native fn
    pub fn get_scope_native_fn(&self, id: &String, pattern: &Vec<Type>) -> Option<&Scope> {
        for scope in self.scopes.iter().rev() {
            if scope.get_native_fn(id, pattern).is_some() { return Some(scope) }
        }
        if self.global.get_native_fn(id, pattern).is_some() { return Some(&self.global) }
        None
    }
    pub fn get_scope_native_fn_mut(&mut self, id: &String, pattern: &Vec<Type>) -> Option<&mut Scope> {
        for scope in self.scopes.iter_mut().rev() {
            if scope.get_native_fn_mut(id, pattern).is_some() { return Some(scope) }
        }
        if self.global.get_native_fn_mut(id, pattern).is_some() { return Some(&mut self.global) }
        None
    }
    
    // create
    pub fn create_var(&mut self, id: String, value: Value, mutable: bool, pos: Position, overwrite: bool) -> Result<(), Error> {
        match self.get_scope_var_mut(&id) {
            None => self.scopes.last_mut().unwrap().create_var(id, value, mutable, pos),
            Some(_) => if overwrite { Err(Error::AlreadyDefined(id)) } else { self.scopes.last_mut().unwrap().create_var(id, value, mutable, pos) }
        }
    }
    pub fn create_fn(&mut self, id: String, func: Function, pos: Position) -> Result<(), Error> {
        match self.get_scope_fn_mut(&id, &func.type_params()) {
            None => self.scopes.last_mut().unwrap().create_fn(id, func, pos),
            Some(scope) => scope.create_fn(id, func, pos)
        }
    }
    pub fn create_native_fn(&mut self, id: String, func: NativFunction, pos: Position) -> Result<(), Error> {
        self.global.create_native_fn(id, func, pos)
    }
    pub fn create_params(&mut self, params: &Vec<(String, Type, bool)>, values: Vec<Value>, poses: Vec<Position>, inline: bool) -> Result<(), Error> {
        let mut value_idx: usize = 0;
        for i in 0..params.len() {
            let (param, param_type, more) = &params[i];
            if *more {
                let mut vec_values: Vec<Value> = vec![];
                let mut pos = poses[value_idx].clone();
                while let Some(value) = values.get(value_idx) {
                    if &value.typ() != param_type { break }
                    pos = Position::between(pos, poses[value_idx].clone());
                    vec_values.push(values[value_idx].clone());
                    value_idx += 1;
                }
                self.create_var(param.clone(), Value::Vector(vec_values, Some(param_type.clone())), false, pos, inline)?;
            } else {
                self.create_var(param.clone(), values[value_idx].clone(), false, poses[value_idx].clone(), inline)?;
                value_idx += 1;
            }
        }
        Ok(())
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
    pub fn fn_exists(&self, id: &String) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.fn_exists(id) { return true }
        }
        self.global.fn_exists(id)
    }
    // get native fn
    pub fn get_native_fn(&self, id: &String, pattern: &Vec<Type>) -> Option<&NativFunction> {
        self.get_scope_native_fn(id, pattern)?.get_native_fn(id, pattern)
    }
    pub fn get_native_fn_mut(&mut self, id: &String, pattern: &Vec<Type>) -> Option<&mut NativFunction> {
        self.get_scope_native_fn_mut(id, pattern)?.get_native_fn_mut(id, pattern)
    }
    pub fn native_fn_exists(&self, id: &String) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.native_fn_exists(id) { return true }
        }
        self.global.native_fn_exists(id)
    }
}
