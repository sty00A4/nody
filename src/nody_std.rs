use crate::*;

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
pub fn std_context() -> Result<Context, Error> {
    let mut context = Context::new();
    let pos = Position::new(0..0, 0..0, &String::from("<STD>"));
    // +
    context.create_native_fn(String::from("+"), NativFunction {
        params: vec![("n".to_string(), Type::Int, false), ("nums".to_string(), Type::Int, true)],
        return_type: Some(Type::Int),
        body: _add_int
    }, pos.clone())?;
    context.create_native_fn(String::from("+"), NativFunction {
        params: vec![("n".to_string(), Type::Float, false), ("nums".to_string(), Type::Float, true)],
        return_type: Some(Type::Float),
        body: _add_float
    }, pos.clone())?;
    context.create_native_fn(String::from("+"), NativFunction {
        params: vec![("n".to_string(), Type::String, false), ("nums".to_string(), Type::String, true)],
        return_type: Some(Type::String),
        body: _add_str
    }, pos.clone())?;
    // -
    context.create_native_fn(String::from("-"), NativFunction {
        params: vec![("n".to_string(), Type::Int, false)],
        return_type: Some(Type::Int),
        body: _neg_int
    }, pos.clone())?;
    context.create_native_fn(String::from("-"), NativFunction {
        params: vec![("n".to_string(), Type::Float, false)],
        return_type: Some(Type::Float),
        body: _neg_float
    }, pos.clone())?;
    context.create_native_fn(String::from("-"), NativFunction {
        params: vec![("n".to_string(), Type::Int, false), ("nums".to_string(), Type::Int, true)],
        return_type: Some(Type::Int),
        body: _sub_int
    }, pos.clone())?;
    context.create_native_fn(String::from("-"), NativFunction {
        params: vec![("n".to_string(), Type::Float, false), ("nums".to_string(), Type::Float, true)],
        return_type: Some(Type::Float),
        body: _sub_float
    }, pos.clone())?;
    // *
    context.create_native_fn(String::from("*"), NativFunction {
        params: vec![("n".to_string(), Type::Int, false), ("nums".to_string(), Type::Int, true)],
        return_type: Some(Type::Int),
        body: _mul_int
    }, pos.clone())?;
    context.create_native_fn(String::from("*"), NativFunction {
        params: vec![("n".to_string(), Type::Float, false), ("nums".to_string(), Type::Float, true)],
        return_type: Some(Type::Float),
        body: _mul_float
    }, pos.clone())?;
    context.create_native_fn(String::from("*"), NativFunction {
        params: vec![("s".to_string(), Type::String, false), ("n".to_string(), Type::Int, false)],
        return_type: Some(Type::String),
        body: _mul_str
    }, pos.clone())?;
    // /
    context.create_native_fn(String::from("/"), NativFunction {
        params: vec![("n".to_string(), Type::Int, false), ("nums".to_string(), Type::Int, true)],
        return_type: Some(Type::Int),
        body: _div_int
    }, pos.clone())?;
    context.create_native_fn(String::from("/"), NativFunction {
        params: vec![("n".to_string(), Type::Float, false), ("nums".to_string(), Type::Float, true)],
        return_type: Some(Type::Float),
        body: _div_float
    }, pos.clone())?;
    // %
    context.create_native_fn(String::from("%"), NativFunction {
        params: vec![("n".to_string(), Type::Int, false), ("nums".to_string(), Type::Int, true)],
        return_type: Some(Type::Int),
        body: _mod_int
    }, pos.clone())?;
    context.create_native_fn(String::from("%"), NativFunction {
        params: vec![("n".to_string(), Type::Float, false), ("nums".to_string(), Type::Float, true)],
        return_type: Some(Type::Float),
        body: _mod_float
    }, pos.clone())?;
    Ok(context)
}