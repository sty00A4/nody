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
        context.change(id, v);
        Ok((None, Return::None))
    } else { panic!("type checking doesn't work") }
}
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
        let (value, _) = interpret(&node, context)?;
        Ok((value, Return::None))
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
// vec
fn _vec(context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    let t = context.get_var(&"t".to_string()).unwrap();
    if let Value::Type(t) = t {
        Ok((Some(Value::Type(Type::Vector(Some(Box::new(t.clone()))))), Return::None))
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

pub fn std_context() -> Result<Context, Error> {
    let mut context = Context::new();
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
    // vec
    context.create_native_fn(String::from("vec"), NativFunction {
        params: vec![("t".to_string(), Type::Type, false)],
        return_type: Some(Type::Vector(Some(Box::new(Type::Any)))),
        body: _vec,
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
    let std_path = String::from("nody_std/std.nd");
    let _ = run_file_context(&std_path, &mut context)?;
    context.scopes = vec![Scope::new()];
    Ok(context)
}