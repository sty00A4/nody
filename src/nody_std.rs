use crate::*;

// let
fn _let(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":id".to_string()).unwrap().clone();
    let v = context.get_var(&":v".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        let len = context.scopes.len();
        match context.scopes.get_mut(len - 2) { // try to mutate the scope before the last
            Some(scope) => scope.create_var(id, v, false, pos, false)?,
            None => context.create_var(id, v, false, pos, false)?
        }
        Ok((None, Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _let_global(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":id".to_string()).unwrap().clone();
    let v = context.get_var(&":v".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        context.global.create_var(id, v, false, pos, false)?;
        Ok((None, Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _mut(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":id".to_string()).unwrap().clone();
    let v = context.get_var(&":v".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        let len = context.scopes.len();
        match context.scopes.get_mut(len - 2) { // try to mutate the scope before the last
            Some(scope) => scope.create_var(id, v, true, pos, false)?,
            None => context.create_var(id, v, true, pos, false)?
        }
        Ok((None, Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _mut_global(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":id".to_string()).unwrap().clone();
    let v = context.get_var(&":v".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        context.global.create_var(id, v, true, pos, false)?;
        Ok((None, Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _set(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let v = context.get_var(&":v".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if context.get_var(&id).is_none() { return Err(Error::NotDefined(id)) }
        if !context.is_mutable(&id).unwrap() { return Err(Error::Immutable(id)) }
        context.change(id, v)?;
        Ok((None, Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _set_path(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&":v".to_string()).unwrap().clone();
    context.trace_push(&context.get_var_pos(&":v".to_string()).unwrap().clone());
    let mut path = context.get_var_mut(&":path".to_string()).unwrap().clone();
    if let Value::Path(path) = &mut path {
        let mutable = path.is_mutable(context)?.unwrap();
        match path.get_mut(context)? {
            Some(value) => if mutable {
                if v.typ() != value.typ() { return Err(Error::ExpectedType(value.typ(), v.typ())) }
                *value = v;
                context.trace_pop();
                Ok((None, Return::None))
            } else {
                Err(Error::ImmutablePath(path.clone()))
            }
            None => Err(Error::NotDefinedPath(path.clone()))
        }
    } else { panic!("type checking doesn't work") }
}
fn _set_index(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&":v".to_string()).unwrap().clone();
    context.trace_push(&context.get_var_pos(&":v".to_string()).unwrap().clone());
    let mut index = context.get_var_mut(&":index".to_string()).unwrap().clone();
    if let Value::Index(index) = &mut index {
        let mutable = index.is_mutable(context)?.unwrap();
        match index.get_mut(context)? {
            Some(value) => if mutable {
                if v.typ() != value.typ() {
                    return Err(Error::ExpectedType(value.typ(), v.typ()))
                }
                *value = v;
                context.trace_pop();
                Ok((None, Return::None))
            } else {
                Err(Error::ImmutableIndex(index.clone()))
            }
            None => Err(Error::NotDefinedIndex(index.clone()))
        }
    } else { panic!("type checking doesn't work") }
}
fn _get(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        match context.get_var(&id) {
            Some(value) => Ok((Some(value.clone()), Return::None)),
            None => Err(Error::NotDefined(id))
        }
    } else { panic!("type checking doesn't work") }
}
fn _get_path(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let path = context.get_var(&":path".to_string()).unwrap().clone();
    if let Value::Path(path) = path {
        match path.get(context)? {
            Some(value) => Ok((Some(value.clone()), Return::None)),
            None => Err(Error::NotDefinedPath(path))
        }
    } else { panic!("type checking doesn't work") }
}
fn _get_index(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let index = context.get_var(&":index".to_string()).unwrap().clone();
    if let Value::Index(index) = index {
        match index.get(context)? {
            Some(value) => Ok((Some(value.clone()), Return::None)),
            None => Err(Error::NotDefinedIndex(index))
        }
    } else { panic!("type checking doesn't work") }
}
fn _exist(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap();
    if let Value::Key(id) = id {
        Ok((Some(Value::Bool(context.get_var(id).is_some())), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _exist_path(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let path = context.get_var(&":path".to_string()).unwrap().clone();
    if let Value::Path(path) = path {
        Ok((Some(Value::Bool(match path.get(context) {
            Ok(value) => value.is_some(),
            Err(_) => false
        })), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _exist_index(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let index = context.get_var(&":index".to_string()).unwrap().clone();
    if let Value::Index(index) = index {
        Ok((Some(Value::Bool(match index.get(context) {
            Ok(value) => value.is_some(),
            Err(_) => false
        })), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _is_mut(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        match context.is_mutable(&id) {
            Some(mutable) => Ok((Some(Value::Bool(mutable)), Return::None)),
            None => Err(Error::NotDefined(id))
        }
    } else { panic!("type checking doesn't work") }
}
fn _is_mut_path(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let path = context.get_var(&":path".to_string()).unwrap().clone();
    if let Value::Path(path) = path {
        match path.is_mutable(context)? {
            Some(mutable) => Ok((Some(Value::Bool(mutable)), Return::None)),
            None => Err(Error::NotDefinedPath(path))
        }
    } else { panic!("type checking doesn't work") }
}
fn _is_mut_index(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let index = context.get_var(&":index".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":index".to_string()).unwrap().clone();
    if let Value::Index(index) = index {
        match index.is_mutable(context)? {
            Some(mutable) => Ok((Some(Value::Bool(mutable)), Return::None)),
            None => Err(Error::NotDefinedIndex(index))
        }
    } else { panic!("type checking doesn't work") }
}
// def
fn _def(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let p = context.get_var(&":p".to_string()).unwrap().clone();
    let body = context.get_var(&":body".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":body".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if let Value::Params(p) = p {
            if let Value::Closure(body) = body {
                let func = Function::new(p, None, Box::new(body), false);
                let len = context.scopes.len();
                match context.scopes.get_mut(len - 2) { // try to mutate the scope before the last
                    Some(scope) => scope.create_fn(id, func, pos)?,
                    None => context.create_fn(id, func, pos)?
                }
                Ok((None, Return::None))
            } else { panic!("type checking doesn't work") }
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _def_return(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let p = context.get_var(&":p".to_string()).unwrap().clone();
    let body = context.get_var(&":body".to_string()).unwrap().clone();
    let return_type = context.get_var(&":return_type".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":body".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if let Value::Params(p) = p {
            if let Value::Closure(body) = body {
                if let Value::Type(return_type) = return_type {
                    let func = Function::new(p, Some(return_type), Box::new(body), false);
                    let len = context.scopes.len();
                    if context.fn_exists(&id) {
                        context.create_fn(id, func, pos)?;
                    } else {
                        match context.scopes.get_mut(len - 2) { // try to mutate the scope before the last
                            Some(scope) => scope.create_fn(id, func, pos)?,
                            None => context.create_fn(id, func, pos)?
                        }
                    }
                    Ok((None, Return::None))
                } else { panic!("type checking doesn't work") }
            } else { panic!("type checking doesn't work") }
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _def_inline(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let p = context.get_var(&":p".to_string()).unwrap().clone();
    let body = context.get_var(&":body".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":body".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if let Value::Params(p) = p {
            if let Value::Closure(body) = body {
                let func = Function::new(p, None, Box::new(body), true);
                let len = context.scopes.len();
                if context.fn_exists(&id) {
                    context.create_fn(id, func, pos)?;
                } else {
                    match context.scopes.get_mut(len - 2) { // try to mutate the scope before the last
                        Some(scope) => scope.create_fn(id, func, pos)?,
                        None => context.create_fn(id, func, pos)?
                    }
                }
                Ok((None, Return::None))
            } else { panic!("type checking doesn't work") }
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _def_return_inline(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let p = context.get_var(&":p".to_string()).unwrap().clone();
    let body = context.get_var(&":body".to_string()).unwrap().clone();
    let return_type = context.get_var(&":return_type".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":body".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if let Value::Params(p) = p {
            if let Value::Closure(body) = body {
                if let Value::Type(return_type) = return_type {
                    let func = Function::new(p, Some(return_type), Box::new(body), true);
                    let len = context.scopes.len();
                    if context.fn_exists(&id) {
                        context.create_fn(id, func, pos)?;
                    } else {
                        match context.scopes.get_mut(len - 2) { // try to mutate the scope before the last
                            Some(scope) => scope.create_fn(id, func, pos)?,
                            None => context.create_fn(id, func, pos)?
                        }
                    }
                    Ok((None, Return::None))
                } else { panic!("type checking doesn't work") }
            } else { panic!("type checking doesn't work") }
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _def_global(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let p = context.get_var(&":p".to_string()).unwrap().clone();
    let body = context.get_var(&":body".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":body".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if let Value::Params(p) = p {
            if let Value::Closure(body) = body {
                let func = Function::new(p, None, Box::new(body), false);
                context.create_fn_global(id, func, pos)?;
                Ok((None, Return::None))
            } else { panic!("type checking doesn't work") }
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _def_return_global(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let p = context.get_var(&":p".to_string()).unwrap().clone();
    let body = context.get_var(&":body".to_string()).unwrap().clone();
    let return_type = context.get_var(&":return_type".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":body".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if let Value::Params(p) = p {
            if let Value::Closure(body) = body {
                if let Value::Type(return_type) = return_type {
                    let func = Function::new(p, Some(return_type), Box::new(body), false);
                    context.create_fn_global(id, func, pos)?;
                    Ok((None, Return::None))
                } else { panic!("type checking doesn't work") }
            } else { panic!("type checking doesn't work") }
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _def_global_inline(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let p = context.get_var(&":p".to_string()).unwrap().clone();
    let body = context.get_var(&":body".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":body".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if let Value::Params(p) = p {
            if let Value::Closure(body) = body {
                let func = Function::new(p, None, Box::new(body), true);
                context.create_fn_global(id, func, pos)?;
                Ok((None, Return::None))
            } else { panic!("type checking doesn't work") }
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _def_return_global_inline(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let p = context.get_var(&":p".to_string()).unwrap().clone();
    let body = context.get_var(&":body".to_string()).unwrap().clone();
    let return_type = context.get_var(&":return_type".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":body".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if let Value::Params(p) = p {
            if let Value::Closure(body) = body {
                if let Value::Type(return_type) = return_type {
                    let func = Function::new(p, Some(return_type), Box::new(body), true);
                    context.create_fn_global(id, func, pos)?;
                    Ok((None, Return::None))
                } else { panic!("type checking doesn't work") }
            } else { panic!("type checking doesn't work") }
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
// control flow
fn _return(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap().clone();
    Ok((Some(v), Return::Return))
}
fn _break(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    Ok((None, Return::Break))
}
fn _do(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let node = context.get_var(&":node".to_string()).unwrap().clone();
    if let Value::Closure(node) = node {
        let res = interpret(&node, context)?;
        Ok(res)
    } else { panic!("type checking doesn't work") }
}
fn _if(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let cond = context.get_var(&"cond".to_string()).unwrap();
    let case = context.get_var(&"case".to_string()).unwrap();
    let else_ = context.get_var(&"else".to_string()).unwrap();
    if cond == &Value::Bool(true) {
        Ok((Some(case.clone()), Return::None))
    } else {
        Ok((Some(else_.clone()), Return::None))
    }
}
fn _if_do(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let cond = context.get_var(&":cond".to_string()).unwrap();
    let case = context.get_var(&":case".to_string()).unwrap().clone();
    if let Value::Closure(case) = case {
        if cond == &Value::Bool(true) {
            return interpret(&case, context)
        }
        Ok((None, Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _if_do_else(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let cond = context.get_var(&":cond".to_string()).unwrap();
    let case = context.get_var(&":case".to_string()).unwrap().clone();
    let else_ = context.get_var(&":else".to_string()).unwrap().clone();
    if let Value::Closure(case) = case {
        if let Value::Closure(else_) = else_ {
            if cond == &Value::Bool(true) {
                interpret(&case, context)
            } else {
                interpret(&else_, context)
            }
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _for(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":id".to_string()).unwrap().clone();
    let iter = context.get_var(&":iter".to_string()).unwrap().clone();
    let body = context.get_var(&":body".to_string()).unwrap().clone();
    if let Value::Closure(body) = body {
        if let Value::Key(id) = id {
            if let Value::Vector(values, _) = iter {
                for v in values {
                    context.create_var(id.clone(), v, false, pos.clone(), true);
                    let (value, ret) = interpret(&body, context)?;
                    if ret == Return::Break { break }
                    if ret == Return::Return { return Ok((value, ret)) }
                }
                Ok((None, Return::None))
            } else { panic!("type checking doesn't work") }
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _for_length(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":id".to_string()).unwrap().clone();
    let length = context.get_var(&":length".to_string()).unwrap().clone();
    let body = context.get_var(&":body".to_string()).unwrap().clone();
    if let Value::Closure(body) = body {
        if let Value::Key(id) = id {
            if let Value::Int(length) = length {
                for i in 0..length {
                    context.create_var(id.clone(), Value::Int(i), false, pos.clone(), true);
                    let (value, ret) = interpret(&body, context)?;
                    if ret == Return::Break { break }
                    if ret == Return::Return { return Ok((value, ret)) }
                }
                Ok((None, Return::None))
            } else { panic!("type checking doesn't work") }
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _for_range(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&":id".to_string()).unwrap().clone();
    let start = context.get_var(&":start".to_string()).unwrap().clone();
    let end = context.get_var(&":end".to_string()).unwrap().clone();
    let body = context.get_var(&":body".to_string()).unwrap().clone();
    if let Value::Closure(body) = body {
        if let Value::Key(id) = id {
            if let Value::Int(start) = start {
                if let Value::Int(end) = end {
                    for i in start..end {
                        context.create_var(id.clone(), Value::Int(i), false, pos.clone(), true);
                        let (value, ret) = interpret(&body, context)?;
                        if ret == Return::Break { break }
                        if ret == Return::Return { return Ok((value, ret)) }
                    }
                    Ok((None, Return::None))
                } else { panic!("type checking doesn't work") }
            } else { panic!("type checking doesn't work") }
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _while(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let cond = context.get_var(&":cond".to_string()).unwrap().clone();
    let body = context.get_var(&":body".to_string()).unwrap().clone();
    if let Value::Closure(body) = body {
        if let Value::Closure(cond) = cond {
            while let (Some(Value::Bool(true)), _) = interpret(&cond, context)? {
                let (value, ret) = interpret(&body, context)?;
                if ret == Return::Break { break }
                if ret == Return::Return { return Ok((value, ret)) }
            }
            Ok((None, Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _loop(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let body = context.get_var(&":body".to_string()).unwrap().clone();
    if let Value::Closure(body) = body {
        loop {
            let (value, ret) = interpret(&body, context)?;
            if ret == Return::Break { break }
            if ret == Return::Return { return Ok((value, ret)) }
        }
        Ok((None, Return::None))
    } else { panic!("type checking doesn't work") }
}
// +
fn _add_int(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let n = context.get_var(&"n".to_string()).unwrap();
    let nums = context.get_var(&"nums".to_string()).unwrap();
    if let Value::Vector(nums, Some(Type::Int)) = nums {
        let mut sum: i64 = 0;
        if let Value::Int(n) = n { sum = *n; }
        for n in nums.iter() {
            if let Value::Int(n) = n {
                sum += *n;
            }
        }
        Ok((Some(Value::Int(sum)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _add_float(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let n = context.get_var(&"n".to_string()).unwrap();
    let nums = context.get_var(&"nums".to_string()).unwrap();
    if let Value::Vector(nums, Some(Type::Float)) = nums {
        let mut sum: f64 = 0.0;
        if let Value::Float(n) = n { sum = *n; }
        for n in nums.iter() {
            if let Value::Float(n) = n {
                sum += *n;
            }
        }
        Ok((Some(Value::Float(sum)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _add_str(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let n = context.get_var(&"n".to_string()).unwrap();
    let nums = context.get_var(&"nums".to_string()).unwrap();
    if let Value::Vector(nums, Some(Type::String)) = nums {
        let mut string = String::new();
        if let Value::String(n) = n { string.push_str(n.as_str()); }
        for n in nums.iter() {
            if let Value::String(n) = n {
                string.push_str(n.as_str());
            }
        }
        Ok((Some(Value::String(string)), Return::None))
    } else { panic!("type checking doesn't work") }
}
// -
fn _sub_int(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let n = context.get_var(&"n".to_string()).unwrap();
    let nums = context.get_var(&"nums".to_string()).unwrap();
    if let Value::Vector(nums, Some(Type::Int)) = nums {
        let mut sum: i64 = 0;
        if let Value::Int(n) = n { sum = *n; }
        for n in nums.iter() {
            if let Value::Int(n) = n {
                sum -= *n;
            }
        }
        Ok((Some(Value::Int(sum)), Return::None))
    } else { panic!("type checking doesn't work"); }
}
fn _neg_int(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let n = context.get_var(&"n".to_string()).unwrap();
    if let Value::Int(n) = n {
        Ok((Some(Value::Int(-n)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _sub_float(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let n = context.get_var(&"n".to_string()).unwrap();
    let nums = context.get_var(&"nums".to_string()).unwrap();
    if let Value::Vector(nums, Some(Type::Float)) = nums {
        let mut sum: f64 = 0.0;
        if let Value::Float(n) = n { sum = *n; }
        for n in nums.iter() {
            if let Value::Float(n) = n {
                sum -= *n;
            }
        }
        Ok((Some(Value::Float(sum)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _neg_float(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let n = context.get_var(&"n".to_string()).unwrap();
    if let Value::Float(n) = n {
        Ok((Some(Value::Float(-n)), Return::None))
    } else { panic!("type checking doesn't work") }
}
// *
fn _mul_int(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let n = context.get_var(&"n".to_string()).unwrap();
    let nums = context.get_var(&"nums".to_string()).unwrap();
    if let Value::Vector(nums, Some(Type::Int)) = nums {
        let mut sum: i64 = 0;
        if let Value::Int(n) = n { sum = *n; }
        for n in nums.iter() {
            if let Value::Int(n) = n {
                sum *= *n;
            }
        }
        Ok((Some(Value::Int(sum)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _mul_float(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let n = context.get_var(&"n".to_string()).unwrap();
    let nums = context.get_var(&"nums".to_string()).unwrap();
    if let Value::Vector(nums, Some(Type::Float)) = nums {
        let mut sum: f64 = 0.0;
        if let Value::Float(n) = n { sum = *n; }
        for n in nums.iter() {
            if let Value::Float(n) = n {
                sum *= *n;
            }
        }
        Ok((Some(Value::Float(sum)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _mul_str(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let s = context.get_var(&"s".to_string()).unwrap();
    let n = context.get_var(&"n".to_string()).unwrap();
    if let Value::String(s) = s {
        if let Value::Int(n) = n {
            Ok((Some(Value::String(s.repeat(max::<i64>(*n, 0) as usize))), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
// /
fn _div_int(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let n = context.get_var(&"n".to_string()).unwrap();
    let nums = context.get_var(&"nums".to_string()).unwrap();
    if let Value::Vector(nums, Some(Type::Int)) = nums {
        let mut sum: i64 = 0;
        if let Value::Int(n) = n { sum = *n; }
        for n in nums.iter() {
            if let Value::Int(n) = n {
                sum /= *n;
            }
        }
        Ok((Some(Value::Int(sum)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _div_float(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let n = context.get_var(&"n".to_string()).unwrap();
    let nums = context.get_var(&"nums".to_string()).unwrap();
    if let Value::Vector(nums, Some(Type::Float)) = nums {
        let mut sum: f64 = 0.0;
        if let Value::Float(n) = n { sum = *n; }
        for n in nums.iter() {
            if let Value::Float(n) = n {
                sum /= *n;
            }
        }
        Ok((Some(Value::Float(sum)), Return::None))
    } else { panic!("type checking doesn't work") }
}
// %
fn _mod_int(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let n = context.get_var(&"n".to_string()).unwrap();
    let nums = context.get_var(&"nums".to_string()).unwrap();
    if let Value::Vector(nums, Some(Type::Int)) = nums {
        let mut sum: i64 = 0;
        if let Value::Int(n) = n { sum = *n; }
        for n in nums.iter() {
            if let Value::Int(n) = n {
                sum %= *n;
            }
        }
        Ok((Some(Value::Int(sum)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _mod_float(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let n = context.get_var(&"n".to_string()).unwrap();
    let nums = context.get_var(&"nums".to_string()).unwrap();
    if let Value::Vector(nums, Some(Type::Float)) = nums {
        let mut sum: f64 = 0.0;
        if let Value::Float(n) = n { sum = *n; }
        for n in nums.iter() {
            if let Value::Float(n) = n {
                sum %= *n;
            }
        }
        Ok((Some(Value::Float(sum)), Return::None))
    } else { panic!("type checking doesn't work") }
}
// =
fn _eq(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let a = context.get_var(&"a".to_string()).unwrap();
    let b = context.get_var(&"b".to_string()).unwrap();
    Ok((Some(Value::Bool(a == b)), Return::None))
}
// !=
fn _neq(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let a = context.get_var(&"a".to_string()).unwrap();
    let b = context.get_var(&"b".to_string()).unwrap();
    Ok((Some(Value::Bool(a == b)), Return::None))
}
// >
fn _gt_int(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let a = context.get_var(&"a".to_string()).unwrap();
    let b = context.get_var(&"b".to_string()).unwrap();
    if let Value::Int(a) = a {
        if let Value::Int(b) = b {
            Ok((Some(Value::Bool(a > b)), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _gt_float(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let a = context.get_var(&"a".to_string()).unwrap();
    let b = context.get_var(&"b".to_string()).unwrap();
    if let Value::Float(a) = a {
        if let Value::Float(b) = b {
            Ok((Some(Value::Bool(a > b)), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
// >=
fn _ge_int(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let a = context.get_var(&"a".to_string()).unwrap();
    let b = context.get_var(&"b".to_string()).unwrap();
    if let Value::Int(a) = a {
        if let Value::Int(b) = b {
            Ok((Some(Value::Bool(a >= b)), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _ge_float(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let a = context.get_var(&"a".to_string()).unwrap();
    let b = context.get_var(&"b".to_string()).unwrap();
    if let Value::Float(a) = a {
        if let Value::Float(b) = b {
            Ok((Some(Value::Bool(a >= b)), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
// <
fn _lt_int(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let a = context.get_var(&"a".to_string()).unwrap();
    let b = context.get_var(&"b".to_string()).unwrap();
    if let Value::Int(a) = a {
        if let Value::Int(b) = b {
            Ok((Some(Value::Bool(a < b)), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _lt_float(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let a = context.get_var(&"a".to_string()).unwrap();
    let b = context.get_var(&"b".to_string()).unwrap();
    if let Value::Float(a) = a {
        if let Value::Float(b) = b {
            Ok((Some(Value::Bool(a < b)), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
// <=
fn _le_int(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let a = context.get_var(&"a".to_string()).unwrap();
    let b = context.get_var(&"b".to_string()).unwrap();
    if let Value::Int(a) = a {
        if let Value::Int(b) = b {
            Ok((Some(Value::Bool(a <= b)), Return::None))
        } else { panic!("type checkng doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _le_float(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let a = context.get_var(&"a".to_string()).unwrap();
    let b = context.get_var(&"b".to_string()).unwrap();
    if let Value::Float(a) = a {
        if let Value::Float(b) = b {
            Ok((Some(Value::Bool(a <= b)), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
// len
fn _len_vec(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Vector(v, t) = v {
        Ok((Some(Value::Int(v.len() as i64)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _len_str(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::String(v) = v {
        Ok((Some(Value::Int(v.len() as i64)), Return::None))
    } else { panic!("type checking doesn't work") }
}
// int
fn _int_int(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    Ok((Some(context.get_var(&"v".to_string()).unwrap().clone()), Return::None))
}
fn _int_float(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Float(v) = v {
        Ok((Some(Value::Int(*v as i64)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _int_char(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Char(v) = v {
        Ok((Some(Value::Int(*v as i64)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _int_bool(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Bool(v) = v {
        Ok((Some(Value::Int(*v as i64)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _int_str(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::String(v) = v {
        if let Ok(v) = v.parse::<i64>() {
            Ok((Some(Value::Int(v)), Return::None))
        } else {
            Err(Error::ParseInt(v.clone()))
        }
    } else { panic!("type checking doesn't work") }
}
// float
fn _float_int(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Int(v) = v {
        Ok((Some(Value::Float(*v as f64)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _float_float(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    Ok((Some(context.get_var(&"v".to_string()).unwrap().clone()), Return::None))
}
fn _float_str(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::String(v) = v {
        if let Ok(v) = v.parse::<f64>() {
            Ok((Some(Value::Float(v)), Return::None))
        } else {
            Err(Error::ParseFloat(v.clone()))
        }
    } else { panic!("type checking doesn't work") }
}
// bool
fn _bool_int(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Int(v) = v {
        Ok((Some(Value::Bool(*v != 0)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _bool_float(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Float(v) = v {
        Ok((Some(Value::Bool(*v != 0.0)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _bool_char(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Char(v) = v {
        Ok((Some(Value::Bool(*v as u8 != 0)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _bool_bool(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    Ok((Some(context.get_var(&"v".to_string()).unwrap().clone()), Return::None))
}
fn _bool_str(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::String(v) = v {
        if let Ok(v) = v.parse::<bool>() {
            Ok((Some(Value::Bool(v)), Return::None))
        } else {
            Err(Error::ParseBool(v.clone()))
        }
    } else { panic!("type checking doesn't work") }
}
// char
fn _char_int(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Int(v) = v {
        Ok((Some(Value::Char(*v as u8 as char)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _char_char(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    Ok((Some(context.get_var(&"v".to_string()).unwrap().clone()), Return::None))
}
fn _char_str(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::String(v) = v {
        if let Ok(v) = v.parse::<char>() {
            Ok((Some(Value::Char(v)), Return::None))
        } else {
            Err(Error::ParseChar(v.clone()))
        }
    } else { panic!("type checking doesn't work") }
}
// str
fn _str(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    Ok((Some(Value::String(context.get_var(&"v".to_string()).unwrap().to_string())), Return::None))
}
// key
fn _key(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    Ok((Some(Value::Key(context.get_var(&"v".to_string()).unwrap().to_string())), Return::None))
}
// path
fn _path_key(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let head = context.get_var(&"head".to_string()).unwrap().clone();
    let sub = context.get_var(&"sub".to_string()).unwrap().clone();
    if let Value::Key(head) = head {
        if let Value::Key(sub) = sub {
            Ok((Some(Value::Path(Path::new(PathWays::Key(head), sub))), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _path_path(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let head = context.get_var(&"head".to_string()).unwrap().clone();
    let sub = context.get_var(&"sub".to_string()).unwrap().clone();
    if let Value::Path(head) = head {
        if let Value::Key(sub) = sub {
            Ok((Some(Value::Path(Path::new(PathWays::Path(Box::new(head)), sub))), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _path_index(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let head = context.get_var(&"head".to_string()).unwrap().clone();
    let sub = context.get_var(&"sub".to_string()).unwrap().clone();
    if let Value::Index(head) = head {
        if let Value::Key(sub) = sub {
            Ok((Some(Value::Path(Path::new(PathWays::Index(Box::new(head)), sub))), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
// index
fn _index_key(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let head = context.get_var(&"head".to_string()).unwrap().clone();
    let idx = context.get_var(&"idx".to_string()).unwrap();
    if let Value::Key(head) = head {
        if let Value::Int(idx) = idx {
            if *idx < 0 { return Err(Error::IllegalNegativeIndex(*idx)) }
            Ok((Some(Value::Index(Index::new(PathWays::Key(head), *idx as usize))), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _index_path(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let head = context.get_var(&"head".to_string()).unwrap().clone();
    let idx = context.get_var(&"idx".to_string()).unwrap();
    if let Value::Path(head) = head {
        if let Value::Int(idx) = idx {
            if *idx < 0 { return Err(Error::IllegalNegativeIndex(*idx)) }
            Ok((Some(Value::Index(Index::new(PathWays::Path(Box::new(head)), *idx as usize))), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _index_index(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let head = context.get_var(&"head".to_string()).unwrap().clone();
    let idx = context.get_var(&"idx".to_string()).unwrap();
    if let Value::Index(head) = head {
        if let Value::Int(idx) = idx {
            if *idx < 0 { return Err(Error::IllegalNegativeIndex(*idx)) }
            Ok((Some(Value::Index(Index::new(PathWays::Index(Box::new(head)), *idx as usize))), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
// vec
fn _vec(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let t = context.get_var(&"t".to_string()).unwrap();
    if let Value::Type(t) = t {
        Ok((Some(Value::Type(Type::Vector(Some(Box::new(t.clone()))))), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _vec_of(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap().clone();
    let t = context.get_var(&"t".to_string()).unwrap().clone();
    if let Value::Type(t) = t {
        if let Value::Vector(values, typ) = v {
            if let Some(typ) = typ { if typ != t { return Err(Error::ExpectedType(t, typ)) } }
            Ok((Some(Value::Vector(values, Some(t))), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _vec_push(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let value = context.get_var(&":v".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if context.is_mutable(&id) == Some(false) { return Err(Error::Immutable(id)) }
        match context.get_var_mut(&id) {
            Some(values) => if let Value::Vector(values, typ) = values {
                match typ {
                    Some(typ) => if value.typ() == *typ {
                        values.push(value);
                        Ok((None, Return::None))
                    } else {
                        return Err(Error::ExpectedType(typ.clone(), value.typ()))
                    }
                    None => {
                        *typ = Some(value.typ());
                        values.push(value);
                        Ok((None, Return::None))
                    }
                }
            } else {
                Err(Error::ExpectedType(Type::Vector(Some(Box::new(Type::Any))), values.typ()))
            }
            None => Err(Error::NotDefined(id))
        }
    } else { panic!("type checking doesn't work") }
}
fn _str_push(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let c = context.get_var(&":c".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if let Value::Char(c) = c {
            if context.is_mutable(&id) == Some(false) { return Err(Error::Immutable(id)) }
            match context.get_var_mut(&id) {
                Some(s) => if let Value::String(s) = s {
                    s.push(c);
                    Ok((None, Return::None))
                } else {
                    Err(Error::ExpectedType(Type::String, s.typ()))
                }
                None => Err(Error::NotDefined(id))
            }
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
fn _vec_pop(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if context.is_mutable(&id) == Some(false) { return Err(Error::Immutable(id)) }
        match context.get_var_mut(&id) {
            Some(values) => if let Value::Vector(values, _) = values {
                Ok((values.pop(), Return::None))
            } else {
                Err(Error::ExpectedType(Type::Vector(Some(Box::new(Type::Any))), values.typ()))
            }
            None => Err(Error::NotDefined(id))
        }
    } else { panic!("type checking doesn't work") }
}
fn _vec_pop_idx(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let idx = context.get_var(&":idx".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if context.is_mutable(&id) == Some(false) { return Err(Error::Immutable(id)) }
        match context.get_var_mut(&id) {
            Some(values) => if let Value::Vector(values, _) = values {
                if let Value::Int(idx) = idx {
                    let idx = if idx < 0 { values.len() - idx.abs() as usize } else { idx as usize };
                    match values.get(idx) {
                        Some(_) => Ok((Some(values.remove(idx)), Return::None)),
                        None => Err(Error::IndexOutOfRange(idx, values.len()))
                    }
                } else { panic!("type checking doesn't work") }
            } else {
                Err(Error::ExpectedType(Type::Vector(Some(Box::new(Type::Any))), values.typ()))
            }
            None => Err(Error::NotDefined(id))
        }
    } else { panic!("type checking doesn't work") }
}
fn _str_pop(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if context.is_mutable(&id) == Some(false) { return Err(Error::Immutable(id)) }
        match context.get_var_mut(&id) {
            Some(s) => if let Value::String(s) = s {
                Ok((match s.pop() { Some(c) => Some(Value::Char(c)), _ => None }, Return::None))
            } else {
                Err(Error::ExpectedType(Type::String, s.typ()))
            }
            None => Err(Error::NotDefined(id))
        }
    } else { panic!("type checking doesn't work") }
}
fn _str_pop_idx(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let id = context.get_var(&":id".to_string()).unwrap().clone();
    let idx = context.get_var(&":idx".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if context.is_mutable(&id) == Some(false) { return Err(Error::Immutable(id)) }
        match context.get_var_mut(&id) {
            Some(s) => if let Value::String(s) = s {
                if let Value::Int(idx) = idx {
                    let idx = if idx < 0 { s.len() - idx.abs() as usize } else { idx as usize };
                    match s.get(idx .. idx + 1) {
                        Some(_) => Ok((Some(Value::Char(s.remove(idx))), Return::None)),
                        None => Err(Error::IndexOutOfRange(idx, s.len()))
                    }
                } else { panic!("type checking doesn't work") }
            } else {
                Err(Error::ExpectedType(Type::Vector(Some(Box::new(Type::Any))), s.typ()))
            }
            None => Err(Error::NotDefined(id))
        }
    } else { panic!("type checking doesn't work") }
}
fn _contains(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let values = context.get_var(&"values".to_string()).unwrap();
    let value = context.get_var(&"value".to_string()).unwrap();
    if let Value::Vector(values, _) = values {
        Ok((Some(Value::Bool(values.contains(value))), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _contains_str(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let s = context.get_var(&"s".to_string()).unwrap();
    let c = context.get_var(&"c".to_string()).unwrap();
    if let Value::String(s) = s {
        if let Value::Char(c) = c {
            Ok((Some(Value::Bool(s.contains(*c))), Return::None))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
// type
fn _type(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    Ok((Some(Value::Type(context.get_var(&"v".to_string()).unwrap().typ())), Return::None))
}
// io
fn _write(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Vector(values, Some(Type::Any)) = v {
        for v in values.iter() { print!("{v}"); }
        Ok((None, Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _print(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Vector(values, Some(Type::Any)) = v {
        for v in values.iter() { println!("{v}"); }
        Ok((None, Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _input(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let msg = context.get_var(&"msg".to_string()).unwrap();
    if let Value::String(msg) = msg {
        print!("{msg}");
        std::io::stdout().flush();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input);
        input = input.trim().to_string();
        Ok((Some(Value::String(input)), Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _error(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let msg = context.get_var(&"msg".to_string()).unwrap();
    if let Value::String(msg) = msg {
        Err(Error::Error(msg.clone()))
    } else { panic!("type checking doesn't work") }
}
// fs
fn _import(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let path = context.get_var(&"path".to_string()).unwrap();
    if let Value::String(path) = path {
        let mut file_context = Context::new(path.clone(), context.std_path.clone());
        file_context.global = context.global.clone();
        run_file_context(path, &mut file_context)?;
        context.global = file_context.global;
        Ok((None, Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _import_var(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let path = context.get_var(&"path".to_string()).unwrap();
    if let Value::String(path) = path {
        let mut file_context = Context::new(path.clone(), context.std_path.clone());
        file_context.global = context.global.clone();
        run_file_context(path, &mut file_context)?;
        context.global = file_context.global;
        Ok((None, Return::None))
    } else { panic!("type checking doesn't work") }
}
fn _read_file(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let path = context.get_var(&"path".to_string()).unwrap();
    let pos = context.get_var_pos(&"path".to_string()).unwrap();
    if let Value::String(path) = path {
        match fs::read_to_string(path) {
            Ok(content) => Ok((Some(Value::String(content)), Return::None)),
            Err(_) => Err(Error::FileNotFound(path.clone()))
        }
    } else { panic!("type checking doesn't work") }
}

pub fn std_context(path: String, std_dir_path: Option<String>) -> Result<Context, Error> {
    let mut context = Context::new(path, std_dir_path.clone());
    let pos = Position::new(0..0, 0..0, &String::from("<STD>"));
    // let
    context.create_native_fn(String::from("let"), NativFunction {
        params: vec![(":id".to_string(), Type::Key, false), (":v".to_string(), Type::Any, false)],
        return_type: None,
        body: _let,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("mut"), NativFunction {
        params: vec![(":id".to_string(), Type::Key, false), (":v".to_string(), Type::Any, false)],
        return_type: None,
        body: _mut,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("set"), NativFunction {
        params: vec![(":id".to_string(), Type::Key, false), (":v".to_string(), Type::Any, false)],
        return_type: None,
        body: _set,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("set"), NativFunction {
        params: vec![(":path".to_string(), Type::Path, false), (":v".to_string(), Type::Any, false)],
        return_type: None,
        body: _set_path,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("set"), NativFunction {
        params: vec![(":index".to_string(), Type::Index, false), (":v".to_string(), Type::Any, false)],
        return_type: None,
        body: _set_index,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("get"), NativFunction {
        params: vec![(":id".to_string(), Type::Key, false)],
        return_type: Some(Type::Any),
        body: _get,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("get"), NativFunction {
        params: vec![(":path".to_string(), Type::Path, false)],
        return_type: Some(Type::Any),
        body: _get_path,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("get"), NativFunction {
        params: vec![(":index".to_string(), Type::Index, false)],
        return_type: Some(Type::Any),
        body: _get_index,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("exist?"), NativFunction {
        params: vec![(":id".to_string(), Type::Key, false)],
        return_type: Some(Type::Bool),
        body: _exist,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("exist?"), NativFunction {
        params: vec![(":path".to_string(), Type::Path, false)],
        return_type: Some(Type::Bool),
        body: _exist_path,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("exist?"), NativFunction {
        params: vec![(":index".to_string(), Type::Index, false)],
        return_type: Some(Type::Bool),
        body: _exist_index,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("mut?"), NativFunction {
        params: vec![(":id".to_string(), Type::Key, false)],
        return_type: Some(Type::Bool),
        body: _is_mut,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("mut?"), NativFunction {
        params: vec![(":path".to_string(), Type::Path, false)],
        return_type: Some(Type::Bool),
        body: _is_mut_path,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("mut?"), NativFunction {
        params: vec![(":index".to_string(), Type::Index, false)],
        return_type: Some(Type::Bool),
        body: _is_mut_index,
        inline: true
    }, pos.clone())?;
    // def
    context.create_native_fn(String::from("def"), NativFunction {
        params: vec![
            (":id".to_string(), Type::Key, false),
            (":p".to_string(), Type::Params, false),
            (":body".to_string(), Type::Closure, false),
            (":return_type".to_string(), Type::Type, false),
        ],
        return_type: None,
        body: _def_return,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("def"), NativFunction {
        params: vec![
            (":id".to_string(), Type::Key, false),
            (":p".to_string(), Type::Params, false),
            (":body".to_string(), Type::Closure, false),
        ],
        return_type: None,
        body: _def,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("def-inline"), NativFunction {
        params: vec![
            (":id".to_string(), Type::Key, false),
            (":p".to_string(), Type::Params, false),
            (":body".to_string(), Type::Closure, false),
            (":return_type".to_string(), Type::Type, false),
        ],
        return_type: None,
        body: _def_return_inline,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("def-inline"), NativFunction {
        params: vec![
            (":id".to_string(), Type::Key, false),
            (":p".to_string(), Type::Params, false),
            (":body".to_string(), Type::Closure, false),
        ],
        return_type: None,
        body: _def_inline,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("def-global"), NativFunction {
        params: vec![
            (":id".to_string(), Type::Key, false),
            (":p".to_string(), Type::Params, false),
            (":body".to_string(), Type::Closure, false),
            (":return_type".to_string(), Type::Type, false),
        ],
        return_type: None,
        body: _def_return_global,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("def-global"), NativFunction {
        params: vec![
            (":id".to_string(), Type::Key, false),
            (":p".to_string(), Type::Params, false),
            (":body".to_string(), Type::Closure, false),
        ],
        return_type: None,
        body: _def_global,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("def-global-inline"), NativFunction {
        params: vec![
            (":id".to_string(), Type::Key, false),
            (":p".to_string(), Type::Params, false),
            (":body".to_string(), Type::Closure, false),
            (":return_type".to_string(), Type::Type, false),
        ],
        return_type: None,
        body: _def_return_global_inline,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("def-global-inline"), NativFunction {
        params: vec![
            (":id".to_string(), Type::Key, false),
            (":p".to_string(), Type::Params, false),
            (":body".to_string(), Type::Closure, false),
        ],
        return_type: None,
        body: _def_global_inline,
        inline: true
    }, pos.clone())?;
    // control flow
    context.create_native_fn(String::from("return"), NativFunction {
        params: vec![("v".to_string(), Type::Any, false)],
        return_type: Some(Type::Any),
        body: _return,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("break"), NativFunction {
        params: vec![],
        return_type: None,
        body: _break,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("do"), NativFunction {
        params: vec![(":node".to_string(), Type::Closure, false)],
        return_type: Some(Type::Any),
        body: _do,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("?"), NativFunction {
        params: vec![
            ("cond".to_string(), Type::Bool, false),
            ("case".to_string(), Type::Any, false),
            ("else".to_string(), Type::Any, false)
        ],
        return_type: Some(Type::Any),
        body: _if,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("if"), NativFunction {
        params: vec![
            (":cond".to_string(), Type::Bool, false),
            (":case".to_string(), Type::Any, false)
        ],
        return_type: Some(Type::Any),
        body: _if_do,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("if"), NativFunction {
        params: vec![
            (":cond".to_string(), Type::Bool, false),
            (":case".to_string(), Type::Any, false),
            (":else".to_string(), Type::Any, false)
        ],
        return_type: Some(Type::Any),
        body: _if_do_else,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("for"), NativFunction {
        params: vec![
            (":id".to_string(), Type::Key, false),
            (":iter".to_string(), Type::Vector(Some(Box::new(Type::Any))), false),
            (":body".to_string(), Type::Closure, false)
        ],
        return_type: None,
        body: _for,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("for"), NativFunction {
        params: vec![
            (":id".to_string(), Type::Key, false),
            (":length".to_string(), Type::Int, false),
            (":body".to_string(), Type::Closure, false)
        ],
        return_type: None,
        body: _for_length,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("for"), NativFunction {
        params: vec![
            (":id".to_string(), Type::Key, false),
            (":start".to_string(), Type::Int, false),
            (":end".to_string(), Type::Int, false),
            (":body".to_string(), Type::Closure, false)
        ],
        return_type: None,
        body: _for_range,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("while"), NativFunction {
        params: vec![
            (":cond".to_string(), Type::Closure, false),
            (":body".to_string(), Type::Closure, false)
        ],
        return_type: None,
        body: _while,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("loop"), NativFunction {
        params: vec![(":body".to_string(), Type::Closure, false)],
        return_type: None,
        body: _loop,
        inline: true
    }, pos.clone())?;
    // +
    context.create_native_fn(String::from("+"), NativFunction {
        params: vec![("n".to_string(), Type::Int, false), ("nums".to_string(), Type::Int, true)],
        return_type: Some(Type::Int),
        body: _add_int,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("+"), NativFunction {
        params: vec![("n".to_string(), Type::Float, false), ("nums".to_string(), Type::Float, true)],
        return_type: Some(Type::Float),
        body: _add_float,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("+"), NativFunction {
        params: vec![("n".to_string(), Type::String, false), ("nums".to_string(), Type::String, true)],
        return_type: Some(Type::String),
        body: _add_str,
        inline: false
    }, pos.clone())?;
    // -
    context.create_native_fn(String::from("-"), NativFunction {
        params: vec![("n".to_string(), Type::Int, false)],
        return_type: Some(Type::Int),
        body: _neg_int,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("-"), NativFunction {
        params: vec![("n".to_string(), Type::Float, false)],
        return_type: Some(Type::Float),
        body: _neg_float,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("-"), NativFunction {
        params: vec![("n".to_string(), Type::Int, false), ("nums".to_string(), Type::Int, true)],
        return_type: Some(Type::Int),
        body: _sub_int,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("-"), NativFunction {
        params: vec![("n".to_string(), Type::Float, false), ("nums".to_string(), Type::Float, true)],
        return_type: Some(Type::Float),
        body: _sub_float,
        inline: false
    }, pos.clone())?;
    // *
    context.create_native_fn(String::from("*"), NativFunction {
        params: vec![("n".to_string(), Type::Int, false), ("nums".to_string(), Type::Int, true)],
        return_type: Some(Type::Int),
        body: _mul_int,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("*"), NativFunction {
        params: vec![("n".to_string(), Type::Float, false), ("nums".to_string(), Type::Float, true)],
        return_type: Some(Type::Float),
        body: _mul_float,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("*"), NativFunction {
        params: vec![("s".to_string(), Type::String, false), ("n".to_string(), Type::Int, false)],
        return_type: Some(Type::String),
        body: _mul_str,
        inline: false
    }, pos.clone())?;
    // /
    context.create_native_fn(String::from("/"), NativFunction {
        params: vec![("n".to_string(), Type::Int, false), ("nums".to_string(), Type::Int, true)],
        return_type: Some(Type::Int),
        body: _div_int,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("/"), NativFunction {
        params: vec![("n".to_string(), Type::Float, false), ("nums".to_string(), Type::Float, true)],
        return_type: Some(Type::Float),
        body: _div_float,
        inline: false
    }, pos.clone())?;
    // %
    context.create_native_fn(String::from("%"), NativFunction {
        params: vec![("n".to_string(), Type::Int, false), ("nums".to_string(), Type::Int, true)],
        return_type: Some(Type::Int),
        body: _mod_int,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("%"), NativFunction {
        params: vec![("n".to_string(), Type::Float, false), ("nums".to_string(), Type::Float, true)],
        return_type: Some(Type::Float),
        body: _mod_float,
        inline: false
    }, pos.clone())?;
    // =
    context.create_native_fn(String::from("="), NativFunction {
        params: vec![("a".to_string(), Type::Any, false), ("b".to_string(), Type::Any, false)],
        return_type: Some(Type::Bool),
        body: _eq,
        inline: false
    }, pos.clone())?;
    // !=
    context.create_native_fn(String::from("!="), NativFunction {
        params: vec![("a".to_string(), Type::Any, false), ("b".to_string(), Type::Any, false)],
        return_type: Some(Type::Bool),
        body: _neq,
        inline: false
    }, pos.clone())?;
    // >
    context.create_native_fn(String::from(">"), NativFunction {
        params: vec![("a".to_string(), Type::Int, false), ("b".to_string(), Type::Int, false)],
        return_type: Some(Type::Bool),
        body: _gt_int,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from(">"), NativFunction {
        params: vec![("a".to_string(), Type::Float, false), ("b".to_string(), Type::Float, false)],
        return_type: Some(Type::Bool),
        body: _gt_float,
        inline: false
    }, pos.clone())?;
    // >=
    context.create_native_fn(String::from(">="), NativFunction {
        params: vec![("a".to_string(), Type::Int, false), ("b".to_string(), Type::Int, false)],
        return_type: Some(Type::Bool),
        body: _ge_int,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from(">="), NativFunction {
        params: vec![("a".to_string(), Type::Float, false), ("b".to_string(), Type::Float, false)],
        return_type: Some(Type::Bool),
        body: _ge_float,
        inline: false
    }, pos.clone())?;
    // <
    context.create_native_fn(String::from("<"), NativFunction {
        params: vec![("a".to_string(), Type::Int, false), ("b".to_string(), Type::Int, false)],
        return_type: Some(Type::Bool),
        body: _lt_int,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("<"), NativFunction {
        params: vec![("a".to_string(), Type::Float, false), ("b".to_string(), Type::Float, false)],
        return_type: Some(Type::Bool),
        body: _lt_float,
        inline: false
    }, pos.clone())?;
    // <=
    context.create_native_fn(String::from("<="), NativFunction {
        params: vec![("a".to_string(), Type::Int, false), ("b".to_string(), Type::Int, false)],
        return_type: Some(Type::Bool),
        body: _le_int,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("<="), NativFunction {
        params: vec![("a".to_string(), Type::Float, false), ("b".to_string(), Type::Float, false)],
        return_type: Some(Type::Bool),
        body: _le_float,
        inline: false
    }, pos.clone())?;
    // len
    context.create_native_fn(String::from("len"), NativFunction {
        params: vec![("v".to_string(), Type::Vector(Some(Box::new(Type::Any))), false)],
        return_type: Some(Type::Int),
        body: _len_vec,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("len"), NativFunction {
        params: vec![("v".to_string(), Type::String, false)],
        return_type: Some(Type::Int),
        body: _len_str,
        inline: false
    }, pos.clone())?;
    // int
    context.create_native_fn(String::from("int"), NativFunction {
        params: vec![("v".to_string(), Type::Int, false)],
        return_type: Some(Type::Int),
        body: _int_int,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("int"), NativFunction {
        params: vec![("v".to_string(), Type::Float, false)],
        return_type: Some(Type::Int),
        body: _int_float,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("int"), NativFunction {
        params: vec![("v".to_string(), Type::Char, false)],
        return_type: Some(Type::Int),
        body: _int_char,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("int"), NativFunction {
        params: vec![("v".to_string(), Type::Bool, false)],
        return_type: Some(Type::Int),
        body: _int_bool,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("int"), NativFunction {
        params: vec![("v".to_string(), Type::String, false)],
        return_type: Some(Type::Int),
        body: _int_str,
        inline: false
    }, pos.clone())?;
    // float
    context.create_native_fn(String::from("float"), NativFunction {
        params: vec![("v".to_string(), Type::Int, false)],
        return_type: Some(Type::Float),
        body: _float_int,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("float"), NativFunction {
        params: vec![("v".to_string(), Type::Float, false)],
        return_type: Some(Type::Float),
        body: _float_float,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("float"), NativFunction {
        params: vec![("v".to_string(), Type::String, false)],
        return_type: Some(Type::Float),
        body: _float_str,
        inline: false
    }, pos.clone())?;
    // bool
    context.create_native_fn(String::from("bool"), NativFunction {
        params: vec![("v".to_string(), Type::Int, false)],
        return_type: Some(Type::Bool),
        body: _bool_int,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("bool"), NativFunction {
        params: vec![("v".to_string(), Type::Float, false)],
        return_type: Some(Type::Bool),
        body: _bool_float,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("bool"), NativFunction {
        params: vec![("v".to_string(), Type::Char, false)],
        return_type: Some(Type::Bool),
        body: _bool_char,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("bool"), NativFunction {
        params: vec![("v".to_string(), Type::Bool, false)],
        return_type: Some(Type::Bool),
        body: _bool_bool,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("bool"), NativFunction {
        params: vec![("v".to_string(), Type::String, false)],
        return_type: Some(Type::Bool),
        body: _bool_str,
        inline: false
    }, pos.clone())?;
    // char
    context.create_native_fn(String::from("char"), NativFunction {
        params: vec![("v".to_string(), Type::Int, false)],
        return_type: Some(Type::Char),
        body: _char_int,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("char"), NativFunction {
        params: vec![("v".to_string(), Type::Char, false)],
        return_type: Some(Type::Char),
        body: _char_char,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("char"), NativFunction {
        params: vec![("v".to_string(), Type::String, false)],
        return_type: Some(Type::Char),
        body: _char_str,
        inline: false
    }, pos.clone())?;
    // str
    context.create_native_fn(String::from("str"), NativFunction {
        params: vec![("v".to_string(), Type::Any, false)],
        return_type: Some(Type::String),
        body: _str,
        inline: false
    }, pos.clone())?;
    // key
    context.create_native_fn(String::from("key"), NativFunction {
        params: vec![("v".to_string(), Type::Any, false)],
        return_type: Some(Type::Key),
        body: _key,
        inline: false
    }, pos.clone())?;
    // path
    context.create_native_fn(String::from("path"), NativFunction {
        params: vec![("head".to_string(), Type::Key, false), ("sub".to_string(), Type::Key, false)],
        return_type: Some(Type::Path),
        body: _path_key,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("path"), NativFunction {
        params: vec![("head".to_string(), Type::Path, false), ("sub".to_string(), Type::Key, false)],
        return_type: Some(Type::Path),
        body: _path_path,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("path"), NativFunction {
        params: vec![("head".to_string(), Type::Index, false), ("sub".to_string(), Type::Key, false)],
        return_type: Some(Type::Path),
        body: _path_index,
        inline: false
    }, pos.clone())?;
    // index
    context.create_native_fn(String::from("index"), NativFunction {
        params: vec![("head".to_string(), Type::Key, false), ("idx".to_string(), Type::Int, false)],
        return_type: Some(Type::Index),
        body: _index_key,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("index"), NativFunction {
        params: vec![("head".to_string(), Type::Path, false), ("idx".to_string(), Type::Int, false)],
        return_type: Some(Type::Index),
        body: _index_path,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("index"), NativFunction {
        params: vec![("head".to_string(), Type::Index, false), ("idx".to_string(), Type::Int, false)],
        return_type: Some(Type::Index),
        body: _index_index,
        inline: false
    }, pos.clone())?;
    // vec
    context.create_native_fn(String::from("vec"), NativFunction {
        params: vec![("t".to_string(), Type::Type, false)],
        return_type: Some(Type::Vector(Some(Box::new(Type::Any)))),
        body: _vec,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("vec"), NativFunction {
        params: vec![
            ("t".to_string(), Type::Type, false),
            ("v".to_string(), Type::Vector(Some(Box::new(Type::Any))), false)
        ],
        return_type: Some(Type::Vector(Some(Box::new(Type::Any)))),
        body: _vec_of,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("push"), NativFunction {
        params: vec![(":id".to_string(), Type::Key, false), (":v".to_string(), Type::Any, false)],
        return_type: Some(Type::Any),
        body: _vec_push,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("str-push"), NativFunction {
        params: vec![(":id".to_string(), Type::Key, false), (":c".to_string(), Type::Char, false)],
        return_type: None,
        body: _str_push,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("pop"), NativFunction {
        params: vec![(":id".to_string(), Type::Key, false)],
        return_type: Some(Type::Any),
        body: _vec_pop,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("pop"), NativFunction {
        params: vec![(":id".to_string(), Type::Key, false), (":idx".to_string(), Type::Int, false)],
        return_type: Some(Type::Any),
        body: _vec_pop_idx,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("str-pop"), NativFunction {
        params: vec![(":id".to_string(), Type::Key, false)],
        return_type: Some(Type::Char),
        body: _str_pop,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("str-pop"), NativFunction {
        params: vec![(":id".to_string(), Type::Key, false), (":idx".to_string(), Type::Int, false)],
        return_type: Some(Type::Char),
        body: _str_pop_idx,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("contains"), NativFunction {
        params: vec![
            ("values".to_string(), Type::Vector(Some(Box::new(Type::Any))), false),
            ("value".to_string(), Type::Any, false)
        ],
        return_type: Some(Type::Bool),
        body: _contains,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("contains"), NativFunction {
        params: vec![
            ("s".to_string(), Type::String, false),
            ("c".to_string(), Type::Char, false)
        ],
        return_type: Some(Type::Bool),
        body: _contains_str,
        inline: false
    }, pos.clone())?;
    // type
    context.create_native_fn(String::from("type"), NativFunction {
        params: vec![("v".to_string(), Type::Any, false)],
        return_type: Some(Type::Type),
        body: _type,
        inline: false
    }, pos.clone())?;
    // io
    context.create_native_fn(String::from("write"), NativFunction {
        params: vec![("v".to_string(), Type::Any, true)],
        return_type: None,
        body: _write,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("print"), NativFunction {
        params: vec![("v".to_string(), Type::Any, true)],
        return_type: None,
        body: _print,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("input"), NativFunction {
        params: vec![("msg".to_string(), Type::String, false)],
        return_type: Some(Type::String),
        body: _input,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("error!"), NativFunction {
        params: vec![("msg".to_string(), Type::String, false)],
        return_type: None,
        body: _error,
        inline: false
    }, pos.clone())?;
    // fs
    context.create_native_fn(String::from("import"), NativFunction {
        params: vec![("path".to_string(), Type::String, false)],
        return_type: None,
        body: _import,
        inline: false
    }, pos.clone())?;
    context.create_native_fn(String::from("read-file"), NativFunction {
        params: vec![("path".to_string(), Type::String, false)],
        return_type: Some(Type::String),
        body: _read_file,
        inline: false
    }, pos.clone())?;
    match std_dir_path {
        Some(std_dir_path) => {
            let std_path = std_dir_path.clone() + "\\std.nd";
            run_file_context(&std_path, &mut context)?;
        },
        None => {}
    }
    context.scopes = vec![Scope::new()];
    Ok(context)
}