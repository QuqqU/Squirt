use std::{cell::RefCell, rc::Rc};

use super::Eval;
use object::Env;

impl Eval {
    pub(super) fn eval_program(
        stmts: &Vec<ast::Statement>,
        env: &Rc<RefCell<Env>>,
    ) -> Box<dyn object::Object> {
        let mut rlt: Box<dyn object::Object> = Box::new(object::Null {});
        for stmt in stmts {
            rlt = Eval::_eval(stmt, env);
            if rlt.object_type() == "ReturnValue" {
                let rlt = rlt
                    .as_any()
                    .downcast_ref::<object::ReturnValue>()
                    .unwrap()
                    .value
                    .clone();
                return rlt;
            }
            else if rlt.object_type() == "Error" {
                return rlt;
            }
        }
        rlt
    }

    pub(super) fn eval_statements(
        stmts: &Vec<ast::Statement>,
        env: &Rc<RefCell<Env>>,
    ) -> Box<dyn object::Object> {
        let mut rlt: Box<dyn object::Object> = Box::new(object::Null {});
        for stmt in stmts {
            rlt = Eval::_eval(stmt, env);
            // println!("{} {}", stmt.to_string(), rlt.inspect());
            if rlt.object_type() == "ReturnValue" {
                return rlt;
            }
            else if rlt.object_type() == "Error" {
                return rlt;
            }
        }
        rlt
    }

    pub(super) fn eval_expressions(
        expressions: &Vec<ast::Expression>,
        env: &Rc<RefCell<Env>>,
    ) -> Vec<Box<dyn object::Object>> {
        let mut v = vec![];
        for exp in expressions {
            let e = Eval::_eval(exp, env);
            if Eval::is_error(&e) {
                return vec![e];
            }
            v.push(e);
        }
        v
    }
}

impl Eval {
    pub(super) fn eval_prefix_expression(
        operator: &str,
        right: Box<dyn object::Object>,
    ) -> Box<dyn object::Object> {
        match operator {
            "!" => Eval::eval_prefix_bang_expression(right),
            "-" => Eval::eval_prefix_minus_expression(right),
            _ => Eval::new_error("Never Occur".to_owned()),
        }
    }

    pub(super) fn eval_prefix_bang_expression(
        right: Box<dyn object::Object>,
    ) -> Box<dyn object::Object> {
        Box::new(match right.object_type() {
            "Bool" => {
                let right = right.as_any().downcast_ref::<object::Bool>().unwrap().value;
                object::static_bool_obj(!right)
            }
            "Integer" => {
                let right = right
                    .as_any()
                    .downcast_ref::<object::Integer>()
                    .unwrap()
                    .value;
                object::static_bool_obj(right == 0)
            }
            "Null" => object::TRUE,
            _ => object::FALSE,
        })
    }

    pub(super) fn eval_prefix_minus_expression(
        right: Box<dyn object::Object>,
    ) -> Box<dyn object::Object> {
        match right.object_type() {
            "Integer" => {
                let right = right
                    .as_any()
                    .downcast_ref::<object::Integer>()
                    .unwrap()
                    .value;
                return Box::new(object::Integer { value: -right });
            }
            _ => {
                let s = format!("Unknown Operator: -{}", &right.object_type());
                return Eval::new_error(s);
            }
        };
    }

    pub(super) fn eval_infix_expression(
        operator: &str,
        left: Box<dyn object::Object>,
        right: Box<dyn object::Object>,
    ) -> Box<dyn object::Object> {
        match operator {
            "+" | "-" | "*" | "/" | "<" | ">" => {
                if left.object_type() == "Integer" && right.object_type() == "Integer" {
                    let &left = left.as_any().downcast_ref::<object::Integer>().unwrap();
                    let left = Box::new(left);
                    let &right = right.as_any().downcast_ref::<object::Integer>().unwrap();
                    let right = Box::new(right);
                    Eval::eval_integer_infix_expression(operator, left, right)
                }
                else {
                    let s = format!(
                        "Type Mismatched: {} {} {}",
                        &left.object_type(),
                        operator,
                        &right.object_type()
                    );
                    Eval::new_error(s)
                }
            }
            "==" | "!=" => {
                if left.object_type() == "Integer" && right.object_type() == "Integer" {
                    let &left = left.as_any().downcast_ref::<object::Integer>().unwrap();
                    let left = Box::new(left);
                    let &right = right.as_any().downcast_ref::<object::Integer>().unwrap();
                    let right = Box::new(right);
                    Eval::eval_integer_infix_expression(operator, left, right)
                }
                else if left.object_type() == "Bool" && right.object_type() == "Bool" {
                    let &left = left.as_any().downcast_ref::<object::Bool>().unwrap();
                    let left = Box::new(left);
                    let &right = right.as_any().downcast_ref::<object::Bool>().unwrap();
                    let right = Box::new(right);
                    Eval::eval_bool_infix_expression(operator, left, right)
                }
                else {
                    let s = format!(
                        "Type Mismatched: {} {} {}",
                        &left.object_type(),
                        operator,
                        &right.object_type()
                    );
                    Eval::new_error(s)
                }
            }
            _ => Eval::new_error("Never Occur".to_owned()),
        }
    }

    pub(super) fn eval_integer_infix_expression(
        operator: &str,
        left: Box<object::Integer>,
        right: Box<object::Integer>,
    ) -> Box<dyn object::Object> {
        match operator {
            "+" => Box::new(object::Integer {
                value: left.value + right.value,
            }),
            "-" => Box::new(object::Integer {
                value: left.value - right.value,
            }),
            "*" => Box::new(object::Integer {
                value: left.value * right.value,
            }),
            "/" => Box::new(object::Integer {
                value: left.value / right.value,
            }),
            "<" => Box::new(object::static_bool_obj(left.value < right.value)),
            ">" => Box::new(object::static_bool_obj(left.value > right.value)),
            "==" => Box::new(object::static_bool_obj(left.value == right.value)),
            "!=" => Box::new(object::static_bool_obj(left.value != right.value)),
            _ => Eval::new_error("Never Occur".to_owned()),
        }
    }

    pub(super) fn eval_bool_infix_expression(
        operator: &str,
        left: Box<object::Bool>,
        right: Box<object::Bool>,
    ) -> Box<dyn object::Object> {
        match operator {
            "==" => Box::new(object::static_bool_obj(left.value == right.value)),
            "!=" => Box::new(object::static_bool_obj(left.value != right.value)),
            _ => Eval::new_error("Never Occur".to_owned()),
        }
    }

    pub(super) fn eval_ident(name: &String, env: &Rc<RefCell<Env>>) -> Box<dyn object::Object> {
        match env.borrow().get(name) {
            Some(v) => v.clone(),
            None => Eval::new_error(format!("Ident not found: {}", name)),
        }
    }

    pub(super) fn put_args_in_function(
        func: Box<object::Function>,
        args: Vec<Box<dyn object::Object>>,
    ) -> Box<dyn object::Object> {
        let closure = Eval::make_func_env(&func, args);
        Eval::eval_program(&func.body, &closure)
    }

    fn make_func_env(
        func: &Box<object::Function>,
        args: Vec<Box<dyn object::Object>>,
    ) -> Rc<RefCell<Env>> {
        let mut closure = Env::wrap_env(func.env.clone()); // it is also 'clone'
        for (i, param) in func.parameters.iter().enumerate() {
            closure.set(param.value.clone(), args[i].clone())
        }
        Rc::new(RefCell::new(closure))
    }
}
