use object::Env;

pub fn eval(node: &dyn ast::Node, env: &mut Env) -> Box<dyn object::Object> {
    let nd = node.as_any();
    if let Some(program) = nd.downcast_ref::<ast::Program>() {
        eval_program(&program.statements, env)
    }
    else if let Some(statement) = nd.downcast_ref::<ast::Statement>() {
        match statement {
            ast::Statement::Expr {
                token: _,
                expression,
            } => eval(expression, env),
            ast::Statement::Return { token: _, value } => {
                let value = eval(value, env);
                if is_error(&value) {
                    return value;
                }
                Box::new(object::ReturnValue { value })
            }
            ast::Statement::Let {
                token: _,
                name,
                value,
            } => {
                let value = eval(value, env);
                if is_error(&value) {
                    return value;
                }
                env.set(name.value.clone(), value.clone());
                // println!("==> {:?}", env.get(&name.value.clone()));
                value
            }
            _ => Box::new(object::Null {}), // TODO : how to handle?
        }
    }
    else if let Some(expression) = nd.downcast_ref::<ast::Expression>() {
        match expression {
            ast::Expression::IntegerLiteral { token: _, value } => {
                Box::new(object::Integer { value: *value })
            }
            ast::Expression::Bool { token: _, value } => Box::new(object::static_bool_obj(*value)),
            ast::Expression::Prefix {
                token: _,
                operator,
                right,
            } => {
                let right = eval(&**right, env);
                if is_error(&right) {
                    return right;
                }
                return eval_prefix_expression(operator, right);
            }
            ast::Expression::Infix {
                token: _,
                left,
                operator,
                right,
            } => {
                let left = eval(&**left, env);
                if is_error(&left) {
                    return left;
                }
                let right = eval(&**right, env);
                if is_error(&right) {
                    return right;
                }
                return eval_infix_expression(operator, left, right);
            }
            ast::Expression::If {
                token: _,
                condition,
                consequence,
                alternative,
            } => {
                let condition = &**condition;
                let condition = eval(condition, env);
                if is_error(&condition) {
                    return condition;
                }

                if is_true(&condition) {
                    eval_statements(consequence, env)
                }
                else {
                    eval_statements(alternative, env)
                }
            }
            ast::Expression::Ident(ast::Identifier { token: _, value }) => eval_ident(value, env),
            ast::Expression::FunctionLiteral {
                token: _,
                parameters,
                body,
            } => Box::new(object::Function {
                parameters: parameters.to_vec(),
                body:       body.to_vec(),
                env:        env.clone(), // todo: need to be ref / it is very high cost #issue 24
            }),
            ast::Expression::FunctionCall {
                token: _,
                func,
                args,
            } => {
                println!("==> {:?} {:?}", func, args);
                let func = eval(&**func, env);
                if is_error(&func) {
                    return func;
                }
                let func = Box::new(match func.as_any().downcast_ref::<object::Function>() {
                    Some(v) => v.clone(),
                    None => return new_error("Not a func".to_string()),
                });
                // above, more fancy? is there way?

                let args = eval_expressions(args, env);
                if args.len() == 1 && is_error(args.first().unwrap()) {
                    return args[0].clone(); // wanna change it to more safe
                }

                put_args_in_function(func, args)
            }
            _ => Box::new(object::Null {}), // TODO : how to handle?
        }
    }
    else {
        Box::new(object::Null {}) // TODO : how to handle?
    }
}

fn is_true(obj: &Box<dyn object::Object>) -> bool {
    match obj.object_type() {
        "Bool" => *obj.as_any().downcast_ref::<object::Bool>().unwrap().value,
        "Integer" => {
            let int_val = obj
                .as_any()
                .downcast_ref::<object::Integer>()
                .unwrap()
                .value;

            int_val != 0
        }
        "Null" => false,
        _ => true,
    }
}

fn is_error(obj: &Box<dyn object::Object>) -> bool {
    obj.object_type() == "Error"
}

// issue #20
// formatted string & variable argument using macro
// format_argument! may helpful
fn new_error(value: String) -> Box<object::Error> {
    Box::new(object::Error { value })
}

fn eval_program(stmts: &Vec<ast::Statement>, env: &mut Env) -> Box<dyn object::Object> {
    let mut rlt: Box<dyn object::Object> = Box::new(object::Null {});
    for stmt in stmts {
        rlt = eval(stmt, env);
        // println!("{} {}", stmt.to_string(), rlt.inspect());
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

fn eval_statements(stmts: &Vec<ast::Statement>, env: &mut Env) -> Box<dyn object::Object> {
    let mut rlt: Box<dyn object::Object> = Box::new(object::Null {});
    for stmt in stmts {
        rlt = eval(stmt, env);
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

fn eval_prefix_expression(
    operator: &str,
    right: Box<dyn object::Object>,
) -> Box<dyn object::Object> {
    match operator {
        "!" => eval_prefix_bang_expression(right),
        "-" => eval_prefix_minus_expression(right),
        _ => new_error("Never Occur".to_owned()),
    }
}

fn eval_prefix_bang_expression(right: Box<dyn object::Object>) -> Box<dyn object::Object> {
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

fn eval_prefix_minus_expression(right: Box<dyn object::Object>) -> Box<dyn object::Object> {
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
            return new_error(s);
        }
    };
}

fn eval_infix_expression(
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
                eval_integer_infix_expression(operator, left, right)
            }
            else {
                let s = format!(
                    "Type Mismatched: {} {} {}",
                    &left.object_type(),
                    operator,
                    &right.object_type()
                );
                new_error(s)
            }
        }
        "==" | "!=" => {
            if left.object_type() == "Integer" && right.object_type() == "Integer" {
                let &left = left.as_any().downcast_ref::<object::Integer>().unwrap();
                let left = Box::new(left);
                let &right = right.as_any().downcast_ref::<object::Integer>().unwrap();
                let right = Box::new(right);
                eval_integer_infix_expression(operator, left, right)
            }
            else if left.object_type() == "Bool" && right.object_type() == "Bool" {
                let &left = left.as_any().downcast_ref::<object::Bool>().unwrap();
                let left = Box::new(left);
                let &right = right.as_any().downcast_ref::<object::Bool>().unwrap();
                let right = Box::new(right);
                eval_bool_infix_expression(operator, left, right)
            }
            else {
                let s = format!(
                    "Type Mismatched: {} {} {}",
                    &left.object_type(),
                    operator,
                    &right.object_type()
                );
                new_error(s)
            }
        }
        _ => new_error("Never Occur".to_owned()),
    }
}

fn eval_integer_infix_expression(
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
        _ => new_error("Never Occur".to_owned()),
    }
}

fn eval_bool_infix_expression(
    operator: &str,
    left: Box<object::Bool>,
    right: Box<object::Bool>,
) -> Box<dyn object::Object> {
    match operator {
        "==" => Box::new(object::static_bool_obj(left.value == right.value)),
        "!=" => Box::new(object::static_bool_obj(left.value != right.value)),
        _ => new_error("Never Occur".to_owned()),
    }
}

fn eval_ident(name: &String, env: &Env) -> Box<dyn object::Object> {
    match env.get(name) {
        Some(v) => v.clone(),
        None => new_error(format!("Ident not found: {}", name)),
    }
}

fn eval_expressions(
    expressions: &Vec<ast::Expression>,
    env: &mut Env,
) -> Vec<Box<dyn object::Object>> {
    let mut v = vec![];
    for exp in expressions {
        let e = eval(exp, env);
        if is_error(&e) {
            return vec![e];
        }
        v.push(e);
    }
    v
}

fn put_args_in_function(
    func: Box<object::Function>,
    args: Vec<Box<dyn object::Object>>,
) -> Box<dyn object::Object> {
    let mut closure = make_func_env(&func, args);
    eval_program(&func.body, &mut closure)
}

fn make_func_env(func: &Box<object::Function>, args: Vec<Box<dyn object::Object>>) -> Env {
    let mut closure = Env::wrap_env(Box::new(func.env.clone())); // it is also 'clone'
    for (param, arg) in func.parameters.iter().zip(args) {
        closure.set(param.value.clone(), arg)
    }
    closure
}
