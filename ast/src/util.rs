// use super::{Expression, Program, Statement};

// impl Expression {
//     pub fn to_string(&self) -> String {
//         match self {
//             Self::Identifier(ident) => format!("{}", ident),
//             Self::Integer(i) => format!("{}", i),
//             Self::Prefix {
//                 token: _,
//                 operator,
//                 right,
//             } => format!("({}{})", operator, right.to_string()),
//             Self::Infix {
//                 token: _,
//                 left,
//                 operator,
//                 right,
//             } => format!("({} {} {})", left.to_string(), operator, right.to_string()),
//             Self::Bool { token: _, value } => format!("{}", value),
//             Self::If {
//                 token: _,
//                 condition,
//                 consequence,
//                 alternative,
//             } => {
//                 if !alternative.is_empty() {
//                     format!(
//                         "if({}) {{ {} }} else {{ {} }}",
//                         condition.to_string(),
//                         consequence.iter().fold("".to_owned(), |acc, a| acc
//                             + &" ".to_owned()
//                             + &a.to_string())[1..]
//                             .to_owned(),
//                         alternative.iter().fold("".to_owned(), |acc, a| acc
//                             + &" ".to_owned()
//                             + &a.to_string())[1..]
//                             .to_owned()
//                     )
//                 }
//                 else {
//                     format!(
//                         "if({}) {{ {} }}",
//                         condition.to_string(),
//                         consequence.iter().fold("".to_owned(), |acc, a| acc
//                             + &" ".to_owned()
//                             + &a.to_string())[1..]
//                             .to_owned()
//                     )
//                 }
//             }
//             Self::FunctionLiteral {
//                 token: _,
//                 parameters,
//                 body,
//             } => {
//                 format!(
//                     "fn({}) {{ {} }}",
//                     parameters
//                         .iter()
//                         .fold("".to_owned(), |acc, a| acc + &", ".to_owned() + &a.value)[2..]
//                         .to_owned(),
//                     body.iter().fold("".to_owned(), |acc, a| acc
//                         + &", ".to_owned()
//                         + &a.to_string())[2..]
//                         .to_owned()
//                 )
//             }
//             Self::FunctionCall {
//                 token: _,
//                 func,
//                 args,
//             } => {
//                 format!(
//                     "{}({})",
//                     func.to_string(),
//                     args.iter().fold("".to_owned(), |acc, a| acc
//                         + &", ".to_owned()
//                         + &a.to_string())[2..]
//                         .to_owned()
//                 )
//             }
//             _ => format!("Undefined"),
//         }
//     }
// }

// impl Statement {
//     pub fn to_string(&self) -> String {
//         match self {
//             Self::Let {
//                 token: _,
//                 name,
//                 value,
//             } => format!("let {} = {};", name.value, value.to_string()),
//             Self::Return { token: _, value } => format!("return {};", value.to_string()),
//             Self::Expr {
//                 token: _,
//                 expression,
//             } => format!("{}", expression.to_string()),
//         }
//     }
// }

// impl Program {
//     pub fn is_empty(&self) -> bool {
//         self.statements.is_empty()
//     }
// }
