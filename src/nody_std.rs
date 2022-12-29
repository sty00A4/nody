use crate::*;

// let
fn _let(context: &mut Context) -> Result<Option<Value>, Error> {
    let id = context.get_var(&"id".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&"id".to_string()).unwrap().clone();
    let v = context.get_var(&"v".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        let len = context.scopes.len();
        match context.scopes.get_mut(len - 2) {
            Some(scope) => scope.create_var(id, v, false, pos, false)?,
            None => context.create_var(id, v, false, pos, false)?
        }
        Ok(None)
    } else { panic!("type checking doesn't work") }
}
fn _mut(context: &mut Context) -> Result<Option<Value>, Error> {
    let id = context.get_var(&"id".to_string()).unwrap().clone();
    let pos = context.get_var_pos(&"id".to_string()).unwrap().clone();
    let v = context.get_var(&"v".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        let len = context.scopes.len();
        match context.scopes.get_mut(len - 2) {
            Some(scope) => scope.create_var(id, v, true, pos, false)?,
            None => context.create_var(id, v, true, pos, false)?
        }
        Ok(None)
    } else { panic!("type checking doesn't work") }
}
fn _set(context: &mut Context) -> Result<Option<Value>, Error> {
    let id = context.get_var(&"id".to_string()).unwrap().clone();
    let v = context.get_var(&"v".to_string()).unwrap().clone();
    if let Value::Key(id) = id {
        if !context.is_mutable(&id).unwrap() { return Err(Error::Immutable(id)) }
        match context.get_var_mut(&id) {
            Some(value) => { *value = v; Ok(None) }
            None => Err(Error::NotDefined(id))
        }
    } else { panic!("type checking doesn't work") }
}
// +
fn _add_int(context: &mut Context) -> Result<Option<Value>, Error> {
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
        Ok(Some(Value::Int(sum)))
    } else { panic!("type checking doesn't work") }
}
fn _add_float(context: &mut Context) -> Result<Option<Value>, Error> {
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
        Ok(Some(Value::Float(sum)))
    } else { panic!("type checking doesn't work") }
}
fn _add_str(context: &mut Context) -> Result<Option<Value>, Error> {
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
        Ok(Some(Value::String(string)))
    } else { panic!("type checking doesn't work") }
}
// -
fn _sub_int(context: &mut Context) -> Result<Option<Value>, Error> {
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
        Ok(Some(Value::Int(sum)))
    } else { panic!("type checking doesn't work"); }
}
fn _neg_int(context: &mut Context) -> Result<Option<Value>, Error> {
    let n = context.get_var(&"n".to_string()).unwrap();
    if let Value::Int(n) = n {
        Ok(Some(Value::Int(-n)))
    } else { panic!("type checking doesn't work") }
}
fn _sub_float(context: &mut Context) -> Result<Option<Value>, Error> {
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
        Ok(Some(Value::Float(sum)))
    } else { panic!("type checking doesn't work") }
}
fn _neg_float(context: &mut Context) -> Result<Option<Value>, Error> {
    let n = context.get_var(&"n".to_string()).unwrap();
    if let Value::Float(n) = n {
        Ok(Some(Value::Float(-n)))
    } else { panic!("type checking doesn't work") }
}
// *
fn _mul_int(context: &mut Context) -> Result<Option<Value>, Error> {
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
        Ok(Some(Value::Int(sum)))
    } else { panic!("type checking doesn't work") }
}
fn _mul_float(context: &mut Context) -> Result<Option<Value>, Error> {
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
        Ok(Some(Value::Float(sum)))
    } else { panic!("type checking doesn't work") }
}
fn _mul_str(context: &mut Context) -> Result<Option<Value>, Error> {
    let s = context.get_var(&"s".to_string()).unwrap();
    let n = context.get_var(&"n".to_string()).unwrap();
    if let Value::String(s) = s {
        if let Value::Int(n) = n {
            Ok(Some(Value::String(s.repeat(max::<i64>(*n, 0) as usize))))
        } else { panic!("type checking doesn't work") }
    } else { panic!("type checking doesn't work") }
}
// /
fn _div_int(context: &mut Context) -> Result<Option<Value>, Error> {
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
        Ok(Some(Value::Int(sum)))
    } else { panic!("type checking doesn't work") }
}
fn _div_float(context: &mut Context) -> Result<Option<Value>, Error> {
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
        Ok(Some(Value::Float(sum)))
    } else { panic!("type checking doesn't work") }
}
// %
fn _mod_int(context: &mut Context) -> Result<Option<Value>, Error> {
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
        Ok(Some(Value::Int(sum)))
    } else { panic!("type checking doesn't work") }
}
fn _mod_float(context: &mut Context) -> Result<Option<Value>, Error> {
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
        Ok(Some(Value::Float(sum)))
    } else { panic!("type checking doesn't work") }
}
// int
fn _int_int(context: &mut Context) -> Result<Option<Value>, Error> {
    Ok(Some(context.get_var(&"v".to_string()).unwrap().clone()))
}
fn _int_float(context: &mut Context) -> Result<Option<Value>, Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Float(v) = v {
        Ok(Some(Value::Int(*v as i64)))
    } else { panic!("type checking doesn't work") }
}
fn _int_char(context: &mut Context) -> Result<Option<Value>, Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Char(v) = v {
        Ok(Some(Value::Int(*v as i64)))
    } else { panic!("type checking doesn't work") }
}
fn _int_bool(context: &mut Context) -> Result<Option<Value>, Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Bool(v) = v {
        Ok(Some(Value::Int(*v as i64)))
    } else { panic!("type checking doesn't work") }
}
fn _int_str(context: &mut Context) -> Result<Option<Value>, Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::String(v) = v {
        if let Ok(v) = v.parse::<i64>() {
            Ok(Some(Value::Int(v)))
        } else {
            Err(Error::ParseInt(v.clone()))
        }
    } else { panic!("type checking doesn't work") }
}
// float
fn _float_int(context: &mut Context) -> Result<Option<Value>, Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Int(v) = v {
        Ok(Some(Value::Float(*v as f64)))
    } else { panic!("type checking doesn't work") }
}
fn _float_float(context: &mut Context) -> Result<Option<Value>, Error> {
    Ok(Some(context.get_var(&"v".to_string()).unwrap().clone()))
}
fn _float_str(context: &mut Context) -> Result<Option<Value>, Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::String(v) = v {
        if let Ok(v) = v.parse::<f64>() {
            Ok(Some(Value::Float(v)))
        } else {
            Err(Error::ParseFloat(v.clone()))
        }
    } else { panic!("type checking doesn't work") }
}
// bool
fn _bool_int(context: &mut Context) -> Result<Option<Value>, Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Int(v) = v {
        Ok(Some(Value::Bool(*v != 0)))
    } else { panic!("type checking doesn't work") }
}
fn _bool_float(context: &mut Context) -> Result<Option<Value>, Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Float(v) = v {
        Ok(Some(Value::Bool(*v != 0.0)))
    } else { panic!("type checking doesn't work") }
}
fn _bool_char(context: &mut Context) -> Result<Option<Value>, Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Char(v) = v {
        Ok(Some(Value::Bool(*v as u8 != 0)))
    } else { panic!("type checking doesn't work") }
}
fn _bool_bool(context: &mut Context) -> Result<Option<Value>, Error> {
    Ok(Some(context.get_var(&"v".to_string()).unwrap().clone()))
}
fn _bool_str(context: &mut Context) -> Result<Option<Value>, Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::String(v) = v {
        if let Ok(v) = v.parse::<bool>() {
            Ok(Some(Value::Bool(v)))
        } else {
            Err(Error::ParseBool(v.clone()))
        }
    } else { panic!("type checking doesn't work") }
}
// char
fn _char_int(context: &mut Context) -> Result<Option<Value>, Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::Int(v) = v {
        Ok(Some(Value::Char(*v as u8 as char)))
    } else { panic!("type checking doesn't work") }
}
fn _char_char(context: &mut Context) -> Result<Option<Value>, Error> {
    Ok(Some(context.get_var(&"v".to_string()).unwrap().clone()))
}
fn _char_str(context: &mut Context) -> Result<Option<Value>, Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    if let Value::String(v) = v {
        if let Ok(v) = v.parse::<char>() {
            Ok(Some(Value::Char(v)))
        } else {
            Err(Error::ParseChar(v.clone()))
        }
    } else { panic!("type checking doesn't work") }
}
// str
fn _str(context: &mut Context) -> Result<Option<Value>, Error> {
    Ok(Some(Value::String(context.get_var(&"v".to_string()).unwrap().to_string())))
}
// key
fn _key(context: &mut Context) -> Result<Option<Value>, Error> {
    Ok(Some(Value::Key(context.get_var(&"v".to_string()).unwrap().to_string())))
}
// vec
fn _vec(context: &mut Context) -> Result<Option<Value>, Error> {
    let t = context.get_var(&"t".to_string()).unwrap();
    if let Value::Type(t) = t {
        Ok(Some(Value::Type(Type::Vector(Some(Box::new(t.clone()))))))
    } else { panic!("type checking doesn't work") }
}
// type
fn _type(context: &mut Context) -> Result<Option<Value>, Error> {
    Ok(Some(Value::Type(context.get_var(&"v".to_string()).unwrap().typ())))
}
// io
fn _print(context: &mut Context) -> Result<Option<Value>, Error> {
    let v = context.get_var(&"v".to_string()).unwrap();
    println!("{v}");
    Ok(None)
}

pub fn std_context() -> Result<Context, Error> {
    let mut context = Context::new();
    let pos = Position::new(0..0, 0..0, &String::from("<STD>"));
    // let
    context.create_native_fn(String::from("let"), NativFunction {
        params: vec![("id".to_string(), Type::Key, false), ("v".to_string(), Type::Any, false)],
        return_type: None,
        body: _let,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("mut"), NativFunction {
        params: vec![("id".to_string(), Type::Key, false), ("v".to_string(), Type::Any, false)],
        return_type: None,
        body: _mut,
        inline: true
    }, pos.clone())?;
    context.create_native_fn(String::from("set"), NativFunction {
        params: vec![("id".to_string(), Type::Key, false), ("v".to_string(), Type::Any, false)],
        return_type: None,
        body: _set,
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
    // print
    context.create_native_fn(String::from("print"), NativFunction {
        params: vec![("v".to_string(), Type::Any, false)],
        return_type: None,
        body: _print,
        inline: false
    }, pos.clone())?;
    Ok(context)
}