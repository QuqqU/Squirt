use ast;
use object;

pub fn eval(node: &dyn ast::Node) -> Box<dyn object::Object> {
    let nd = node.as_any();
    if let Some(program) = nd.downcast_ref::<ast::Program>() {
        eval_statements(&program.statements)
    }
    else if let Some(statement) = nd.downcast_ref::<ast::Statement>() {
        match statement {
            ast::Statement::Expr {
                token: _,
                expression,
            } => eval(expression),
            _ => Box::new(object::Null {}),
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
                let right = eval(&**right);
                return eval_prefix_expression(operator, right);
            }
            ast::Expression::Infix {
                token: _,
                left,
                operator,
                right,
            } => {
                let left = eval(&**left);
                let right = eval(&**right);
                return eval_infix_expression(operator, left, right);
            }
            ast::Expression::If {
                token: _,
                condition,
                consequence,
                alternative,
            } => {
                let condition = &**condition;
                let condition = eval(condition);

                if is_true(condition) {
                    eval_statements(consequence)
                }
                else {
                    eval_statements(alternative)
                }
            }
            _ => Box::new(object::Null {}),
        }
    }
    else {
        Box::new(object::Null {})
    }
}

fn is_true(obj: Box<dyn object::Object>) -> bool {
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

fn eval_statements(stmts: &Vec<ast::Statement>) -> Box<dyn object::Object> {
    let mut rlt: Box<dyn object::Object> = Box::new(object::Null {});
    for stmt in stmts {
        rlt = eval(stmt);
        // println!("{} {}", stmt.to_string(), rlt.inspect());
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
        _ => Box::new(object::Null {}),
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
        _ => return Box::new(object::NULL),
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
                Box::new(object::Null {})
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
                Box::new(object::Null {})
            }
        }
        _ => Box::new(object::Null {}),
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
        _ => Box::new(object::Null {}),
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
        _ => Box::new(object::Null {}),
    }
}
