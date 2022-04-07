use std::{cell::RefCell, rc::Rc};

use object::Env;
use parser::Parser;

mod unit_eval;
mod util;

pub struct Eval {
    env: Rc<RefCell<Env>>,
}

impl Eval {
    pub fn new() -> Self {
        Self { env: Env::new() }
    }

    pub fn run(&self, input: String) -> String {
        let program = Parser::parse(input);
        if program.is_empty() {
            return String::new();
        }
        let e = Eval::_eval(&program, &self.env);
        e.inspect()
    }

    fn _eval(node: &dyn ast::Node, env: &Rc<RefCell<Env>>) -> Box<dyn object::Object> {
        let nd = node.as_any();
        if let Some(program) = nd.downcast_ref::<ast::Program>() {
            Eval::eval_program(&program.statements, env)
        }
        else if let Some(statement) = nd.downcast_ref::<ast::Statement>() {
            match statement {
                ast::Statement::Expr {
                    token: _,
                    expression,
                } => Eval::_eval(expression, env),
                ast::Statement::Return { token: _, value } => {
                    let value = Eval::_eval(value, env);
                    if Eval::is_error(&value) {
                        return value;
                    }
                    Box::new(object::ReturnValue { value })
                }
                ast::Statement::Let {
                    token: _,
                    name,
                    value,
                } => {
                    let value = Eval::_eval(value, env);
                    if Eval::is_error(&value) {
                        return value;
                    }
                    env.borrow_mut().set(name.value.clone(), value.clone());
                    // println!("==> {:?}", env.get(&name.value.clone()));
                    Box::new(object::NULL) // remove output in let stmt
                }
            }
        }
        else if let Some(expression) = nd.downcast_ref::<ast::Expression>() {
            match expression {
                ast::Expression::IntegerLiteral { token: _, value } => {
                    Box::new(object::Integer { value: *value })
                }
                ast::Expression::Bool { token: _, value } => {
                    Box::new(object::static_bool_obj(*value))
                }
                ast::Expression::Prefix {
                    token: _,
                    operator,
                    right,
                } => {
                    let right = Eval::_eval(&**right, env);
                    if Eval::is_error(&right) {
                        return right;
                    }
                    return Eval::eval_prefix_expression(operator, right);
                }
                ast::Expression::Infix {
                    token: _,
                    left,
                    operator,
                    right,
                } => {
                    let _left = Eval::_eval(&**left, env);
                    if Eval::is_error(&_left) {
                        return _left;
                    }
                    let right = Eval::_eval(&**right, env);
                    if Eval::is_error(&right) {
                        return right;
                    }

                    if operator == "=" {
                        let name = match &**left {
                            ast::Expression::Ident(ast::Identifier { token: _, value }) => value,
                            _ => {
                                return Eval::new_error(
                                    "Cannot Assign to non-identifier".to_string(),
                                )
                            }
                        };
                        env.borrow_mut().set(name.to_owned(), right.clone());
                        // println!("==> {:?}", env.get(&name.value.clone()));
                        return right;
                    }
                    return Eval::eval_infix_expression(operator, _left, right);
                }
                ast::Expression::If {
                    token: _,
                    condition,
                    consequence,
                    alternative,
                } => {
                    let condition = &**condition;
                    let condition = Eval::_eval(condition, env);
                    if Eval::is_error(&condition) {
                        return condition;
                    }

                    if Eval::is_true(&condition) {
                        Eval::eval_statements(consequence, env)
                    }
                    else {
                        Eval::eval_statements(alternative, env)
                    }
                }
                ast::Expression::Ident(ast::Identifier { token: _, value }) => {
                    Eval::eval_ident(value, env)
                }
                ast::Expression::FunctionLiteral {
                    token: _,
                    parameters,
                    body,
                } => Box::new(object::Function {
                    parameters: parameters.to_vec(),
                    body:       body.to_vec(),
                    env:        Rc::clone(env), // todo: need to be ref / it is very high cost #issue 24
                }),
                ast::Expression::FunctionCall {
                    token: _,
                    func,
                    args,
                } => {
                    let func = Eval::_eval(&**func, env);

                    if Eval::is_error(&func) {
                        return func;
                    }
                    let func = Box::new(match func.as_any().downcast_ref::<object::Function>() {
                        Some(v) => v.clone(),
                        None => return Eval::new_error("Not a func".to_string()),
                    });
                    // above, more fancy? is there way?

                    let args = Eval::eval_expressions(args, env);
                    if args.len() == 1 && Eval::is_error(args.first().unwrap()) {
                        return args[0].clone(); // wanna change it to more safe
                    }

                    Eval::put_args_in_function(func, args)
                }
                _ => Box::new(object::Null {}), // TODO : how to handle?
            }
        }
        else {
            Box::new(object::Null {}) // TODO : how to handle?
        }
    }
}
