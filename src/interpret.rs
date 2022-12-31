use crate::*;

pub fn find_index(idx: &i64, size: usize) -> usize {
    if *idx < 0 { size - idx.abs() as usize } else { *idx as usize }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Return { None, Return, Break, Continue }
pub fn interpret(node: &Node, context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    match node {
        Node::None { pos:_ } => Ok((None, Return::None)),
        Node::Int { v, pos:_ } => Ok((Some(Value::Int(*v)), Return::None)),
        Node::Float { v, pos:_ } => Ok((Some(Value::Float(*v)), Return::None)),
        Node::Char { v, pos:_ } => Ok((Some(Value::Char(*v)), Return::None)),
        Node::Bool { v, pos:_ } => Ok((Some(Value::Bool(*v)), Return::None)),
        Node::String { v, pos:_ } => Ok((Some(Value::String(v.clone())), Return::None)),
        Node::Vector { nodes, pos:_ } => {
            let mut values: Vec<Value> = vec![];
            let mut typ: Option<Type> = None;
            for n in nodes.iter() {
                let (value, _) = interpret(n, context)?;
                if value.is_none() {
                    context.trace_push(n.pos());
                    return Err(Error::Expected)
                }
                let value = value.unwrap();
                if typ.is_none() {
                    typ = Some(value.typ());
                } else if typ != Some(value.typ()) {
                    context.trace_push(n.pos());
                    return Err(Error::ExpectedType(typ.unwrap(), value.typ()))
                }
                values.push(value);
            }
            Ok((Some(Value::Vector(values, typ)), Return::None))
        }
        Node::Type { v, pos:_ } => Ok((Some(Value::Type(v.clone())), Return::None)),
        Node::Word { v, pos } => match context.get_var(v) {
            Some(v) => Ok((Some(v.clone()), Return::None)),
            None => {
                context.trace_push(pos);
                Err(Error::NotDefined(v.clone()))
            }
        }
        Node::Key { v, pos:_ } => Ok((Some(Value::Key(v.clone())), Return::None)),
        Node::Closure { node, pos } => Ok((Some(Value::Closure(node.as_ref().clone())), Return::None)),
        Node::Params { params: node_params, pos } => {
            let mut params: Vec<(String, Type, bool)> = vec![];
            for (param, type_node, more) in node_params.iter() {
                let (typ, _) = interpret(type_node, context)?;
                if let Some(typ) = typ {
                    if let Value::Type(typ) = typ {
                        params.push((param.clone(), typ, *more));
                    } else {
                        context.trace_push(pos);
                        return Err(Error::ExpectedType(Type::Type, typ.typ()))
                    }
                } else {
                    context.trace_push(pos);
                    return Err(Error::Expected)
                }
            }
            Ok((Some(Value::Params(params)), Return::None))
        }
        Node::Body { nodes, pos:_ } => {
            context.push();
            for node in nodes.iter() {
                let (value, ret) = interpret(node, context)?;
                if ret != Return::None {
                    context.pop();
                    return Ok((value, ret))
                }
            }
            context.pop();
            Ok((None, Return::None))
        }
        Node::Node { head, args, pos } => {
            context.push();
            // get arguments
            let mut values: Vec<Value> = vec![];
            let mut types: Vec<Type> = vec![];
            let mut poses: Vec<Position> = vec![];
            for arg in args.iter() {
                if let (Some(value), _) = interpret(arg, context)? {
                    poses.push(arg.pos().clone());
                    types.push(value.typ());
                    values.push(value);
                } else {
                    context.trace_push(arg.pos());
                    return Err(Error::ExpectedArg)
                }
            }
            // try to get a function
            if let Node::Word { v, pos: word_pos } = head.as_ref() {
                match context.get_native_fn(v, &types) {
                    Some(func) => {
                        let mut func_context = Context::call(context, func.inline);
                        func_context.create_params(&func.params, values, poses, func.inline)?;
                        let res = (func.body)(&mut func_context)?;
                        context.after_call(func_context, func.inline);
                        context.pop();
                        return Ok(res)
                    }
                    None => match context.get_fn(v, &types) {
                        Some(func) => {
                            let mut func_context = Context::call(context, func.inline);
                            func_context.create_params(&func.params, values, poses, func.inline)?;
                            let res = interpret(&func.body, &mut func_context)?;
                            context.after_call(func_context, func.inline);
                            context.pop();
                            return Ok(res)
                        }
                        None => match context.get_var(v) {
                            Some(_) => {}
                            None => if context.fn_exists(v) || context.native_fn_exists(v) {
                                context.trace_push(word_pos);
                                return Err(Error::FunctionPatternNotFound(v.clone(), types))
                            } else {
                                context.trace_push(word_pos);
                                return Err(Error::NotDefined(v.clone()))
                            }
                        }
                    }
                }
            }
            // not a function
            if let Some(head_value) = interpret(head, context)?.0 {
                if types.len() == 0 {
                    context.pop();
                    return Ok((Some(head_value), Return::None))
                }
                match head_value {
                    Value::Type(typ) => match context.get_native_fn(&typ.to_string(), &types) {
                        Some(func) => {
                            let mut func_context = Context::call(context, func.inline);
                            func_context.create_params(&func.params, values, poses, func.inline)?;
                            let res = (func.body)(&mut func_context)?;
                            context.after_call(func_context, func.inline);
                            context.pop();
                            return Ok(res)
                        }
                        None => if context.fn_exists(&typ.to_string()) || context.native_fn_exists(&typ.to_string()) {
                            context.trace_push(pos);
                            Err(Error::InvalidCastBetween(typ.clone(), types[0].clone()))
                        } else {
                            context.trace_push(head.pos());
                            Err(Error::InvalidHeadCastType(typ.clone()))
                        }
                    }
                    Value::Vector(vec_values, typ) => if values.len() == 1 {
                        context.pop();
                        match &values[0] {
                            Value::Int(idx) => {
                                let idx = find_index(idx, vec_values.len());
                                match vec_values.get(idx) {
                                    Some(value) => Ok((Some(value.clone()), Return::None)),
                                    None => {
                                        context.trace_push(&poses[0]);
                                        Err(Error::IndexOutOfRange(idx, vec_values.len()))
                                    }
                                }
                            }
                            _ => {
                                context.trace_push(&poses[0]);
                                Err(Error::ExpectedTypes(vec![Type::Int], types[0].clone()))
                            }
                        }
                    } else {
                        context.trace_push(&poses[0]);
                        Err(Error::ValuePatternNotFound(Type::Vector(Some(Box::new(Type::Any))), types))
                    }
                    Value::String(string) => if values.len() == 1 {
                        context.pop();
                        if let Value::Int(idx) = &values[0] {
                            let idx = find_index(idx, string.len());
                            match string.get(idx .. idx + 1) {
                                Some(c) => Ok((Some(Value::Char(c.chars().collect::<Vec<char>>()[0])), Return::None)),
                                None => {
                                    context.trace_push(&poses[0]);
                                    Err(Error::IndexOutOfRange(idx, string.len()))
                                }
                            }
                        } else {
                            context.trace_push(&poses[0]);
                            Err(Error::ExpectedTypes(vec![Type::Int], types[0].clone()))
                        }
                    } else if values.len() == 2 {
                        context.pop();
                        if let Value::Int(idx1) = &values[0] {
                            if let Value::Int(idx2) = &values[1] {
                                if *idx1 < 0 {
                                    context.trace_push(&poses[0]);
                                    return Err(Error::IllegalNegativeIndex(*idx1))
                                }
                                if *idx2 < 0 {
                                    context.trace_push(&poses[1]);
                                    return Err(Error::IllegalNegativeIndex(*idx2))
                                }
                                if *idx1 as usize >= string.len() {
                                    context.trace_push(&poses[0]);
                                    return Err(Error::IndexOutOfRange(*idx1 as usize, string.len()))
                                }
                                if *idx2 as usize >= string.len() {
                                    context.trace_push(&poses[0]);
                                    return Err(Error::IndexOutOfRange(*idx2 as usize, string.len()))
                                }
                                Ok((Some(Value::String(string.get(*idx1 as usize .. *idx2 as usize).unwrap().to_string())), Return::None))
                            } else {
                                context.trace_push(&poses[0]);
                                Err(Error::ExpectedTypes(vec![Type::Int], types[0].clone()))
                            }
                        } else {
                            context.trace_push(&poses[0]);
                            Err(Error::ExpectedTypes(vec![Type::Int], types[0].clone()))
                        }
                    } else {
                        context.trace_push(&poses[0]);
                        Err(Error::ValuePatternNotFound(Type::Vector(Some(Box::new(Type::Any))), types))
                    }
                    Value::Function(func) => {
                        let mut func_context = Context::call(context, func.inline);
                        func_context.create_params(&func.params, values, poses, func.inline)?;
                        let res = interpret(&func.body, &mut func_context)?;
                        context.after_call(func_context, func.inline);
                        context.pop();
                        Ok(res)
                    }
                    Value::NativFunction(func) => {
                        let mut func_context = Context::call(context, func.inline);
                        func_context.create_params(&func.params, values, poses, func.inline)?;
                        let res = (func.body)(&mut func_context)?;
                        context.after_call(func_context, func.inline);
                        context.pop();
                        Ok(res)
                    }
                    _ => {
                        context.trace_push(head.pos());
                        Err(Error::InvalidHeadValue(head_value.clone()))
                    }
                }
            } else {
                context.trace_push(head.pos());
                Err(Error::Expected)
            }
        }
    }
}