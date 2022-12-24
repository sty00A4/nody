use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Return { None, Return }
pub fn interpret(node: &Node, context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    match node {
        Node::Int { v, pos } => Ok((Some(Value::Int(*v)), Return::None)),
        Node::Float { v, pos } => Ok((Some(Value::Float(*v)), Return::None)),
        Node::Bool { v, pos } => Ok((Some(Value::Bool(*v)), Return::None)),
        Node::String { v, pos } => Ok((Some(Value::String(v.clone())), Return::None)),
        Node::Type { v, pos } => Ok((Some(Value::Type(v.clone())), Return::None)),
        Node::Word { v, pos } => match context.get_var(v) {
            Some(v) => Ok((Some(v.clone()), Return::None)),
            None => match context.get_fn_first(v) {
                Some(func) => Ok((Some(Value::Function(func.clone())), Return::None)),
                None => match context.get_nativ_fn_first(v) {
                    Some(func) => Ok((Some(Value::NativFunction(func.clone())), Return::None)),
                    None => Err(Error::NotDefined(v.clone()))
                }
            }
        }
        Node::Body { nodes, pos } => {
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
        Node::Node { head, args, pos } => if let Some(head) = interpret(head, context)?.0 {
            match head {
                Value::Function(_) => {
                    todo!("call function");
                    Ok((None, Return::None))
                }
                Value::NativFunction(_) => {
                    todo!("call nativ-function");
                    Ok((None, Return::None))
                }
                _ => Err(Error::ExpectedTypes(
                    vec![Type::Function(vec![], None), Type::NativFunction(vec![], None)],
                    head.typ()
                ))
            }
        } else {
            Err(Error::Expected)
        }
        _ => todo!("{node}")
    }
}