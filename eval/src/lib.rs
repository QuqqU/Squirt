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
            _ => Box::new(object::Null {}),
        }
    }
    else {
        Box::new(object::Null {})
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
