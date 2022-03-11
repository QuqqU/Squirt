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
